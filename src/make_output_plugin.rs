use crate::{
    err_or_ignore, msg, Cfg, Creature, Item, LastList, Log, MsgTone, PluginInfo, PluginName,
    ResponsiblePlugins, Subrecord,
};
use anyhow::{anyhow, Result};
use std::{
    cmp::max,
    collections::{hash_map::Entry, HashMap},
    path::Path,
};
use tes3::esp::{
    FileType, FixedString, Header, LeveledCreature, LeveledItem, ObjectFlags, Plugin, TES3Object,
};

#[derive(Default)]
pub(crate) struct ListCounts {
    pub(crate) total: usize,
    pub(crate) unique: usize,
    pub(crate) merged: usize,
    pub(crate) placed: usize,
    pub(crate) untouched: usize,
    pub(crate) master: usize,
    pub(crate) deleted_subrecord: usize,
}

struct Helper<'a> {
    cfg: &'a Cfg,
    counts: ListCounts,
    masters_wordy: HashMap<PluginName<'a>, (usize, PluginName<'a>, u64)>,
    threshold_resolved: ThresholdMessages<'a>,
    threshold_skipped: ThresholdMessages<'a>,
    threshold_warnings: ThresholdMessages<'a>,
    untouched_lists: Vec<UntouchedList<'a>>,
    deleted_subrecords: Vec<DeletedSubrecords<'a>>,
}

impl<'a> Helper<'a> {
    fn new(cfg: &'a Cfg) -> Helper<'a> {
        Helper {
            cfg,
            counts: ListCounts::default(),
            masters_wordy: HashMap::new(),
            threshold_resolved: ThresholdMessages::new(ThresholdMessageKind::Resolved),
            threshold_skipped: ThresholdMessages::new(ThresholdMessageKind::Skipped),
            threshold_warnings: ThresholdMessages::new(ThresholdMessageKind::Warning),
            untouched_lists: Vec::new(),
            deleted_subrecords: Vec::new(),
        }
    }
}

struct ThresholdMessages<'a> {
    kind: ThresholdMessageKind,
    messages: Vec<ThresholdMessageRaw<'a>>,
}

impl<'a> ThresholdMessages<'a> {
    fn new(kind: ThresholdMessageKind) -> ThresholdMessages<'a> {
        ThresholdMessages {
            kind,
            messages: Vec::new(),
        }
    }

    fn push(
        &mut self,
        ratio: f64,
        threshold: f64,
        log_t: &'a str,
        id: String,
        initial_plugin: PluginName<'a>,
        delete: &[(Subrecord, Subrecord, ResponsiblePlugins<'a>)],
    ) {
        let mut plugins = Vec::new();
        for (_, _, responsible_plugins) in delete {
            for responsible_plugin in responsible_plugins.iter() {
                if !plugins.contains(&responsible_plugin) {
                    plugins.push(responsible_plugin);
                }
            }
        }
        self.messages.push(ThresholdMessageRaw {
            ratio,
            threshold,
            log_t,
            id,
            initial_plugin,
            responsible_plugins_str: plugins.into_iter().map(|x| x.as_str()).collect(),
        });
    }

    fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

enum ThresholdMessageKind {
    Resolved,
    Skipped,
    Warning,
}

struct ThresholdMessageRaw<'a> {
    ratio: f64,
    threshold: f64,
    log_t: &'a str,
    id: String,
    initial_plugin: PluginName<'a>,
    responsible_plugins_str: ResponsiblePluginsStr<'a>,
}

struct DeletedSubrecords<'a> {
    log_t: &'a str,
    id: String,
    initial_plugin: PluginName<'a>,
    subrecords: Vec<(Subrecord, ResponsiblePluginsStr<'a>)>,
}

struct UntouchedList<'a> {
    log_t: &'a str,
    id: String,
    initial_plugin: &'a str,
    last_plugin: &'a str,
}

type ResponsiblePluginsStr<'a> = Vec<&'a str>;

