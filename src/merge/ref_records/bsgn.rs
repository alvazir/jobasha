use super::{
    generic_make_merge, generic_ref_record_method_spells, generic_ref_record_methods, print_as, show_object_flags, spell_to_lowercase,
    LowSpells, MergeLog, OptionRecordMergeLog, RawPlugin, SpecificFlags, SpellsHelper,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::convert::identity;
use tes3::esp::{Birthsign, ObjectFlags, TES3Object};

pub(crate) struct BsgnRef<'a> {
    pub flags: ObjectFlags,
    pub id: &'a str,
    pub name: &'a str,
    pub texture: &'a str,
    pub description: &'a str,
    pub spells: Vec<String>,
    pub base: &'a Birthsign,
    pub low: LowSpells,
}

generic_ref_record_method_spells!(BsgnRef);
generic_ref_record_methods!(
    (BsgnRef, Birthsign, id),
    (name, texture, description),
    (flags),
    (LowSpells),
    (spells),
    ()
);

generic_make_merge!(
    bsgn,
    (BsgnRef, Birthsign, id),
    (flags=ObjectFlags, name.&, texture.&, description.&),
    (spells),
    (),
    (),
    ()
);
