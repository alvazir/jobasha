use super::{
    fields_are_equal, get_previous_cell_option_field, NonOptAtmosphereData, NonOptMapColor, NonOptRegion, NonOptWaterHeight,
    OptionRecordMergeLog, SpecificFlags,
};
use crate::{CellRecordMap, Cfg};
use anyhow::Result;
use paste::paste;
use tes3::esp::Cell;

pub(super) fn ignore_or_omit_redundant_values(
    interior: bool,
    merged: &mut Cell,
    last: &Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    macro_rules! is_equal_field {
        ($field:ident) => {
            if merged.$field == last.$field {
                true
            } else if last.$field.is_none() {
                specific_flags.changes += 1;
                true
            } else {
                false
            }
        };
    }
    macro_rules! exit_if_fields_are_equal {
        ($struct:ident) => {
            if $struct.equal() {
                specific_flags.equal_after_specific = true;
                return Ok(());
            }
        };
    }
    macro_rules! omit_redundant_values {
        ($fields_equal:ident, $($field:ident),+) => {
            paste! {
                $(if merged.$field.is_some()
                    && [<omit_redundant_values_ $field>](merged, last, map, option_log, specific_flags, cfg)?
                {
                    $fields_equal.$field = true;
                    exit_if_fields_are_equal!($fields_equal);
                })+
            }
        };
    }
    if interior {
        let mut fields_equal = FieldsInt::new(
            if !cfg.merge.ignore_secondary_fog_density {
                fields_are_equal!(merged, last, flags, data:flags, data:grid:1)
            } else {
                fields_are_equal!(merged, last, flags, data:flags)
            },
            // COMMENT: RGNN and NAM5 may be used with 0x80 aka Behave_Like_Exterior cells
            is_equal_field!(region),
            is_equal_field!(map_color),
            is_equal_field!(water_height),
            is_equal_field!(atmosphere_data),
        );
        if fields_equal.other && merged.data.grid != last.data.grid {
            specific_flags.changes += 1;
        }
        exit_if_fields_are_equal!(fields_equal);
        if !cfg.merge.keep_redundant_values {
            omit_redundant_values!(fields_equal, region, map_color, water_height, atmosphere_data);
            if omit_whgt_if_no_has_water_flag(merged, option_log, specific_flags, cfg)? {
                exit_if_fields_are_equal!(fields_equal);
            }
        }
    } else {
        let mut fields_equal = FieldsExt::new(
            fields_are_equal!(merged, last, flags, name, data, water_height, atmosphere_data),
            is_equal_field!(region),
            is_equal_field!(map_color),
        );
        exit_if_fields_are_equal!(fields_equal);
        if !cfg.merge.keep_redundant_values {
            omit_redundant_values!(fields_equal, region, map_color);
        }
    }
    Ok(())
}

struct FieldsInt {
    other: bool,
    region: bool,
    map_color: bool,
    water_height: bool,
    atmosphere_data: bool,
}

impl FieldsInt {
    fn equal(&self) -> bool {
        self.other && self.region && self.map_color && self.water_height && self.atmosphere_data
    }

    fn new(other: bool, region: bool, map_color: bool, water_height: bool, atmosphere_data: bool) -> Self {
        Self {
            other,
            region,
            map_color,
            water_height,
            atmosphere_data,
        }
    }
}

struct FieldsExt {
    other: bool,
    region: bool,
    map_color: bool,
}

impl FieldsExt {
    fn equal(&self) -> bool {
        self.other && self.region && self.map_color
    }

    fn new(other: bool, region: bool, map_color: bool) -> Self {
        Self { other, region, map_color }
    }
}

fn omit_whgt_if_no_has_water_flag(
    merged: &mut Cell,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<bool> {
    if !merged.data.flags.contains(tes3::esp::CellFlags::HAS_WATER) && merged.water_height.is_some() {
        if !cfg.meta.silent {
            option_log.field_changed_redundant(
                "water_height",
                format_args!("{:?}", NonOptWaterHeight(&merged.water_height)),
                format_args!("{:?}", NonOptWaterHeight(&None)),
                cfg,
            )?;
        }
        merged.water_height = None;
        specific_flags.changes += 1;
        Ok(true)
    } else {
        Ok(false)
    }
}

macro_rules! make_omit_redundant_values_functions {
    ($($field:ident),+) => {
        paste! {
$(fn [<omit_redundant_values_ $field>](
    merged: &mut Cell,
    last: &Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<bool> {
    if merged.$field.as_ref() == get_previous_cell_option_field!(map, $field) {
        if !cfg.meta.silent {
            option_log.field_changed_redundant(
                stringify!($field),
                format_args!("{:?}", [<NonOpt $field:camel>](&merged.$field)),
                format_args!("{:?}", "None"),
                cfg,
            )?;
        }
        merged.$field = None;
        if last.$field.is_some() {
            specific_flags.changes += 1;
        }
        Ok(true)
    } else {
        Ok(false)
    }
})+
        }
    };
}

make_omit_redundant_values_functions!(region, map_color, water_height, atmosphere_data);