macro_rules! make_lists {
    ($output_plugin:ident, $name:ident, $helper:ident, $tes3_kind:ident, $log:ident) => {
        for mut o in $name.into_iter() {
            $helper.counts.total += o.count;
            $helper.counts.unique += 1;
            if o.count > 1 || $helper.cfg.all_lists {
                let mut $name = o.list;

                if o.count > 1 {
                    if !$helper.cfg.no_delete && !o.delete.is_empty() {
                        delete_subrecords(
                            &mut $name,
                            &mut o.list_lowercased,
                            &o.id,
                            o.first,
                            o.delete,
                            $helper.cfg.$name.threshold,
                            &$helper.cfg.$name.log_t,
                            &o.masters[0],
                            &mut $helper,
                        )?;
                    }
                    $name.sort_by(|(name1, _), (name2, _)| {
                        name1.to_lowercase().cmp(&name2.to_lowercase())
                    });
                    $name.sort_by_key(|level| level.1);
                }

                if $helper.cfg.all_lists
                    || merged_and_last_differ(
                        &o.flags,
                        &o.list_flags,
                        &o.chance_nones,
                        o.list_lowercased,
                        &mut o.last,
                    )
                {
                    append_masters(
                        o.masters,
                        &mut $helper.masters_wordy,
                        &mut $helper.counts.master,
                        $helper.cfg,
                        $log,
                    )?;
                    $output_plugin.push(TES3Object::$tes3_kind($tes3_kind {
                        flags: o.flags.last().unwrap().clone(),
                        id: o.id,
                        list_flags: *o.list_flags.last().unwrap(),
                        chance_none: *o.chance_nones.last().unwrap(),
                        $name,
                    }));
                    if o.count > 1 {
                        $helper.counts.merged += 1;
                    }
                    $helper.counts.placed += 1;
                } else {
                    $helper.untouched_lists.push(UntouchedList {
                        log_t: &$helper.cfg.$name.log_t,
                        id: o.id,
                        initial_plugin: &o.masters[0].name,
                        last_plugin: &o.last_plugin_name.unwrap(),
                    });
                    $helper.counts.untouched += 1;
                    if o.count > 1 {
                        $helper.counts.merged += 1;
                    }
                }
            }
        }
    };
}

