use super::{
    generic_make_merge, generic_ref_record_method_inventory, generic_ref_record_methods, inventory_to_lowercase, print_as, show_flags,
    show_object_flags, InventoryHelper, LowInventory, MergeLog, OptionRecordMergeLog, RawPlugin, SpecificFlags,
};
use crate::{Cfg, IntermediateRecords};
use anyhow::{Context, Result};
use paste::paste;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::convert::identity;
use tes3::esp::{Container, ContainerFlags, FixedString, ObjectFlags, TES3Object};

pub(crate) struct ContRef<'a> {
    pub flags: ObjectFlags,
    pub id: &'a str,
    pub name: &'a str,
    pub script: &'a str,
    pub mesh: &'a str,
    pub encumbrance: &'a f32,
    pub container_flags: ContainerFlags,
    pub inventory: Vec<(i32, FixedString<32>)>,
    pub base: &'a Container,
    pub low: LowInventory,
}

show_flags!(ContainerFlags, ORGANIC, RESPAWNS, IS_BASE);

generic_ref_record_method_inventory!(ContRef);
generic_ref_record_methods!(
    (ContRef, Container, id),
    (name, script, mesh, encumbrance),
    (flags, container_flags),
    (LowInventory),
    (inventory),
    ()
);

generic_make_merge!(
    cont,
    (ContRef, Container, id),
    (flags=ObjectFlags, name.&, script.&, mesh.&, encumbrance.&, container_flags=ContainerFlags),
    (inventory),
    (),
    (),
    ()
);
