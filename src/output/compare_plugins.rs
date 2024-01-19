use super::{select_placed, Helper, Levc, Levi, ListDiffStats};
use crate::{msg, Cfg, ComparePlugin, ListCounts, Log, MsgTone, OutputFile, PluginKind};
use anyhow::{anyhow, Context, Result};
use std::{
    collections::{HashMap, HashSet},
    fmt::Write as _,
};
use tes3::esp::{Header, LeveledCreature, LeveledItem, Plugin, TES3Object};

pub(super) fn compare_plugins(h: &mut Helper, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let mut plugins_differ = 0;
    if !cfg.no_compare {
        let mut new = if !cfg.compare_only {
            &h.merge.plugin
        } else {
            &h.compare.previous.plugin
        };
        let (mut old, mut old_name) = if cfg.compare_with.is_empty() {
            (&h.compare.previous, "")
        } else {
            (&h.compare.compare_with, cfg.compare_with.as_ref())
        };
        let mut merge_changed = false;
        #[rustfmt::skip]
        compare_plugin(new, old, old_name, &cfg.output, &h.counts, &mut merge_changed, &mut plugins_differ, cfg, log)
            .with_context(|| plugin_compare_failure_msg(&cfg.output.name, old_name))?;
        if cfg.delev && cfg.delev_distinct {
            new = &h.delev.plugin;
            (old, old_name) = if cfg.compare_delev_with.is_empty() {
                (&h.compare.delev_previous, "")
            } else {
                (&h.compare.delev_compare_with, cfg.compare_delev_with.as_ref())
            };
            #[rustfmt::skip]
            compare_plugin(new, old, old_name, &cfg.delev_output, &h.counts, &mut merge_changed, &mut plugins_differ, cfg, log)
                .with_context(|| plugin_compare_failure_msg(&cfg.delev_output.name, old_name))?;
        }
        if cfg.compare_only && plugins_differ != 0 {
            h.exit_code = 3;
        }
    }
    Ok(())
}

fn plugin_compare_failure_msg(output_name: &str, old_name: &str) -> String {
    if old_name.is_empty() {
        format!("Failed to compare {:?} with previous version", output_name)
    } else {
        format!("Failed to compare {:?} with {:?}", output_name, old_name)
    }
}

#[allow(clippy::too_many_arguments)]
fn compare_plugin(
    new_plugin: &Plugin,
    old: &ComparePlugin,
    old_name: &str,
    output: &OutputFile,
    counts: &ListCounts,
    merge_changed: &mut bool,
    plugins_differ: &mut i32,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if !old.loaded || is_equal(new_plugin, old, old_name, output, cfg, log)? {
        return Ok(());
    } else if !cfg.compare_only && is_empty(new_plugin, old, old_name, output, counts, cfg, log)? {
        *merge_changed = true;
        return Ok(());
    }
    let header_diff = match compare_headers(new_plugin, old, old_name, output, counts, merge_changed, cfg) {
        Ok(header_diff) => header_diff,
        Err(error) => {
            let text = plugin_compare_failure_msg(&output.name, old_name);
            *merge_changed = true;
            msg(format!("{text}: {error}"), MsgTone::Ugly, 0, cfg, log)?;
            return Ok(());
        }
    };
    let (levc_diff, levi_diff, levc_short, levi_short) =
        compare_lists(new_plugin, old, output, cfg, log).with_context(|| "Failed to compare lists")?;
    if header_diff.is_empty() && levc_diff.is_empty() && levi_diff.is_empty() {
        msg(plugin_compare_msg(&output.name, old_name, true), MsgTone::Good, 0, cfg, log)?;
        Ok(())
    } else {
        *plugins_differ = 3;
        *merge_changed = true;
        let level = cfg.guts.verboseness_details_compare_plugins;
        msg(plugin_compare_msg(&output.name, old_name, false), MsgTone::Ugly, 0, cfg, log)?;
        msg("", MsgTone::Neutral, level, cfg, log)?;
        if !header_diff.is_empty() {
            let tab1 = &cfg.guts.compare_tab_l1;
            msg(format!("{tab1}Plugin header(TES3, old -> new):"), MsgTone::Neutral, 0, cfg, log)?;
            msg(&header_diff[..header_diff.len() - 1], MsgTone::Neutral, 0, cfg, log)?;
            msg("", MsgTone::Neutral, level, cfg, log)?;
        }
        if !levc_diff.is_empty() {
            msg_with_details_suggestion(levc_short, MsgTone::Neutral, 0, level, cfg, log)?;
            msg(&levc_diff[..levc_diff.len() - 1], MsgTone::Neutral, level, cfg, log)?;
            msg("", MsgTone::Neutral, level, cfg, log)?;
        }
        if !levi_diff.is_empty() {
            msg_with_details_suggestion(levi_short, MsgTone::Neutral, 0, level, cfg, log)?;
            msg(&levi_diff[..levi_diff.len() - 1], MsgTone::Neutral, level, cfg, log)?;
            msg("", MsgTone::Neutral, level, cfg, log)?;
        }
        Ok(())
    }
}

