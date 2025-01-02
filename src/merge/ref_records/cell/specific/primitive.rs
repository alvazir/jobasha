use super::get_previous_cell_option_field;
use crate::{CellRecordMap, Cfg};
use anyhow::{anyhow, Result};
use tes3::esp::Cell;

pub(super) fn primitive_exterior(merged: &mut Cell, last: &Cell) {
    let merged_name = merged.name.clone();
    *merged = last.clone();
    merged.name = merged_name;
}

pub(super) fn primitive_interior(merged: &mut Cell, last: &Cell, map: &CellRecordMap, cfg: &Cfg) -> Result<()> {
    *merged = last.clone();
    if merged.atmosphere_data.is_none() {
        merged.atmosphere_data = get_previous_cell_option_field!(map, atmosphere_data).cloned();
    }
    let Some(ref mut ambi) = merged.atmosphere_data else {
        // COMMENT: Clearly a bug, because empty AMBI would be set previously
        return Err(anyhow!("Bug: merged.atmosphere_data.is_none()"));
    };
    ambi.fog_density = cfg.guts.multipatch_fogbug_fixed_value;
    Ok(())
}
