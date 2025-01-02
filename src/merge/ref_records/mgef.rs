use super::{
    generic_make_merge, generic_ref_record_methods, print_as, show_flags, show_object_flags, MergeLog, OptionRecordMergeLog,
    RawPlugin, SpecificFlags,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::convert::identity;
use tes3::esp::{EffectId, MagicEffect, MagicEffectData, MagicEffectFlags, ObjectFlags, TES3Object};

#[cfg(test)]
mod tests;
#[cfg(test)]
use super::{
    assert_eq_inner, test_basic, test_debug, test_debug_all_equal, test_debug_compare_to_the_last, test_debug_equal_to_the_last,
    test_debug_list_all_plugins, test_debug_single, test_flags, test_init, test_log, test_log_flags, test_merge,
};

pub(crate) struct MgefRef<'a> {
    pub flags: ObjectFlags,
    pub effect_id: &'a EffectId,
    pub icon: &'a str,
    pub texture: &'a str,
    pub bolt_sound: &'a str,
    pub cast_sound: &'a str,
    pub hit_sound: &'a str,
    pub area_sound: &'a str,
    pub cast_visual: &'a str,
    pub bolt_visual: &'a str,
    pub hit_visual: &'a str,
    pub area_visual: &'a str,
    pub description: &'a str,
    pub data: MagicEffectData,
}

show_flags!(
    MagicEffectFlags,
    TARGET_SKILL,
    TARGET_ATTRIBUTE,
    NO_DURATION,
    NO_MAGNITUDE,
    HARMFUL,
    CONTINUOUS_VFX,
    CAN_CAST_SELF,
    CAN_CAST_TOUCH,
    CAN_CAST_TARGET,
    ALLOW_SPELLMAKING,
    ALLOW_ENCHANTING,
    NEGATIVE_LIGHTING,
    APPLIED_ONCE,
    UNKNOWN_CHAMELEON,
    NON_RECASTABLE,
    ILLEGAL_DAEDRA,
    UNREFLECTABLE,
    CASTER_LINKED
);

generic_ref_record_methods!(
    (MgefRef, MagicEffect, effect_id),
    (
        icon,
        texture,
        bolt_sound,
        cast_sound,
        hit_sound,
        area_sound,
        cast_visual,
        bolt_visual,
        hit_visual,
        area_visual,
        description
    ),
    (flags, data),
    (),
    (),
    ()
);

generic_make_merge!(
    mgef,
    (MgefRef, MagicEffect, effect_id),
    (
        flags=ObjectFlags,
        icon.&,
        texture.&,
        bolt_sound.&,
        cast_sound.&,
        hit_sound.&,
        area_sound.&,
        cast_visual.&,
        bolt_visual.&,
        hit_visual.&,
        area_visual.&,
        description.&,
        data:school,
        data:base_cost,
        data:flags=MagicEffectFlags,
        data:color:0,
        data:color:1,
        data:color:2,
        data:speed,
        data:size,
        data:size_cap
        ),
    (),
    (),
    (),
    ()
);
