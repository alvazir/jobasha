// COMMENT: used in [Birthsign, Creature, Npc, Race].
use super::{
    add_and_log_field_lengthen, added_to_owned, commit, get_low_sorted_last, get_vec_element, log_field_shorten, OptionRecordMergeLog,
};
use crate::{BirthsignRecordMap, Cfg, CreatureRecordMap, NpcRecordMap, PluginInfo, RaceRecordMap};
use anyhow::{Context, Result};
use paste::paste;
mod self_macro;
mod super_macro;
use self_macro::process;
pub(super) use super_macro::make;

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::test_basic_spells_and_travel;

macro_rules! log_field_extend {
    ($option_log:ident, $shorten:expr, $element:expr, $record:expr, $map:ident, $cfg:ident) => {
        $option_log.field_extend(
            "spells",
            $shorten,
            format_args!("\"{}\"", $element),
            &$record.plugin_info.name,
            &$map,
            $cfg,
        )?;
    };
}

type Added<'a> = (String, &'a str, &'a PluginInfo);

#[derive(Default)]
pub(crate) struct SpellsHelper<'a> {
    generated: bool,
    lowercased: Vec<Vec<String>>,
    added: Vec<Added<'a>>,
    deleted: Vec<(usize, &'a PluginInfo)>,
}

impl<'a> SpellsHelper<'a> {
    make!(spells, crea, Creature, npc_, Npc, bsgn, Birthsign, race, Race);
    process!(crea, Creature, npc_, Npc, bsgn, Birthsign, race, Race);
    commit!(String, Spells, 0);
    get_low_sorted_last!(String, Spells);
    added_to_owned!(String, Added);
}

pub(crate) fn to_lowercase(arg: &str) -> String {
    arg.to_lowercase()
}

get_vec_element!(spells, String, str);
