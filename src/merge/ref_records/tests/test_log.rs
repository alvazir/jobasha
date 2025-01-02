macro_rules! test_log {
    ($short:ident, $long:ident, $short_log:expr, $long_log:expr, $values:ident:$field:ident$(:$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn log_no_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(log.test_file(), "");
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn log_merge_silent() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.no_log = true;
            cfg.quiet = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), "");
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn log_merge_no_log() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.no_log = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), "");
        }

        #[test]
        fn log_merge_quiet() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.quiet = true;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn log_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn log_merge_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.verbose = 1;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), $short_log);
        }

        #[test]
        fn log_merge_verbose_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))?);
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.verbose = 2;
            src[0].$field$(.$subfield)? = $values[0].clone();
            src[1].$field$(.$subfield)? = $values[1].clone();
            src[2].$field$(.$subfield)? = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), $long_log);
        }
    };
}

macro_rules! test_log_flags {
    ($short:ident, $long:ident, $plus_before_minus:literal, $short_log:expr, $long_log:expr, $values:ident:$field:ident$(:$subfield:ident)?$(, ($manual_field:ident = $manual_value:expr))*$(; cfg=$cfgl1:ident$(:$cfgl2:ident)? = $cfgval:literal)*) => {

        #[test]
        fn no_merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            cfg.merge.interdependent_flags = false;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
            assert_eq!(log.test_file(), "");
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn merge_silent() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            cfg.merge.interdependent_flags = false;
            cfg.no_log = true;
            cfg.quiet = true;
            src[0].$field$(.$subfield)? = $values[0];
            src[1].$field$(.$subfield)? = $values[1];
            src[2].$field$(.$subfield)? = $values[0];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), "");
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn merge_no_log() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            cfg.merge.interdependent_flags = false;
            cfg.no_log = true;
            src[0].$field$(.$subfield)? = $values[0];
            src[1].$field$(.$subfield)? = $values[1];
            src[2].$field$(.$subfield)? = $values[0];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), "");
        }

        #[test]
        fn merge_quiet() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            cfg.merge.interdependent_flags = false;
            cfg.quiet = true;
            src[0].$field$(.$subfield)? = $values[0];
            src[1].$field$(.$subfield)? = $values[1];
            src[2].$field$(.$subfield)? = $values[0];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn merge() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            cfg.merge.interdependent_flags = false;
            cfg.merge.plus_before_minus = $plus_before_minus;
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            src[0].$field$(.$subfield)? = $values[1];
            src[0].$field$(.$subfield)?.insert($values[2]);
            src[1].$field$(.$subfield)? = $values[3];
            src[2].$field$(.$subfield)? = $values[2];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), "");
        }

        #[test]
        fn merge_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            cfg.merge.interdependent_flags = false;
            cfg.merge.plus_before_minus = $plus_before_minus;
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.verbose = 1;
            src[0].$field$(.$subfield)? = $values[1];
            src[0].$field$(.$subfield)?.insert($values[2]);
            src[1].$field$(.$subfield)? = $values[3];
            src[2].$field$(.$subfield)? = $values[2];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), $short_log);
        }

        #[test]
        fn merge_verbose_verbose() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            cfg.merge.interdependent_flags = false;
            cfg.merge.plus_before_minus = $plus_before_minus;
            cfg.guts.verboseness_details_merge_record_merged = 1;
            cfg.guts.verboseness_details_merge_field_changed = 2;
            cfg.guts.debug_level_merge_compare_to_the_last = 1;
            cfg.verbose = 2;
            src[0].$field$(.$subfield)? = $values[1];
            src[0].$field$(.$subfield)?.insert($values[2]);
            src[1].$field$(.$subfield)? = $values[3];
            src[2].$field$(.$subfield)? = $values[2];
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
            assert_eq!(log.test_file(), $long_log);
            assert_eq!(log.test_text(), $long_log);
        }
    };
}

pub(crate) use {test_log, test_log_flags};
