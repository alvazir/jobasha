macro_rules! test_log_ai_packages {
    ($short:ident, $long:ident, $short_log:expr, $long_log:expr$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn merge_to_empty_from_one() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Wander) -> (Empty) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_empty_from_one_silent() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            cfg.no_log = true;
            cfg.quiet = true;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert!(log.test_file().is_empty());
            assert!(log.test_text().is_empty());
        }

        #[test]
        fn merge_to_empty_from_one_verbose_no_log() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            cfg.no_log = true;
            cfg.verbose = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert!(log.test_file().is_empty());
            assert!(!log.test_text().is_empty());
        }

        #[test]
        fn merge_to_empty_from_one_verbose() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.verbose = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), $short_log);
        }

        #[test]
        fn merge_to_empty_from_one_verbose_verbose() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.verbose = 2;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), $long_log);
        }

        #[test]
        fn merge_to_empty_from_many() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[0].clone());
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Multiple) -> (Empty) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_many_from_empty() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[1].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Empty) -> (Multiple) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_many_from_one() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[0].clone());
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Wander) -> (Multiple) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_many_from_many() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[0].clone());
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[1].clone());
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Multiple) -> (Multiple) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_one_from_empty() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[1].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Empty) -> (Wander) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_one_from_one() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander,
                values_ai_packages_travel$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_travel[0].clone();
            src[1].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = values_ai_packages_travel[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Travel) -> (Wander) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_one_from_many_not_same() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander,
                values_ai_packages_activate$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[0].clone());
            src[1].ai_packages = values_ai_packages_activate[1].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Multiple) -> (Activate) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_one_from_many_same_equal() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[0].clone());
            src[1].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Multiple) -> (Wander) [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_one_from_many_same_not_equal() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_wander$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[0].clone());
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[2].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(1).collect::<Vec<_>>().join("\n"),
                "\"ai_packages\": (Wander), \"distance\": 0 -> 1 [\"Plugin1.esp\"]"
            );
        }

        #[test]
        fn merge_to_one_from_one_all_subfield_types() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_follow$(, ($manual_field = $manual_value))?
            );
            src[0].ai_packages = values_ai_packages_follow[0].clone();
            src[1].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
                duration: 17,
                location: [2.0, 0.0, 1.0],
                target: FixedString("1".to_string()),
                ..Default::default()
            })];
            src[2].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
                location: [3.0, 0.0, 2.0],
                target: FixedString("2".to_string()),
                ..Default::default()
            })];
            let mut expected = src[1].clone();
            expected.ai_packages = vec![AiPackage::Follow(AiFollowPackage {
                duration: 17,
                location: [2.0, 0.0, 2.0],
                target: FixedString("2".to_string()),
                ..Default::default()
            })];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
            assert_eq!(
                log.test_file().lines().skip(1).take(6).collect::<Vec<_>>().join("\n"),
                r#""ai_packages": (Follow), "duration": 0 -> 17 ["Plugin1.esp"]
"ai_packages": (Follow), "target": FixedString("0") -> FixedString("1") ["Plugin1.esp"]
"ai_packages": (Follow), "location.0": 3.0 -> 2.0 ["Plugin1.esp"]
"ai_packages": (Follow), "location.2": 0.0 -> 1.0 ["Plugin1.esp"]
"ai_packages": (Follow), "target": FixedString("1") -> FixedString("2") ["Plugin2.esp"]
"ai_packages": (Follow), "location.2": 1.0 -> 2.0 ["Plugin2.esp"]"#
            );
        }

        #[test]
        fn merge_to_one_from_one_all_subfield_types_silent() {
            test_init!(
                src,
                plugins,
                cfg,
                $long,
                3,
                values_ai_packages_follow$(, ($manual_field = $manual_value))?
            );
            cfg.no_log = true;
            cfg.quiet = true;
            src[0].ai_packages = values_ai_packages_follow[0].clone();
            src[1].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
                duration: 17,
                location: [2.0, 0.0, 1.0],
                target: FixedString("1".to_string()),
                ..Default::default()
            })];
            src[2].ai_packages = vec![AiPackage::Follow(AiFollowPackage {
                location: [3.0, 0.0, 2.0],
                target: FixedString("2".to_string()),
                ..Default::default()
            })];
            let mut expected = src[1].clone();
            expected.ai_packages = vec![AiPackage::Follow(AiFollowPackage {
                duration: 17,
                location: [2.0, 0.0, 2.0],
                target: FixedString("2".to_string()),
                ..Default::default()
            })];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
            assert!(log.test_file().is_empty());
            assert!(log.test_text().is_empty());
        }
    };
}

pub(crate) use test_log_ai_packages;
