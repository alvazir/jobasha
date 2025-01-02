use super::{assert_eq, *};

const MULTIPATCH_FOGBUG_FIXED_VALUE: f32 = 0.01;

const FOGBUG: Option<AtmosphereData> = Some(AtmosphereData {
    fog_density: 0.0,
    ambient_color: [0; 4],
    sunlight_color: [0; 4],
    fog_color: [0; 4],
});

const FIXED: Option<AtmosphereData> = Some(AtmosphereData {
    fog_density: MULTIPATCH_FOGBUG_FIXED_VALUE,
    ambient_color: [0; 4],
    sunlight_color: [0; 4],
    fog_color: [0; 4],
});

const ATMOSPHERE_DATA: Option<AtmosphereData> = Some(AtmosphereData {
    fog_density: 1.0,
    ambient_color: [0; 4],
    sunlight_color: [0; 4],
    fog_color: [0; 4],
});

const DATA: CellData = CellData {
    flags: CellFlags::IS_INTERIOR,
    grid: (0, 1065353216),
};

#[test]
fn no_multipatch_single() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        1,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.multipatch.fogbug = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = false;
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn no_multipatch_all_equal() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        4,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.multipatch.fogbug = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = false;
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn no_multipatch_merge() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.merge.keep_redundant_values = true;
    cfg.multipatch.fogbug = true;
    cfg.multipatch.cellnames = true;
    src[1].data.flags.insert(CellFlags::RESTING_IS_ILLEGAL);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = false;
    cfg.multipatch.cellnames = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = false;
    cfg.merge.cell = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    assert!(log.test_file().contains("Skipped"));
}

#[test]
fn no_multipatch_equal() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.multipatch.fogbug = true;
    cfg.multipatch.cellnames = true;
    cfg.merge.keep_redundant_values = true;
    src[1].data.flags.insert(CellFlags::RESTING_IS_ILLEGAL);
    src[2].data.flags.insert(CellFlags::RESTING_IS_ILLEGAL);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = false;
    cfg.multipatch.cellnames = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_single() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        1,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = FOGBUG)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    let mut result = src[0].clone();
    result.atmosphere_data = FIXED;
    cfg.multipatch.fogbug = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    cfg.merge.ignore_secondary_fog_density = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    cfg.merge.ignore_secondary_fog_density = false;
    result.data.grid = (0, 1008981770);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_all_equal() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        7,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = FOGBUG)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    let mut result = src[0].clone();
    result.atmosphere_data = FIXED;
    cfg.multipatch.fogbug = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    cfg.merge.ignore_secondary_fog_density = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    cfg.merge.ignore_secondary_fog_density = false;
    result.data.grid = (0, 1008981770);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_equal() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    src[1].atmosphere_data = FOGBUG;
    src[2].atmosphere_data = FOGBUG;
    let mut result = src[0].clone();
    result.atmosphere_data = FIXED;
    cfg.multipatch.fogbug = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("multipatch"));
    cfg.multipatch.fogbug = true;
    cfg.merge.ignore_secondary_fog_density = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    cfg.merge.ignore_secondary_fog_density = false;
    result.data.grid = (0, 1008981770);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn multipatch_merge() {
    //   | cell | fogbug | ignore_secondary | debug
    // 1 | +    | -           | -                | -
    // 2 | +    | +           | +                | -
    // 3 | +    | +           | -                | -
    // 4 | +    | +           | -                | +
    // 5 | -    | +           | -                | +
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    src[1].atmosphere_data = FOGBUG;
    let mut fogbugged_result = src[1].clone();
    fogbugged_result.data.grid = (0, 0);
    cfg.merge.cell = true;
    cfg.multipatch.fogbug = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, fogbugged_result, dst[0]);
    assert!(!log.test_file().contains("multipatch"));
    let mut result = src[1].clone();
    result.atmosphere_data = FIXED;
    cfg.multipatch.fogbug = true;
    cfg.merge.ignore_secondary_fog_density = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    cfg.merge.ignore_secondary_fog_density = false;
    result.data.grid = (0, 1008981770);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
    cfg.guts.debug_level_merge_multipatch_attempt = 1;
    cfg.debug = 1;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("Trying to multipatch"));
    cfg.merge.cell = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert!(!log.test_file().contains("ultipatched"));
    assert!(log.test_file().contains("Trying to multipatch"));
}