pub(crate) fn make_output_plugin(
    creatures: Vec<Creature>,
    items: Vec<Item>,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<(Plugin, ListCounts, bool)> {
    let mut warning = false;
    let mut output_plugin = Plugin::new();
    let mut helper = Helper::new(cfg);
    let mut lists = Vec::new();
    if !cfg.creatures.no {
        make_lists!(lists, creatures, helper, LeveledCreature, log);
    }
    if !cfg.items.no {
        make_lists!(lists, items, helper, LeveledItem, log);
    }
    if !helper.deleted_subrecords.is_empty() {
        show_deleted_subrecords(
            &helper.deleted_subrecords,
            helper.counts.deleted_subrecord,
            cfg,
            log,
        )?;
    }
    if !helper.untouched_lists.is_empty() {
        show_untouched_lists(&helper.untouched_lists, cfg, log)?;
    }
    if !helper.threshold_resolved.is_empty() {
        show_threshold_messages(&helper.threshold_resolved, cfg, log)?;
    }
    if !helper.threshold_skipped.is_empty() {
        show_threshold_messages(&helper.threshold_skipped, cfg, log)?;
    }
    if !helper.threshold_warnings.is_empty() {
        show_threshold_messages(&helper.threshold_warnings, cfg, log)?;
        warning = true;
    }
    let masters = make_masters(&helper);
    let header = make_header(&helper, masters);
    output_plugin.objects.push(TES3Object::Header(header));
    output_plugin.objects.extend(lists);
    Ok((output_plugin, helper.counts, warning))
}

#[allow(clippy::too_many_arguments)]
fn delete_subrecords<'a>(
    list: &mut Vec<Subrecord>,
    list_lowercased: &mut Vec<Subrecord>,
    id: &String,
    first: Vec<(Subrecord, Subrecord)>,
    delete: Vec<(Subrecord, Subrecord, ResponsiblePlugins<'a>)>,
    threshold: f64,
    log_t: &'a str,
    plugin_info: &'a PluginInfo,
    helper: &mut Helper<'a>,
) -> Result<()> {
    if !helper.cfg.extended_delete
        && !helper
            .cfg
            .always_delete
            .contains(&plugin_info.name_lowercased)
    {
        return Ok(());
    };
    let ratio = 100.0 * delete.len() as f64 / first.len() as f64;
    if helper.cfg.extended_delete
        && ratio > threshold
        && ratio >= helper.cfg.guts.auto_resolve_lower_limit
        && !helper
            .cfg
            .always_delete
            .contains(&plugin_info.name_lowercased)
    {
        helper.threshold_resolved.push(
            ratio,
            threshold,
            log_t,
            id.to_owned(),
            &plugin_info.name,
            &delete,
        );
    } else {
        if helper.cfg.extended_delete && ratio > threshold {
            if helper
                .cfg
                .always_delete
                .contains(&plugin_info.name_lowercased)
            {
                helper.threshold_skipped.push(
                    ratio,
                    threshold,
                    log_t,
                    id.to_owned(),
                    &plugin_info.name,
                    &delete,
                );
            } else {
                helper.threshold_warnings.push(
                    ratio,
                    threshold,
                    log_t,
                    id.to_owned(),
                    &plugin_info.name,
                    &delete,
                );
            }
        }
        let mut subrecords = Vec::new();
        for (subrecord_lowercased, subrecord, responsible_plugins) in delete.into_iter() {
            let index = match list_lowercased
                .iter()
                .position(|x| x == &subrecord_lowercased)
            {
                Some(index) => index,
                None => {
                    return Err(anyhow!("Failed to delete subrecord. This error should've never happened. List id: {}, subrecord id: {}, initial plugin: {}, responsible plugin: {}", id, subrecord.0, &plugin_info.name, responsible_plugins.into_iter().map(|x| x.as_str()).collect::<Vec<_>>().join(", ")));
                }
            };

            list_lowercased.swap_remove(index);
            list.swap_remove(index);
            subrecords.push((
                subrecord,
                responsible_plugins
                    .into_iter()
                    .map(|x| x.as_str())
                    .collect(),
            ));
            helper.counts.deleted_subrecord += 1;
        }
        helper.deleted_subrecords.push(DeletedSubrecords {
            log_t,
            id: id.to_owned(),
            initial_plugin: &plugin_info.name,
            subrecords,
        });
    }
    Ok(())
}

fn append_masters<'a>(
    masters_src: Vec<&'a PluginInfo>,
    masters_dst: &mut HashMap<PluginName<'a>, (usize, PluginName<'a>, u64)>,
    count: &mut usize,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    for plugin_info in masters_src {
        if let Entry::Vacant(v) = masters_dst.entry(&plugin_info.name) {
            let size = match Path::new(&plugin_info.path).metadata() {
                Ok(meta) => meta.len(),
                Err(error) => {
                    let text = format!(
                        "Failed to get the size of \"{}\" with error \"{}\"",
                        plugin_info.path.display(),
                        error
                    );
                    err_or_ignore(text, cfg, log)?;
                    0
                }
            };
            v.insert((*count, &plugin_info.name, size));
            *count += 1;
        }
    }
    Ok(())
}

fn make_header(helper: &Helper, masters: Vec<(String, u64)>) -> Header {
    Header {
        flags: ObjectFlags::default(),
        version: helper.cfg.guts.header_version,
        file_type: FileType::Esp,
        author: FixedString(String::from(&helper.cfg.guts.header_author)),
        description: FixedString(String::from(&helper.cfg.guts.header_description)),
        num_objects: helper.counts.placed as u32,
        masters,
    }
}

