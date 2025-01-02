use super::{assert_eq, *};

#[test]
fn no_merge_s_s_s() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn no_merge_s_s_s_n() {
    test_init!(src, plugins, cfg, Npc, 4, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge_s_s_s_s_n() {
    test_init!(src, plugins, cfg, Npc, 5, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[3].data.stats = values_some_npc_data_skills_attributes[3].clone();
    src[3].data.gold = 1;
    let mut expected = Npc::default();
    expected.npc_flags = NpcFlags::AUTO_CALCULATE;
    expected.data.gold = 1;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
}

#[test]
fn merge_s_n_s_s_n() {
    test_init!(src, plugins, cfg, Npc, 5, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[3].data.stats = values_some_npc_data_skills_attributes[3].clone();
    src[3].data.gold = 1;
    let mut expected = Npc::default();
    expected.data.stats = values_some_npc_data_skills_attributes[3].clone();
    expected.data.gold = 1;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
}

#[test]
fn merge_n_s_s_s_n() {
    test_init!(src, plugins, cfg, Npc, 5, values_some_npc_data_skills_attributes);
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[3].data.stats = values_some_npc_data_skills_attributes[3].clone();
    src[3].data.gold = 1;
    let mut expected = Npc::default();
    expected.data.gold = 1;
    expected.data.stats = values_some_npc_data_skills_attributes[3].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
}

#[test]
fn merge_none_with_auto_calculate() {
    test_init!(src, plugins, cfg, Npc, 5, values_npc_flags);
    let mut expected = Npc::default();
    src[1].npc_flags = values_npc_flags[1];
    src[2].npc_flags = values_npc_flags[2];
    src[3].npc_flags = values_npc_flags[4];
    src[4].npc_flags = values_npc_flags[2];
    expected.npc_flags = values_npc_flags[4];
    let npc_stats_1 = NpcStats::default();
    let mut npc_stats_2 = NpcStats::default();
    let mut npc_stats_3 = NpcStats::default();
    let mut npc_stats_4 = NpcStats::default();
    npc_stats_2.magicka = 2;
    npc_stats_3.magicka = 3;
    npc_stats_4.magicka = 2;
    npc_stats_4.fatigue = 4;
    npc_stats_2.attributes[6] = 2;
    npc_stats_3.attributes[6] = 3;
    npc_stats_2.skills[24] = 2;
    src[1].data.stats = Some(npc_stats_1);
    src[2].data.stats = Some(npc_stats_2);
    src[3].data.stats = Some(npc_stats_3);
    src[4].data.stats = Some(npc_stats_4);
    expected.data.stats = None;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
}

#[test]
fn no_merge_s_n_n_without_auto_calculate_0() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "Warning: Invalid NPC_ record: \"\": \"npc_flags\": NpcFlags(0x0) [\"Plugin1.esp\"] { AUTO_CALCULATE flag is missing despite \"data.stats\" not set }\n");
}

#[test]
fn merge_s_n_n_without_auto_calculate_0_1() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    let mut expected = Npc::default();
    expected.npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn no_merge_s_n_n_without_auto_calculate_0_2() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn merge_s_n_n_with_auto_calculate_0_1() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn merge_s_n_n_with_auto_calculate_0_2() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn no_merge_n_n_s_with_auto_calculate_2() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn no_merge_n_n_s_with_auto_calculate_1_2() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn merge_n_n_s_with_auto_calculate_0_2() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    let mut expected = Npc::default();
    expected.data.stats = values_some_npc_data_skills_attributes[2].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn merge_s_s_n_without_auto_calculate_not_equal_after_fix() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[1].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[1].flags = ObjectFlags::MODIFIED;
    let mut expected = Npc::default();
    expected.npc_flags = NpcFlags::AUTO_CALCULATE;
    expected.flags = ObjectFlags::MODIFIED;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
    assert_eq!(log.test_warn(), "Warning: Fixed NPC_ record: \"\": \"npc_flags\": NpcFlags(0x0) -> NpcFlags(AUTO_CALCULATE) { AUTO_CALCULATE flag was missing despite \"data.stats\" not set }\n");

    let msg_fixed = "Fixed NPC_ record: \"\":\n\"npc_flags\": NpcFlags(0x0) -> NpcFlags(AUTO_CALCULATE)";
    println!("file = {}", log.test_file());
    assert!(log.test_file().contains(msg_fixed));

    cfg.guts.verboseness_details_merge_field_changed = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    println!("text = {}", log.test_text());
    assert!(!log.test_text().contains(msg_fixed));

    cfg.verbose = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    println!("text = {}", log.test_text());
    assert!(log.test_text().contains(msg_fixed));
}

#[test]
fn no_merge_s_s_n_without_auto_calculate_equal_after_fix() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    cfg.guts.debug_level_merge_skipped_equal_to_the_last = 2;
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[0].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[1].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[2].flags = ObjectFlags::MODIFIED;
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn no_merge_s_s_n_without_auto_calculate_equal_after_fix_with_debug() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    cfg.debug = 2;
    src[0].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[0].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[1].data.stats = values_some_npc_data_skills_attributes[2].clone();
    src[2].flags = ObjectFlags::MODIFIED;
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "Warning: Fixed NPC_ record: \"\": \"npc_flags\": NpcFlags(0x0) -> NpcFlags(AUTO_CALCULATE) { AUTO_CALCULATE flag was missing despite \"data.stats\" not set }\n");

    let msg_fixed = "Fixed NPC_ record: \"\":\n\"npc_flags\": NpcFlags(0x0) -> NpcFlags(AUTO_CALCULATE)";
    println!("file = {}", log.test_file());
    assert!(log.test_file().contains(msg_fixed));

    cfg.guts.verboseness_details_merge_field_changed = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    println!("text = {}", log.test_text());
    assert!(!log.test_text().contains(msg_fixed));

    cfg.verbose = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    println!("text = {}", log.test_text());
    assert!(log.test_text().contains(msg_fixed));
}