#[test]
fn no_warning_when_cell_and_fogbug_off() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    src[1].atmosphere_data = FOGBUG;
    cfg.merge.cell = false;
    cfg.multipatch.fogbug = false;
    cfg.multipatch.cellnames = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
}

#[test]
fn multipatch_merge_no_cell_merged_is_ok() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = FOGBUG)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    src[1].atmosphere_data = ATMOSPHERE_DATA;
    src[1].data.grid = (0, 1008981770);
    cfg.merge.cell = false;
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert!(log.test_file().contains("multipatched"));
    assert!(log.test_file().contains("Multipatched"));
}

#[test]
fn multipatch_primitive_cell_false() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = FOGBUG),
        (region = Some("region".to_string()))
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    cfg.multipatch.primitive = false;
    src[1].region = Some("another_region".to_string());
    let mut result = src[1].clone();
    result.atmosphere_data = FIXED;
    result.data.grid = (0, 1008981770);
    cfg.merge.cell = false;
    cfg.multipatch.fogbug = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    cfg.multipatch.primitive = true;
    cfg.merge.keep_redundant_values = true;
    let mut primitive_result = result.clone();
    primitive_result.region = Some("region".to_string());
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, primitive_result, dst[0]);
    cfg.merge.keep_redundant_values = false;
    primitive_result.region = None;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, primitive_result, dst[0]);
    assert!(log.test_file().contains("multipatched"));
}

#[test]
fn multipatch_primitive_cell_true() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = ATMOSPHERE_DATA)
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    cfg.merge.cell = true;
    cfg.multipatch.fogbug = true;
    cfg.multipatch.primitive = false;
    src[1].atmosphere_data = FOGBUG;
    let mut result = src[1].clone();
    result.atmosphere_data = FIXED;
    result.data.grid = (0, 1008981770);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    cfg.multipatch.primitive = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
}

#[test]
fn multipatch_primitive_last_ambi_empty_no_panic() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        4,
        values_atmosphere_data,
        (data = DATA),
        (atmosphere_data = FOGBUG),
        (region = Some("region".to_string()))
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    src[3].atmosphere_data = None;
    src[1].region = Some("another_region".to_string());
    cfg.merge.cell = false;
    cfg.multipatch.fogbug = true;
    cfg.multipatch.primitive = true;
    cfg.merge.keep_redundant_values = true;
    let mut primitive_result = src[1].clone();
    primitive_result.region = Some("region".to_string());
    primitive_result.atmosphere_data = FIXED;
    primitive_result.data.grid = (0, 1008981770);
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, primitive_result, dst[0]);
}

#[test]
fn multipatch_fix_ambi() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        4,
        values_atmosphere_data,
        (data = DATA),
        (region = Some("region".to_string()))
    );
    cfg.guts.multipatch_fogbug_fixed_value = MULTIPATCH_FOGBUG_FIXED_VALUE;
    cfg.merge.cell = false;
    cfg.multipatch.fogbug = true;
    cfg.multipatch.primitive = true;
    cfg.merge.keep_redundant_values = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
    cfg.merge.cell = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "");
    src[1].region = Some("another_region".to_string());
    let mut result = src[1].clone();
    result.atmosphere_data = ATMOSPHERE_DATA;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert_eq!(log.test_warn(), "Warning: Fixed CELL record: \"\": \"atmosphere_data\": None -> ambient: (0,0,0), sunlight: (0,0,0), fog: (0,0,0), fog_density: 1 { at least one instance of the interior cell must contain atmosphere_data }\n");
    cfg.merge.cell = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
    assert_eq!(log.test_warn(), "Warning: Incomplete CELL record: \"\": \"atmosphere_data\": None { at least one instance of the interior cell must contain atmosphere_data }\n");
}