fn make_masters(helper: &Helper) -> Vec<(String, u64)> {
    let mut masters_sorted: Vec<(usize, &String, u64)> =
        helper.masters_wordy.values().cloned().collect();
    masters_sorted.sort();
    masters_sorted
        .into_iter()
        .map(|(_, b, c)| (b.to_owned(), c))
        .collect()
}

fn show_deleted_subrecords(
    list: &[DeletedSubrecords],
    subrecords_count: usize,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    let list_len = list.len();
    let mut text = format!(
        "{} subrecord{} from {} leveled list{} {} deleted",
        subrecords_count,
        if subrecords_count == 1 { "" } else { "s" },
        list_len,
        if list_len == 1 { "" } else { "s" },
        if subrecords_count == 1 { "was" } else { "were" },
    );
    text += if cfg.verbose > 0 {
        ":"
    } else {
        match cfg.no_log {
            true => &cfg.guts.suffix_add_v_suggestion_no_log,
            false => &cfg.guts.suffix_add_v_suggestion,
        }
    };
    msg(&text, MsgTone::Good, 0, cfg, log)?;
    text.clear();
    text = format!(
        "\n\t{:>2} {:<32} {:<1} {:<32} {:<16} {}\n",
        "LV", "DELETED SUBRECORD", "T", "LEVELED LIST", "INITIAL PLUGIN", "RESPONSIBLE PLUGINS"
    );
    for list_item in list.iter() {
        for subrecord in list_item.subrecords.iter() {
            text.push_str(&format!(
                "\t{:>2} {:<32} {:<1} {:<32} {:<16} {}\n",
                subrecord.0 .1,
                subrecord.0 .0,
                list_item.log_t,
                list_item.id,
                list_item.initial_plugin,
                subrecord.1.join(", "),
            ));
        }
    }
    msg(text, MsgTone::Neutral, 1, cfg, log)?;
    Ok(())
}

fn show_untouched_lists(list: &[UntouchedList], cfg: &Cfg, log: &mut Log) -> Result<()> {
    let list_len = list.len();
    let mut text = format!(
        "{} merged leveled list{} identical to last loaded list{} hence not placed into the output plugin",
        list_len,
        if list_len == 1 { " was" } else { "s were" },
        if list_len == 1 { "" } else { "s" }
    );
    text += if cfg.verbose > 0 {
        ":"
    } else {
        match cfg.no_log {
            true => &cfg.guts.suffix_add_2v_suggestion_no_log,
            false => &cfg.guts.suffix_add_2v_suggestion,
        }
    };
    msg(&text, MsgTone::Good, 0, cfg, log)?;
    text.clear();
    text = format!(
        "\n\t{:<1} {:<32} {:<32} {}\n",
        "T", "LEVELED LIST", "INITIAL PLUGIN", "LAST PLUGIN"
    );
    for list_item in list.iter() {
        text.push_str(&format!(
            "\t{:<1} {:<32} {:<32} {}\n",
            list_item.log_t, list_item.id, list_item.initial_plugin, list_item.last_plugin
        ));
    }
    msg(text, MsgTone::Neutral, 2, cfg, log)?;
    Ok(())
}

