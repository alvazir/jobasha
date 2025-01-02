use super::{
    ai_package_variant, ai_packages_equal, fields_are_equal, generic_make_merge, generic_ref_record_method_inventory,
    generic_ref_record_method_spells, generic_ref_record_method_travel_destinations, generic_ref_record_methods,
    inventory_to_lowercase, print_as, show_flags, show_object_flags, show_service_flags, sort_travel_destinations, spell_to_lowercase,
    travel_destination_to_lowercase, AiPackagesHelper, InventoryHelper, LowInventorySpellsTravelDestinations, MergeLog,
    OptionRecordMergeLog, RawPlugin, SpecificFlags, SpellsHelper, TravelDestinationsHelper,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{convert::identity, mem::discriminant};
use tes3::esp::{AiData, AiPackage, FixedString, Npc, NpcData, NpcFlags, ObjectFlags, ServiceFlags, TES3Object, TravelDestination};
mod self_macro;
mod specific;
use self_macro::{merge_data_stats_fields, merge_data_stats_fields_inner_loop, npc_compare_data_stats, npc_process_data_stats};
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

const HEALTH: &str = "data.stats.health";
const MAGICKA: &str = "data.stats.magicka";
const FATIGUE: &str = "data.stats.fatigue";

const ATTRIBUTES: [&str; 8] = [
    "data.stats.strength",
    "data.stats.intelligence",
    "data.stats.willpower",
    "data.stats.agility",
    "data.stats.speed",
    "data.stats.endurance",
    "data.stats.personality",
    "data.stats.luck",
];

const SKILLS: [&str; 27] = [
    "data.stats.block",
    "data.stats.armorer",
    "data.stats.medium_armor",
    "data.stats.heavy_armor",
    "data.stats.blunt_weapon",
    "data.stats.long_blade",
    "data.stats.axe",
    "data.stats.spear",
    "data.stats.athletics",
    "data.stats.enchant",
    "data.stats.destruction",
    "data.stats.alteration",
    "data.stats.illusion",
    "data.stats.conjuration",
    "data.stats.mysticism",
    "data.stats.restoration",
    "data.stats.alchemy",
    "data.stats.unarmored",
    "data.stats.security",
    "data.stats.sneak",
    "data.stats.acrobatics",
    "data.stats.light_armor",
    "data.stats.short_blade",
    "data.stats.marksman",
    "data.stats.mercantile",
    "data.stats.speechcraft",
    "data.stats.hand_to_hand",
];

#[allow(non_camel_case_types)]
pub(crate) struct Npc_Ref<'a> {
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
    pub race: &'a str,
    pub class: &'a str,
    pub faction: &'a str,
    pub head: &'a str,
    pub hair: &'a str,
    pub npc_flags: NpcFlags,
    pub blood_type: &'a u8,
    pub data: NpcData,
    pub base: &'a Npc,
    pub low: LowInventorySpellsTravelDestinations,
}

show_flags!(NpcFlags, FEMALE, ESSENTIAL, RESPAWN, IS_BASE, AUTO_CALCULATE);

generic_ref_record_method_inventory!(Npc_Ref);
generic_ref_record_method_spells!(Npc_Ref);
generic_ref_record_method_travel_destinations!(Npc_Ref);

generic_ref_record_methods!(
    (Npc_Ref, Npc, id),
    (name, script, mesh, race, class, faction, head, hair, blood_type),
    (flags, ai_data, npc_flags, data),
    (LowInventorySpellsTravelDestinations),
    (inventory, spells, travel_destinations),
    (ai_packages)
);

generic_make_merge!(
    npc_,
    (Npc_Ref, Npc, id),
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
        race.&,
        class.&,
        faction.&,
        head.&,
        hair.&,
        npc_flags=NpcFlags,
        blood_type.&,
        data:level,
        data:disposition,
        data:reputation,
        data:rank,
        data:gold
    ),
    (inventory, spells, travel_destinations),
    (ai_packages),
    (npc_process_data_stats, npc_compare_data_stats),
    (specific)
);
