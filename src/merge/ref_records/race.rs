use super::{
    generic_make_merge, generic_ref_record_method_spells, generic_ref_record_methods, print_as, show_flags, show_object_flags,
    spell_to_lowercase, LowSpells, MergeLog, OptionRecordMergeLog, RawPlugin, SpecificFlags, SpellsHelper,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::convert::identity;
// use tes3::esp::{ObjectFlags, Race, RaceData, RaceFlags, SkillBonuses, SkillId, TES3Object};
use tes3::esp::{ObjectFlags, Race, RaceData, RaceFlags, TES3Object};

pub(crate) struct RaceRef<'a> {
    pub flags: ObjectFlags,
    pub id: &'a str,
    pub name: &'a str,
    pub spells: Vec<String>,
    pub description: &'a str,
    pub data: RaceData,
    pub base: &'a Race,
    pub low: LowSpells,
}

show_flags!(RaceFlags, PLAYABLE, BEAST_RACE);

generic_ref_record_method_spells!(RaceRef);
generic_ref_record_methods!((RaceRef, Race, id), (name, description), (flags, data), (LowSpells), (spells), ());

generic_make_merge!(
    race,
    (RaceRef, Race, id),
    (
        flags=ObjectFlags,
        name.&,
        description.&,
// data:skill_bonuses:skill_0,
// data:skill_bonuses:bonus_0,
// data:skill_bonuses:skill_1,
// data:skill_bonuses:bonus_1,
// data:skill_bonuses:skill_2,
// data:skill_bonuses:bonus_2,
// data:skill_bonuses:skill_3,
// data:skill_bonuses:bonus_3,
// data:skill_bonuses:skill_4,
// data:skill_bonuses:bonus_4,
// data:skill_bonuses:skill_5,
// data:skill_bonuses:bonus_5,
// data:skill_bonuses:skill_6,
// data:skill_bonuses:bonus_6,
        data:strength;0,
        data:strength;1,
        data:intelligence;0,
        data:intelligence;1,
        data:willpower;0,
        data:willpower;1,
        data:agility;0,
        data:agility;1,
        data:speed;0,
        data:speed;1,
        data:endurance;0,
        data:endurance;1,
        data:personality;0,
        data:personality;1,
        data:luck;0,
        data:luck;1,
        data:height;0,
        data:height;1,
        data:weight;0,
        data:weight;1,
        data:flags=RaceFlags
    ),
    (spells),
    (),
    (),
    ()
);
