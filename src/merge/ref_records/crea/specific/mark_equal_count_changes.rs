use super::{count_changes, SpecificFlags};
use crate::Cfg;
use tes3::esp::Creature;

pub(super) fn mark_equal_count_changes(merged: &mut Creature, last: &Creature, specific_flags: &mut SpecificFlags, cfg: &Cfg) {
    specific_flags.equal_after_specific = true;
    if cfg.meta.debug_compare_equal {
        count_changes!(
            merged,
            last,
            specific_flags,
            flags,
            name,
            script,
            mesh,
            ai_data:hello,
            ai_data:fight,
            ai_data:flee,
            ai_data:alarm,
            ai_data:services,
            sound,
            scale,
            creature_flags,
            blood_type,
            data:creature_type,
            data:level,
            data:strength,
            data:intelligence,
            data:willpower,
            data:agility,
            data:speed,
            data:endurance,
            data:personality,
            data:luck,
            data:health,
            data:magicka,
            data:fatigue,
            data:soul,
            data:combat,
            data:magic,
            data:stealth,
            data:gold,
            data:attack1:0,
            data:attack1:1,
            data:attack2:0,
            data:attack2:1,
            data:attack3:0,
            data:attack3:1,
            inventory,
            spells,
            travel_destinations,
            ai_packages
        );
    }
}