fn plugin_compare_msg(output_name: &str, old_name: &str, equal: bool) -> String {
    let (eq_or_neq, end, old, new) = if equal {
        ("is equal to", "", "", "")
    } else {
        ("is different from", ":", "(old)", "(new)")
    };
    if old_name.is_empty() {
        format!("Plugin {:?}{new} {eq_or_neq} previous version{old}{end}", output_name)
    } else {
        format!("Plugins {:?}{new} {eq_or_neq} {:?}{old}{end}", output_name, old_name)
    }
}

fn msg_with_details_suggestion(mut text: String, tone: MsgTone, verbose: u8, details: u8, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if cfg.verbose < cfg.guts.verboseness_details_compare_plugins {
        write!(
            text,
            ", add {:v<details$}{} for details",
            "-",
            if cfg.no_log { "" } else { " or check log" },
            details = details as usize + 1,
        )?
    };
    msg(text, tone, verbose, cfg, log)
}

fn is_equal(new_plugin: &Plugin, old: &ComparePlugin, old_name: &str, output: &OutputFile, cfg: &Cfg, log: &mut Log) -> Result<bool> {
    let mut is_equal = false;
    if old.compared && old.equal || new_plugin.objects == old.plugin.objects {
        let (level, text) = if old_name.is_empty() {
            (
                if cfg.dry_run { 0 } else { 1 },
                format!("Plugin {:?} is equal to previous version", output.name),
            )
        } else {
            (0, format!("Plugins {:?} and {:?} are equal", output.name, old_name))
        };
        is_equal = true;
        msg(&text, MsgTone::Neutral, level, cfg, log)?;
    }
    Ok(is_equal)
}

