use super::{
    ai_package_variant, ai_packages_equal, count_changes, generic_make_merge, generic_ref_record_method_inventory,
    generic_ref_record_method_spells, generic_ref_record_method_travel_destinations, generic_ref_record_methods,
    inventory_to_lowercase, print_as, show_flags, show_object_flags, show_service_flags, sort_travel_destinations, spell_to_lowercase,
    travel_destination_to_lowercase, AiPackagesHelper, InventoryHelper, LowInventorySpellsTravelDestinations, MergeLog,
    OptionRecordMergeLog, RawPlugin, SpecificFlags, SpellsHelper, TravelDestinationsHelper,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{convert::identity, fmt, mem::discriminant};
use tes3::esp::{
    AiData, AiPackage, Creature, CreatureData, CreatureFlags, FixedString, ObjectFlags, ServiceFlags, TES3Object, TravelDestination,
};
mod specific;
use specific::{specific, specific_multipatch, specific_multipatch_check};

#[cfg(test)]
mod tests;
#[cfg(test)]
use super::{
    assert_eq_inner, test_basic, test_basic_ai_packages, test_basic_inventory, test_basic_spells_and_travel, test_debug_all_equal,
    test_debug_compare_to_the_last, test_debug_compare_to_the_last_ai_packages, test_debug_compare_to_the_last_vector_fields,
    test_debug_equal_to_the_last, test_debug_list_all_plugins, test_debug_single, test_flags, test_init, test_log,
    test_log_ai_packages, test_log_flags, test_logs_vector_fields, test_merge,
};

const SUMMONS: [&str; 22] = [
    "ancestor_ghost_summon",
    "ancestor_ghost_variner",
    "atronach_flame_summon",
    "atronach_frost_summon",
    "atronach_storm_summon",
    "bm_bear_black_summon",
    "bm_wolf_bone_summon",
    "bm_wolf_grey_summon",
    "bonelord_summon",
    "bonewalker_greater_summ",
    "bonewalker_summon",
    "centurion_fire_dead",
    "centurion_sphere_summon",
    "clannfear_summon",
    "daedroth_summon",
    "dremora_summon",
    "fabricant_summon",
    "golden saint_summon",
    "hunger_summon",
    "scamp_summon",
    "skeleton_summon",
    "wraith_sul_senipul",
];

pub(crate) struct CreaRef<'a> {
    pub flags: ObjectFlags,
    pub id: &'a str,
    pub name: &'a str,
    pub script: &'a str,
    pub mesh: &'a str,
    pub inventory: Vec<(i32, FixedString<32>)>,
    pub spells: Vec<String>,
    pub ai_data: AiData,
    pub ai_packages: Vec<AiPackage>,
    pub travel_destinations: Vec<TravelDestination>,
    pub sound: &'a str,
    pub scale: &'a Option<f32>,
    pub creature_flags: CreatureFlags,
    pub blood_type: &'a u8,
    pub data: CreatureData,
    pub base: &'a Creature,
    pub low: LowInventorySpellsTravelDestinations,
}

struct NonOptScale<'a>(&'a Option<f32>);

impl fmt::Debug for NonOptScale<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:.2}",
            match self.0 {
                None => &1.0,
                Some(value) => value,
            }
        )
    }
}

show_flags!(
    CreatureFlags,
    BIPED,
    RESPAWN,
    WEAPON_AND_SHIELD,
    IS_BASE,
    SWIMS,
    FLIES,
    WALKS,
    ESSENTIAL
);

generic_ref_record_method_inventory!(CreaRef);
generic_ref_record_method_spells!(CreaRef);
generic_ref_record_method_travel_destinations!(CreaRef);

generic_ref_record_methods!(
    (CreaRef, Creature, id),
    (name, script, mesh, sound, scale, blood_type),
    (flags, ai_data, creature_flags, data),
    (LowInventorySpellsTravelDestinations),
    (inventory, spells, travel_destinations),
    (ai_packages)
);

generic_make_merge!(
    crea,
    (CreaRef, Creature, id),
    (
        flags=ObjectFlags,
        name.&,
        script.&,
        mesh.&,
        ai_data:hello,
        ai_data:fight,
        ai_data:flee,
        ai_data:alarm,
        ai_data:services=ServiceFlags,
        sound.&,
        scale.&::print_as::NonOptScale,
        creature_flags=CreatureFlags,
        blood_type.&,
        data:creature_type,
        data:level,
        data:strength,
        data:intelligence,
        data:willpower,
        data:agility,
        data:speed,
        data:endurance,
        data:personality,
        data:luck,
        data:health,
        data:magicka,
        data:fatigue,
        data:soul,
        data:combat,
        data:magic,
        data:stealth,
        data:gold,
        data:attack1:0,
        data:attack1:1,
        data:attack2:0,
        data:attack2:1,
        data:attack3:0,
        data:attack3:1
    ),
    (inventory, spells, travel_destinations),
    (ai_packages),
    (),
    (specific)
);
