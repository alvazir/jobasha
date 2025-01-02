use tes3::esp::TravelDestination;
mod count_changes_macro;
mod fields_are_equal_macro;
mod make_merge_macro;
mod method_macro;
mod print_as_macro;
mod show_flags_macro;
pub(super) use count_changes_macro::count_changes;
pub(super) use fields_are_equal_macro::fields_are_equal;
pub(super) use make_merge_macro::generic_make_merge;
pub(super) use method_macro::{
    generic_ref_record_method_inventory, generic_ref_record_method_spells, generic_ref_record_method_travel_destinations,
    generic_ref_record_methods,
};
pub(super) use print_as_macro::print_as;
pub(super) use show_flags_macro::show_flags;

#[derive(Default)]
pub(super) struct LowInventory {
    pub(super) inventory: Vec<(i32, String)>,
}

#[derive(Default)]
pub(super) struct LowSpells {
    pub(super) spells: Vec<String>,
}

#[derive(Default)]
pub(super) struct LowInventorySpellsTravelDestinations {
    pub(super) inventory: Vec<(i32, String)>,
    pub(super) spells: Vec<String>,
    pub(super) travel_destinations: Vec<TravelDestination>,
}
