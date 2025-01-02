use super::{
    assert_eq_inner, merge_npc_, test_basic, test_basic_ai_packages, test_basic_inventory, test_basic_spells_and_travel,
    test_debug_all_equal, test_debug_compare_to_the_last, test_debug_compare_to_the_last_ai_packages,
    test_debug_compare_to_the_last_vector_fields, test_debug_equal_to_the_last, test_debug_list_all_plugins, test_debug_single,
    test_flags, test_init, test_log, test_log_ai_packages, test_log_flags, test_logs_vector_fields, test_merge, AiPackage, MergeLog,
    Npc, RawPlugin,
};
use crate::{Cfg, IntermediateRecords, PluginInfo};
use paste::paste;
use pretty_assertions::assert_eq;
use std::iter::repeat;
use tes3::esp::{
    AiActivatePackage, AiEscortPackage, AiFollowPackage, AiTravelPackage, AiWanderPackage, FixedString, NpcData, NpcFlags, NpcStats,
    ObjectFlags, ServiceFlags, TES3Object, TravelDestination,
};

mod ai_packages;
mod basic;
mod complex;
mod multi;
mod npc_stats;
mod redundant_values;
mod travel_destinations;
