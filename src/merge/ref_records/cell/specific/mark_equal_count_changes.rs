use super::{count_changes, SpecificFlags};
use crate::Cfg;
use tes3::esp::Cell;

pub(super) fn mark_equal_count_changes(interior: bool, merged: &mut Cell, last: &Cell, specific_flags: &mut SpecificFlags, cfg: &Cfg) {
    specific_flags.equal_after_specific = true;
    if cfg.meta.debug_compare_equal {
        if interior {
            count_changes!(
                merged,
                last,
                specific_flags,
                flags,
                data:flags,
                data:grid,
                region,
                map_color,
                water_height,
                atmosphere_data
            );
        } else {
            count_changes!(
                merged,
                last,
                specific_flags,
                flags,
                name,
                data:flags,
                region,
                map_color
            );
        }
    }
}
