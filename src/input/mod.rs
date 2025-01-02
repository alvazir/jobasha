use crate::{err_or_ignore, msg, CellRecordMap, Cfg, LlCreatureRecords, LlItemRecords, Log, MsgTone, PluginInfo, Progress};
use anyhow::{anyhow, Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::{io::ErrorKind, sync::mpsc, thread};
use tes3::esp::{Plugin, TES3Object};
pub(crate) mod merge;
pub(crate) mod structs;
use merge::{CellKey, IntermediateRecords};
use structs::{InputHelper, ReadStats};

pub(super) fn get_records<'a>(
    plugins: &'a [PluginInfo],
    cfg: &'a Cfg,
    log: &mut Log,
) -> Result<(LlCreatureRecords<'a>, LlItemRecords<'a>, IntermediateRecords<'a>, ReadStats)> {
    let mut ll_creatures = LlCreatureRecords::default();
    let mut ll_items = LlItemRecords::default();
    let mut intermediate_records = IntermediateRecords::default();
    let mut stats = ReadStats::default();
    if cfg.compare_only {
        return Ok((ll_creatures, ll_items, intermediate_records, stats));
    }
    let mut helper = InputHelper::new(cfg, &plugins[0]);
    let mut progress = Progress::new(plugins.len(), cfg);
    let mut skipped_plugins = Vec::new();
    let get_cell = cfg.merge.cell || cfg.multipatch.cellnames || cfg.multipatch.fogbug;
    let get_crea = cfg.merge.crea || cfg.multipatch.summons;

    let (tx_object, rx_object) = mpsc::channel();
    thread::spawn(move || {
        for object in rx_object {
            drop(object);
        }
    });
    let (tx_reference, rx_reference) = mpsc::channel();
    thread::spawn(move || {
        for reference in rx_reference {
            drop(reference);
        }
    });

    for (plugin_info, count) in plugins.iter().zip(1u64..) {
        if !progress.off {
            progress.tick(count);
        }
        helper.set_plugin(count, plugin_info);
        let mut plugin = Plugin::new();
        if let Err(error) = plugin.load_path(&plugin_info.path) {
            if matches!(error.kind(), ErrorKind::InvalidData) {
                if let Some(tag) = error.to_string().strip_prefix("Unexpected Tag: ") {
                    if cfg.skip_unexpected_tags
                        || (!cfg.no_skip_unexpected_tags_default
                            && cfg.guts.skip_unexpected_tags_default.contains(&tag.to_lowercase()))
                    {
                        skipped_plugins.push(format!(
                            "Plugin \"{}\" will be skipped, because it contains known unexpected record type: {}",
                            &plugin_info.name, tag
                        ));
                        continue;
                    } else {
                        return Err(anyhow!("Failed to read plugin \"{}\"\n{}\nUse either --skip \"{0}\" to skip this plugin or --skip-unexpected-tags to skip all similar plugins\nConsider reporting the error to add this tag to the list of unexpected tags to skip by default", &plugin_info.name, error));
                    }
                }
            };
            let text = format!("Failed to read plugin \"{}\"\n{}", &plugin_info.name, error);
            err_or_ignore(text, cfg, log)?;
            continue;
        };
        let Some(header) = plugin.objects.first() else {
            let text = format!("Failed to read plugin \"{}\"\nPlugin is empty", &plugin_info.name);
            err_or_ignore(text, cfg, log)?;
            continue;
        };
        stats.get_records(header);

        for object in plugin.objects.into_iter() {
            macro_rules! match_object {
                ($($type:ident:$obj:ident),+) => {
                    paste! {
                        match object {
                            TES3Object::LeveledCreature(levc) if !cfg.creatures.skip => {
                                ll_creatures.get_levc(levc, &mut helper);
                            }
                            TES3Object::LeveledItem(levi) if !cfg.items.skip => {
                                ll_items.get_levi(levi, &mut helper);
                            }
                            TES3Object::Cell(mut cell) if get_cell => {
                                if !cfg.merge.references && !cell.references.is_empty() {
                                    tx_reference.send(cell.references).with_context(|| "Bug: failed to send references to tx_reference channel")?;
                                    cell.references = hashbrown::HashMap::new();
                                }
                                intermediate_records.get_cell(cell, helper.plugin_info);
                            }
                            TES3Object::Creature(mut crea) if get_crea => {
                                if crea.scale == Some(1.0) {
                                    crea.scale = None;
                                }
                                intermediate_records.get_crea(crea, helper.plugin_info);
                            }
                            $(TES3Object::$obj($type) if cfg.merge.$type => {
                                    intermediate_records.[<get_ $type>]($type, helper.plugin_info);
                            },)+
                            _ => {
                                tx_object.send(object).with_context(|| "Bug: failed to send object to tx_object channel")?;
                            }
                        }
                    }
                }
            }
            match_object!(
                gmst:GameSetting,
                clas:Class,
                race:Race,
                soun:Sound,
                skil:Skill,
                mgef:MagicEffect,
                bsgn:Birthsign,
                spel:Spell,
                stat:Static,
                door:Door,
                misc:MiscItem,
                weap:Weapon,
                cont:Container,
                body:Bodypart,
                ligh:Light,
                ench:Enchanting,
                npc_:Npc,
                armo:Armor,
                clot:Clothing,
                repa:RepairItem,
                acti:Activator,
                appa:Apparatus,
                lock:Lockpick,
                prob:Probe,
                ingr:Ingredient,
                book:Book,
                alch:Alchemy,
                sndg:SoundGen
            );
        }
    }

    if get_cell {
        let skip_0x40 = !cfg.guts.no_skip_unknown_cell_flags;
        intermediate_records
            .cell
            .par_iter_mut()
            .try_for_each(|(key, map)| preprocess_cell(key, map, skip_0x40))
            .with_context(|| "Bug: failed to preprocess cells")?;
    }
    stats.get_plugins(plugins.len() - skipped_plugins.len());
    stats.get_speed(progress.finish());
    if !skipped_plugins.is_empty() {
        msg(skipped_plugins.join("\n"), MsgTone::Neutral, 0, cfg, log)?;
    }
    Ok((ll_creatures, ll_items, intermediate_records, stats))
}

fn preprocess_cell(key: &CellKey, map: &mut CellRecordMap<'_>, skip_0x40: bool) -> Result<()> {
    if let CellKey::Exterior(_) = key {
        if skip_0x40 {
            remove_unknown_cell_flags(map);
        }

        name_exterior_cell(map).with_context(|| "Bug: failed to name exterior cell")?;
    }
    Ok(())
}

fn remove_unknown_cell_flags(map: &mut CellRecordMap<'_>) {
    for record in map.records.iter_mut() {
        let truncated_flags = !!record.cell.data.flags;
        if record.cell.data.flags != truncated_flags {
            record.cell.data.flags = truncated_flags;
        }
    }
}

fn name_exterior_cell(map: &mut CellRecordMap<'_>) -> Result<()> {
    let cell = &map.record(0)?.cell;
    let name = if cell.name.is_empty() {
        match &cell.region {
            Some(region) => region,
            None => "wilderness",
        }
    } else {
        &cell.name
    };
    map.name = format!(
        "{name}{separator}({x}, {y})",
        separator = if name.is_empty() { "" } else { " " },
        x = cell.data.grid.0,
        y = cell.data.grid.1
    );
    Ok(())
}
