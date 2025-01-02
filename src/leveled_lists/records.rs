use super::messages::{DeletedSubrecords, DeleveledSubrecords, LlMessages, UntouchedList};
use crate::{
    get_delev_segment_ceil, get_plugin_size, Cfg, DelevSkipPatterns, InputHelper, ListCounts, ListKind, Log, PluginInfo, PluginName,
    RawPlugins, ResponsiblePlugins,
};
use anyhow::{anyhow, Result};
use hashbrown::{hash_map::Entry, HashMap};
use paste::paste;
use rand::{rngs::ThreadRng, Rng};
use tes3::esp::{LeveledCreature, LeveledCreatureFlags, LeveledItem, LeveledItemFlags, ObjectFlags, TES3Object};
mod self_macro;
use self_macro::ll_record_methods;

pub(crate) type LlElement = (String, u16);

#[derive(Clone, PartialEq)]
pub(crate) struct LlCreatureLast {
    pub(crate) flag: ObjectFlags,
    pub(crate) list_flag: LeveledCreatureFlags,
    pub(crate) chance_none: u8,
    pub(crate) list: Vec<LlElement>,
}

impl Default for LlCreatureLast {
    fn default() -> Self {
        LlCreatureLast {
            flag: ObjectFlags::default(),
            list_flag: LeveledCreatureFlags::CALCULATE_FROM_ALL_LEVELS,
            chance_none: u8::default(),
            list: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct LlItemLast {
    pub(crate) flag: ObjectFlags,
    pub(crate) list_flag: LeveledItemFlags,
    pub(crate) chance_none: u8,
    pub(crate) list: Vec<LlElement>,
}

impl Default for LlItemLast {
    fn default() -> Self {
        LlItemLast {
            flag: ObjectFlags::default(),
            list_flag: LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS,
            chance_none: u8::default(),
            list: Vec::new(),
        }
    }
}
#[derive(Clone, PartialEq)]
pub(crate) struct LlCreatureRecord<'a> {
    pub(crate) flags: Vec<ObjectFlags>,
    pub(crate) id: String,
    pub(crate) leveled_creature_flags: Vec<LeveledCreatureFlags>,
    pub(crate) chance_nones: Vec<u8>,
    pub(crate) list: Vec<LlElement>,
    pub(crate) list_lowercased: Vec<LlElement>,
    pub(crate) first: Vec<(LlElement, LlElement)>,
    pub(crate) delete: Vec<(LlElement, LlElement, ResponsiblePlugins<'a>)>,
    pub(crate) count: usize,
    pub(crate) plugin_name_lowercased: PluginName<'a>,
    pub(crate) masters: Vec<&'a PluginInfo>,
    pub(crate) last: LlCreatureLast,
    pub(crate) last_plugin_name: Option<PluginName<'a>>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct LlItemRecord<'a> {
    pub(crate) flags: Vec<ObjectFlags>,
    pub(crate) id: String,
    pub(crate) leveled_item_flags: Vec<LeveledItemFlags>,
    pub(crate) chance_nones: Vec<u8>,
    pub(crate) list: Vec<LlElement>,
    pub(crate) list_lowercased: Vec<LlElement>,
    pub(crate) first: Vec<(LlElement, LlElement)>,
    pub(crate) delete: Vec<(LlElement, LlElement, ResponsiblePlugins<'a>)>,
    pub(crate) count: usize,
    pub(crate) plugin_name_lowercased: PluginName<'a>,
    pub(crate) masters: Vec<&'a PluginInfo>,
    pub(crate) last: LlItemLast,
    pub(crate) last_plugin_name: Option<PluginName<'a>>,
}

ll_record_methods!(
    levi,
    LeveledItem,
    LlItemRecord,
    items,
    helper,
    leveled_item_flags,
    LlItemLast,
    item_differs
);

ll_record_methods!(
    levc,
    LeveledCreature,
    LlCreatureRecord,
    creatures,
    helper,
    leveled_creature_flags,
    LlCreatureLast,
    creature_differs
);

#[allow(clippy::too_many_arguments)]
fn delete_subrecords<'a>(
    list: &mut Vec<LlElement>,
    list_lowercased: &mut Vec<LlElement>,
    id: &String,
    first: Vec<(LlElement, LlElement)>,
    delete: Vec<(LlElement, LlElement, ResponsiblePlugins<'a>)>,
    threshold: f64,
    log_t: &'a str,
    plugin_info: &'a PluginInfo,
    counts: &mut ListCounts,
    messages: &mut LlMessages<'a>,
    cfg: &Cfg,
) -> Result<()> {
    if !cfg.extended_delete && !cfg.always_delete.contains(&plugin_info.name_lowercased) {
        return Ok(());
    };
    let ratio = 100.0 * delete.len() as f64 / first.len() as f64;
    if cfg.extended_delete
        && ratio > threshold
        && ratio >= cfg.guts.auto_resolve_lower_limit
        && !cfg.always_delete.contains(&plugin_info.name_lowercased)
    {
        messages
            .threshold_resolved
            .push(ratio, threshold, log_t, id.to_owned(), &plugin_info.name, &delete);
    } else {
        if cfg.extended_delete && ratio > threshold {
            if cfg.always_delete.contains(&plugin_info.name_lowercased) {
                messages
                    .threshold_skipped
                    .push(ratio, threshold, log_t, id.to_owned(), &plugin_info.name, &delete);
            } else {
                messages
                    .threshold_warnings
                    .push(ratio, threshold, log_t, id.to_owned(), &plugin_info.name, &delete);
            }
        }
        let mut subrecords = Vec::new();
        for (subrecord_lowercased, subrecord, responsible_plugins) in delete.into_iter() {
            let index = match list_lowercased.iter().position(|x| x == &subrecord_lowercased) {
                Some(index) => index,
                None => {
                    return Err(anyhow!("Failed to delete subrecord. This error should've never happened. List id: {}, subrecord id: {}, initial plugin: {}, responsible plugin: {}", id, subrecord.0, &plugin_info.name, responsible_plugins.into_iter().map(|x| x.as_str()).collect::<Vec<_>>().join(", ")));
                }
            };

            list_lowercased.swap_remove(index);
            list.swap_remove(index);
            subrecords.push((subrecord, responsible_plugins.into_iter().map(|x| x.as_str()).collect()));
            counts.merge.deleted_subrecord += 1;
        }
        messages.deleted_subrecords.push(DeletedSubrecords {
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
            let size = get_plugin_size(&plugin_info.path, cfg, log)?;
            v.insert((*count, &plugin_info.name, size));
            *count += 1;
        }
    }
    Ok(())
}

fn creature_differs(
    flags: &[ObjectFlags],
    list_flags: &[LeveledCreatureFlags],
    chance_none: &[u8],
    mut merged: Vec<LlElement>,
    last: &mut LlCreatureLast,
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
            .collect::<Vec<LlElement>>();
        last_list.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));
        last_list.sort_by_key(|level| level.1);
        merged != last_list
    }
}

fn item_differs(
    flags: &[ObjectFlags],
    list_flags: &[LeveledItemFlags],
    chance_none: &[u8],
    mut merged: Vec<LlElement>,
    last: &mut LlItemLast,
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
            .collect::<Vec<LlElement>>();
        last_list.sort_by(|(name1, _), (name2, _)| name1.cmp(name2));
        last_list.sort_by_key(|level| level.1);
        merged != last_list
    }
}

#[allow(clippy::too_many_arguments)]
fn delevel_list<'a>(
    list: &[LlElement],
    id: &str,
    kind: &'a ListKind,
    plugin_info: &'a PluginInfo,
    rng: &mut ThreadRng,
    cfg: &Cfg,
    counts: &mut ListCounts,
    messages: &mut LlMessages<'a>,
) -> Vec<LlElement> {
    let mut res = Vec::new();
    if list.iter().any(|(_, level)| level > &kind.delev_to) {
        if delev_skip(id, &cfg.delev_skip_list, &cfg.delev_no_skip_list) {
            return res;
        }
        let mut subrecords = Vec::new();
        for (name, level) in list.iter() {
            if level > &kind.delev_to && !delev_skip(name, &cfg.delev_skip_subrecord, &cfg.delev_no_skip_subrecord) {
                let mut new_level = if kind.delev_segment > 0 && level >= &kind.delev_segment {
                    if !cfg.delev_segment_progressive {
                        kind.delev_segment_ceil
                    } else {
                        get_delev_segment_ceil(level, kind.delev_segment, kind.delev_to, cfg.delev_segment_ratio)
                    }
                } else {
                    kind.delev_to
                };
                if cfg.delev_random {
                    new_level = rng.gen_range(new_level..=*level);
                };
                res.push((name.clone(), new_level));
                subrecords.push(((name.to_lowercase(), *level), new_level));
                counts.delev.deleveled_subrecord += 1;
            } else {
                res.push((name.clone(), *level));
            }
        }
        messages.deleveled_subrecords.push(DeleveledSubrecords {
            log_t: &kind.log_t,
            id: id.to_owned(),
            initial_plugin: &plugin_info.name,
            subrecords,
        });
    };
    res
}

fn delev_skip(string: &str, patterns: &DelevSkipPatterns, no_patterns: &DelevSkipPatterns) -> bool {
    if !patterns.is_empty {
        let string_lowercased = string.to_lowercase();
        if (patterns.exact.iter().any(|exact| &string_lowercased == exact)
            || patterns.prefix.iter().any(|prefix| string_lowercased.starts_with(prefix))
            || patterns.suffix.iter().any(|suffix| string_lowercased.ends_with(suffix))
            || patterns.infix.iter().any(|infix| string_lowercased.contains(infix)))
            && !delev_no_skip(&string_lowercased, no_patterns)
        {
            return true;
        }
    }
    false
}

fn delev_no_skip(string_lowercased: &str, patterns: &DelevSkipPatterns) -> bool {
    !patterns.is_empty
        && (patterns.exact.iter().any(|exact| string_lowercased == exact)
            || patterns.prefix.iter().any(|prefix| string_lowercased.starts_with(prefix))
            || patterns.suffix.iter().any(|suffix| string_lowercased.ends_with(suffix))
            || patterns.infix.iter().any(|infix| string_lowercased.contains(infix)))
}
