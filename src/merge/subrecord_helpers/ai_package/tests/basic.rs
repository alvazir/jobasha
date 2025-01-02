macro_rules! test_basic_ai_packages {
    ($short:ident, $long:ident, $values:ident$(, ($manual_field:ident = $manual_value:expr))?) => {

        #[test]
        fn no_merge_e_o_m() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[1].ai_packages = $values[1].clone();
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.extend($values[2].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_o_o_m() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[0].ai_packages = $values[0].clone();
            src[1].ai_packages = $values[1].clone();
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.extend($values[2].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_o_m_o_e() {
            test_init!(src, plugins, cfg, $long, 4, $values);
            src[0].ai_packages = $values[0].clone();
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.extend($values[1].clone());
            src[2].ai_packages = $values[2].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn no_merge_m_m_m() {
            test_init!(src, plugins, cfg, $long, 3, $values);
            src[0].ai_packages = $values[0].clone();
            src[0].ai_packages.extend($values[0].clone());
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.extend($values[1].clone());
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.extend($values[1].clone());
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_e_m_m_m_e() {
            test_init!(src, plugins, cfg, $long, 5, $values$(, ($manual_field = $manual_value))*);
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.extend($values[1].clone());
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.extend($values[1].clone());
            src[3].ai_packages = src[1].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn merge_m_m_m_m_m() {
            test_init!(src, plugins, cfg, $long, 5, $values$(, ($manual_field = $manual_value))*);
            src[0].ai_packages = $values[0].clone();
            src[0].ai_packages.extend($values[0].clone());
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.extend($values[1].clone());
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.extend($values[1].clone());
            src[3].ai_packages = src[1].ai_packages.clone();
            src[4].ai_packages = src[0].ai_packages.clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn merge_s_s_s_s() {
            test_init!(src, plugins, cfg, $long, 4, $values$(, ($manual_field = $manual_value))*);
            src[0].ai_packages = $values[0].clone();
            src[1].ai_packages = $values[1].clone();
            src[2].ai_packages = $values[2].clone();
            src[3].ai_packages = $values[1].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn no_merge_s_m_m_s() {
            test_init!(src, plugins, cfg, $long, 4, $values);
            src[0].ai_packages = $values[0].clone();
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[2].ai_packages.push(AiPackage::Escort(AiEscortPackage::default()));
            src[3].ai_packages = $values[3].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_s_m_m_s() {
            test_init!(src, plugins, cfg, $long, 4, $values$(, ($manual_field = $manual_value))*);
            src[0].ai_packages = $values[0].clone();
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[2].ai_packages = $values[2].clone();
            src[2].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[2].ai_packages.push(AiPackage::Escort(AiEscortPackage::default()));
            src[3].ai_packages = $values[0].clone();
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }

        #[test]
        fn no_merge_m_m_s_m() {
            test_init!(src, plugins, cfg, $long, 4, $values);
            src[0].ai_packages = $values[0].clone();
            src[0].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[0].ai_packages.push(AiPackage::Escort(AiEscortPackage::default()));
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[2].ai_packages = $values[2].clone();
            src[3].ai_packages = $values[3].clone();
            src[3].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[3].ai_packages.push(AiPackage::Escort(AiEscortPackage::default()));
            test_merge!($short, src, plugins, cfg, log, im, res, dst:0);
        }

        #[test]
        fn merge_m_m_s_m() {
            test_init!(src, plugins, cfg, $long, 4, $values$(, ($manual_field = $manual_value))*);
            src[0].ai_packages = $values[0].clone();
            src[0].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[0].ai_packages.push(AiPackage::Escort(AiEscortPackage::default()));
            src[1].ai_packages = $values[1].clone();
            src[1].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[2].ai_packages = $values[2].clone();
            src[3].ai_packages = $values[0].clone();
            src[3].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
            src[3].ai_packages.push(AiPackage::Escort(AiEscortPackage::default()));
            test_merge!($short, src, plugins, cfg, log, im, res, dst:1);
            assert_eq_inner!($long, src[2], dst[0]);
        }
    };
}

pub(crate) use test_basic_ai_packages;