fn is_empty(
    new_plugin: &Plugin,
    old: &ComparePlugin,
    old_name: &str,
    output: &OutputFile,
    counts: &ListCounts,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<bool> {
    let mut prefix = "";
    let mut infix = "";
    if select_placed(output, counts, cfg) == 0 {
        (prefix, infix) = ("Unable to compare absent", "plugin with");
    } else if new_plugin.objects.len() < 2 {
        (prefix, infix) = ("Unable to compare empty", "plugin with");
    } else if old.plugin.objects.len() < 2 {
        (prefix, infix) = ("Unable to compare", "plugin with empty");
    }
    if prefix.is_empty() {
        Ok(false)
    } else {
        let name = if matches!(output.kind, PluginKind::Delev) {
            "delev"
        } else {
            "output"
        };
        let text = if old_name.is_empty() {
            format!("{prefix} {name} {infix} previous version")
        } else {
            format!("{prefix} {name} {infix} {old_name:?}")
        };
        msg(text, MsgTone::Neutral, 0, cfg, log)?;
        Ok(true)
    }
}

fn compare_headers(
    new: &Plugin,
    old: &ComparePlugin,
    old_name: &str,
    output: &OutputFile,
    counts: &ListCounts,
    merge_changed: &mut bool,
    cfg: &Cfg,
) -> Result<String> {
    let mut diff = String::new();
    if new.objects.is_empty() {
        return Err(anyhow!("Plugin {:?} is empty or invalid", output.name));
    } else if old.plugin.objects.is_empty() {
        return Err(anyhow!("Plugin {} is empty or invalid", get_old_name(old_name)));
    }
    match &new.objects[0] {
        TES3Object::Header(new_header) => match &old.plugin.objects[0] {
            TES3Object::Header(old_header) => {
                if new_header != old_header {
                    let tab = &cfg.guts.compare_tab_l2;
                    if new_header.num_objects != old_header.num_objects {
                        writeln!(diff, "{tab}~ Records {} -> {}", old_header.num_objects, new_header.num_objects,)?;
                    };
                    let mut new_header_masters_len = new_header.masters.len();
                    let mut cfg_output_name_low = String::new();
                    let merge_was_written = if matches!(output.kind, PluginKind::Delev) && counts.delev.placed > 0 {
                        cfg_output_name_low = cfg.output.name.to_lowercase();
                        new_header_masters_len += 1;
                        true
                    } else {
                        false
                    };
                    if new_header_masters_len != old_header.masters.len() {
                        writeln!(diff, "{tab}~ Masters {} -> {}", old_header.masters.len(), new_header.masters.len(),)?;
                    };
                    if new_header.masters != old_header.masters {
                        let old_masters = get_masters_from_header(old_header);
                        let new_masters = get_masters_from_header(new_header);
                        let mut headers_set: HashSet<&String> = HashSet::new();
                        for (new_name, new_len, new_name_low) in new_masters.iter() {
                            if let Some((_, old_len, _)) = old_masters.iter().find(|(_, _, old_name_low)| old_name_low == new_name_low)
                            {
                                if new_len != old_len {
                                    writeln!(diff, "{tab}~ MAST {new_name:?} [{old_len} -> {new_len}]")?;
                                }
                                headers_set.insert(new_name_low);
                            } else if !cfg.compare_common {
                                writeln!(diff, "{tab}+ MAST {new_name:?}")?;
                            }
                        }
                        for (old_name, _, old_name_low) in old_masters.iter() {
                            if merge_was_written && old_name_low == &cfg_output_name_low {
                                if *merge_changed {
                                    writeln!(diff, "{tab}~ MAST {:?} [size may have changed]", cfg.output.name)?;
                                }
                                continue;
                            };
                            if !cfg.compare_common && !headers_set.contains(old_name_low) {
                                writeln!(diff, "{tab}- MAST {old_name:?}")?;
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(anyhow!("Header is invalid, moved or missing in {}", get_old_name(old_name)));
            }
        },
        _ => {
            return Err(anyhow!("Header is invalid, moved or missing in {:?}", output.name));
        }
    }
    Ok(diff)
}

fn get_old_name(old_name: &str) -> String {
    if old_name.is_empty() {
        "previous version".to_string()
    } else {
        format!("{old_name:?}")
    }
}

fn get_masters_from_header(header: &Header) -> Vec<(&String, &u64, String)> {
    header
        .masters
        .iter()
        .map(|(name, len)| (name, len, name.to_lowercase()))
        .collect::<Vec<(&String, &u64, String)>>()
}

fn get_list_indexes(plugin: &Plugin) -> (Levc, Levi, usize, usize) {
    let (mut levc, mut levi): (Levc, Levi) = (HashMap::new(), HashMap::new());
    let (mut c_count, mut i_count): (usize, usize) = (0, 0);
    macro_rules! get_list_indexes {
        ($count:ident, $list:ident, $thing:ident) => {{
            $count += 1;
            $list
                .entry($thing.id.to_lowercase())
                .and_modify(|o| {
                    o.0 += 1;
                    o.1 = &$thing
                })
                .or_insert((1, &$thing));
        }};
    }
    for record in plugin.objects.iter() {
        match record {
            TES3Object::LeveledCreature(leveled_creature) => get_list_indexes!(c_count, levc, leveled_creature),
            TES3Object::LeveledItem(leveled_item) => get_list_indexes!(i_count, levi, leveled_item),
            _ => {}
        }
    }
    (levc, levi, c_count, i_count)
}

fn check_multiple_lists_with_same_name(
    levc: &Levc,
    levi: &Levi,
    c_count: usize,
    i_count: usize,
    output: &OutputFile,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if levc.len() != c_count || levi.len() != i_count {
        let mut text = format!(
            "{}Bug: multiple lists with the same name found in {:?}:\n",
            &cfg.guts.compare_tab_l1, output.name
        );
        macro_rules! get_multi {
            ($kind:ident) => {
                $kind
                    .iter()
                    .filter(|x| x.1 .0 > 1)
                    .map(|x| (x.0, x.1 .0))
                    .collect::<Vec<(&String, usize)>>()
            };
        }
        for (mut lists, kind) in [get_multi!(levc), get_multi!(levi)].into_iter().zip(["LEVC", "LEVI"].iter()) {
            lists.sort();
            for (name, count) in lists.iter() {
                writeln!(text, "{}{kind}({}): {:?}", &cfg.guts.compare_tab_l2, count, name)?;
            }
        }
        msg(text, MsgTone::Bad, 0, cfg, log)?;
    }
    Ok(())
}

fn compare_lists(
    new: &Plugin,
    old: &ComparePlugin,
    output: &OutputFile,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<(String, String, String, String)> {
    let (mut diffc, mut diffi, mut shortc, mut shorti) = (String::new(), String::new(), String::new(), String::new());
    let (mut statc, mut stati) = (ListDiffStats::default(), ListDiffStats::default());
    let tab1 = &cfg.guts.compare_tab_l1;
    let tab2 = &cfg.guts.compare_tab_l2;
    let tab3 = &cfg.guts.compare_tab_l3;
    let mut objects: HashMap<String, Vec<&(String, u16)>> = HashMap::new();
    let mut remaining_objects: Vec<(u16, String, &String)> = Vec::new();
    let (mut old_levc, mut old_levi, old_c_count, old_i_count) = get_list_indexes(&old.plugin);
    let (new_levc, new_levi, new_c_count, new_i_count) = get_list_indexes(new);
    if !cfg.compare_only {
        check_multiple_lists_with_same_name(&new_levc, &new_levi, new_c_count, new_i_count, output, cfg, log)
            .with_context(|| "Failed to check multiple lists with the same name in the output plugin")?;
    }
    macro_rules! compare_lists {
        ($diff:ident, $old_lists:ident, $new_lists:ident, $old_count:ident, $new_count:ident, $kind:ident, $flags:ident, $kind_field:ident, $stat:ident, $short:ident, $name:expr, $acronym:expr) => {
            if $old_lists != $new_lists {
                if $old_count != $new_count {
                    writeln!($diff, "{tab2}~ Lists {} -> {}", $old_count, $new_count)?;
                }
                let mut lists = $new_lists.iter().collect::<Vec<(&String, &(usize, &$kind))>>();
                lists.sort_by_key(|v| v.0);
                for (name_low, (_, list)) in lists.iter() {
                    match $old_lists.remove(*name_low) {
                        Some((_, old_list)) => {
                            if list != &old_list {
                                let mut differs = false;
                                macro_rules! list_differs {
                                    () => {
                                        if !differs {
                                            differs = true;
                                            $stat.changed += 1;
                                            writeln!($diff, "{tab2}~ {} {:?}:", $acronym, list.id)?;
                                        }
                                    };
                                }
                                if list.flags != old_list.flags {
                                    list_differs!();
                                    writeln!($diff, "{tab3}~ Record flags {:?} -> {:?}", old_list.flags, list.flags)?;
                                }
                                if list.$flags != old_list.$flags {
                                    list_differs!();
                                    writeln!($diff, "{tab3}~ List flags {:?} -> {:?}", old_list.$flags, list.$flags,)?;
                                }
                                if list.chance_none != old_list.chance_none {
                                    list_differs!();
                                    writeln!($diff, "{tab3}~ Chance none {} -> {}", old_list.chance_none, list.chance_none)?;
                                }
                                if list.$kind_field != old_list.$kind_field {
                                    objects.clear();
                                    for i in list.$kind_field.iter() {
                                        objects.entry(i.0.to_lowercase()).and_modify(|o| o.push(i)).or_insert(vec![i]);
                                    }
                                    for old_creature in &old_list.$kind_field {
                                        let old_low = old_creature.0.to_lowercase();
                                        match objects.get_mut(&old_low) {
                                            None => {
                                                list_differs!();
                                                writeln!($diff, "{tab3}- {} {:?} [{}]", $name, old_creature.0, old_creature.1)?;
                                            }
                                            Some(object) => {
                                                if object.len() > 1 {
                                                    if let Some(position) = object.iter().position(|x| x.1 == old_creature.1) {
                                                        object.swap_remove(position);
                                                    } else {
                                                        object.sort_by_key(|x| x.1);
                                                        let new_level = object.swap_remove(0).1;
                                                        list_differs!();
                                                        writeln!(
                                                            $diff,
                                                            "{tab3}~ {} {:?} [{} -> {}]",
                                                            $name, old_creature.0, old_creature.1, new_level
                                                        )?;
                                                        // $name, old_creature.0, new_level, old_creature.1
                                                    }
                                                } else {
                                                    if old_creature.1 != object[0].1 {
                                                        list_differs!();
                                                        writeln!(
                                                            $diff,
                                                            "{tab3}~ {} {:?} [{} -> {}]",
                                                            $name, old_creature.0, old_creature.1, object[0].1
                                                        )?;
                                                        // $name, old_creature.0, object[0].1, old_creature.1
                                                    }
                                                    objects.remove(&old_low);
                                                }
                                            }
                                        }
                                    }
                                    if !objects.is_empty() {
                                        remaining_objects.clear();
                                        for object in objects.iter() {
                                            for x in object.1.iter() {
                                                remaining_objects.push((x.1, object.0.clone(), &x.0));
                                            }
                                        }
                                        remaining_objects.sort();
                                        for object in &remaining_objects {
                                            list_differs!();
                                            writeln!($diff, "{tab3}+ {} {:?} [{}]", $name, object.2, object.0)?;
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            $stat.added += 1;
                            if !cfg.compare_common {
                                writeln!($diff, "{tab2}+ {} {:?}", $acronym, list.id)?;
                            }
                        }
                    }
                }
                let mut old_lists_sorted = $old_lists.iter().collect::<Vec<(&String, &(usize, &$kind))>>();
                old_lists_sorted.sort_by_key(|v| v.0);
                for (_, (_, list)) in old_lists_sorted.iter() {
                    $stat.removed += 1;
                    if !cfg.compare_common {
                        writeln!($diff, "{tab2}- {} {:?}", $acronym, list.id)?;
                    }
                }
                if !$diff.is_empty() {
                    write!($short, "{tab1}{} leveled lists({}, old -> new):", $name, $acronym)?;
                }
                if $stat.added > 0 {
                    write!($short, " +{}", $stat.added)?;
                }
                if $stat.removed > 0 {
                    write!($short, " -{}", $stat.removed)?;
                }
                if $stat.changed > 0 {
                    write!($short, " ~{}", $stat.changed)?;
                }
            }
        };
    }
    #[rustfmt::skip]
    compare_lists!(diffc, old_levc, new_levc, old_c_count, new_c_count, LeveledCreature, leveled_creature_flags, creatures, statc, shortc, "Creature", "LEVC");
    #[rustfmt::skip]
    compare_lists!(diffi, old_levi, new_levi, old_i_count, new_i_count, LeveledItem, leveled_item_flags, items, stati, shorti, "Item", "LEVI");
    Ok((diffc, diffi, shortc, shorti))
}
