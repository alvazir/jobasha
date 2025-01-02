use super::{assert_eq, *};

mod interior {
    use super::{assert_eq, *};
    test_basic_spells_and_travel!(
        npc_,
        Npc,
        travel_destinations,
        values_travel_destinations_i,
        (npc_flags = NpcFlags::AUTO_CALCULATE)
    );
    test_basic!(npc_, Npc, values_travel_destinations_i:travel_destinations, (npc_flags = NpcFlags::AUTO_CALCULATE));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": - \"cell_1\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_2\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_3\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": + \"cell_4\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_travel_destinations_i:travel_destinations,
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
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": + \"cell_4\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_1\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_2\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_3\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_travel_destinations_i:travel_destinations,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }
}

mod exterior {
    use super::{assert_eq, *};

    mod similarity {
        use super::{assert_eq, *};

        #[test]
        fn no_merge() {
            test_init!(
                src,
                plugins,
                cfg,
                Npc,
                3,
                values_travel_destinations_e,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
            cfg.merge.destination_similarity = 1.0;
            src[1].travel_destinations = values_travel_destinations_e[4].clone();
            src[2].travel_destinations = values_travel_destinations_e[9].clone();
            let mut expected = Npc::default();
            expected.npc_flags = NpcFlags::AUTO_CALCULATE;
            expected.travel_destinations = values_travel_destinations_e[4].clone();
            expected.travel_destinations.extend(values_travel_destinations_e[9].clone());
            test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!(Npc, expected, dst[0]);
        }

        #[test]
        fn merge() {
            test_init!(src, plugins, cfg, Npc, 3, values_travel_destinations_e);
            cfg.merge.destination_similarity = 1024.0;
            src[1].travel_destinations = values_travel_destinations_e[4].clone();
            src[2].travel_destinations = values_travel_destinations_e[9].clone();
            test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
        }
    }

    test_basic_spells_and_travel!(
        npc_,
        Npc,
        travel_destinations,
        values_travel_destinations_e,
        (npc_flags = NpcFlags::AUTO_CALCULATE)
    );
    test_basic!(npc_, Npc, values_travel_destinations_e:travel_destinations, (npc_flags = NpcFlags::AUTO_CALCULATE));

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                npc_,
                Npc,
                false,
                "Merged NPC_ record: \"\"\n",
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": - \"\"(1111,1111,1111)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(2222,2222,2222)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(3333,3333,3333)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": + \"\"(4444,4444,4444)(0,0,0) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_travel_destinations_e:travel_destinations,
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
                "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": + \"\"(4444,4444,4444)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(1111,1111,1111)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(2222,2222,2222)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(3333,3333,3333)(0,0,0) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n",
                values_travel_destinations_e:travel_destinations,
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
                "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"travel_destinations\": quantity 2 != 1\nMerged NPC_ record: \"\"",
                "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"travel_destinations\": different despite equal quantity\nMerged NPC_ record: \"\"",
                values_travel_destinations_e:travel_destinations,
                (npc_flags = NpcFlags::AUTO_CALCULATE)
            );
        }
    }
}
