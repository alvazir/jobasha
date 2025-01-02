use super::{assert_eq, *};

#[test]
fn verbose_atmosphere_data() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        4,
        values_atmosphere_data,
        (data = CellData {
            flags: CellFlags::IS_INTERIOR,
            grid: (0, 1065353216)
        })
    );
    src[1].atmosphere_data = values_atmosphere_data[1].clone();
    src[2].atmosphere_data = values_atmosphere_data[2].clone();
    src[3].atmosphere_data = values_atmosphere_data[1].clone();
    cfg.merge.verbose_atmosphere_data = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[2], dst[0]);
    assert_eq!(
        log.test_file().split("\n").nth(2).unwrap(),
        "\"atmosphere_data\": ambient: (1,2,3) -> (0,0,0), sunlight: (0,0,0) -> (4,5,6) [\"Plugin2.esp\"]"
    );
    cfg.merge.verbose_atmosphere_data = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[2], dst[0]);
    assert_eq!(
        log.test_file().split("\n").nth(2).unwrap(),
        "\"atmosphere_data\": ambient: (1,2,3), sunlight: (0,0,0), fog: (0,0,0), fog_density: 1 -> ambient: (0,0,0), sunlight: (4,5,6), fog: (0,0,0), fog_density: 1 [\"Plugin2.esp\"]"
    );
}

#[test]
fn ignore_secondary_fog_density() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = CellData {
            flags: CellFlags::IS_INTERIOR,
            grid: (0, 1056964608) // 0.5
        })
    );
    src[0].atmosphere_data = values_atmosphere_data[1].clone();
    src[1].atmosphere_data = values_atmosphere_data[2].clone();
    src[2].atmosphere_data = values_atmosphere_data[1].clone();
    cfg.merge.ignore_secondary_fog_density = false;
    let mut result = src[1].clone();
    result.data.grid = (0, 1065353216); // 1.0
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("synced"));
    assert!(log.test_file().contains("secondary fog density"));
    cfg.merge.ignore_secondary_fog_density = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("synced"));
    assert!(!log.test_file().contains("secondary fog density"));
}

#[test]
fn keep_redundant_values_interior() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        3,
        values_atmosphere_data,
        (data = CellData {
            flags: CellFlags::IS_INTERIOR,
            grid: (0, 1065353216)
        }),
        (water_height = Some(1.1))
    );
    src[0].atmosphere_data = values_atmosphere_data[1].clone();
    src[1].atmosphere_data = values_atmosphere_data[1].clone();
    src[2].atmosphere_data = values_atmosphere_data[1].clone();
    src[1].water_height = Some(2.2);
    let mut result = src[1].clone();
    result.atmosphere_data = None;
    result.water_height = None;
    cfg.merge.keep_redundant_values = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("redundant value"));
    cfg.merge.keep_redundant_values = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("redundant value"));
}

#[test]
fn keep_redundant_values_exterior() {
    test_init!(src, plugins, cfg, Cell, 3, values_map_color, (region = Some("region".to_string())));
    src[0].map_color = values_map_color[1].clone();
    src[1].map_color = values_map_color[2].clone();
    src[2].map_color = values_map_color[1].clone();
    let mut result = src[1].clone();
    result.region = None;
    cfg.merge.keep_redundant_values = false;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, result, dst[0]);
    assert!(log.test_file().contains("redundant value"));
    cfg.merge.keep_redundant_values = true;
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, src[1], dst[0]);
    assert!(!log.test_file().contains("redundant value"));
}
