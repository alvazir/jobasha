macro_rules! test_basic {
    ($short:ident, $long:ident, $($values:ident:$field:ident$(:$subfield:ident$(:$index:tt)?)?),+$(, ($manual_field:ident$(:$manual_subfield:ident)? = $manual_value:expr))*$(; cfg=$cfgl1:ident$(:$cfgl2:ident)? = $cfgval:literal)*) => {
        test_basic!($short, $long, $($values:$field$(:$subfield$(:$index)?)?),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*; id=id=>"id_1".to_string()=>"id_2".to_string()$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
    };
    ($short:ident, $long:ident, $($values:ident:$field:ident$(:$subfield:ident$(:$index:tt)?)?),+$(, ($manual_field:ident$(:$manual_subfield:ident)? = $manual_value:expr))*; id=$id:ident$(:$subid:ident)?=>$id1_value:expr=>$id2_value:expr$(; cfg=$cfgl1:ident$(:$cfgl2:ident)? = $cfgval:literal)*) => {

        #[test]
        fn no_merge_0_records() {
            test_init!(src, plugins, cfg, $long, 0, $($values),+);
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_1_record() {
            test_init!(src, plugins, cfg, $long, 1, $($values),+);
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_2_records() {
            test_init!(src, plugins, cfg, $long, 2, $($values),+);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_0_0_0() {
            test_init!(src, plugins, cfg, $long, 3, $($values),+);
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_0_1_1() {
            test_init!(src, plugins, cfg, $long, 3, $($values),+);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_0_1_2() {
            test_init!(src, plugins, cfg, $long, 3, $($values),+);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_0_1_0_2() {
            test_init!(src, plugins, cfg, $long, 4, $($values),+);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[3].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_0_1_0_2_1_1_2_2_3() {
            test_init!(src, plugins, cfg, $long, 9, $($values),+);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[3].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[6].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[7].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[8].$field$(.$subfield$(.$index)?)? = $values[3].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_0_1_0() {
            test_init!(src, plugins, cfg, $long, 3, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
        }

        #[test]
        fn merge_0_1_2_0() {
            test_init!(src, plugins, cfg, $long, 4, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn merge_0_1_2_1_2_1() {
            test_init!(src, plugins, cfg, $long, 6, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[3].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn merge_0_1_2_1_2_1_0() {
            test_init!(src, plugins, cfg, $long, 7, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[3].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn merge_0_1_1_0_2_2_0() {
            test_init!(src, plugins, cfg, $long, 7, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[4], dst[0]);
        }

        #[test]
        fn merge_0_1_1_0_2_2_0_1() {
            test_init!(src, plugins, cfg, $long, 8, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[7].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[4], dst[0]);
        }

        #[test]
        fn merge_0_1_1_0_2_2_1_0() {
            test_init!(src, plugins, cfg, $long, 8, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[6].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[4], dst[0]);
        }

        #[test]
        fn merge_0_1_1_0_2_2_1_1() {
            test_init!(src, plugins, cfg, $long, 8, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[6].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[7].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[4], dst[0]);
        }

        #[test]
        fn merge_0_0_1_0_2_1_0() {
            test_init!(src, plugins, cfg, $long, 7, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[4], dst[0]);
        }

        #[test]
        fn merge_0_0_1_0_2_3_1_2_0() {
            test_init!(src, plugins, cfg, $long, 9, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*$(; cfg=$cfgl1$(:$cfgl2)? = $cfgval)*);
            $(src[2].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[4].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            $(src[5].$field$(.$subfield$(.$index)?)? = $values[3].to_owned();)+
            $(src[6].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            $(src[7].$field$(.$subfield$(.$index)?)? = $values[2].to_owned();)+
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[5], dst[0]);
        }

        #[test]
        fn no_merge_0_1_0_different_ids() {
            test_init!(src, plugins, cfg, $long, 3, $($values),+$(, ($manual_field$(:$manual_subfield)? = $manual_value))*);
            $(src[1].$field$(.$subfield$(.$index)?)? = $values[1].to_owned();)+
            src[1].$id$(.$subid)? = $id1_value;
            src[2].$id$(.$subid)? = $id2_value;
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }
    };
}

pub(crate) use test_basic;
