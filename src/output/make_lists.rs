use super::{DeletedSubrecords, DeleveledSubrecords, Helper, UntouchedList};
use crate::{
    get_plugin_size, Cfg, ComparePlugins, Creature, DelevSkipPatterns, Item, LastCreature, LastItem, Log, PluginInfo, PluginName,
    ResponsiblePlugins, Subrecord,
};
use anyhow::{anyhow, Result};
use std::collections::{hash_map::Entry, HashMap};
use tes3::esp::{LeveledCreature, LeveledCreatureFlags, LeveledItem, LeveledItemFlags, ObjectFlags, TES3Object};

macro_rules! make_lists {
    ($name:ident, $tes3_kind:ident, $flags_kind:ident, $kind_differs:ident, $h:ident, $cfg:ident, $log:ident) => {
        for mut o in $name.into_iter() {
            $h.counts.total.total += o.count;
            $h.counts.total.unique += 1;
            if o.count > 1 || $cfg.all_lists || $cfg.delev {
                let mut $name = o.list;

                if o.count > 1 {
                    if !$cfg.no_delete && !o.delete.is_empty() {
                        delete_subrecords(
                            &mut $name,
                            &mut o.list_lowercased,
                            &o.id,
                            o.first,
                            o.delete,
                            $cfg.$name.threshold,
                            &$cfg.$name.log_t,
                            &o.masters[0],
                            &mut $h,
                            $cfg,
                        )?;
                    }
                    $name.sort_by(|(name1, _), (name2, _)| name1.to_lowercase().cmp(&name2.to_lowercase()));
                    $name.sort_by_key(|level| level.1);
                }

                let is_merge = if o.count > 1
                    && !$cfg.all_lists
                    && $kind_differs(&o.flags, &o.$flags_kind, &o.chance_nones, o.list_lowercased, &mut o.last)
                {
                    true
                } else {
                    false
                };

                let mut is_delev = false;
                if $cfg.delev && !$cfg.$name.skip_delev {
                    let delev_list = delevel_list(
                        &$name,
                        &$cfg.$name.delev_to,
                        &o.id,
                        &$cfg.$name.log_t,
                        &o.masters[0],
                        $cfg,
                        &mut $h,
                    );
                    if !delev_list.is_empty() {
                        $h.counts.delev.deleveled += 1;
                        if $cfg.delev_distinct {
                            let x = o.masters.clone();
                            append_masters(x, &mut $h.delev.masters, &mut $h.counts.delev.master, $cfg, $log)?;
                            $h.delev.lists.push(TES3Object::$tes3_kind($tes3_kind {
                                flags: o.flags.last().unwrap().clone(),
                                id: o.id.clone(),
                                $flags_kind: *o.$flags_kind.last().unwrap(),
                                chance_none: *o.chance_nones.last().unwrap(),
                                $name: delev_list,
                            }));
                            $h.counts.delev.placed += 1;
                        } else {
                            is_delev = true;
                            $name = delev_list;
                        }
                    }
                };

                if $cfg.all_lists || is_delev || is_merge {
                    append_masters(
                        o.masters,
                        &mut $h.merge.masters,
                        if $cfg.delev && $cfg.delev_distinct {
                            &mut $h.counts.merge.master
                        } else {
                            &mut $h.counts.total.master
                        },
                        $cfg,
                        $log,
                    )?;
                    $h.merge.lists.push(TES3Object::$tes3_kind($tes3_kind {
                        flags: o.flags.last().unwrap().clone(),
                        id: o.id,
                        $flags_kind: *o.$flags_kind.last().unwrap(),
                        chance_none: *o.chance_nones.last().unwrap(),
                        $name,
                    }));
                    if o.count > 1 {
                        $h.counts.merge.merged += 1;
                        if is_merge || $cfg.all_lists {
                            $h.counts.merge.placed += 1;
                            if !is_delev || $cfg.all_lists {
                                $h.counts.total.placed += 1;
                            }
                        } else {
                            $h.counts.merge.untouched += 1;
                        }
                    } else if $cfg.all_lists {
                        $h.counts.merge.placed += 1;
                        $h.counts.total.placed += 1;
                    }
                    if is_delev {
                        $h.counts.delev.placed += 1;
                        if !$cfg.all_lists {
                            $h.counts.total.placed += 1;
                        }
                    }
                } else if o.count > 1 {
                    $h.messages.untouched_lists.push(UntouchedList {
                        log_t: &$cfg.$name.log_t,
                        id: o.id,
                        initial_plugin: &o.masters[0].name,
                        last_plugin: &o.last_plugin_name.unwrap(),
                    });
                    if !is_merge {
                        $h.counts.merge.untouched += 1;
                    }
                    $h.counts.merge.merged += 1;
                }
            }
        }
    };
}

pub(super) fn make_lists<'a>(
    creatures: Vec<Creature<'a>>,
    items: Vec<Item<'a>>,
    plugins_to_compare: ComparePlugins,
    cfg: &'a Cfg,
    log: &mut Log,
) -> Result<Helper<'a>> {
    let mut h = Helper::new(plugins_to_compare);
    if !cfg.creatures.skip {
        make_lists!(creatures, LeveledCreature, leveled_creature_flags, creature_differs, h, cfg, log);
    }
    if !cfg.items.skip {
        make_lists!(items, LeveledItem, leveled_item_flags, item_differs, h, cfg, log);
    }
    Ok(h)
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
    h: &mut Helper<'a>,
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
        h.messages
            .threshold_resolved
            .push(ratio, threshold, log_t, id.to_owned(), &plugin_info.name, &delete);
    } else {
        if cfg.extended_delete && ratio > threshold {
            if cfg.always_delete.contains(&plugin_info.name_lowercased) {
                h.messages
                    .threshold_skipped
                    .push(ratio, threshold, log_t, id.to_owned(), &plugin_info.name, &delete);
            } else {
                h.messages
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
            h.counts.merge.deleted_subrecord += 1;
        }
        h.messages.deleted_subrecords.push(DeletedSubrecords {
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
    flags: &Vec<ObjectFlags>,
    list_flags: &Vec<LeveledCreatureFlags>,
    chance_none: &Vec<u8>,
    mut merged: Vec<Subrecord>,
    last: &mut LastCreature,
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

fn item_differs(
    flags: &Vec<ObjectFlags>,
    list_flags: &Vec<LeveledItemFlags>,
    chance_none: &Vec<u8>,
    mut merged: Vec<Subrecord>,
    last: &mut LastItem,
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

fn delevel_list<'a>(
    list: &[Subrecord],
    delev_lvl: &u16,
    id: &str,
    log_t: &'a str,
    plugin_info: &'a PluginInfo,
    cfg: &Cfg,
    h: &mut Helper<'a>,
) -> Vec<Subrecord> {
    let mut res = Vec::new();
    if list.iter().any(|(_, level)| level > delev_lvl) {
        if delev_skip(id, &cfg.delev_skip_list, &cfg.delev_no_skip_list) {
            return res;
        }
        let mut subrecords = Vec::new();
        for (name, level) in list.iter() {
            if level > delev_lvl && !delev_skip(name, &cfg.delev_skip_subrecord, &cfg.delev_no_skip_subrecord) {
                res.push((name.clone(), *delev_lvl));
                subrecords.push(((name.to_lowercase(), *level), *delev_lvl));
                h.counts.delev.deleveled_subrecord += 1;
            } else {
                res.push((name.clone(), *level));
            }
        }
        h.messages.deleveled_subrecords.push(DeleveledSubrecords {
            log_t,
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
