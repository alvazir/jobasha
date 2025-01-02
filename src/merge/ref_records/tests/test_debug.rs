macro_rules! test_debug_compare_to_the_last {
    ($short:ident, $long:ident, $log_short:expr, $log:expr, $values:ident:$field:ident$(.$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn no_merge_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn no_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn debug_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
            assert_eq!(log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
        }

        #[test]
        fn debug_quiet() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.quiet = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn debug_quiet_no_log() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.debug = 1;
            cfg.quiet = true;
            cfg.no_log = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }
    }
}

macro_rules! test_debug_list_all_plugins {
    ($short:ident, $long:ident, $log_short:expr, $log:expr, $values:ident:$field:ident$(.$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn no_show_no_debug_no_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn no_show_no_debug_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn no_show_debug_no_merge() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.debug = 1;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn show_debug_no_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.guts.debug_level_merge_skipped_all_equal = 1;
            cfg.debug = 1;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn show_debug_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.debug = 1;
            cfg.guts.verboseness_details_merge_field_changed = 1;
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq!(log.test_file().split("\n").nth(0).unwrap(), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn show_debug_no_merge_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.guts.debug_level_merge_skipped_all_equal = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(log.test_text().contains($log_short));
        }

        #[test]
        fn show_debug_merge_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.debug = 1;
            cfg.guts.verboseness_details_merge_field_changed = 1;
            cfg.verbose = 1;
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq!(log.test_file().split("\n").nth(0).unwrap(), $log);
            assert_eq!(log.test_text().split("\n").nth(0).unwrap(), $log);
        }

        #[test]
        fn show_debug_merge_quiet() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.debug = 1;
            cfg.quiet = true;
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq!(log.test_file().split("\n").nth(0).unwrap(), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn show_debug_no_merge_quiet_no_log() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.debug = 1;
            cfg.quiet = true;
            cfg.no_log = true;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log_short));
        }

        #[test]
        fn show_debug_merge_verbose_no_log() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_list_all_plugins = 1;
            cfg.debug = 1;
            cfg.verbose = 1;
            cfg.no_log = true;
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log_short));
            assert_eq!(log.test_text().split("\n").nth(0).unwrap(), $log);
        }
    }
}

macro_rules! test_debug_equal_to_the_last {
    ($short:ident, $long:ident, $log:expr, $values:ident:$field:ident$(.$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn merge_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 3;
            cfg.debug = 3;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn no_debug() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 3;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn debug() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 3;
            cfg.debug = 3;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert_eq!(&log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn debug_verbose() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 3;
            cfg.debug = 3;
            cfg.verbose = 3;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
            assert_eq!(&log.test_text().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
        }

        #[test]
        fn debug_quiet() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 3;
            cfg.debug = 3;
            cfg.quiet = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(2).collect::<Vec<_>>().join("\n"), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn debug_quiet_no_log() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 3;
            cfg.debug = 3;
            cfg.quiet = true;
            cfg.no_log = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }
    }
}

macro_rules! test_debug_all_equal {
    ($short:ident, $long:ident, $log2:expr, $log3:expr, $values:ident:$field:ident$(.$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn merge_debug() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log2));
            assert!(!log.test_file().contains($log3));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log2));
            assert!(!log.test_text().contains($log3));
        }

        #[test]
        fn no_debug() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log2));
            assert!(!log.test_file().contains($log3));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log2));
            assert!(!log.test_text().contains($log3));
        }

        #[test]
        fn debug_2() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(0).collect::<Vec<_>>().join("\n"), $log2);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log2));
            assert!(!log.test_text().contains($log3));
        }

        #[test]
        fn debug_3() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(0).collect::<Vec<_>>().join("\n"), $log3);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log2));
            assert!(!log.test_text().contains($log3));
        }

        #[test]
        fn debug_2_verbose() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            cfg.verbose = 4;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(0).collect::<Vec<_>>().join("\n"), $log2);
            assert_eq!(&log.test_text().lines().skip(0).collect::<Vec<_>>().join("\n"), $log2);
        }

        #[test]
        fn debug_3_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            cfg.verbose = 4;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(0).collect::<Vec<_>>().join("\n"), $log3);
            assert_eq!(&log.test_text().lines().skip(0).collect::<Vec<_>>().join("\n"), $log3);
        }


        #[test]
        fn debug_2_quiet() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            cfg.quiet = true;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(&log.test_file().lines().skip(0).collect::<Vec<_>>().join("\n"), $log2);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log2));
        }

        #[test]
        fn debug_3_quiet_no_log() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_all_equal = 4;
            cfg.debug = 4;
            cfg.quiet = true;
            cfg.no_log = true;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log3));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log3));
        }
    }
}

