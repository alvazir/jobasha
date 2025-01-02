use super::{assert_eq, *};

mod exterior {
    use super::{assert_eq, *};
    test_basic!(cell, Cell,
        values_object_flags:flags,
        values_string:name,
        values_exterior_flags:data:flags,
        // COMMENT: values_data_grid:data:grid doesn't make sense for exterior
        values_region:region,
        values_map_color:map_color,
        // COMMENT: values_water_height:water_height doesn't make sense for exterior
        // COMMENT: values_atmosphere_data:atmosphere_data doesn't make sense for exterior
        (map_color = Some([0,1,2,0])),
        (region = Some(String::from("exterior")));
        id=data:grid=>(1, 1)=>(2, 2)
    );
}

mod interior {
    use super::{assert_eq, *};
    test_basic!(cell, Cell,
        values_object_flags:flags,
        // COMMENT: values_string:name doesn't make sense for interior
        values_interior_flags:data:flags,
        values_data_grid:data:grid,
        values_region:region,
        values_map_color:map_color,
        values_water_height:water_height,
        values_atmosphere_data:atmosphere_data,
        (water_height = Some(0.1)),
        (map_color = Some([0,1,2,0])),
        (region = Some(String::from("interior"))),
        (atmosphere_data = Some(AtmosphereData { fog_density: 1.0, ..Default::default()})),
        (data = CellData { flags: CellFlags::IS_INTERIOR, grid: (0, 1065353216) });
        id=name=>String::from("cell1")=>String::from("cell2");
        cfg=merge:keep_redundant_values = true;
        cfg=merge:ignore_secondary_fog_density = true
    );
}
