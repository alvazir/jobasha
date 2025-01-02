use super::{assert_eq, *};

test_basic!(crea, Creature,
    values_object_flags:flags,
    values_string:name,
    values_string:script,
    values_string:mesh,
    values_inventory:inventory,
    values_spells:spells,
    values_i16:ai_data:hello,
    values_i8:ai_data:fight,
    values_i8:ai_data:flee,
    values_i8:ai_data:alarm,
    values_service_flags:ai_data:services,
    values_ai_packages_follow:ai_packages,
    values_travel_destinations_e:travel_destinations,
    values_string:sound,
    values_scale:scale,
    values_creature_flags:creature_flags,
    values_u8:blood_type,
    values_creature_type:data:creature_type,
    values_u32:data:level,
    values_u32:data:strength,
    values_u32:data:intelligence,
    values_u32:data:willpower,
    values_u32:data:agility,
    values_u32:data:speed,
    values_u32:data:endurance,
    values_u32:data:personality,
    values_u32:data:luck,
    values_u32:data:health,
    values_u32:data:magicka,
    values_u32:data:fatigue,
    values_u32:data:soul,
    values_u32:data:combat,
    values_u32:data:magic,
    values_u32:data:stealth,
    values_u32:data:gold,
    values_u32:data:attack1:0,
    values_u32:data:attack1:1,
    values_u32:data:attack2:0,
    values_u32:data:attack2:1,
    values_u32:data:attack3:0,
    values_u32:data:attack3:1
);
