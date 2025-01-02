macro_rules! test_debug_compare_to_the_last_ai_packages {
    ($short:ident, $long:ident, $log1:expr, $log2:expr, $log3:expr, $log4:expr$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn quantity_no_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[2].clone());
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("text = {}", log.test_file());
            assert!(!log.test_file().contains("Comparing to the last instance"));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[2].clone());
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log1);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[2].clone());
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log1);
            assert_eq!(log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"), $log1);
        }

        #[test]
        fn quantity_equal_type_no_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander, values_ai_packages_follow$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_follow[1].clone();
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("text = {}", log.test_file());
            assert!(!log.test_file().contains("Comparing to the last instance"));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_type_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander, values_ai_packages_follow$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_follow[1].clone();
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log2);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_type_debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander, values_ai_packages_follow$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_follow[1].clone();
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log2);
            assert_eq!(log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"), $log2);
        }

        #[test]
        fn quantity_equal_single_no_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("text = {}", log.test_file());
            assert!(!log.test_file().contains("Comparing to the last instance"));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_single_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"), $log3);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_single_debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"), $log3);
            assert_eq!(log.test_text().lines().skip(3).collect::<Vec<_>>().join("\n"), $log3);
        }

        #[test]
        fn quantity_equal_multi_no_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[1].clone());
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[2].clone());
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages.extend(values_ai_packages_wander[1].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("text = {}", log.test_file());
            assert!(!log.test_file().contains("Comparing to the last instance"));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_multi_debug() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[1].clone());
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[2].clone());
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages.extend(values_ai_packages_wander[1].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log4);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_multi_debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, values_ai_packages_wander$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].ai_packages = values_ai_packages_wander[0].clone();
            src[0].ai_packages.extend(values_ai_packages_wander[1].clone());
            src[1].ai_packages = values_ai_packages_wander[1].clone();
            src[1].ai_packages.extend(values_ai_packages_wander[2].clone());
            src[2].ai_packages = values_ai_packages_wander[0].clone();
            src[2].ai_packages.extend(values_ai_packages_wander[1].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log4);
            assert_eq!(log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"), $log4);
        }
    };
}

pub(crate) use test_debug_compare_to_the_last_ai_packages;
