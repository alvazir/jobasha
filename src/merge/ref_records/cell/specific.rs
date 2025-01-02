use super::{
    count_changes, fields_are_equal, fog_from_float, fog_to_float, print_as, FogDensityGrid, NonOptAtmosphereData, NonOptMapColor,
    NonOptRegion, NonOptWaterHeight, OptionRecordMergeLog, SpecificFlags,
};
use crate::{CellKey, CellRecordMap, Cfg};
use anyhow::{Context, Result};
use tes3::esp::Cell;
mod get_previous_cell_option_field;
mod mark_equal_count_changes;
mod multipatch_cellnames;
mod multipatch_fogbug;
mod primitive;
mod redundant_values;
mod sync_secondary_fog_density;
use get_previous_cell_option_field::get_previous_cell_option_field;
use mark_equal_count_changes::mark_equal_count_changes;
use multipatch_cellnames::multipatch_cellnames;
use multipatch_fogbug::{multipatch_fogbug_and_get_ambi_fog_density, multipatch_fogbug_log};
use primitive::{primitive_exterior, primitive_interior};
use redundant_values::ignore_or_omit_redundant_values;
use sync_secondary_fog_density::sync_secondary_fog_density;

const MULTIPATCH_KIND: &str = "fogbug";

macro_rules! sync_secondary_fog_density {
    ($ambi:expr, $merged:ident, $map:ident, $option_log:ident, $cfg:ident) => {
        if !$cfg.merge.ignore_secondary_fog_density {
            let grid_fog_density = fog_to_float($merged.data.grid.1);
            if grid_fog_density != $ambi {
                sync_secondary_fog_density($ambi, grid_fog_density, $merged, $map, $option_log, $cfg)?;
            }
        }
    };
}

pub(crate) fn specific(
    id_low: &CellKey,
    merged: &mut Cell,
    last: &Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    if let CellKey::Exterior(_) = id_low {
        if cfg.meta.skip_exterior {
            mark_equal_count_changes(false, merged, last, specific_flags, cfg);
            return Ok(());
        }
        // COMMENT: exterior cells seem to always have HAS_WATER flag and it makes sense
        if cfg.meta.multipatch_cellnames {
            if merged.name != last.name {
                if cfg.multipatch.primitive && !cfg.merge.cell {
                    primitive_exterior(merged, last);
                }
                multipatch_cellnames(merged, last, map, option_log, specific_flags, cfg)
                    .with_context(|| "Bug: failed to multipatch_cellnames()")?;
            } else {
                if cfg.meta.debug_multipatch_attempt {
                    option_log.multipatch_attempt(&map, cfg)?;
                }
                mark_equal_count_changes(false, merged, last, specific_flags, cfg);
                return Ok(());
            }
        }
        ignore_or_omit_redundant_values(false, merged, last, map, option_log, specific_flags, cfg)
            .with_context(|| "Bug: failed to ignore_or_omit_redundant_values()")?;
    } else {
        if cfg.meta.skip_interior {
            mark_equal_count_changes(true, merged, last, specific_flags, cfg);
            return Ok(());
        }
        if cfg.meta.fix_fog {
            let (ambi_fog_density, multipatched, prev_ambi) = multipatch_fogbug_and_get_ambi_fog_density(merged, map, option_log, cfg)
                .with_context(|| "Bug: failed to multipatch_fogbug_and_get_ambi_fog_density()")?;
            if cfg.multipatch.fogbug {
                if multipatched {
                    if cfg.multipatch.primitive && !cfg.merge.cell {
                        primitive_interior(merged, last, map, cfg)
                            .with_context(|| "Bug: failed to primitive_interior() while multipatching")?;
                    }
                    multipatch_fogbug_log(prev_ambi, merged, last, map, option_log, specific_flags, cfg)
                        .with_context(|| "Bug: failed to multipatch_fogbug_log()")?;
                } else if !cfg.merge.cell {
                    if cfg.meta.debug_multipatch_attempt {
                        option_log.multipatch_attempt(&map, cfg)?;
                    }
                    mark_equal_count_changes(true, merged, last, specific_flags, cfg);
                    return Ok(());
                }
            }
            sync_secondary_fog_density!(ambi_fog_density, merged, map, option_log, cfg);
        }
        ignore_or_omit_redundant_values(true, merged, last, map, option_log, specific_flags, cfg)
            .with_context(|| "Bug: failed to ignore_or_omit_redundant_values()")?;
    }
    Ok(())
}

pub(crate) fn specific_multipatch_check(id_low: &CellKey, merged: &Cell, cfg: &Cfg) -> bool {
    if let CellKey::Interior(_) = id_low {
        if let Some(ref ambi) = merged.atmosphere_data {
            if ambi.fog_density == 0.0 || (!cfg.merge.ignore_secondary_fog_density && merged.data.grid.1 == 0) {
                return true;
            }
        }
        // COMMENT: atmosphere_data.is_none() may not be broken technically
    }
    false
}

pub(crate) fn specific_multipatch(
    merged: &mut Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    let (ambi_fog_density, multipatched, prev_ambi) = multipatch_fogbug_and_get_ambi_fog_density(merged, map, option_log, cfg)
        .with_context(|| "Bug: failed to multipatch_fogbug_and_get_ambi_fog_density()")?;
    merged.references.clear();
    if !cfg.meta.silent {
        specific_flags.multipatched = Some(MULTIPATCH_KIND);
        if multipatched {
            if cfg.merge.verbose_atmosphere_data {
                option_log.field_changed_or_multipatched(
                    MULTIPATCH_KIND,
                    "atmosphere_data",
                    format_args!(
                        "{:?} -> {:?}",
                        print_as!(NonOptAtmosphereData:&prev_ambi),
                        print_as!(NonOptAtmosphereData:&merged.atmosphere_data)
                    ),
                    &map,
                    cfg,
                )?;
            } else {
                option_log.field_changed_or_multipatched(
                    MULTIPATCH_KIND,
                    "atmosphere_data",
                    format_args!("fog_density: \"0\" -> \"{}\"", cfg.guts.multipatch_fogbug_fixed_value),
                    &map,
                    cfg,
                )?;
            }
        }
    }
    sync_secondary_fog_density!(ambi_fog_density, merged, map, option_log, cfg);
    ignore_or_omit_redundant_values(true, merged, &map.record(0)?.cell, map, option_log, specific_flags, cfg)
        .with_context(|| "Bug: failed to ignore_or_omit_redundant_values()")?;
    Ok(())
}
