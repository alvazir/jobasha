// COMMENT: used in [Container, Creature, Npc].
use super::{add_and_log_field_lengthen, get_vec_element, log_field_shorten, OptionRecordMergeLog};
use crate::{Cfg, ContainerRecordMap, CreatureRecordMap, NpcRecordMap, PluginInfo};
use anyhow::{Context, Result};
use paste::paste;
use tes3::esp::FixedString;
mod self_macro;
mod super_macro;
use self_macro::{make, process};
pub(super) use super_macro::{added_to_owned, commit, get_low_sorted_last};

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::test_basic_inventory;

macro_rules! log_field_extend {
    ($option_log:ident, $shorten:expr, $element:expr, $record:expr, $map:ident, $cfg:ident) => {
        $option_log.field_extend(
            "inventory",
            $shorten,
            format_args!("\"{}\"({})", $element.1.as_str(), $element.0),
            &$record.plugin_info.name,
            &$map,
            $cfg,
        )?;
    };
}

type InventoryRecord = (i32, FixedString<32>);
type InventoryRecordLow = (i32, String);
type Added<'a> = (InventoryRecordLow, &'a InventoryRecord, &'a PluginInfo);

#[derive(Default)]
pub(crate) struct InventoryHelper<'a> {
    generated: bool,
    lowercased: Vec<Vec<InventoryRecordLow>>,
    added: Vec<Added<'a>>,
    deleted: Vec<(InventoryRecordLow, usize, &'a PluginInfo)>,
}

impl<'a> InventoryHelper<'a> {
    make!(crea, Creature, npc_, Npc, cont, Container);
    process!(crea, Creature, npc_, Npc, cont, Container);
    commit!(InventoryRecord, Inventory, 1);
    get_low_sorted_last!(InventoryRecordLow, Inventory);
    added_to_owned!(InventoryRecord, Added);
}

pub(crate) fn to_lowercase(arg: &InventoryRecord) -> InventoryRecordLow {
    (arg.0, arg.1.to_lowercase())
}

get_vec_element!(inventory, (i32, FixedString<32>), (i32, FixedString<32>));
