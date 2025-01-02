use super::{assert_eq, *};

#[test]
fn no_merge() {
    test_init!(
        src,
        plugins,
        cfg,
        Npc,
        5,
        values_string,
        values_object_flags,
        values_npc_flags,
        values_inventory,
        values_spells,
        values_travel_destinations_e,
        values_ai_packages_activate
    );
    src[1].head = values_string[1].clone();
    src[1].hair = values_string[1].clone();
    src[1].flags = values_object_flags[1];
    src[1].npc_flags = values_npc_flags[1];
    src[1].spells = values_spells[1].clone();
    src[1].inventory = values_inventory[1].clone();
    src[1].travel_destinations = values_travel_destinations_e[1].clone();
    src[1].ai_packages = values_ai_packages_activate[1].clone();
    src[2].head = values_string[2].clone();
    src[2].hair = values_string[2].clone();
    src[2].flags = values_object_flags[2];
    src[2].npc_flags = values_npc_flags[2];
    src[2].spells = values_spells[2].clone();
    src[2].inventory = values_inventory[2].clone();
    src[2].travel_destinations = values_travel_destinations_e[2].clone();
    src[2].ai_packages = values_ai_packages_activate[2].clone();
    src[3].head = values_string[2].clone();
    src[3].hair = values_string[2].clone();
    src[3].flags = values_object_flags[2];
    src[3].npc_flags = values_npc_flags[2];
    src[3].spells = values_spells[2].clone();
    src[3].inventory = values_inventory[2].clone();
    src[3].travel_destinations = values_travel_destinations_e[2].clone();
    src[3].ai_packages = values_ai_packages_activate[2].clone();
    src[4].head = values_string[3].clone();
    src[4].hair = values_string[3].clone();
    src[4].flags = values_object_flags[3];
    src[4].npc_flags = values_npc_flags[3];
    src[4].spells = values_spells[3].clone();
    src[4].inventory = values_inventory[3].clone();
    src[4].travel_destinations = values_travel_destinations_e[3].clone();
    src[4].ai_packages = values_ai_packages_activate[3].clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:0);
}

#[test]
fn merge() {
    test_init!(
        src,
        plugins,
        cfg,
        Npc,
        5,
        values_string,
        values_object_flags,
        values_npc_flags,
        values_service_flags,
        values_inventory,
        values_spells,
        values_travel_destinations,
        values_ai_packages_wander
    );
    let mut expected = Npc::default();
    src[1].faction = values_string[1].clone();
    src[2].faction = values_string[2].clone();
    expected.faction = values_string[2].clone();
    src[2].class = values_string[1].clone();
    src[3].class = values_string[1].clone();
    expected.class = values_string[1].clone();
    src[2].spells = values_spells[3].clone();
    src[3].spells = values_spells[6].clone();
    src[4].spells = values_spells[8].clone();
    expected.spells = values_spells[5].clone();
    src[2].inventory = values_inventory[3].clone();
    src[3].inventory = values_inventory[6].clone();
    src[4].inventory = values_inventory[8].clone();
    expected.inventory = values_inventory[5].clone();
    src[1].travel_destinations = values_travel_destinations[0].clone();
    src[2].travel_destinations = values_travel_destinations[4].clone();
    src[3].travel_destinations = values_travel_destinations[10].clone();
    src[4].travel_destinations = values_travel_destinations[14].clone();
    expected.travel_destinations = values_travel_destinations[6].clone();
    expected.travel_destinations.extend(values_travel_destinations[16].clone());
    src[1].ai_data.flee = 1;
    src[2].ai_data.flee = 2;
    src[3].ai_data.flee = 1;
    expected.ai_data.flee = 2;
    src[1].flags = values_object_flags[1];
    src[4].flags = values_object_flags[4];
    expected.flags = values_object_flags[4];
    src[1].ai_data.services = values_service_flags[1];
    src[2].ai_data.services = values_service_flags[12];
    src[3].ai_data.services = values_service_flags[13];
    src[4].ai_data.services = values_service_flags[1];
    expected.ai_data.services = values_service_flags[13];
    src[1].npc_flags = values_npc_flags[1];
    src[2].npc_flags = values_npc_flags[2];
    src[3].npc_flags = values_npc_flags[3];
    src[4].npc_flags = values_npc_flags[2];
    expected.npc_flags = values_npc_flags[3];
    let npc_stats_1 = NpcStats::default();
    let mut npc_stats_2 = NpcStats::default();
    let mut npc_stats_3 = NpcStats::default();
    let mut npc_stats_4 = NpcStats::default();
    let mut npc_stats_expected = NpcStats::default();
    npc_stats_2.magicka = 2;
    npc_stats_3.magicka = 3;
    npc_stats_4.magicka = 2;
    npc_stats_expected.magicka = 3;
    npc_stats_4.fatigue = 4;
    npc_stats_expected.fatigue = 4;
    npc_stats_2.attributes[6] = 2;
    npc_stats_3.attributes[6] = 3;
    npc_stats_expected.attributes[6] = 3;
    npc_stats_2.skills[24] = 2;
    npc_stats_expected.skills[24] = 2;
    src[1].data.stats = Some(npc_stats_1);
    src[2].data.stats = Some(npc_stats_2);
    src[3].data.stats = Some(npc_stats_3);
    src[4].data.stats = Some(npc_stats_4);
    expected.data.stats = Some(npc_stats_expected);
    src[1].ai_packages = values_ai_packages_wander[1].clone();
    src[2].ai_packages = values_ai_packages_wander[2].clone();
    src[3].ai_packages = values_ai_packages_wander[1].clone();
    src[4].ai_packages = values_ai_packages_wander[1].clone();
    src[4].ai_packages.push(AiPackage::Travel(AiTravelPackage::default()));
    expected.ai_packages = src[4].ai_packages.clone();
    test_merge!(npc_, src, plugins, cfg, log, im, res, dst:1);
    assert_eq_inner!(Npc, expected, dst[0]);
}
