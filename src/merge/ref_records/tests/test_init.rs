macro_rules! test_init {
            ($src:ident, $plugins:ident, $cfg:ident, $long:ident, $repeat:expr, $($values:ident),+$(, ($manual_field:ident$(:$manual_subfield:ident)? = $manual_value:expr))*$(; cfg=$cfgl1:ident$(:$cfgl2:ident)? = $cfgval:literal)*) => {

                #[allow(unused_macros)]
                macro_rules! values_string { () => { [
                    String::new(),
                    String::from("string_1"),
                    String::from("string_2"),
                    String::from("string_3"),
                ] }; }

                #[allow(unused_macros)]
                macro_rules! values_i8 { () => { [30, 31, 32, 33] }; }

                #[allow(unused_macros)]
                macro_rules! values_u8 { () => { [20, 21, 22, 23] }; }

                #[allow(unused_macros)]
                macro_rules! values_i16 { () => { [10, 11, 12, 13] }; }

                #[allow(unused_macros)]
                macro_rules! values_f32 { () => { [0.0, 1.1, 2.02, 3.003] }; }

                #[allow(unused_macros)]
                macro_rules! values_u32 { () => { [0, 1, 2, 3] }; }

                #[allow(unused_macros)]
                macro_rules! values_scale { () => {[Some(0.8), Some(0.9), Some(1.1), Some(1.2)]}; }

                #[allow(unused_macros)]
                macro_rules! values_region { () => {[
                    Some(String::from("region_0")),
                    Some(String::from("region_1")),
                    Some(String::from("region_2")),
                    Some(String::from("region_3")),
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_map_color { () => {[
                    Some([202, 165, 96, 0]),
                    Some([203, 164, 96, 0]),
                    Some([204, 163, 96, 0]),
                    Some([205, 162, 96, 0]),
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_water_height { () => {[Some(1.0), Some(100.0), Some(-100.0), Some(9999.99)]}; }

                #[allow(unused_macros)]
                macro_rules! values_atmosphere_data { () => {[
                    Some(AtmosphereData { fog_density: 0.1, ..Default::default() }),
                    Some(AtmosphereData { fog_density: 1.0, ambient_color: [1, 2, 3, 0], ..Default::default() }),
                    Some(AtmosphereData { fog_density: 1.0, sunlight_color: [4, 5, 6, 0], ..Default::default() }),
                    Some(AtmosphereData { fog_density: 1.0, fog_color: [7, 8, 9, 0], ..Default::default() }),
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_data_grid { () => {[
                    (0, 1061997773), // 0.8
                    (0, 1065353216), // 1.0
                    (0, 1056964608), // 0.5
                    (0, 1061158912), // 0.75
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_exterior_flags { () => {{
                    // COMMENT: actually every exterior flag has HAS_WATER flag
                    let mut last_flags = CellFlags::RESTING_IS_ILLEGAL;
                     // COMMENT: actually exterior should not have this flag
                    last_flags.insert(CellFlags::BEHAVES_LIKE_EXTERIOR);
                    [CellFlags::default(), CellFlags::HAS_WATER, CellFlags::RESTING_IS_ILLEGAL, last_flags]
                }}; }

                #[allow(unused_macros)]
                macro_rules! values_interior_flags { () => {{
                    let flags_0 = CellFlags::IS_INTERIOR;
                    let mut flags_1 = CellFlags::IS_INTERIOR;
                    flags_1.insert(CellFlags::HAS_WATER);
                    let mut flags_2 = CellFlags::IS_INTERIOR;
                    flags_2.insert(CellFlags::RESTING_IS_ILLEGAL);
                    let mut flags_3 = CellFlags::IS_INTERIOR;
                    flags_3.insert(CellFlags::BEHAVES_LIKE_EXTERIOR);
                    [flags_0, flags_1, flags_2, flags_3]
                }}; }

                #[allow(unused_macros)]
                macro_rules! values_magic_effect_color { () => {[
                    (10, 50, 0),
                    (110, 120, 130),
                    (210, 220, 230),
                    (250, 17, 1),
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_creature_type { () => { [
                    CreatureType::Normal,
                    CreatureType::Daedra,
                    CreatureType::Undead,
                    CreatureType::Humanoid,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_effect_school { () => { [
                    EffectSchool::Alteration,
                    EffectSchool::Conjuration,
                    EffectSchool::Destruction,
                    EffectSchool::Illusion,
                    EffectSchool::Mysticism,
                    EffectSchool::Restoration,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_magic_effect_flags { () => {[
                    MagicEffectFlags::TARGET_SKILL,
                    MagicEffectFlags::TARGET_ATTRIBUTE,
                    MagicEffectFlags::NO_DURATION,
                    MagicEffectFlags::NO_MAGNITUDE,
                    MagicEffectFlags::HARMFUL,
                    MagicEffectFlags::CONTINUOUS_VFX,
                    MagicEffectFlags::CAN_CAST_SELF,
                    MagicEffectFlags::CAN_CAST_TOUCH,
                    MagicEffectFlags::CAN_CAST_TARGET,
                    MagicEffectFlags::ALLOW_SPELLMAKING,
                    MagicEffectFlags::ALLOW_ENCHANTING,
                    MagicEffectFlags::NEGATIVE_LIGHTING,
                    MagicEffectFlags::APPLIED_ONCE,
                    MagicEffectFlags::UNKNOWN_CHAMELEON,
                    MagicEffectFlags::NON_RECASTABLE,
                    MagicEffectFlags::ILLEGAL_DAEDRA,
                    MagicEffectFlags::UNREFLECTABLE,
                    MagicEffectFlags::CASTER_LINKED,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_creature_flags { () => {[
                    CreatureFlags::BIPED,
                    CreatureFlags::RESPAWN,
                    CreatureFlags::WEAPON_AND_SHIELD,
                    CreatureFlags::IS_BASE,
                    CreatureFlags::SWIMS,
                    CreatureFlags::FLIES,
                    CreatureFlags::WALKS,
                    CreatureFlags::ESSENTIAL,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_object_flags { () => {[
                    ObjectFlags::MODIFIED,
                    ObjectFlags::DELETED,
                    ObjectFlags::PERSISTENT,
                    ObjectFlags::IGNORED,
                    ObjectFlags::BLOCKED,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_npc_flags { () => {[
                    NpcFlags::FEMALE,
                    NpcFlags::ESSENTIAL,
                    NpcFlags::RESPAWN,
                    NpcFlags::IS_BASE,
                    NpcFlags::AUTO_CALCULATE,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_service_flags { () => {[
                    ServiceFlags::BARTERS_WEAPONS,
                    ServiceFlags::BARTERS_ARMOR,
                    ServiceFlags::BARTERS_CLOTHING,
                    ServiceFlags::BARTERS_BOOKS,
                    ServiceFlags::BARTERS_INGREDIENTS,
                    ServiceFlags::BARTERS_LOCKPICKS,
                    ServiceFlags::BARTERS_PROBES,
                    ServiceFlags::BARTERS_LIGHTS,
                    ServiceFlags::BARTERS_APPARATUS,
                    ServiceFlags::BARTERS_REPAIR_ITEMS,
                    ServiceFlags::BARTERS_MISC_ITEMS,
                    ServiceFlags::OFFERS_SPELLS,
                    ServiceFlags::BARTERS_ENCHANTED_ITEMS,
                    ServiceFlags::BARTERS_ALCHEMY,
                    ServiceFlags::OFFERS_TRAINING,
                    ServiceFlags::OFFERS_SPELLMAKING,
                    ServiceFlags::OFFERS_ENCHANTING,
                    ServiceFlags::OFFERS_REPAIRS,
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_some_npc_data { () => {[
                    Some(NpcStats { health: 0, magicka: 0, ..Default::default() }),
                    Some(NpcStats { health: 1, magicka: 1, ..Default::default() }),
                    Some(NpcStats { health: 2, magicka: 2, ..Default::default() }),
                    Some(NpcStats { health: 3, magicka: 3, ..Default::default() }),
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_some_npc_data_skills_attributes { () => {{
                    let mut npc_stats_0 = NpcStats::default();
                    let mut npc_stats_1 = NpcStats::default();
                    let mut npc_stats_2 = NpcStats::default();
                    let mut npc_stats_3 = NpcStats::default();
                    npc_stats_0.skills[2] = 0;
                    npc_stats_1.skills[2] = 1;
                    npc_stats_2.skills[2] = 2;
                    npc_stats_3.skills[2] = 3;
                    npc_stats_0.skills[25] = 3;
                    npc_stats_1.skills[25] = 2;
                    npc_stats_2.skills[25] = 1;
                    npc_stats_3.skills[25] = 0;
                    npc_stats_0.attributes[4] = 0;
                    npc_stats_1.attributes[4] = 1;
                    npc_stats_2.attributes[4] = 2;
                    npc_stats_3.attributes[4] = 3;
                    npc_stats_0.attributes[6] = 0;
                    npc_stats_1.attributes[6] = 1;
                    npc_stats_2.attributes[6] = 2;
                    npc_stats_3.attributes[6] = 3;
                    [Some(npc_stats_0), Some(npc_stats_1), Some(npc_stats_2), Some(npc_stats_3)]
                }}; }

                #[allow(unused_macros)]
                macro_rules! values_ai_packages_wander { () => {[
                    vec![AiPackage::Wander(AiWanderPackage { idle7: 3, distance: 0, ..Default::default() })],
                    vec![AiPackage::Wander(AiWanderPackage { idle7: 2, distance: 1, ..Default::default() })],
                    vec![AiPackage::Wander(AiWanderPackage { idle7: 1, distance: 2, ..Default::default() })],
                    vec![AiPackage::Wander(AiWanderPackage { idle7: 0, distance: 3, ..Default::default() })],
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_ai_packages_follow { () => {[
                    vec![AiPackage::Follow(AiFollowPackage { location: [3.0,0.0,0.0], target: FixedString("0".to_string()), ..Default::default() })],
                    vec![AiPackage::Follow(AiFollowPackage { location: [2.0,0.0,1.0], target: FixedString("1".to_string()), ..Default::default() })],
                    vec![AiPackage::Follow(AiFollowPackage { location: [1.0,0.0,2.0], target: FixedString("2".to_string()), ..Default::default() })],
                    vec![AiPackage::Follow(AiFollowPackage { location: [0.0,0.0,3.0], target: FixedString("3".to_string()), ..Default::default() })],
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_ai_packages_escort { () => {[
                    vec![AiPackage::Escort(AiEscortPackage { duration: 0, cell: "0".to_string(), ..Default::default() })],
                    vec![AiPackage::Escort(AiEscortPackage { duration: 1, cell: "1".to_string(), ..Default::default() })],
                    vec![AiPackage::Escort(AiEscortPackage { duration: 2, cell: "2".to_string(), ..Default::default() })],
                    vec![AiPackage::Escort(AiEscortPackage { duration: 3, cell: "3".to_string(), ..Default::default() })],
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_ai_packages_activate { () => {[
                    vec![AiPackage::Activate(AiActivatePackage { reset: 0, target: FixedString("3".to_string()), ..Default::default() })],
                    vec![AiPackage::Activate(AiActivatePackage { reset: 1, target: FixedString("2".to_string()), ..Default::default() })],
                    vec![AiPackage::Activate(AiActivatePackage { reset: 2, target: FixedString("1".to_string()), ..Default::default() })],
                    vec![AiPackage::Activate(AiActivatePackage { reset: 3, target: FixedString("0".to_string()), ..Default::default() })],
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_ai_packages_travel { () => {[
                    vec![AiPackage::Travel(AiTravelPackage { location: [3.0,0.0,0.0], reset: 13, ..Default::default() })],
                    vec![AiPackage::Travel(AiTravelPackage { location: [2.1,0.1,1.0], reset: 12, ..Default::default() })],
                    vec![AiPackage::Travel(AiTravelPackage { location: [1.2,0.2,2.0], reset: 11, ..Default::default() })],
                    vec![AiPackage::Travel(AiTravelPackage { location: [0.3,0.3,3.0], reset: 10, ..Default::default() })],
                ]}; }

                #[allow(unused_macros)]
                macro_rules! values_spells { () => { [
                    vec!["spell_0".to_string()],
                    vec!["spell_0".to_string(), "spell_1".to_string()],
                    vec!["spell_0".to_string(), "spell_1".to_string(), "spell_2".to_string()],
                    vec!["spell_0".to_string(), "spell_1".to_string(), "spell_2".to_string(), "spell_3".to_string()],
                    vec!["spell_4".to_string()],
                    vec!["spell_0".to_string(), "spell_1".to_string(), "spell_2".to_string(), "spell_3".to_string(), "spell_4".to_string()],
                    vec!["spell_0".to_string(), "spell_4".to_string()],
                    vec!["spell_1".to_string(), "spell_2".to_string(), "spell_3".to_string()],
                    vec!["SPELL_1".to_string(), "SPELL_2".to_string(), "SPELL_3".to_string()],
                ] }; }

                #[allow(unused_macros)]
                macro_rules! values_inventory { () => {{
                        let i0 = (0_i32, FixedString::<32>("inventory_0".to_string()));
                        let i1 = (1_i32, FixedString::<32>("inventory_1".to_string()));
                        let i2 = (2_i32, FixedString::<32>("inventory_2".to_string()));
                        let i3 = (3_i32, FixedString::<32>("inventory_3".to_string()));
                        let i4 = (4_i32, FixedString::<32>("inventory_4".to_string()));
                        [
                            vec![i0.clone()],
                            vec![i0.clone(), i1.clone()],
                            vec![i0.clone(), i1.clone(), i2.clone()],
                            vec![i0.clone(), i1.clone(), i2.clone(), i3.clone()],
                            vec![i4.clone()],
                            vec![i0.clone(), i1.clone(), i2.clone(), i3.clone(), i4.clone()],
                            vec![i0.clone(), i4.clone()],
                            vec![i1.clone(), i2.clone(), i3.clone()],
                            vec![(1_i32, FixedString::<32>("INVENTORY_1".to_string())), (2_i32, FixedString::<32>("INVENTORY_2".to_string())), (3_i32, FixedString::<32>("INVENTORY_3".to_string()))],
                        ]
                    }}; }

                #[allow(unused_macros)]
                macro_rules! values_travel_destinations_i { () => {{
                        let tdi0 = TravelDestination { cell: "cell_0".to_string(), ..Default::default() };
                        let tdi1 = TravelDestination { cell: "cell_1".to_string(), ..Default::default() };
                        let tdi2 = TravelDestination { cell: "cell_2".to_string(), ..Default::default() };
                        let tdi3 = TravelDestination { cell: "cell_3".to_string(), ..Default::default() };
                        let tdi4 = TravelDestination { cell: "cell_4".to_string(), ..Default::default() };
                        let tdic1 = TravelDestination { cell: "CELL_1".to_string(), ..Default::default() };
                        let tdic2 = TravelDestination { cell: "CELL_2".to_string(), ..Default::default() };
                        let tdic3 = TravelDestination { cell: "CELL_3".to_string(), ..Default::default() };
                        [
                            vec![tdi0.clone()],
                            vec![tdi0.clone(), tdi1.clone()],
                            vec![tdi0.clone(), tdi1.clone(), tdi2.clone()],
                            vec![tdi0.clone(), tdi1.clone(), tdi2.clone(), tdi3.clone()],
                            vec![tdi4.clone()],
                            vec![tdi0.clone(), tdi1.clone(), tdi2.clone(), tdi3.clone(), tdi4.clone()],
                            vec![tdi0.clone(), tdi4.clone()],
                            vec![tdi1.clone(), tdi2.clone(), tdi3.clone()],
                            vec![tdic1.clone(), tdic2.clone(), tdic3.clone()],
                            vec![tdi0.clone()],
                        ]
                    }}; }

                #[allow(unused_macros)]
                macro_rules! values_travel_destinations_e { () => {{
                        let tde0 = TravelDestination::default();
                        let tde1 = TravelDestination { translation: [1111_f32, 1111_f32, 1111_f32], ..Default::default() };
                        let tde2 = TravelDestination { translation: [2222_f32, 2222_f32, 2222_f32], ..Default::default() };
                        let tde3 = TravelDestination { translation: [3333_f32, 3333_f32, 3333_f32], ..Default::default() };
                        let tde4 = TravelDestination { translation: [4444_f32, 4444_f32, 4444_f32], ..Default::default() };
                        let tde5 = TravelDestination { translation: [4455_f32, 4455_f32, 4455_f32], ..Default::default() };
                        [
                            vec![tde0.clone()],
                            vec![tde0.clone(), tde1.clone()],
                            vec![tde0.clone(), tde1.clone(), tde2.clone()],
                            vec![tde0.clone(), tde1.clone(), tde2.clone(), tde3.clone()],
                            vec![tde4.clone()],
                            vec![tde0.clone(), tde1.clone(), tde2.clone(), tde3.clone(), tde4.clone()],
                            vec![tde0.clone(), tde4.clone()],
                            vec![tde1.clone(), tde2.clone(), tde3.clone()],
                            vec![tde1.clone(), tde2.clone(), tde3.clone()],
                            vec![tde5.clone()],
                        ]
                    }}; }

                #[allow(unused_macros)]
                macro_rules! values_travel_destinations { () => {{
                    let mut joined = values_travel_destinations_i!().to_vec();
                    joined.extend(values_travel_destinations_e!().to_vec());
                    joined
                }};}

                #[allow(unused_mut)]
                let mut $src = repeat(
                    $long {
                        $($manual_field: $manual_value,)*
                        ..Default::default()
                    }
                ).take($repeat).collect::<Vec<$long>>();
                #[allow(unused_mut)]
                let mut $plugins = (0..$repeat)
                    .collect::<Vec<usize>>()
                    .iter()
                    .map(|num| {
                        PluginInfo {
                            name: format!("Plugin{num}.esp"),
                            ..Default::default()
                        }
                    })
                    .collect::<Vec<PluginInfo>>();
                #[allow(unused_mut)]
                let mut $cfg = Cfg::default();
                $cfg.guts.debug_level_merge_list_all_plugins = u8::MAX;
                $cfg.guts.debug_level_merge_skipped_all_equal = u8::MAX;
                $cfg.guts.debug_level_merge_multipatch_attempt = u8::MAX;
                $cfg.merge.interdependent_flags = true;
                $cfg.merge.cell = true;
                $($cfg.$cfgl1$(.$cfgl2)? = $cfgval;)*
                $(#[allow(unused)]
                let $values = $values!();)+
            }
        }

pub(crate) use test_init;
