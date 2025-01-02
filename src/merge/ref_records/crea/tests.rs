use super::{
    assert_eq_inner, merge_crea, test_basic, test_basic_ai_packages, test_basic_inventory, test_basic_spells_and_travel,
    test_debug_all_equal, test_debug_compare_to_the_last, test_debug_compare_to_the_last_ai_packages,
    test_debug_compare_to_the_last_vector_fields, test_debug_equal_to_the_last, test_debug_list_all_plugins, test_debug_single,
    test_flags, test_init, test_log, test_log_ai_packages, test_log_flags, test_logs_vector_fields, test_merge, AiPackage, Creature,
    MergeLog, RawPlugin,
};
use crate::{Cfg, IntermediateRecords, PluginInfo};
use paste::paste;
use pretty_assertions::assert_eq;
use std::iter::repeat;
use tes3::esp::{
    AiActivatePackage, AiEscortPackage, AiFollowPackage, AiTravelPackage, AiWanderPackage, CreatureFlags, CreatureType, FixedString,
    ObjectFlags, ServiceFlags, TES3Object, TravelDestination,
};

mod ai_packages;
mod basic;
mod complex;
mod multi;
mod summons;
mod travel_destinations;
