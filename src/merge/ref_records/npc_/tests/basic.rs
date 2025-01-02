use super::{assert_eq, *};

mod object_flags {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_object_flags:flags, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_flags!(npc_, Npc, values_object_flags:flags, (npc_flags = NpcFlags::AUTO_CALCULATE));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_log_flags!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_object_flags:flags, (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_log_flags!(
                npc_,
                Npc,
                true,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"flags\": + IGNORED [\"Plugin1.esp\"]\n\"flags\": - DELETED [\"Plugin1.esp\"]\n\"flags\": - PERSISTENT [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_object_flags:flags, (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }
}

mod script {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_string:script, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod inventory {
    use super::{assert_eq, *};
    test_basic_inventory!(npc_, Npc, values_inventory, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_inventory:inventory, (npc_flags = NpcFlags::AUTO_CALCULATE));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"inventory\": - \"inventory_1\"(1) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_2\"(2) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_3\"(3) [\"Plugin1.esp\"]\n\"inventory\": + \"inventory_4\"(4) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_inventory:inventory,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                npc_,
                Npc,
                true,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"inventory\": + \"inventory_4\"(4) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_1\"(1) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_2\"(2) [\"Plugin1.esp\"]\n\"inventory\": - \"inventory_3\"(3) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_inventory:inventory,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }

    mod debug {
        use super::{assert_eq, *};

        mod compare_to_the_last {
            use super::{assert_eq, *};
            test_debug_compare_to_the_last_vector_fields!(
                npc_,
                Npc,
                "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"inventory\": quantity 2 != 1\nMerged NPC_ record: \"\"",
                "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"inventory\": different despite equal quantity\nMerged NPC_ record: \"\"",
                values_inventory:inventory,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }
}

mod spells {
    use super::{assert_eq, *};
    test_basic_spells_and_travel!(npc_, Npc, spells, values_spells, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_spells:spells, (npc_flags = NpcFlags::AUTO_CALCULATE));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"spells\": - \"spell_1\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_2\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_3\" [\"Plugin1.esp\"]\n\"spells\": + \"spell_4\" [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_spells:spells,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                npc_,
                Npc,
                true,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"spells\": + \"spell_4\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_1\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_2\" [\"Plugin1.esp\"]\n\"spells\": - \"spell_3\" [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_spells:spells,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }
}

mod ai_data_alarm {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_i8:ai_data:alarm, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod service_flags {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_service_flags:ai_data:services, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_flags!(npc_, Npc, values_service_flags:ai_data:services, (npc_flags = NpcFlags::AUTO_CALCULATE));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_log_flags!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"ai_data.services\": - BARTERS_ARMOR [\"Plugin1.esp\"]\n\"ai_data.services\": - BARTERS_CLOTHING [\"Plugin1.esp\"]\n\"ai_data.services\": + BARTERS_BOOKS [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_service_flags:ai_data:services, (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_log_flags!(
                npc_,
                Npc,
                true,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"ai_data.services\": + BARTERS_BOOKS [\"Plugin1.esp\"]\n\"ai_data.services\": - BARTERS_ARMOR [\"Plugin1.esp\"]\n\"ai_data.services\": - BARTERS_CLOTHING [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_service_flags:ai_data:services, (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }
}

mod npc_flags {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_npc_flags:npc_flags, (data = NpcData { stats: Some(NpcStats::default()), ..Default::default()} ));
    test_flags!(npc_, Npc, values_npc_flags:npc_flags, (data = NpcData { stats: Some(NpcStats::default()), ..Default::default()} ));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_log_flags!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"npc_flags\": - ESSENTIAL [\"Plugin1.esp\"]\n\"npc_flags\": - RESPAWN [\"Plugin1.esp\"]\n\"npc_flags\": + IS_BASE [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_npc_flags:npc_flags, (data = NpcData { stats: Some(NpcStats::default()), ..Default::default()} )
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_log_flags!(
                npc_,
                Npc,
                true,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"npc_flags\": + IS_BASE [\"Plugin1.esp\"]\n\"npc_flags\": - ESSENTIAL [\"Plugin1.esp\"]\n\"npc_flags\": - RESPAWN [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_npc_flags:npc_flags, (data = NpcData { stats: Some(NpcStats::default()), ..Default::default()} )
            );
        }
    }
}

mod blood_type {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_u8:blood_type, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod data_level {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_i16:data:level, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod data_gold {
    use super::{assert_eq, *};
    test_basic!(npc_, Npc, values_u32:data:gold, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod log {
    use super::{assert_eq, *};
    test_log!(
        npc_,
        Npc,
        "Merged NPC_ record: \"\"\n",
        "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"name\": \"\" -> \"string_1\" [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
        values_string:name,
        (npc_flags = NpcFlags::AUTO_CALCULATE)
    );
}

mod debug {
    use super::{assert_eq, *};

    mod compare_to_the_last {
        use super::{assert_eq, *};
        test_debug_compare_to_the_last!(
            npc_,
            Npc,
            "Comparing to the last instance",
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.disposition\": 11 != 10\nMerged NPC_ record: \"\"",
            values_i16:data.disposition,
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
    }

    mod test_list_all_plugins {
        use super::{assert_eq, *};
        test_debug_list_all_plugins!(
            npc_,
            Npc,
            "[\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]",
            "Merging NPC_ record: \"\" [\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]",
            values_string:class,
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
    }

    mod test_equal_to_the_last {
        use super::{assert_eq, *};
        test_debug_equal_to_the_last!(
            npc_,
            Npc,
            "Skipped NPC_ record: \"\" { Equal to the last instance }",
            values_u32:data.gold,
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
    }

    mod test_all_equal {
        use super::{assert_eq, *};
        test_debug_all_equal!(
            npc_,
            Npc,
            "Skipped NPC_ record: \"\" { All 2 instances are equal in [\"Plugin0.esp\", \"Plugin1.esp\"] }",
            "Skipped NPC_ record: \"\" { All 3 instances are equal in [\"Plugin0.esp\"...\"Plugin2.esp\"] }",
            values_string:script,
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
    }

    mod test_single {
        use super::{assert_eq, *};
        test_debug_single!(
            npc_,
            Npc,
            "Skipped NPC_ record: \"\" { The only instance is in [\"Plugin0.esp\"] }",
            values_i8:ai_data.flee,
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
    }
}
