use super::{assert_eq, *};

mod interior {
    use super::{assert_eq, *};
    test_basic_spells_and_travel!(crea, Creature, travel_destinations, values_travel_destinations_i);
    test_basic!(crea, Creature, values_travel_destinations_i:travel_destinations);

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": - \"cell_1\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_2\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_3\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": + \"cell_4\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_travel_destinations_i:travel_destinations
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": + \"cell_4\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_1\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_2\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"cell_3\"(0,0,0)(0,0,0) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_travel_destinations_i:travel_destinations
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
                "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"travel_destinations\": quantity 2 != 1\nMerged CREA record: \"\"",
                "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"travel_destinations\": different despite equal quantity\nMerged CREA record: \"\"",
                values_travel_destinations_i:travel_destinations
            );
        }
    }
}

mod exterior {
    use super::{assert_eq, *};

    mod log {
        use super::{assert_eq, *};

        mod default {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                false,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": - \"\"(1111,1111,1111)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(2222,2222,2222)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(3333,3333,3333)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": + \"\"(4444,4444,4444)(0,0,0) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_travel_destinations_e:travel_destinations
            );
        }

        mod plus_before_minus {
            use super::{assert_eq, *};
            test_logs_vector_fields!(
                crea,
                Creature,
                true,
                "Merged CREA record: \"\"\n",
                "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"travel_destinations\": + \"\"(4444,4444,4444)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(1111,1111,1111)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(2222,2222,2222)(0,0,0) [\"Plugin1.esp\"]\n\"travel_destinations\": - \"\"(3333,3333,3333)(0,0,0) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n",
                values_travel_destinations_e:travel_destinations
            );
        }
    }

    mod similarity {
        use super::{assert_eq, *};

        #[test]
        fn no_merge() {
            test_init!(src, plugins, cfg, Creature, 3, values_travel_destinations_e);
            cfg.merge.destination_similarity = 7.7;
            src[1].travel_destinations = values_travel_destinations_e[9].clone();
            src[2].travel_destinations = values_travel_destinations_e[4].clone();
            let mut expected = Creature::default();
            expected.travel_destinations = values_travel_destinations_e[9].clone();
            expected.travel_destinations.extend(values_travel_destinations_e[4].clone());
            test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!(Creature, expected, dst[0]);
        }

        #[test]
        fn merge() {
            test_init!(src, plugins, cfg, Creature, 3, values_travel_destinations_e);
            cfg.merge.destination_similarity = 2222.0;
            src[1].travel_destinations = values_travel_destinations_e[9].clone();
            src[2].travel_destinations = values_travel_destinations_e[4].clone();
            test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
        }
    }

    test_basic_spells_and_travel!(crea, Creature, travel_destinations, values_travel_destinations_e);
    test_basic!(crea, Creature, values_travel_destinations_e:travel_destinations);
}
