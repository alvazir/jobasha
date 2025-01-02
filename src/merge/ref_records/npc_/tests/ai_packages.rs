use super::{assert_eq, *};

mod activate {
    use super::{assert_eq, *};
    test_basic_ai_packages!(npc_, Npc, values_ai_packages_activate, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_ai_packages_activate:ai_packages, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod escort {
    use super::{assert_eq, *};
    test_basic_ai_packages!(npc_, Npc, values_ai_packages_escort, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_ai_packages_escort:ai_packages, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod follow {
    use super::{assert_eq, *};
    test_basic_ai_packages!(npc_, Npc, values_ai_packages_follow, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_ai_packages_follow:ai_packages, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod travel {
    use super::{assert_eq, *};
    test_basic_ai_packages!(npc_, Npc, values_ai_packages_travel, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_ai_packages_travel:ai_packages, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod wander {
    use super::{assert_eq, *};
    test_basic_ai_packages!(npc_, Npc, values_ai_packages_wander, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_basic!(npc_, Npc, values_ai_packages_wander:ai_packages, (npc_flags = NpcFlags::AUTO_CALCULATE));
}

mod complex {
    use super::{assert_eq, *};

    #[test]
    fn no_merge() {
        test_init!(
            src,
            plugins,
            cfg,
            Npc,
            5,
            values_ai_packages_wander,
            values_ai_packages_travel,
            values_ai_packages_activate
        );
        src[0].ai_packages = values_ai_packages_wander[0].clone();
        src[1].ai_packages = values_ai_packages_travel[1].clone();
        src[2].ai_packages = values_ai_packages_wander[2].clone();
        src[3].ai_packages = values_ai_packages_travel[3].clone();
        src[4].ai_packages = values_ai_packages_activate[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    }

    #[test]
    fn merge() {
        test_init!(
            src,
            plugins,
            cfg,
            Npc,
            12,
            values_ai_packages_follow,
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
        src[1].ai_packages = vec![AiPackage::Follow(AiFollowPackage::default())];
        src[2].ai_packages = values_ai_packages_follow[2].clone();
        src[3].ai_packages = values_ai_packages_follow[3].clone();
        src[4].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
            cell: "4".to_string(),
            reset: 17,
            ..Default::default()
        })];
        src[5].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
            cell: "4".to_string(),
            reset: 67,
            ..Default::default()
        })];
        src[6].ai_packages = values_ai_packages_follow[3].clone();
        src[6].ai_packages.extend(values_ai_packages_follow[3].clone());
        src[8].ai_packages = src[2].ai_packages.clone();
        src[9].ai_packages = src[3].ai_packages.clone();
        src[10].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
            cell: "6".to_string(),
            ..Default::default()
        })];
        let mut expected = Npc::default();
        expected.npc_flags = NpcFlags::AUTO_CALCULATE;
        expected.ai_packages = vec![AiPackage::Follow(AiFollowPackage {
            cell: "6".to_string(),
            reset: 67,
            location: [1.0, 0.0, 3.0],
            target: FixedString("3".to_string()),
            ..Default::default()
        })];
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, expected, dst[0]);
    }
}

mod log {
    use super::{assert_eq, *};

    test_log_ai_packages!(
            npc_,
            Npc,
            "Merged NPC_ record: \"\"\n",
            "Merging NPC_ record: \"\" [\"Plugin0.esp\"...\"Plugin2.esp\"]\n\"ai_packages\": (Wander) -> (Empty) [\"Plugin1.esp\"]\nMerged NPC_ record: \"\"\n", (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
}

mod debug {
    use super::{assert_eq, *};

    mod compare_to_the_last {
        use super::{assert_eq, *};
        test_debug_compare_to_the_last_ai_packages!(
            npc_,
            Npc,
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": quantity 2 != 1\nMerged NPC_ record: \"\"",
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": type \"Follow\" != \"Wander\"\nMerged NPC_ record: \"\"",
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": different despite the same type\nMerged NPC_ record: \"\"",
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"ai_packages\": different despite equal quantity\nMerged NPC_ record: \"\"",
            (npc_flags = NpcFlags::AUTO_CALCULATE)
        );
    }
}
