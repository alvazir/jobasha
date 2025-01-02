use super::{assert_eq, *};

#[test]
fn no_merge_exterior() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        4,
        values_region,
        values_object_flags,
        values_exterior_flags
    );
    src[1].region = values_region[1].clone();
    src[1].flags = values_object_flags[1];
    src[1].data.flags = values_exterior_flags[1];
    src[3].region = values_region[3].clone();
    src[3].flags = values_object_flags[3];
    src[3].data.flags = values_exterior_flags[3];
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn no_merge_interior() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        4,
        values_region,
        values_object_flags,
        values_interior_flags,
        (data = CellData {
            flags: CellFlags::IS_INTERIOR,
            grid: (0, 1065353216)
        })
    );
    src[1].region = values_region[1].clone();
    src[1].flags = values_object_flags[1];
    src[1].data.flags = values_interior_flags[1];
    src[3].region = values_region[3].clone();
    src[3].flags = values_object_flags[3];
    src[3].data.flags = values_interior_flags[3];
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge_exterior() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        5,
        values_string,
        values_object_flags,
        values_exterior_flags,
        values_region,
        values_map_color
    );
    let mut expected = Cell::default();
    src[1].name = values_string[1].clone();
    src[2].name = values_string[2].clone();
    expected.name = values_string[2].clone();
    src[2].flags = values_object_flags[1].clone();
    src[3].flags = values_object_flags[1].clone();
    expected.flags = values_object_flags[1].clone();
    src[0].data.flags = values_exterior_flags[1].clone();
    src[1].data.flags = values_exterior_flags[2].clone();
    src[2].data.flags = values_exterior_flags[2].clone();
    src[3].data.flags = values_exterior_flags[2].clone();
    src[4].data.flags = values_exterior_flags[3].clone();
    expected.data.flags = values_exterior_flags[3].clone();
    src[2].region = values_region[3].clone();
    src[3].region = values_region[2].clone();
    expected.region = None;
    src[1].map_color = values_map_color[1];
    src[2].map_color = values_map_color[2];
    src[3].map_color = values_map_color[1];
    expected.map_color = values_map_color[2];
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, expected, dst[0]);
}

#[test]
fn merge_interior() {
    test_init!(
        src,
        plugins,
        cfg,
        Cell,
        5,
        values_object_flags,
        values_interior_flags,
        values_region,
        values_map_color,
        values_water_height,
        values_atmosphere_data,
        (data = CellData {
            flags: CellFlags::IS_INTERIOR,
            grid: (0, 1065353216)
        })
    );
    let mut expected = Cell::default();
    src[2].flags = values_object_flags[1].clone();
    src[3].flags = values_object_flags[1].clone();
    expected.flags = values_object_flags[1].clone();
    src[0].data.flags = values_interior_flags[1].clone();
    src[1].data.flags = values_interior_flags[2].clone();
    src[2].data.flags = values_interior_flags[2].clone();
    src[3].data.flags = values_interior_flags[2].clone();
    src[4].data.flags = values_interior_flags[3].clone();
    expected.data.flags = values_interior_flags[3].clone();
    src[2].region = values_region[3].clone();
    src[3].region = values_region[2].clone();
    expected.region = None;
    src[1].map_color = values_map_color[1];
    src[2].map_color = values_map_color[2];
    src[3].map_color = values_map_color[1];
    expected.map_color = values_map_color[2];
    src[2].water_height = values_water_height[3].clone();
    src[3].water_height = values_water_height[2].clone();
    expected.water_height = None;
    src[1].atmosphere_data = values_atmosphere_data[1].clone();
    src[2].atmosphere_data = values_atmosphere_data[2].clone();
    src[3].atmosphere_data = values_atmosphere_data[1].clone();
    expected.atmosphere_data = values_atmosphere_data[2].clone();
    expected.data.grid = (0, 1065353216); // 1.0
    test_merge!(cell, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Cell, expected, dst[0]);
}
