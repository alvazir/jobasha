use super::{
    assert_eq_inner, merge_cell, test_basic, test_debug_all_equal, test_debug_compare_to_the_last, test_debug_equal_to_the_last,
    test_debug_list_all_plugins, test_debug_single, test_flags, test_init, test_log, test_log_flags, test_merge, AtmosphereData, Cell,
    CellData, CellFlags, MergeLog, ObjectFlags, RawPlugin, TES3Object,
};
use crate::{Cfg, IntermediateRecords, PluginInfo};
use paste::paste;
use pretty_assertions::assert_eq;
use std::iter::repeat;

mod basic;
mod cellnames;
mod complex;
mod fogbug;
mod multi;
mod specific_merge_options;
