use super::{fields_are_equal, OptionRecordMergeLog, SpecificFlags};
use crate::{Cfg, NpcRecordMap};
use anyhow::{Context, Result};
use tes3::esp::{Npc, NpcFlags};

pub(crate) fn specific(
    _id_low: &str,
    merged: &mut Npc,
    last: &Npc,
    map: &NpcRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    fix_autocalc(merged, last, map, option_log, specific_flags, cfg).with_context(|| "Failed to fix AUTO_CALCULATE flag")?;
    if !cfg.merge.keep_redundant_values && !specific_flags.equal_after_specific {
        ignore_or_omit_redundant_values(merged, last, map, option_log, specific_flags, cfg)
            .with_context(|| "Failed to ignore or omit redundant values")?;
    }
    Ok(())
}

fn fix_autocalc(
    merged: &mut Npc,
    last: &Npc,
    map: &NpcRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    if merged.npc_flags.contains(NpcFlags::AUTO_CALCULATE) {
        if merged.data.stats.is_some() {
            merged.data.stats = None;
            if merged == last {
                specific_flags.equal_after_specific = true;
            }
            if !cfg.meta.silent {
                option_log.warn(
                    format_args!("Fixed NPC_ record: {id:?}: \"data.stats\": \"Manual\" -> \"Auto\" {{ \"data.stats\" were defined despite AUTO_CALCULATE flag was set }}", id = map.record_id_debug()?),
                        &map,
                        cfg,
                )?;
                option_log.record_fixed("data.stats", format_args!("\"Manual\""), format_args!("\"Auto\""), &map, cfg)?;
            }
        }
    } else if merged.data.stats.is_none() {
        let from = merged.npc_flags;
        merged.npc_flags.insert(NpcFlags::AUTO_CALCULATE);
        if merged == last {
            specific_flags.equal_after_specific = true;
        }
        if !cfg.meta.silent {
            let to = merged.npc_flags;
            option_log.warn(
                format_args!("Fixed NPC_ record: {id:?}: \"npc_flags\": {from:?} -> {to:?} {{ AUTO_CALCULATE flag was missing despite \"data.stats\" not set }}", id = map.record_id_debug()?),
                    &map,
                    cfg,
            )?;
            option_log.record_fixed("npc_flags", format_args!("{from:?}"), format_args!("{to:?}"), &map, cfg)?;
        }
    }
    Ok(())
}

fn ignore_or_omit_redundant_values(
    merged: &mut Npc,
    last: &Npc,
    map: &NpcRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    fn is_default_mesh(record: &Npc, mesh: &str) -> bool {
        match record.npc_flags.contains(NpcFlags::FEMALE) {
            true => mesh.to_ascii_lowercase() == "base_anim_female.nif",
            false => mesh.to_ascii_lowercase() == "base_anim.nif",
        }
    }
    let merged_mesh_is_default = if !merged.mesh.is_empty() {
        if !is_default_mesh(merged, &merged.mesh) {
            return Ok(());
        }
        true
    } else {
        false
    };
    let previous_mesh = map
        .records
        .iter()
        .rev()
        .filter(|x| !x.npc_.mesh.is_empty())
        .map(|x| x.npc_.mesh.as_ref())
        .next()
        .unwrap_or("");
    if is_default_mesh(merged, previous_mesh) {
        if merged_mesh_is_default {
            if !cfg.meta.silent {
                option_log.field_changed_redundant("mesh", format_args!("{:?}", merged.mesh), format_args!("{:?}", ""), cfg)?;
            }
            merged.mesh.clear();
        }
        if fields_are_equal!(
            merged,
            last,
            flags,
            name,
            script,
            inventory,
            spells,
            ai_data,
            ai_packages,
            travel_destinations,
            race,
            class,
            faction,
            head,
            hair,
            npc_flags,
            blood_type,
            data
        ) {
            specific_flags.equal_after_specific = true;
            if merged.mesh != last.mesh {
                specific_flags.changes += 1;
            }
        }
    };
    Ok(())
}

pub(crate) fn specific_multipatch_check(_id_low: &str, _merged: &Npc, _cfg: &Cfg) -> bool {
    false
}

pub(crate) fn specific_multipatch(
    _merged: &mut Npc,
    _map: &NpcRecordMap,
    _option_log: &mut OptionRecordMergeLog,
    _specific_flags: &mut SpecificFlags,
    _cfg: &Cfg,
) -> Result<()> {
    Ok(())
}