fn show_threshold_messages(list: &ThresholdMessages, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let list_len = list.messages.len();
    let warnings_off = if cfg.no_threshold_warnings { 99 } else { 0 };
    let mut text: String;
    match list.kind {
        ThresholdMessageKind::Resolved => {
            text = format!(
                "{} leveled list{} automatically excluded from subrecord deletion mode",
                list_len,
                if list_len == 1 { " was" } else { "s were" },
            );
            text += if cfg.verbose > 0 {
                ":"
            } else {
                match cfg.no_log {
                    true => &cfg.guts.suffix_add_v_suggestion_no_log,
                    false => &cfg.guts.suffix_add_v_suggestion,
                }
            };
            msg(&text, MsgTone::Good, max(0, warnings_off), cfg, log)?;
        }
        ThresholdMessageKind::Skipped => {
            text = format!(
                "{} merged leveled list{} had subrecords auto-deleted due to always-delete rule",
                list_len,
                if list_len == 1 { "" } else { "s" },
            );
            text += if cfg.verbose > 0 {
                ":"
            } else {
                match cfg.no_log {
                    true => &cfg.guts.suffix_add_2v_suggestion_no_log,
                    false => &cfg.guts.suffix_add_2v_suggestion,
                }
            };
            msg(&text, MsgTone::Good, max(0, warnings_off), cfg, log)?;
        }
        ThresholdMessageKind::Warning => {
            text = format!(
            "{} list{} ratio of deleted/initial subrecords higher than threshold, though subrecords were deleted:",
            list_len,
            if list_len == 1 { " has" } else { "s have" },
        );
            msg(&text, MsgTone::Bad, max(0, warnings_off), cfg, log)?;
        }
    }
    text.clear();
    text = format!(
        "\n\t{:>5} {:<5} {:<1} {:<32} {:<32} {}",
        "RATIO", "THOLD", "T", "LEVELED LIST", "INITIAL PLUGIN", "RESPONSIBLE PLUGINS"
    );
    let mut plugins: Vec<String> = Vec::new();
    for ratio_message_raw in list.messages.iter() {
        if !plugins.contains(ratio_message_raw.initial_plugin) {
            plugins.push(ratio_message_raw.initial_plugin.to_owned());
        }
        text.push_str(&format!(
            "\n\t{:>5} {:<5} {:<1} {:<32} {:<32} {}",
            format!("{:.0}%", ratio_message_raw.ratio),
            format!("{}%", ratio_message_raw.threshold),
            ratio_message_raw.log_t,
            ratio_message_raw.id,
            ratio_message_raw.initial_plugin,
            ratio_message_raw.responsible_plugins_str.join(", "),
        ))
    }
    match list.kind {
        ThresholdMessageKind::Skipped => {}
        _ => {
            text.push_str(&format!(
                "\n
Following plugin{} probably not designed for deletion from leveled lists:
\t\"{}\"

Consider performing any of these actions:
\t1. Disable subrecord deletion for leveled lists from {} with --never-delete:
\t\t--never-delete \"{}\"
\t2. Increase ratio threshold with --threshold-creatures / --threshold-items
\t3. Disable subrecord deletion at all with --no-delete
\t4. Disable this warning with --no-threshold-warnings
",
                if list_len == 1 { " is" } else { "s are" },
                plugins.join("\", \""),
                if list_len == 1 {
                    "this plugin"
                } else {
                    "these plugins"
                },
                plugins.join(","),
            ));
        }
    }
    msg(
        text,
        MsgTone::Neutral,
        max(
            match list.kind {
                ThresholdMessageKind::Warning => 0,
                _ => 1,
            },
            warnings_off,
        ),
        cfg,
        log,
    )
}

fn merged_and_last_differ(
    flags: &Vec<ObjectFlags>,
    list_flags: &Vec<u32>,
    chance_none: &Vec<u8>,
    mut merged: Vec<Subrecord>,
    last: &mut LastList,
) -> bool {
    if (flags.len() > 1 && flags.last().unwrap() != &last.flag)
        || (list_flags.len() > 1 && list_flags.last().unwrap() != &last.list_flag)
        || (chance_none.len() > 1 && chance_none.last().unwrap() != &last.chance_none)
    {
        true
    } else {
        merged.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));
        merged.sort_by_key(|level| level.1);
        let mut last_list = last
            .list
            .iter()
            .map(|(name, level)| (name.to_lowercase(), *level))
            .collect::<Vec<Subrecord>>();
        last_list.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));
        last_list.sort_by_key(|level| level.1);
        merged != last_list
    }
}
