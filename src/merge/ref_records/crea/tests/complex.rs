use super::{assert_eq, *};

#[test]
fn no_merge() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        4,
        values_string,
        values_u32,
        values_scale,
        values_object_flags,
        values_creature_flags,
        values_inventory,
        values_spells,
        values_travel_destinations_i,
        values_ai_packages_follow
    );
    src[1].name = values_string[1].clone();
    src[1].data.attack3.1 = values_u32[1].clone();
    src[1].scale = values_scale[1].clone();
    src[1].flags = values_object_flags[1];
    src[1].creature_flags = values_creature_flags[1];
    src[1].spells = values_spells[1].clone();
    src[1].inventory = values_inventory[1].clone();
    src[1].travel_destinations = values_travel_destinations_i[1].clone();
    src[1].ai_packages = values_ai_packages_follow[1].clone();
    src[3].name = values_string[3].clone();
    src[3].data.attack3.1 = values_u32[3].clone();
    src[3].scale = values_scale[3].clone();
    src[3].flags = values_object_flags[3];
    src[3].creature_flags = values_creature_flags[3];
    src[3].spells = values_spells[3].clone();
    src[3].inventory = values_inventory[3].clone();
    src[3].travel_destinations = values_travel_destinations_i[3].clone();
    src[3].ai_packages = values_ai_packages_follow[3].clone();
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge() {
    test_init!(
        src,
        plugins,
        cfg,
        Creature,
        5,
        values_string,
        values_u32,
        values_i16,
        values_u8,
        values_scale,
        values_object_flags,
        values_creature_flags,
        values_creature_type,
        values_service_flags,
        values_inventory,
        values_spells,
        values_travel_destinations,
        values_ai_packages_travel
    );
    let mut expected = Creature::default();
    src[1].sound = values_string[1].clone();
    src[2].sound = values_string[2].clone();
    expected.sound = values_string[2].clone();
    src[2].mesh = values_string[1].clone();
    src[3].mesh = values_string[1].clone();
    expected.mesh = values_string[1].clone();
    src[0].spells = values_spells[5].clone();
    src[1].spells = values_spells[2].clone();
    src[2].spells = values_spells[2].clone();
    src[3].spells = values_spells[2].clone();
    src[4].spells = values_spells[3].clone();
    expected.spells = values_spells[2].clone();
    src[2].inventory = values_inventory[7].clone();
    src[3].inventory = values_inventory[8].clone();
    expected.inventory = values_inventory[7].clone();
    src[1].travel_destinations = values_travel_destinations[14].clone();
    src[3].travel_destinations = values_travel_destinations[8].clone();
    src[4].travel_destinations = values_travel_destinations[7].clone();
    expected.travel_destinations = values_travel_destinations[14].clone();
    expected.travel_destinations.extend(values_travel_destinations[8].clone());
    src[1].ai_data.hello = values_i16[1];
    src[2].ai_data.hello = values_i16[2];
    src[3].ai_data.hello = values_i16[1];
    expected.ai_data.hello = values_i16[2];
    src[1].flags = values_object_flags[1];
    src[2].flags = values_object_flags[2];
    src[3].flags = values_object_flags[2];
    expected.flags = values_object_flags[2];
    src[1].scale = values_scale[1];
    src[2].scale = values_scale[2];
    src[3].scale = values_scale[1];
    src[4].scale = None;
    expected.scale = values_scale[2];
    src[1].blood_type = values_u8[1];
    src[2].blood_type = values_u8[2];
    src[3].blood_type = values_u8[1];
    expected.blood_type = values_u8[2];
    src[1].ai_data.services = values_service_flags[1];
    src[2].ai_data.services = values_service_flags[12];
    src[3].ai_data.services = values_service_flags[13];
    src[4].ai_data.services = values_service_flags[1];
    expected.ai_data.services = values_service_flags[13];
    src[1].creature_flags = values_creature_flags[1];
    src[2].creature_flags = values_creature_flags[2];
    src[3].creature_flags = values_creature_flags[3];
    src[4].creature_flags = values_creature_flags[2];
    expected.creature_flags = values_creature_flags[3];
    src[1].data.creature_type = values_creature_type[1];
    src[2].data.creature_type = values_creature_type[2];
    src[3].data.creature_type = values_creature_type[2];
    expected.data.creature_type = values_creature_type[2];
    src[1].data.magic = values_u32[1];
    src[2].data.magic = values_u32[2];
    src[3].data.magic = values_u32[3];
    src[4].data.magic = values_u32[2];
    expected.data.magic = values_u32[3];
    src[1].ai_packages = values_ai_packages_travel[1].clone();
    src[2].ai_packages = values_ai_packages_travel[2].clone();
    src[3].ai_packages = values_ai_packages_travel[1].clone();
    src[4].ai_packages = values_ai_packages_travel[1].clone();
    src[4].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
    expected.ai_packages = src[4].ai_packages.clone();
    test_merge!(crea, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Creature, expected, dst[0]);
}
