use super::{assert_eq, *};

#[test]
fn no_merge() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].mesh = values_string[1].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn merge_empty_to_default_with_another_field_changed_keep() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].mesh = String::from("base_anim.nif");
    src[1].name = values_string[1].clone();
    cfg.merge.keep_redundant_values = true;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn merge_empty_to_default_with_another_field_changed_no_keep() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].name = values_string[1].clone();
    let expected = src[1].clone();
    src[1].mesh = String::from("base_anim.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
}

#[test]
fn no_merge_empty_to_default() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].mesh = String::from("BASE_anim.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge_empty_to_default_keep() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].mesh = String::from("BASE_anim.nif");
    cfg.merge.keep_redundant_values = true;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn merge_empty_to_non_default() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].mesh = String::from("xxx_base_anim.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn merge_non_default_and_default() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[0].mesh = String::from("xxx_base_anim.nif");
    src[1].mesh = String::from("base_anim.nif");
    src[2].mesh = String::from("xxx_base_anim.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn merge_default_and_non_default() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[0].mesh = String::from("base_anim.nif");
    src[1].mesh = String::from("xxx_base_anim.nif");
    src[2].mesh = String::from("base_anim.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn no_merge_empty_to_default_female() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[0].npc_flags.insert(NpcFlags::FEMALE);
    src[1].mesh = String::from("BASE_anim_female.nif");
    src[1].npc_flags.insert(NpcFlags::FEMALE);
    src[2].npc_flags.insert(NpcFlags::FEMALE);
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge_empty_to_default_wrong_sex() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[1].mesh = String::from("base_anim_FEMALE.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, src[1], dst[0]);
}

#[test]
fn no_merge_default_to_empty_no_log() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[0].mesh = String::from("BASE_anim.nif");
    src[2].mesh = String::from("BASE_anim.nif");
    cfg.guts.debug_level_merge_skipped_equal_to_the_last = 1;
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "");
}

#[test]
fn no_merge_default_to_empty_log() {
    test_init!(src, plugins, cfg, Npc, 3, values_string, (npc_flags = NpcFlags::AUTO_CALCULATE));
    src[0].mesh = String::from("BASE_anim.nif");
    src[2].mesh = String::from("BASE_anim.nif");
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
    println!("{}", log.test_file());
    assert!(log.test_file().contains("(ignored)"));
}
