use super::{assert_eq, *};

mod activate {
    use super::{assert_eq, *};
    test_basic_ai_packages!(crea, Creature, values_ai_packages_activate);
    test_basic!(crea, Creature, values_ai_packages_activate:ai_packages);
}

mod escort {
    use super::{assert_eq, *};
    test_basic_ai_packages!(crea, Creature, values_ai_packages_escort);
    test_basic!(crea, Creature, values_ai_packages_escort:ai_packages);
}

mod follow {
    use super::{assert_eq, *};
    test_basic_ai_packages!(crea, Creature, values_ai_packages_follow);
    test_basic!(crea, Creature, values_ai_packages_follow:ai_packages);
}

mod travel {
    use super::{assert_eq, *};
    test_basic_ai_packages!(crea, Creature, values_ai_packages_travel);
    test_basic!(crea, Creature, values_ai_packages_travel:ai_packages);
}

mod wander {
    use super::{assert_eq, *};
    test_basic_ai_packages!(crea, Creature, values_ai_packages_wander);
    test_basic!(crea, Creature, values_ai_packages_wander:ai_packages);
}

mod complex {
    use super::{assert_eq, *};

    #[test]
    fn no_merge() {
        test_init!(
            src,
            plugins,
            cfg,
            Creature,
            5,
            values_ai_packages_wander,
            values_ai_packages_follow,
            values_ai_packages_escort
        );
        src[0].ai_packages = values_ai_packages_wander[0].clone();
        src[1].ai_packages = values_ai_packages_follow[1].clone();
        src[2].ai_packages = values_ai_packages_wander[2].clone();
        src[3].ai_packages = values_ai_packages_follow[3].clone();
        src[4].ai_packages = values_ai_packages_escort[0].clone();
        test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    }

    #[test]
    fn merge() {
        test_init!(src, plugins, cfg, Creature, 13, values_ai_packages_escort);
        src[1].ai_packages = vec![AiPackage::Escort(AiEscortPackage::default())];
        src[2].ai_packages = values_ai_packages_escort[2].clone();
        src[3].ai_packages = values_ai_packages_escort[3].clone();
        src[4].ai_packages = vec![AiPackage::Escort(AiEscortPackage {
            target: FixedString("4".to_string()),
            reset: 17,
            ..Default::default()
        })];
        src[5].ai_packages = vec![AiPackage::Escort(AiEscortPackage {
            target: FixedString("6".to_string()),
            reset: 67,
            ..Default::default()
        })];
        src[6].ai_packages = values_ai_packages_escort[3].clone();
        src[6].ai_packages.extend(values_ai_packages_escort[3].clone());
        src[8].ai_packages = src[2].ai_packages.clone();
        src[9].ai_packages = src[3].ai_packages.clone();
        src[10].ai_packages = vec![AiPackage::Escort(AiEscortPackage {
            reset: 68,
            target: FixedString("4".to_string()),
            ..Default::default()
        })];
        src[11].ai_packages = vec![
            AiPackage::Travel(AiTravelPackage::default()),
            AiPackage::Wander(AiWanderPackage::default()),
            AiPackage::Travel(AiTravelPackage::default()),
        ];
        src[12].ai_packages = vec![AiPackage::Escort(AiEscortPackage {
            reset: 67,
            ..Default::default()
        })];
        let mut expected = Creature::default();
        expected.ai_packages = vec![AiPackage::Escort(AiEscortPackage {
            target: FixedString("6".to_string()),
            cell: "3".to_string(),
            reset: 68,
            duration: 3,
            ..Default::default()
        })];
        test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Creature, expected, dst[0]);
    }
}

mod log {
    use super::{assert_eq, *};

    test_log_ai_packages!(
            crea,
            Creature,
            "Merged CREA record: \"\"\n",
            "Merging CREA record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"ai_packages\": (Wander) -> (Empty) [\"Plugin1.esp\"]\nMerged CREA record: \"\"\n"
        );
}

mod debug {
    use super::{assert_eq, *};

    mod compare_to_the_last {
        use super::{assert_eq, *};

        test_debug_compare_to_the_last_ai_packages!(
            crea,
            Creature,
            "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": quantity 2 != 1\nMerged CREA record: \"\"",
            "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": type \"Follow\" != \"Wander\"\nMerged CREA record: \"\"",
            "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": different despite the same type\nMerged CREA record: \"\"",
            "Comparing to the last instance of CREA record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": different despite equal quantity\nMerged CREA record: \"\""
        );
    }
}
