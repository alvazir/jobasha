use super::{
    ai_package_variant, ai_packages_equal, inventory_to_lowercase, sort_travel_destinations, spell_to_lowercase,
    travel_destination_to_lowercase, AiPackagesHelper, InventoryHelper, MergeLog, OptionRecordMergeLog, RawPlugin, SpellsHelper,
    TravelDestinationsHelper,
};
use paste::paste;
use tes3::esp::{ObjectFlags, ServiceFlags};
mod bsgn;
mod cell;
mod cont;
mod crea;
mod generic;
mod mgef;
mod npc_;
mod race;
mod skil;
pub(super) use bsgn::merge_bsgn;
pub(super) use cell::merge_cell;
pub(super) use cont::merge_cont;
pub(super) use crea::merge_crea;
use generic::{
    count_changes, fields_are_equal, generic_make_merge, generic_ref_record_method_inventory, generic_ref_record_method_spells,
    generic_ref_record_method_travel_destinations, generic_ref_record_methods, print_as, show_flags, LowInventory,
    LowInventorySpellsTravelDestinations, LowSpells,
};
pub(super) use mgef::merge_mgef;
pub(super) use npc_::merge_npc_;
pub(super) use race::merge_race;
pub(super) use skil::merge_skil;

#[cfg(test)]
mod tests;
#[cfg(test)]
use super::{
    test_basic_ai_packages, test_basic_inventory, test_basic_spells_and_travel, test_debug_compare_to_the_last_ai_packages,
    test_debug_compare_to_the_last_vector_fields, test_log_ai_packages, test_logs_vector_fields,
};
#[cfg(test)]
use tests::{
    assert_eq_inner, test_basic, test_debug, test_debug_all_equal, test_debug_compare_to_the_last, test_debug_equal_to_the_last,
    test_debug_list_all_plugins, test_debug_single, test_flags, test_init, test_log, test_log_flags, test_merge,
};

#[derive(Default)]
struct SpecificFlags {
    changes: usize,
    equal_after_specific: bool,
    multipatched: Option<&'static str>,
}

show_flags!(ObjectFlags, MODIFIED, DELETED, PERSISTENT, IGNORED, BLOCKED);

show_flags!(
    ServiceFlags,
    BARTERS_WEAPONS,
    BARTERS_ARMOR,
    BARTERS_CLOTHING,
    BARTERS_BOOKS,
    BARTERS_INGREDIENTS,
    BARTERS_LOCKPICKS,
    BARTERS_PROBES,
    BARTERS_LIGHTS,
    BARTERS_APPARATUS,
    BARTERS_REPAIR_ITEMS,
    BARTERS_MISC_ITEMS,
    OFFERS_SPELLS,
    BARTERS_ENCHANTED_ITEMS,
    BARTERS_ALCHEMY,
    OFFERS_TRAINING,
    OFFERS_SPELLMAKING,
    OFFERS_ENCHANTING,
    OFFERS_REPAIRS
);
