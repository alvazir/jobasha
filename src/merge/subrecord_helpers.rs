use super::OptionRecordMergeLog;
mod ai_package;
mod generic;
mod inventory;
mod spells;
mod travel_destinations;
use generic::{add_and_log_field_lengthen, get_vec_element, log_field_shorten};
use inventory::{added_to_owned, commit, get_low_sorted_last};
use spells::make;
pub(super) use {
    ai_package::{ai_package_variant, ai_packages_equal, AiPackagesHelper},
    inventory::{to_lowercase as inventory_to_lowercase, InventoryHelper},
    spells::{to_lowercase as spell_to_lowercase, SpellsHelper},
    travel_destinations::{sort_travel_destinations, to_lowercase as travel_destination_to_lowercase, TravelDestinationsHelper},
};

#[cfg(test)]
pub(super) use ai_package::{test_basic_ai_packages, test_debug_compare_to_the_last_ai_packages, test_log_ai_packages};
#[cfg(test)]
pub(super) use generic::{test_debug_compare_to_the_last_vector_fields, test_logs_vector_fields};
#[cfg(test)]
pub(super) use inventory::test_basic_inventory;
#[cfg(test)]
pub(super) use spells::test_basic_spells_and_travel;
