// COMMENT: some tests are too bothersome to make for interior, though doesn't add value
use super::{assert_eq, *};

mod object_flags {
    use super::{assert_eq, *};

    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_object_flags:flags, (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
        test_flags!(cell, Cell, values_object_flags:flags, (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
    }

    mod exterior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_object_flags:flags; id=data:grid=>(1, 1)=>(2, 2));
        test_flags!(cell, Cell, values_object_flags:flags; id=data:grid=>(1, 1)=>(2, 2));

        mod log {
            use super::{assert_eq, *};
            mod default {
                use super::{assert_eq, *};
                test_log_flags!(
                    cell,
                    Cell,
                    false,
                    "Merged CELL record: \"\"\n",
                    "Merging CELL record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\nMerged CELL record: \"\"\n",
                    values_object_flags:flags
                );
            }

            mod plus_before_minus {
                use super::{assert_eq, *};
                test_log_flags!(
                    cell,
                    Cell,
                    true,
                    "Merged CELL record: \"\"\n",
                    "Merging CELL record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\nMerged CELL record: \"\"\n",
                    values_object_flags:flags
                );
            }
        }
    }
}

mod name {
    use super::{assert_eq, *};

    // COMMENT: only works for exterior obviously
    mod exterior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_string:name; id=data:grid=>(1, 1)=>(2, 2));
    }
}

mod data_flags {
    use super::{assert_eq, *};

    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_interior_flags:data:flags, (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
    }

    mod exterior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_exterior_flags:data:flags; id=data:grid=>(1, 1)=>(2, 2));
        test_flags!(cell, Cell, values_exterior_flags:data:flags; id=data:grid=>(1, 1)=>(2, 2));

        mod log {
            use super::{assert_eq, *};
            mod default {
                use super::{assert_eq, *};
                test_log_flags!(
                    cell,
                    Cell,
                    false,
                    "Merged CELL record: \"\"\n",
                    "Merging CELL record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"data.flags\": - HAS_WATER [\"Plugin1.esp\"]\n\"data.flags\": + BEHAVES_LIKE_EXTERIOR [\"Plugin1.esp\"]\nMerged CELL record: \"\"\n",
                    values_exterior_flags:data:flags
                );
            }

            mod plus_before_minus {
                use super::{assert_eq, *};
                test_log_flags!(
                    cell,
                    Cell,
                    true,
                    "Merged CELL record: \"\"\n",
                    "Merging CELL record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"data.flags\": + BEHAVES_LIKE_EXTERIOR [\"Plugin1.esp\"]\n\"data.flags\": - HAS_WATER [\"Plugin1.esp\"]\nMerged CELL record: \"\"\n",
                    values_exterior_flags:data:flags
                );
            }
        }
    }
}

mod data_grid {
    use super::{assert_eq, *};

    // COMMENT: only works for interior obviously
    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_data_grid:data:grid, values_object_flags:flags, (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true; cfg=merge:ignore_secondary_fog_density = true);
    }
}

mod region {
    use super::{assert_eq, *};

    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_region:region, (region = Some(String::from("string"))), (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
    }

    mod exterior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_region:region, (region = Some(String::from("string"))); id=data:grid=>(1, 1)=>(2, 2));
    }
}

mod map_color {
    use super::{assert_eq, *};

    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_map_color:map_color, (map_color = Some([0,1,2,0])), (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
    }

    mod exterior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_map_color:map_color, (map_color = Some([0,1,2,0])); id=data:grid=>(1, 1)=>(2, 2));
    }
}

mod water_height {
    use super::{assert_eq, *};

    // COMMENT: only works for interior obviously
    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_water_height:water_height, (water_height = Some(0.1)), (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
    }
}

mod atmosphere_data {
    use super::{assert_eq, *};

    // COMMENT: only works for interior obviously
    mod interior {
        use super::{assert_eq, *};
        test_basic!(cell, Cell, values_atmosphere_data:atmosphere_data, (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) }), (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})); id=name=>String::from("cell1")=>String::from("cell2"); cfg=merge:keep_redundant_values = true);
    }
}

mod log {
    use super::{assert_eq, *};

    mod exterior {
        use super::{assert_eq, *};
        test_log!(
            cell,
            Cell,
            "Merged CELL record: \"\"\n",
            "Merging CELL record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"name\": \"\" -> \"string_1\" [\"Plugin1.esp\"]\nMerged CELL record: \"\"\n",
            values_string:name
        );
    }
}

mod debug {
    use super::{assert_eq, *};
    mod exterior {
        use super::{assert_eq, *};
        mod compare_to_the_last {
            use super::{assert_eq, *};
            test_debug_compare_to_the_last!(
                cell,
                Cell,
                "Comparing to the last instance",
                "Comparing to the last instance of CELL record: \"\" [\"Plugin2.esp\"]\n\"name\": \"string_1\" != \"\"\nMerged CELL record: \"\"",
                values_string:name
            );
        }

        mod test_list_all_plugins {
            use super::{assert_eq, *};
            test_debug_list_all_plugins!(
                cell,
                Cell,
                "[\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]",
                "Merging CELL record: \"\" [\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]",
                values_string:name
            );
        }

        mod test_equal_to_the_last {
            use super::{assert_eq, *};
            test_debug_equal_to_the_last!(
                cell,
                Cell,
                "Skipped CELL record: \"\" { Equal to the last instance }",
                values_string:name
            );
        }

        mod test_all_equal {
            use super::{assert_eq, *};
            test_debug_all_equal!(
                cell,
                Cell,
                "Skipped CELL record: \"\" { All 2 instances are equal in [\"Plugin0.esp\", \"Plugin1.esp\"] }",
                "Skipped CELL record: \"\" { All 3 instances are equal in [\"Plugin0.esp\"...\"Plugin2.esp\"] }",
                values_string:name
            );
        }

        mod test_single {
            use super::{assert_eq, *};
            test_debug_single!(
                cell,
                Cell,
                "Skipped CELL record: \"\" { The only instance is in [\"Plugin0.esp\"] }",
                values_string:name
            );
        }
    }
}
