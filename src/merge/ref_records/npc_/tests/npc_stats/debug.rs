use super::{assert_eq, *};

mod compare_to_the_last {
    use super::{assert_eq, *};

    #[test]
    fn merged_manual_last_auto_no_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        println!("file = {}", log.test_file());
        assert!(!log.test_file().contains("Comparing to the last instance"));
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn merged_manual_last_auto_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats\": \"Manual\" != \"Auto\"\nMerged NPC_ record: \"\""
        );
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn merged_manual_last_auto_debug_verbose() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        cfg.verbose = 1;
        src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats\": \"Manual\" != \"Auto\"\nMerged NPC_ record: \"\""
        );
        assert_eq!(
            log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats\": \"Manual\" != \"Auto\"\nMerged NPC_ record: \"\""
        );
    }

    #[test]
    fn merged_auto_last_manual_no_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
        src[2].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        println!("file = {}", log.test_file());
        assert!(!log.test_file().contains("Comparing to the last instance"));
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn merged_auto_last_manual_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
        src[2].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats\": \"Auto\" != \"Manual\"\nMerged NPC_ record: \"\""
        );
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn merged_auto_last_manual_debug_verbose() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        cfg.verbose = 1;
        src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
        src[2].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats\": \"Auto\" != \"Manual\"\nMerged NPC_ record: \"\""
        );
        assert_eq!(
            log.test_text().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats\": \"Auto\" != \"Manual\"\nMerged NPC_ record: \"\""
        );
    }

    #[test]
    fn health_magicka_no_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        src[0].data.stats = values_some_npc_data[0].clone();
        src[1].data.stats = values_some_npc_data[1].clone();
        src[2].data.stats = values_some_npc_data[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        println!("file = {}", log.test_file());
        assert!(!log.test_file().contains("Comparing to the last instance"));
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn health_magicka_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        src[0].data.stats = values_some_npc_data[0].clone();
        src[1].data.stats = values_some_npc_data[1].clone();
        src[2].data.stats = values_some_npc_data[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.health\": 1 != 0\n\"data.stats.magicka\": 1 != 0\nMerged NPC_ record: \"\""
        );
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn health_magicka_debug_verbose() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        cfg.verbose = 1;
        src[0].data.stats = values_some_npc_data[0].clone();
        src[1].data.stats = values_some_npc_data[1].clone();
        src[2].data.stats = values_some_npc_data[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.health\": 1 != 0\n\"data.stats.magicka\": 1 != 0\nMerged NPC_ record: \"\""
        );
        assert_eq!(
            log.test_text().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.health\": 1 != 0\n\"data.stats.magicka\": 1 != 0\nMerged NPC_ record: \"\""
        );
    }

    #[test]
    fn health_magicka_debug_quiet() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        cfg.quiet = true;
        src[0].data.stats = values_some_npc_data[0].clone();
        src[1].data.stats = values_some_npc_data[1].clone();
        src[2].data.stats = values_some_npc_data[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(
            log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"),
            "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.health\": 1 != 0\n\"data.stats.magicka\": 1 != 0\nMerged NPC_ record: \"\""
        );
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn health_magicka_debug_quiet_no_log() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        cfg.quiet = true;
        cfg.no_log = true;
        src[0].data.stats = values_some_npc_data[0].clone();
        src[1].data.stats = values_some_npc_data[1].clone();
        src[2].data.stats = values_some_npc_data[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        println!("file = {}", log.test_file());
        assert!(!log.test_file().contains("Comparing to the last instance"));
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn skills_attributes_no_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
        src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[2].data.stats = values_some_npc_data_skills_attributes[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        println!("file = {}", log.test_file());
        assert!(!log.test_file().contains("Comparing to the last instance"));
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn skills_attributes_debug() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
        src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[2].data.stats = values_some_npc_data_skills_attributes[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(log.test_file().lines().skip(5).collect::<Vec<_>>().join("\n"), "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.speed\": 1 != 0\n\"data.stats.personality\": 1 != 0\n\"data.stats.medium_armor\": 1 != 0\n\"data.stats.speechcraft\": 2 != 3\nMerged NPC_ record: \"\"");
        println!("text = {}", log.test_text());
        assert!(!log.test_text().contains("Comparing to the last instance"));
    }

    #[test]
    fn skills_attributes_debug_verbose() {
        test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
        cfg.guts.debug_level_merge_compare_to_the_last = 1;
        cfg.debug = 1;
        cfg.verbose = 1;
        src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
        src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
        src[2].data.stats = values_some_npc_data_skills_attributes[0].clone();
        test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
        assert_eq_inner!(Npc, src[1], dst[0]);
        assert_eq!(log.test_file().lines().skip(5).collect::<Vec<_>>().join("\n"), "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.speed\": 1 != 0\n\"data.stats.personality\": 1 != 0\n\"data.stats.medium_armor\": 1 != 0\n\"data.stats.speechcraft\": 2 != 3\nMerged NPC_ record: \"\"");
        assert_eq!(log.test_text().lines().skip(5).collect::<Vec<_>>().join("\n"), "Comparing to the last instance of NPC_ record: \"\" [\"Plugin2.esp\"]\n\"data.stats.speed\": 1 != 0\n\"data.stats.personality\": 1 != 0\n\"data.stats.medium_armor\": 1 != 0\n\"data.stats.speechcraft\": 2 != 3\nMerged NPC_ record: \"\"");
    }
}
