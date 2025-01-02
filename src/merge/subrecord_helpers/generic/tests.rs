macro_rules! test_logs_vector_fields {
    ($short:ident, $long:ident, $plus_before_minus:literal, $short_log:expr, $long_log:expr, $values:ident:$field:ident$(, ($manual_field:ident = $manual_value:expr))?) => {

            #[test]
            fn no_merge() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
                assert_eq!(log.test_file(), "");
                assert_eq!(log.test_text(), "");
            }

            #[test]
            fn merge_silent() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                cfg.no_log = true;
                cfg.quiet = true;
                src[0].$field = $values[0].clone();
                src[1].$field = $values[1].clone();
                src[2].$field = $values[0].clone();
                test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
                assert_eq_inner!($long, src[1], dst[0]);
                assert_eq!(log.test_file(), "");
                assert_eq!(log.test_text(), "");
            }

            #[test]
            fn merge_no_log() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                cfg.no_log = true;
                src[0].$field = $values[0].clone();
                src[1].$field = $values[1].clone();
                src[2].$field = $values[0].clone();
                test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
                assert_eq_inner!($long, src[1], dst[0]);
                assert_eq!(log.test_file(), "");
            }

            #[test]
            fn merge_quiet() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                cfg.quiet = true;
                src[0].$field = $values[0].clone();
                src[1].$field = $values[1].clone();
                src[2].$field = $values[0].clone();
                test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
                assert_eq_inner!($long, src[1], dst[0]);
                assert_eq!(log.test_text(), "");
            }

            #[test]
            fn merge() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                cfg.merge.plus_before_minus = $plus_before_minus;
                cfg.guts.verboseness_details_merge_record_merged = 1;
                cfg.guts.verboseness_details_merge_field_changed = 2;
                cfg.guts.debug_level_merge_compare_to_the_last = 1;
                src[0].$field = $values[3].clone();
                src[1].$field = $values[6].clone();
                src[2].$field = $values[5].clone();
                test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
                assert_eq_inner!($long, src[1], dst[0]);
                assert_eq!(log.test_file(), $long_log);
                assert_eq!(log.test_text(), "");
            }

            #[test]
            fn merge_verbose() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                cfg.merge.plus_before_minus = $plus_before_minus;
                cfg.guts.verboseness_details_merge_record_merged = 1;
                cfg.guts.verboseness_details_merge_field_changed = 2;
                cfg.guts.debug_level_merge_compare_to_the_last = 1;
                cfg.verbose = 1;
                src[0].$field = $values[3].clone();
                src[1].$field = $values[6].clone();
                src[2].$field = $values[5].clone();
                test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
                assert_eq_inner!($long, src[1], dst[0]);
                assert_eq!(log.test_file(), $long_log);
                assert_eq!(log.test_text(), $short_log);
            }

            #[test]
            fn merge_verbose_verbose() {
                test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
                cfg.merge.plus_before_minus = $plus_before_minus;
                cfg.guts.verboseness_details_merge_record_merged = 1;
                cfg.guts.verboseness_details_merge_field_changed = 2;
                cfg.guts.debug_level_merge_compare_to_the_last = 1;
                cfg.verbose = 2;
                src[0].$field = $values[3].clone();
                src[1].$field = $values[6].clone();
                src[2].$field = $values[5].clone();
                test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
                assert_eq_inner!($long, src[1], dst[0]);
                assert_eq!(log.test_file(), $long_log);
                assert_eq!(log.test_text(), $long_log);
        }
    };
}

macro_rules! test_debug_compare_to_the_last_vector_fields {
    ($short:ident, $long:ident, $log1:expr, $log2:expr, $values:ident:$field:ident$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn quantity_no_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].$field = $values[0].clone();
            src[1].$field = $values[1].clone();
            src[2].$field = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains("Comparing to the last instance"));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].$field = $values[0].clone();
            src[1].$field = $values[1].clone();
            src[2].$field = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log1);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].$field = $values[0].clone();
            src[1].$field = $values[1].clone();
            src[2].$field = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log1);
            assert_eq!(log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"), $log1);
        }

        #[test]
        fn quantity_equal_no_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].$field = $values[0].clone();
            src[1].$field = $values[4].clone();
            src[2].$field = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("text = {}", log.test_file());
            assert!(!log.test_file().contains("Comparing to the last instance"));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].$field = $values[0].clone();
            src[1].$field = $values[4].clone();
            src[2].$field = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"), $log2);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains("Comparing to the last instance"));
        }

        #[test]
        fn quantity_equal_debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].$field = $values[0].clone();
            src[1].$field = $values[4].clone();
            src[2].$field = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(3).collect::<Vec<_>>().join("\n"), $log2);
            assert_eq!(log.test_text().lines().skip(3).collect::<Vec<_>>().join("\n"), $log2);
        }
    };
}

pub(crate) use {test_debug_compare_to_the_last_vector_fields, test_logs_vector_fields};
