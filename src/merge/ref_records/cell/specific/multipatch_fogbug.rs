use super::{
    fog_to_float, get_previous_cell_option_field, print_as, NonOptAtmosphereData, NonOptMapColor, NonOptRegion, NonOptWaterHeight,
    OptionRecordMergeLog, SpecificFlags, MULTIPATCH_KIND,
};
use crate::{CellRecordMap, Cfg};
use anyhow::{anyhow, Context, Result};
use tes3::esp::{AtmosphereData, Cell};

pub(super) fn multipatch_fogbug_and_get_ambi_fog_density(
    merged: &mut Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    cfg: &Cfg,
) -> Result<(f32, bool, Option<AtmosphereData>)> {
    let Some(ref mut ambi) = merged.atmosphere_data else {
        let fog_density = fix_ambi(merged, map, option_log, cfg)?;
        return Ok((fog_density, false, None));
    };
    if cfg.multipatch.fogbug {
        if cfg.merge.cell {
            if ambi.fog_density == 0.0 {
                let prev_ambi = get_prev_ambi(ambi, cfg);
                ambi.fog_density = cfg.guts.multipatch_fogbug_fixed_value;
                return Ok((ambi.fog_density, true, prev_ambi));
            }
        } else {
            let Some(last_ambi) = get_previous_cell_option_field!(map, atmosphere_data) else {
                // COMMENT: Clearly a bug, because empty AMBI would go to fix_ambi()
                return Err(anyhow!("Bug: get_previous_cell_option_field() returned None"));
            };
            if last_ambi.fog_density == 0.0 {
                let prev_ambi = get_prev_ambi(ambi, cfg);
                if ambi.fog_density == 0.0 {
                    ambi.fog_density = cfg.guts.multipatch_fogbug_fixed_value;
                }
                return Ok((ambi.fog_density, true, prev_ambi));
            }
        }
    }
    Ok((ambi.fog_density, false, None))
}

pub(super) fn multipatch_fogbug_log(
    prev_ambi: Option<AtmosphereData>,
    merged: &mut Cell,
    last: &Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    if cfg.merge.cell {
        if !cfg.meta.silent {
            if cfg.meta.debug_multipatch_attempt {
                option_log.multipatch_attempt(&map, cfg)?;
            }
            option_log.field_multipatched(
                "fogbug",
                "atmosphere_data",
                format_args!("fog_density: \"0\" -> \"{}\"", cfg.guts.multipatch_fogbug_fixed_value),
                cfg,
            )?;
        }
    } else {
        merged.references.clear();
        if !cfg.meta.silent {
            specific_flags.multipatched = Some(MULTIPATCH_KIND);
            option_log
                .equal_to_the_last_or_clear(&map, cfg)
                .with_context(|| "Bug: failed to succeed equal_to_the_last_or_clear()")?;
            if cfg.meta.debug_multipatch_attempt {
                option_log.multipatch_attempt(&map, cfg)?;
            }
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
            macro_rules! log_changed_field {
                ($field:ident$(:$subfield:ident)?$(::print_as::$print_as:ident)?) => {
                    option_log.field_changed_or_multipatched(
                        "",
                        stringify!($field$(.$subfield)?),
                        format_args!(
                            "{:?} -> {:?}",
                            print_as!($($print_as:)?&last.$field$(.$subfield)?),
                            print_as!($($print_as:)?&merged.$field$(.$subfield)?)
                        ),
                        &map,
                        cfg,
                    )?;
                };
            }
            macro_rules! compare_fields {
                ($($field:ident$(:$subfield:ident)?$(::print_as::$print_as:ident)?),+) => {
                    $(if merged.$field$(.$subfield)? != last.$field$(.$subfield)? {
                        log_changed_field!($field$(:$subfield)?$(::print_as::$print_as)?);
                    })+
                };
            }
            compare_fields!(
                flags,
                data:flags,
                data:grid,
                region::print_as::NonOptRegion,
                map_color::print_as::NonOptMapColor,
                water_height::print_as::NonOptWaterHeight
            );
        }
    }
    Ok(())
}

fn fix_ambi(merged: &mut Cell, map: &CellRecordMap, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<f32> {
    let mut fog_density = fog_to_float(merged.data.grid.1);
    if cfg.multipatch.fogbug && fog_density == 0.0 {
        fog_density = cfg.guts.multipatch_fogbug_fixed_value;
    }
    if !cfg.merge.cell && cfg.multipatch.fogbug {
        option_log.warn(
                format_args!(
                    "Incomplete CELL record: {id:?}: \"atmosphere_data\": None {{ at least one instance of the interior cell must contain atmosphere_data }}",
                    id = map.record_id_debug()?,
                ),
                &map,
                cfg,
            )?;
    } else {
        merged.atmosphere_data = Some(AtmosphereData {
            ambient_color: [0; 4],
            fog_color: [0; 4],
            sunlight_color: [0; 4],
            fog_density,
        });
        option_log.warn(
            format_args!(
                "Fixed CELL record: {id:?}: \"atmosphere_data\": None -> {to:?} {{ at least one instance of the interior cell must contain atmosphere_data }}",
                id = map.record_id_debug()?,
                to = print_as!(NonOptAtmosphereData:&merged.atmosphere_data),
            ),
            &map,
            cfg,
        )?;
    }
    Ok(fog_density)
}

fn get_prev_ambi(ambi: &AtmosphereData, cfg: &Cfg) -> Option<AtmosphereData> {
    if cfg.merge.verbose_atmosphere_data {
        Some(ambi.clone())
    } else {
        None
    }
}
