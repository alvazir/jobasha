use super::{
    count_changes, fields_are_equal, generic_make_merge, generic_ref_record_methods, print_as, show_flags, show_object_flags,
    MergeLog, OptionRecordMergeLog, RawPlugin, SpecificFlags,
};
use crate::{Cfg, IntermediateRecords, RecordMap};
use anyhow::{anyhow, Context, Result};
use hashbrown::HashMap;
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{convert::identity, fmt};
use tes3::esp::{AtmosphereData, Cell, CellData, CellFlags, ObjectFlags, Reference, TES3Object};
mod specific;
use specific::{specific, specific_multipatch, specific_multipatch_check};

#[cfg(test)]
mod tests;
#[cfg(test)]
use super::{
    assert_eq_inner, test_basic, test_debug_all_equal, test_debug_compare_to_the_last, test_debug_equal_to_the_last,
    test_debug_list_all_plugins, test_debug_single, test_flags, test_init, test_log, test_log_flags, test_merge,
};

pub(crate) struct CellRef<'a> {
    pub flags: ObjectFlags,
    pub name: &'a str,
    pub data: CellData,
    pub region: &'a Option<String>,
    pub map_color: &'a Option<[u8; 4]>,
    pub water_height: &'a Option<f32>,
    pub atmosphere_data: &'a Option<AtmosphereData>,
    pub references: &'a HashMap<(u32, u32), Reference>,
}

struct FogDensityGrid((i32, i32));
struct NonOptRegion<'a>(&'a Option<String>);
struct NonOptMapColor<'a>(&'a Option<[u8; 4]>);
struct NonOptWaterHeight<'a>(&'a Option<f32>);
struct NonOptAtmosphereData<'a>(&'a Option<AtmosphereData>);
struct ShowCompactAtmosphereData<'a>(&'a AtmosphereData, &'a AtmosphereData);

// COMMENT: according to tes3cmd secondary fog density is the last byte in interior data
// COMMENT:   my($flags, $unk, $fogden) = unpack("LLf", $buff); // f = float
impl fmt::Debug for FogDensityGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, fog_density: {}({}))", self.0 .0, fog_to_float(self.0 .1), self.0 .1)
    }
}

impl fmt::Debug for NonOptRegion<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            None => write!(f, "None"),
            Some(region) => write!(f, "\"{region}\""),
        }
    }
}

impl fmt::Debug for NonOptMapColor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            None => write!(f, "None"),
            Some(color) => write!(f, "({},{},{})", color[0], color[1], color[2]),
        }
    }
}

impl fmt::Debug for NonOptWaterHeight<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            None => write!(f, "None"),
            Some(water_height) => write!(f, "{water_height:.0}"),
        }
    }
}

impl fmt::Debug for NonOptAtmosphereData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            None => write!(f, "None"),
            Some(atmosphere_data) => write!(
                f,
                "ambient: ({},{},{}), sunlight: ({},{},{}), fog: ({},{},{}), fog_density: {}",
                atmosphere_data.ambient_color[0],
                atmosphere_data.ambient_color[1],
                atmosphere_data.ambient_color[2],
                atmosphere_data.sunlight_color[0],
                atmosphere_data.sunlight_color[1],
                atmosphere_data.sunlight_color[2],
                atmosphere_data.fog_color[0],
                atmosphere_data.fog_color[1],
                atmosphere_data.fog_color[2],
                atmosphere_data.fog_density
            ),
        }
    }
}

impl fmt::Debug for ShowCompactAtmosphereData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut not_empty = false;
        macro_rules! color {
            ($field:ident, $name:literal) => {
                if self.0.$field != self.1.$field {
                    write!(
                        f,
                        "{}{}: ({},{},{}) -> ({},{},{})",
                        if not_empty { ", " } else { "" },
                        $name,
                        &self.0.$field[0],
                        &self.0.$field[1],
                        &self.0.$field[2],
                        &self.1.$field[0],
                        &self.1.$field[1],
                        &self.1.$field[2]
                    )?;
                    not_empty = true;
                };
            };
        }
        color!(ambient_color, "ambient");
        color!(sunlight_color, "sunlight");
        color!(fog_color, "fog");
        if self.0.fog_density != self.1.fog_density {
            write!(
                f,
                "{}fog_density: {} -> {}",
                if not_empty { ", " } else { "" },
                &self.0.fog_density,
                &self.1.fog_density
            )?;
        };
        Ok(())
    }
}

fn show_compact_atmosphere_data<'a, T: RecordMap<'a>>(
    option_log: &mut OptionRecordMergeLog,
    field_name: &'static str,
    opt_merged: &'a Option<AtmosphereData>,
    opt_new: &'a Option<AtmosphereData>,
    plugin_name: &'a str,
    map: &'a T,
    cfg: &Cfg,
) -> Result<()> {
    let Some(merged) = opt_merged else {
        return option_log.field_changed(
            field_name,
            format_args!("None"),
            format_args!("{:?}", NonOptAtmosphereData(opt_new)),
            plugin_name,
            map,
            cfg,
        );
    };
    let Some(new) = opt_new else {
        return Err(anyhow!("Bug: new value must not be None"));
    };
    option_log.field_changed_custom(
        field_name,
        format_args!("{:?}", ShowCompactAtmosphereData(merged, new)),
        plugin_name,
        map,
        cfg,
    )
}

fn show_compact_data_grid<'a, T: RecordMap<'a>>(
    option_log: &mut OptionRecordMergeLog,
    field_name: &'static str,
    merged: (i32, i32),
    new: (i32, i32),
    plugin_name: &'a str,
    map: &'a T,
    cfg: &Cfg,
) -> Result<()> {
    if merged.1 != new.1 {
        option_log.field_changed_custom(
            field_name,
            format_args!("fog_density: {:?} -> {:?}", fog_to_float(merged.1), fog_to_float(new.1),),
            plugin_name,
            map,
            cfg,
        )?;
    }
    Ok(())
}

fn fog_to_float(fog: i32) -> f32 {
    f32::from_le_bytes(fog.to_le_bytes())
}

fn fog_from_float(fog: f32) -> i32 {
    i32::from_le_bytes(fog.to_le_bytes())
}

show_flags!(CellFlags, IS_INTERIOR, HAS_WATER, RESTING_IS_ILLEGAL, BEHAVES_LIKE_EXTERIOR);

generic_ref_record_methods!(
    (CellRef, Cell),
    (name, region, map_color, water_height, atmosphere_data, references),
    (flags, data),
    (),
    (),
    ()
);

generic_make_merge!(
    cell,
    (CellRef, Cell, id),
    (
        flags=ObjectFlags,
        name.&,
        data:flags=CellFlags,
        data:grid::print_as::FogDensityGrid::print_compact::ignore_secondary_fog_density::debug_level_to_log::debug_level_merge_interior_grid_change,
        region.&::print_as::NonOptRegion::revert_to_none::false,
        map_color.&::print_as::NonOptMapColor::revert_to_none::false,
        water_height.&::print_as::NonOptWaterHeight::revert_to_none::false,
        atmosphere_data.&::print_as::NonOptAtmosphereData::print_compact::verbose_atmosphere_data::revert_to_none::false,
        references.&
    ),
    (),
    (),
    (),
    (specific)
);
