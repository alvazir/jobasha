use super::{assert_eq, *};

mod object_flags {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_object_flags:flags);
    test_flags!(crea, Creature, values_object_flags:flags);

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_log_flags!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_object_flags:flags
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_log_flags!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_object_flags:flags
            );
        }
    }
}

mod name {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_string:name);
}

mod inventory {
    use super::{assert_eq, *};
    test_basic_inventory!(crea, Creature, values_inventory);
    test_basic!(crea, Creature, values_inventory:inventory);

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"inventory\": - \"inventory_1\"(1) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_2\"(2) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_3\"(3) [\"Plugin1.esp\"]\n\"inventory\": + \"inventory_4\"(4) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_inventory:inventory
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"inventory\": + \"inventory_4\"(4) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_1\"(1) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_2\"(2) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_3\"(3) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_inventory:inventory
            );
        }
    }
}

mod spells {
    use super::{assert_eq, *};
    test_basic_spells_and_travel!(crea, Creature, spells, values_spells);
    test_basic!(crea, Creature, values_spells:spells);

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"spells\": - \"spell_1\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_2\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_3\" [\"Plugin1.esp\"]\n\"spells\": + \"spell_4\" [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_spells:spells
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"spells\": + \"spell_4\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_1\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_2\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_3\" [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_spells:spells
            );
        }
    }

    mod debug {
        use super::{assert_eq, *};

        mod compare_to_the_last {
            use super::{assert_eq, *};
            test_debug_compare_to_the_last_vector_fields!(
                crea,
                Creature,
                "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"spells\": quantity 2 != 1\nMerged CREA record: \"\"",
                "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"spells\": different despite equal quantity\nMerged CREA record: \"\"",
                values_spells:spells
            );
        }
    }
}

mod ai_data_fight {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_i8:ai_data:fight);
}

mod ai_data_hello {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_i16:ai_data:hello);
}

mod service_flags {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_service_flags:ai_data:services);
    test_flags!(crea, Creature, values_service_flags:ai_data:services);

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_log_flags!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"ai_data.services\": - BARTERS_ARMOR [\"Plugin1.esp\"]\n\"ai_data.services\": - BARTERS_CLOTHING [\"Plugin1.esp\"]\n\"ai_data.services\": + BARTERS_BOOKS [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_service_flags:ai_data:services
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_log_flags!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"ai_data.services\": + BARTERS_BOOKS [\"Plugin1.esp\"]\n\"ai_data.services\": - BARTERS_ARMOR [\"Plugin1.esp\"]\n\"ai_data.services\": - BARTERS_CLOTHING [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_service_flags:ai_data:services
            );
        }
    }
}

mod scale {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_scale:scale);
}

mod creature_flags {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_creature_flags:creature_flags);
    test_flags!(crea, Creature, values_creature_flags:creature_flags);

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_log_flags!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"creature_flags\": - RESPAWN [\"Plugin1.esp\"]\n\"creature_flags\": - WEAPON_AND_SHIELD [\"Plugin1.esp\"]\n\"creature_flags\": + IS_BASE [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_creature_flags:creature_flags
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_log_flags!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"creature_flags\": + IS_BASE [\"Plugin1.esp\"]\n\"creature_flags\": - RESPAWN [\"Plugin1.esp\"]\n\"creature_flags\": - WEAPON_AND_SHIELD [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_creature_flags:creature_flags
            );
        }
    }
}

mod blood_type {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_u8:blood_type);
}

mod data_creature_type {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_creature_type:data:creature_type);
}

mod data_magicka {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_u32:data:magicka);
}

mod data_attack2_0 {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_u32:data:attack2:0);
}

mod data_attack1_1 {
    use super::{assert_eq, *};
    test_basic!(crea, Creature, values_u32:data:attack1:1);
}

mod log {
    use super::{assert_eq, *};

    test_log!(
        crea,
        Creature,
        "Merged CREA record: \"\"\n",
        "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"sound\": \"\" -> \"string_1\" [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
        values_string:sound
    );
}

mod debug {
    use super::{assert_eq, *};

    mod compare_to_the_last {
        use super::{assert_eq, *};
        test_debug_compare_to_the_last!(
            crea,
            Creature,
            "Comparing to the last instance",
            "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"name\": \"string_1\" != \"\"\nMerged CREA record: \"\"",
            values_string:name
        );
    }

    mod test_list_all_plugins {
        use super::{assert_eq, *};
        test_debug_list_all_plugins!(
            crea,
            Creature,
            "[\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]",
            "Merging CREA record: \"\" [\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]",
            values_i8:ai_data.alarm
        );
    }

    mod test_equal_to_the_last {
        use super::{assert_eq, *};
        test_debug_equal_to_the_last!(
            crea,
            Creature,
            "Skipped CREA record: \"\" { Equal to the last instance }",
            values_u32:data.willpower
        );
    }

    mod test_all_equal {
        use super::{assert_eq, *};
        test_debug_all_equal!(
            crea,
            Creature,
            "Skipped CREA record: \"\" { All 2 instances are equal in [\"Plugin0.esp\", \"Plugin1.esp\"] }",
            "Skipped CREA record: \"\" { All 3 instances are equal in [\"Plugin0.esp\"...\"Plugin2.esp\"] }",
            values_string:mesh
        );
    }

    mod test_single {
        use super::{assert_eq, *};
        test_debug_single!(
            crea,
            Creature,
            "Skipped CREA record: \"\" { The only instance is in [\"Plugin0.esp\"] }",
            values_string:script
        );
    }
}
