use super::{
    generic_make_merge, generic_ref_record_methods, print_as, show_object_flags, MergeLog, OptionRecordMergeLog, RawPlugin,
    SpecificFlags,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::convert::identity;
use tes3::esp::{ObjectFlags, Skill, SkillData, SkillId, TES3Object};

pub(crate) struct SkilRef<'a> {
    pub flags: ObjectFlags,
    pub skill_id: &'a SkillId,
    pub data: SkillData,
    pub description: &'a str,
}

generic_ref_record_methods!((SkilRef, Skill, skill_id), (description), (flags, data), (), (), ());

generic_make_merge!(
    skil,
    (SkilRef, Skill, skill_id),
    (
        flags=ObjectFlags,
        data:governing_attribute,
        data:specialization,
        data:actions;0,
        data:actions;1,
        data:actions;2,
        data:actions;3,
        description.&
    ),
    (),
    (),
    (),
    ()
);
