use super::{assert_eq, *};

#[test]
fn no_multipatch_no_merge_no_flag_no_crea() {
    test_init!(src, plugins, cfg, Creature, 3, values_string);
    cfg.merge.crea = false;
    cfg.multipatch.summons = false;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "");
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "");
}

#[test]
fn no_multipatch_no_merge_no_flag() {
    test_init!(src, plugins, cfg, Creature, 3, values_string);
    cfg.merge.crea = true;
    cfg.multipatch.summons = false;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "");
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "");
}

#[test]
fn no_multipatch_no_merge() {
    test_init!(src, plugins, cfg, Creature, 3, values_string);
    cfg.merge.crea = true;
    cfg.multipatch.summons = true;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "");
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_file(), "Trying to multipatch CREA record: \"\"\n");
}

#[test]
fn no_multipatch_merge_no_flag() {
    test_init!(src, plugins, cfg, Creature, 3, values_string);
    cfg.merge.crea = true;
    cfg.multipatch.summons = false;
    src[1].name = values_string[1].clone();
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, src[1], dst[0]);
    assert!(log.test_file().contains("Merged"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Merged"));
    assert!(!log.test_file().contains("Trying to multipatch"));
}

#[test]
fn no_multipatch_merge() {
    test_init!(src, plugins, cfg, Creature, 3, values_string);
    cfg.merge.crea = true;
    cfg.multipatch.summons = true;
    src[1].name = values_string[1].clone();
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, src[1], dst[0]);
    assert!(log.test_file().contains("Merged"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Merged"));
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn no_multipatch_no_merge_single_no_flag() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        1,
        values_string,
        (name = String::from("ancestor_ghost_summon"))
    );
    cfg.multipatch.summons = false;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert!(log.test_file().contains("Skipped"));
    assert!(log.test_file().contains("The only"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert!(log.test_file().contains("Skipped"));
    assert!(!log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_no_merge_single() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        1,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.multipatch.summons = true;
    let mut result = src[0].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    assert!(log.test_file().contains("The only"));
    assert!(log.test_file().contains("Multipatched"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Multipatched"));
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_no_merge_single_case() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        1,
        values_string,
        (id = String::from("Ancestor_ghost_summoN"))
    );
    cfg.multipatch.summons = true;
    let mut result = src[0].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    assert!(log.test_file().contains("Multipatched"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Multipatched"));
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn no_multipatch_no_merge_all_equal_no_flag() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        2,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.multipatch.summons = false;
    cfg.guts.debug_level_merge_skipped_all_equal = 0;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert!(log.test_file().contains("Skipped"));
    assert!(log.test_file().contains("All 2"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
    assert!(log.test_file().contains("Skipped"));
    assert!(!log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_no_merge_all_equal() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        2,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.guts.debug_level_merge_skipped_all_equal = 0;
    cfg.multipatch.summons = true;
    let mut result = src[0].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    assert!(log.test_file().contains("All 2"));
    assert!(log.test_file().contains("Multipatched"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Multipatched"));
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_merge_no_crea() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        3,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.merge.crea = false;
    cfg.multipatch.summons = true;
    src[1].name = "another_name".to_string();
    let mut result = src[1].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    assert!(log.test_file().contains("Skipped"));
    assert!(log.test_file().contains("Multipatched"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_merge() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        3,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.merge.crea = true;
    cfg.multipatch.summons = true;
    src[1].name = "another_name".to_string();
    let mut result = src[1].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    assert!(log.test_file().contains("Merged"));
    assert!(log.test_file().contains("multipatch"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_merge_no_crea_merged_is_ok() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        3,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.merge.crea = false;
    cfg.multipatch.summons = true;
    src[1].name = "another_name".to_string();
    src[1].flags = ObjectFlags::PERSISTENT;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, src[1], dst[0]);
    assert!(log.test_file().contains("Skipped"));
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    assert!(!log.test_file().contains("Trying to multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_primitive_crea_false() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        3,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.multipatch.primitive = false;
    cfg.merge.crea = false;
    cfg.multipatch.summons = true;
    src[1].name = "another_name".to_string();
    let mut result = src[1].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    cfg.multipatch.primitive = true;
    let mut primitive_result = src[2].clone();
    primitive_result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, primitive_result, dst[0]);
}

#[test]
fn multipatch_primitive_crea_true() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        3,
        values_string,
        (id = String::from("ancestor_ghost_summon"))
    );
    cfg.multipatch.primitive = false;
    cfg.merge.crea = true;
    cfg.multipatch.summons = true;
    src[1].name = "another_name".to_string();
    let mut result = src[1].clone();
    result.flags.insert(ObjectFlags::PERSISTENT);
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
    cfg.multipatch.primitive = true;
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, result, dst[0]);
}
