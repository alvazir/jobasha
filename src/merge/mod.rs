use crate::{Cfg, IntermediateRecords, ListCounts, LlCreatureRecords, LlItemRecords, LlMessages, Log};
use anyhow::{Context, Result};
use paste::paste;
use rand::rngs::ThreadRng;
mod log;
mod raw_plugins;
mod ref_records;
mod subrecord_helpers;
use log::{MergeLog, OptionRecordMergeLog};
use raw_plugins::RawPlugin;
pub(crate) use raw_plugins::RawPlugins;
use ref_records::{merge_bsgn, merge_cell, merge_cont, merge_crea, merge_mgef, merge_npc_, merge_race, merge_skil};
use subrecord_helpers::{
    ai_package_variant, ai_packages_equal, inventory_to_lowercase, sort_travel_destinations, spell_to_lowercase,
    travel_destination_to_lowercase, AiPackagesHelper, InventoryHelper, SpellsHelper, TravelDestinationsHelper,
};

#[cfg(test)]
use subrecord_helpers::{
    test_basic_ai_packages, test_basic_inventory, test_basic_spells_and_travel, test_debug_compare_to_the_last_ai_packages,
    test_debug_compare_to_the_last_vector_fields, test_log_ai_packages, test_logs_vector_fields,
};

pub(super) fn merge_records<'a>(
    creatures: LlCreatureRecords<'a>,
    items: LlItemRecords<'a>,
    im2_records: IntermediateRecords,
    cfg: &'a Cfg,
    log: &mut Log,
) -> Result<(RawPlugins<'a>, ListCounts, i32)> {
    let mut raw_plugins = RawPlugins::new(cfg);
    let mut counts = ListCounts::default();
    let mut messages = LlMessages::new();
    if cfg.compare_only {
        return Ok((raw_plugins, counts, messages.exit_code()));
    }
    let mut merge_log = MergeLog::default();
    if !(cfg.merge.skip && cfg.multipatch.skip) {
        macro_rules! select_merge_condition {
            ($short:ident) => {
                cfg.merge.$short
            };
            ($short:ident::$condition:expr) => {
                select_merge_condition!($short) || $condition
            };
        }
        macro_rules! merge {
            ($($short:ident$(::$condition:expr)?),+) => { paste! {
                $(if select_merge_condition!($short$(::$condition)?) {
                    [<merge_ $short>](&im2_records, &mut raw_plugins.merge, cfg, &mut merge_log)
                        .with_context(|| format!("Failed to merge {}", stringify!($short).to_uppercase()))?;
                })+
            }};
        }
        merge!(
            race,
            skil,
            mgef,
            bsgn,
            cont,
            crea::cfg.multipatch.summons,
            npc_,
            cell::cfg.multipatch.cellnames || cfg.multipatch.fogbug
        );
    }

    let mut rng = ThreadRng::default();
    if !cfg.creatures.skip {
        creatures.make_levc(&mut raw_plugins, &mut messages, &mut counts, &mut rng, cfg, log)?;
    }
    if !cfg.items.skip {
        items.make_levi(&mut raw_plugins, &mut messages, &mut counts, &mut rng, cfg, log)?;
    }

    if !cfg.meta.silent {
        merge_log.msg(cfg, log)?;
        messages.show(&counts, cfg, log)?;
    }

    Ok((raw_plugins, counts, messages.exit_code()))
}
