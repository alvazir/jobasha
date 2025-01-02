macro_rules! test_basic_inventory {
    ($short:ident, $long:ident, $values:ident$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn no_merge_e_o() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[1].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_o_e() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_m_o_e() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[0].inventory = $values[1].clone();
            src[1].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_m_m() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[1].clone();
            src[1].inventory = $values[2].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_m_m_reverse() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[2].clone();
            src[1].inventory = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_e_m_o() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[1].inventory = $values[1].clone();
            src[2].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
        }

        #[test]
        fn merge_e_o_m() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[1].inventory = $values[3].clone();
            src[2].inventory = $values[4].clone();
            let mut expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            expected.inventory = $values[5].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn no_merge_duplicates_m_o() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            src[0].inventory.extend($values[0].clone());
            src[1].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_duplicates_e_m_o() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            src[2].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
        }

        #[test]
        fn no_merge_duplicates_o_m() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_duplicates_e_o_m() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[1].inventory = $values[0].clone();
            src[2].inventory = $values[0].clone();
            src[2].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_duplicates_o_m_e() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[0].clone();
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[0], dst[0]);
        }

        #[test]
        fn no_merge_duplicates_m_o_e() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[0].inventory = $values[0].clone();
            src[0].inventory.extend($values[0].clone());
            src[1].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_duplicates_m_m_equal_quantity() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            src[0].inventory.extend($values[0].clone());
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_duplicates_m_m_e_equal_quantity() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[0].inventory = $values[0].clone();
            src[0].inventory.extend($values[0].clone());
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_duplicates_m_m_not_equal_quantity() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            src[0].inventory.extend($values[0].clone());
            src[0].inventory.extend($values[0].clone());
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_duplicates_m_m_not_equal_quantity_rev() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            src[0].inventory.extend($values[0].clone());
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            src[1].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_duplicates_e_m_m() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            src[2].inventory = $values[0].clone();
            src[2].inventory.extend($values[0].clone());
            src[2].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_duplicates_e_m_m_rev() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[1].inventory = $values[0].clone();
            src[1].inventory.extend($values[0].clone());
            src[1].inventory.extend($values[0].clone());
            src[2].inventory = $values[0].clone();
            src[2].inventory.extend($values[0].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[1], dst[0]);
        }

        #[test]
        fn no_merge_o_o() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[0].clone();
            src[1].inventory = $values[4].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_e_o_o() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[1].inventory = $values[0].clone();
            src[2].inventory = $values[4].clone();
            let mut expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            expected.inventory = $values[6].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn no_merge_m_o() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            src[0].inventory = $values[5].clone();
            src[1].inventory = $values[4].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_e_e_m_o_e() {
            test_init!(src, plugins, cfg, $long, 5, $values$(, ($manual_field = $manual_value))*);
            src[2].inventory = $values[5].clone();
            src[3].inventory = $values[4].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn no_merge_m_m_o_repeated_delete() {

            test_init!(src, plugins, cfg, $long, 3, $values);
            src[0].inventory = $values[2].clone();
            src[1].inventory = $values[1].clone();
            src[2].inventory = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_m_o_o() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[5].clone();
            src[1].inventory = $values[4].clone();
            src[2].inventory = $values[0].clone();
            let expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn merge_m_o_duplicates() {
            test_init!(src, plugins, cfg, $long, 2, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[4].clone();
            src[0].inventory.extend($values[5].clone());
            src[1].inventory = $values[4].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_m_o_o_duplicates() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[5].clone();
            src[0].inventory.extend($values[4].clone());
            src[1].inventory = $values[4].clone();
            src[2].inventory = $values[0].clone();
            let expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn merge_m_m_o_duplicates() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[5].clone();
            src[1].inventory = $values[4].clone();
            src[1].inventory.extend($values[4].clone());
            src[2].inventory = $values[0].clone();
            let mut expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            expected.inventory = $values[4].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn merge_add_and_delete() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[3].clone();
            src[1].inventory = $values[6].clone();
            src[2].inventory = $values[5].clone();
            let mut expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            expected.inventory = $values[6].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn merge_add_and_delete_everything() {
            test_init!(src, plugins, cfg, $long, 3, $values$(, ($manual_field = $manual_value))*);
            src[0].inventory = $values[6].clone();
            src[1].inventory = $values[3].clone();
            let mut expected = $long { $($manual_field:$manual_value,)* ..Default::default() };
            expected.inventory = $values[7].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, expected, dst[0]);
        }

        #[test]
        fn no_merge_different_cases() {
            test_init!(src, plugins, cfg, $long, 2, $values);
            cfg.guts.debug_level_merge_skipped_equal_to_the_last = 2;
            src[0].inventory = $values[7].clone();
            src[1].inventory = $values[8].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }
    };
}

pub(crate) use test_basic_inventory;