#[test]
fn merge_n_s_s_with_auto_calculate_not_equal_after_fix() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[1].flags = ObjectFlags::MODIFIED;
    src[2].data.stats = values_some_npc_data_skills_attributes[2].clone();
    let mut expected = Npc::default();
    expected.npc_flags = NpcFlags::AUTO_CALCULATE;
    expected.flags = ObjectFlags::MODIFIED;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
    assert_eq!(log.test_warn(), "Warning: Fixed NPC_ record: \"\": \"data.stats\": \"Manual\" -> \"Auto\" { \"data.stats\" were defined despite AUTO_CALCULATE flag was set }\n");

    let msg_fixed = "Fixed NPC_ record: \"\":\n\"data.stats\": \"Manual\" -> \"Auto\"";
    println!("file = {}", log.test_file());
    assert!(log.test_file().contains(msg_fixed));

    cfg.guts.verboseness_details_merge_field_changed = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    println!("text = {}", log.test_text());
    assert!(!log.test_text().contains(msg_fixed));

    cfg.verbose = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    println!("text = {}", log.test_text());
    assert!(log.test_text().contains(msg_fixed));
}

#[test]
fn no_merge_n_s_s_with_auto_calculate_equal_after_fix() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    cfg.guts.debug_level_merge_skipped_equal_to_the_last = 2;
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[1].flags = ObjectFlags::MODIFIED;
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[2].flags = ObjectFlags::MODIFIED;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn no_merge_n_s_s_with_auto_calculate_equal_after_fix_with_debug() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    cfg.debug = 2;
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    src[1].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[1].flags = ObjectFlags::MODIFIED;
    src[2].npc_flags = NpcFlags::AUTO_CALCULATE;
    src[2].flags = ObjectFlags::MODIFIED;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "Warning: Fixed NPC_ record: \"\": \"data.stats\": \"Manual\" -> \"Auto\" { \"data.stats\" were defined despite AUTO_CALCULATE flag was set }\n");

    let msg_fixed = "Fixed NPC_ record: \"\":\n\"data.stats\": \"Manual\" -> \"Auto\"";
    println!("file = {}", log.test_file());
    assert!(log.test_file().contains(msg_fixed));

    cfg.guts.verboseness_details_merge_field_changed = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    println!("text = {}", log.test_text());
    assert!(!log.test_text().contains(msg_fixed));

    cfg.verbose = 2;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    println!("text = {}", log.test_text());
    assert!(log.test_text().contains(msg_fixed));
}

#[test]
fn no_merge_last_stats_none_autocalc_not_set() {
    test_init!(src, plugins, cfg, Npc, 3, values_some_npc_data_skills_attributes);
    src[0].data.stats = values_some_npc_data_skills_attributes[0].clone();
    src[1].data.stats = values_some_npc_data_skills_attributes[1].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "Warning: Invalid NPC_ record: \"\": \"npc_flags\": NpcFlags(0x0) [\"Plugin2.esp\"] { AUTO_CALCULATE flag is missing despite \"data.stats\" not set }\n");
    println!("file = {}", log.test_file());
    assert!(log.test_file().contains("\"data.stats\": \"Manual\" -> \"Auto\""));
}
