use super::{fog_from_float, print_as, FogDensityGrid, OptionRecordMergeLog};
use crate::{CellRecordMap, Cfg};
use anyhow::Result;
use tes3::esp::Cell;

pub(super) fn sync_secondary_fog_density(
    ambi_fog_density: f32,
    grid_fog_density: f32,
    merged: &mut Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    cfg: &Cfg,
) -> Result<()> {
    let new = fog_from_float(ambi_fog_density);
    if !cfg.meta.silent {
        if cfg.debug >= cfg.guts.debug_level_merge_interior_grid_change {
            option_log.field_changed_fog_density(
                format_args!("{:?}", print_as!(FogDensityGrid:merged.data.grid)),
                format_args!("{:?}", print_as!(FogDensityGrid:(merged.data.grid.0, new))),
                &map,
                cfg,
            )?;
        } else {
            option_log.field_changed_fog_density(
                format_args!("{:?}", grid_fog_density),
                format_args!("{:?}", ambi_fog_density),
                &map,
                cfg,
            )?;
        }
    }
    merged.data.grid.1 = new;
    Ok(())
}