macro_rules! test_debug_single {
    ($short:ident, $long:ident, $log:expr, $values:ident:$field:ident$(.$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn no_debug() {
            test_init!(src, plugins, cfg, $long, 1, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_single = 5;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn debug_multi() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_single = 5;
            cfg.debug = 5;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn debug() {
            test_init!(src, plugins, cfg, $long, 1, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_single = 5;
            cfg.debug = 5;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(log.test_file().split("\n").nth(0).unwrap(), $log);
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }

        #[test]
        fn debug_verbose() {
            test_init!(src, plugins, cfg, $long, 1, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_single = 5;
            cfg.debug = 5;
            cfg.verbose = 5;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(log.test_file().split("\n").nth(0).unwrap(), $log);
            assert_eq!(log.test_text().split("\n").nth(0).unwrap(), $log);
        }

        #[test]
        fn debug_verbose_no_log() {
            test_init!(src, plugins, cfg, $long, 1, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_single = 5;
            cfg.debug = 5;
            cfg.verbose = 5;
            cfg.no_log = true;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            assert_eq!(log.test_text().split("\n").nth(0).unwrap(), $log);
        }

        #[test]
        fn debug_quiet_no_log() {
            test_init!(src, plugins, cfg, $long, 1, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.debug_level_merge_skipped_single = 5;
            cfg.debug = 5;
            cfg.quiet = true;
            cfg.no_log = true;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            println!("file = {}", log.test_file());
            assert!(!log.test_file().contains($log));
            println!("text = {}", log.test_text());
            assert!(!log.test_text().contains($log));
        }
    };
}

macro_rules! test_debug {
    (
        $short:ident,
        $long:ident,
        $log:expr,
        $values:ident:$field:ident$(.$subfield:ident)?
        $(, ($manual_field:ident = $manual_value:expr))?
    ) => {
        test_debug!($short, $long, $log, $values:$field$(.$subfield)?$(, ($manual_field = $manual_value))?; name="\"\"");
    };
    (
        $short:ident,
        $long:ident,
        $log:expr,
        $values:ident:$field:ident$(.$subfield:ident)?
        $(, ($manual_field:ident = $manual_value:expr))?
        ; name=$name:expr
    ) => {

        mod compare_to_the_last {
            use super::{assert_eq, *};
            test_debug_compare_to_the_last!(
                $short,
                $long,
                "Comparing to the last instance",
                $log,
                $values:$field$(.$subfield)?$(, ($manual_field = $manual_value))?
            );
        }

        mod list_all_plugins {
            use super::{assert_eq, *};
            test_debug_list_all_plugins!(
                $short,
                $long,
                &format!("[\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]"),
                &format!("Merging {} record: {} [\"Plugin0.esp\", \"Plugin1.esp\", \"Plugin2.esp\"]", stringify!($short).to_uppercase(), $name),
                $values:$field$(.$subfield)?$(, ($manual_field = $manual_value))?
            );
        }

        mod equal_to_the_last {
            use super::{assert_eq, *};
            test_debug_equal_to_the_last!(
                $short,
                $long,
                &format!("Skipped {} record: {} {{ Equal to the last instance }}", stringify!($short).to_uppercase(), $name),
                $values:$field$(.$subfield)?$(, ($manual_field = $manual_value))?
            );
        }

        mod all_equal {
            use super::{assert_eq, *};
            test_debug_all_equal!(
                $short,
                $long,
                &format!("Skipped {} record: {} {{ All 2 instances are equal in [\"Plugin0.esp\", \"Plugin1.esp\"] }}", stringify!($short).to_uppercase(), $name),
                &format!("Skipped {} record: {} {{ All 3 instances are equal in [\"Plugin0.esp\"...\"Plugin2.esp\"] }}", stringify!($short).to_uppercase(), $name),
                $values:$field$(.$subfield)?$(, ($manual_field = $manual_value))?
            );
        }

        mod single {
            use super::{assert_eq, *};
            test_debug_single!(
                $short,
                $long,
                &format!("Skipped {} record: {} {{ The only instance is in [\"Plugin0.esp\"] }}", stringify!($short).to_uppercase(), $name),
                $values:$field$(.$subfield)?$(, ($manual_field = $manual_value))?
            );
        }
    };
}

pub(crate) use {
    test_debug, test_debug_all_equal, test_debug_compare_to_the_last, test_debug_equal_to_the_last, test_debug_list_all_plugins,
    test_debug_single,
};
