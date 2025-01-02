use super::{assert_eq, *};

#[test]
fn no_multipatch_merge() {
    test_init!(src, plugins, cfg, Cell, 3, values_string);
    src[1].name = values_string[1].clone();
    cfg.merge.cell = true;
    cfg.multipatch.cellnames = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.cellnames = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.merge.cell = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatched"));
}

#[test]
fn no_multipatch_single() {
    test_init!(src, plugins, cfg, Cell, 1, values_string);
    cfg.merge.cell = false;
    cfg.multipatch.cellnames = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
}

#[test]
fn no_multipatch_all_equal() {
    test_init!(src, plugins, cfg, Cell, 8, values_string);
    cfg.merge.cell = false;
    cfg.multipatch.cellnames = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
}

#[test]
fn no_multipatch_equal() {
    test_init!(src, plugins, cfg, Cell, 3, values_string);
    src[1].name = values_string[1].clone();
    src[2].name = values_string[1].clone();
    cfg.merge.cell = false;
    cfg.multipatch.cellnames = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
}

#[test]
fn multipatch_not_equal() {
    test_init!(src, plugins, cfg, Cell, 3, values_string);
    src[1].name = values_string[1].clone();
    cfg.merge.cell = false;
    cfg.multipatch.cellnames = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(log.test_file().contains("multipatched"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Trying to multipatch"));
    cfg.multipatch.cellnames = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatched"));
}

#[test]
fn no_warning_when_cell_and_cellnames_off() {
    test_init!(src, plugins, cfg, Cell, 3, values_string);
    src[1].name = values_string[1].clone();
    cfg.merge.cell = false;
    cfg.multipatch.cellnames = false;
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn multipatch_primitive_cell_false() {
    test_init!(src, plugins, cfg, Cell, 3, values_string, (region = Some("region".to_string())));
    cfg.multipatch.primitive = false;
    cfg.merge.cell = false;
    cfg.multipatch.cellnames = true;
    src[1].name = values_string[1].clone();
    src[1].region = Some("another_region".to_string());
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(log.test_file().contains("multipatched"));
    cfg.multipatch.primitive = true;
    cfg.merge.keep_redundant_values = true;
    let mut primitive_result = src[2].clone();
    primitive_result.name = values_string[1].clone();
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, primitive_result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    cfg.merge.keep_redundant_values = false;
    primitive_result.region = None;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, primitive_result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
}

#[test]
fn multipatch_primitive_cell_true() {
    test_init!(src, plugins, cfg, Cell, 3, values_string, (region = Some("region".to_string())));
    cfg.merge.cell = true;
    cfg.multipatch.primitive = false;
    cfg.multipatch.cellnames = true;
    src[1].name = values_string[1].clone();
    src[1].region = Some("another_region".to_string());
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    cfg.multipatch.primitive = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
}
