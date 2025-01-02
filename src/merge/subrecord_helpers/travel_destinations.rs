// COMMENT: used in [Creature, Npc]
use super::{add_and_log_field_lengthen, commit, get_vec_element, log_field_shorten, make, OptionRecordMergeLog};
use crate::{Cfg, CreatureRecordMap, NpcRecordMap, PluginInfo};
use anyhow::{Context, Result};
use paste::paste;
use std::cmp::Ordering;
use tes3::esp::TravelDestination;
mod self_macro;
use self_macro::process;

macro_rules! log_field_extend {
    ($option_log:ident, $shorten:expr, $element:expr, $record:expr, $map:ident, $cfg:ident) => {
        $option_log.field_extend(
            "travel_destinations",
            $shorten,
            format_args!(
                "\"{}\"({},{},{})({},{},{})",
                $element.cell,
                $element.translation[0],
                $element.translation[1],
                $element.translation[2],
                $element.rotation[0],
                $element.rotation[1],
                $element.rotation[2],
            ),
            &$record.plugin_info.name,
            &$map,
            $cfg,
        )?;
    };
}

type Added<'a> = (TravelDestination, &'a TravelDestination, &'a PluginInfo);
type Deleted<'a> = (usize, &'a PluginInfo, usize);

#[derive(Default)]
pub(crate) struct TravelDestinationsHelper<'a> {
    generated: bool,
    lowercased: Vec<Vec<TravelDestination>>,
    added: Vec<Vec<Added<'a>>>,
    deleted: Vec<Deleted<'a>>,
}

impl<'a> TravelDestinationsHelper<'a> {
    make!(travel_destinations, crea::add_second_deleted_index::true, Creature, npc_::add_second_deleted_index::true, Npc);
    process!(crea, Creature, npc_, Npc);
    commit!(TravelDestination, TravelDestinations, 0);

    pub(crate) fn get_low_sorted_last(&self) -> Result<Vec<&TravelDestination>> {
        let mut res = self
            .lowercased
            .last()
            .with_context(|| "Bug: failed to get TravelDestinationsHelper.lowercased.last()")?
            .iter()
            .collect::<Vec<&TravelDestination>>();
        sort_travel_destinations(&mut res);
        Ok(res)
    }

    fn added_to_owned(added: &[Added]) -> Result<TravelDestination> {
        let res = added
            .last()
            .with_context(|| "Bug: failed to get TravelDestinationsHelperHelper.added.last()")?
            .1
            .to_owned();
        Ok(res)
    }
}

fn is_replacement(subrecord: &TravelDestination, base: &TravelDestination, cfg: &Cfg) -> bool {
    match subrecord.cell.is_empty() {
        true => {
            base.cell.is_empty()
                && (subrecord.translation[0] - base.translation[0]).abs() <= cfg.merge.destination_similarity
                && (subrecord.translation[1] - base.translation[1]).abs() <= cfg.merge.destination_similarity
                && (subrecord.translation[2] - base.translation[2]).abs() <= cfg.merge.destination_similarity
        }
        false => subrecord.cell == base.cell,
    }
}

pub(crate) fn sort_travel_destinations(travel_destinations: &mut [&TravelDestination]) {
    travel_destinations.sort_by(|a, b| {
        a.cell
            .cmp(&b.cell)
            .then_with(|| a.translation[0].partial_cmp(&b.translation[0]).unwrap_or(Ordering::Equal))
            .then_with(|| a.translation[1].partial_cmp(&b.translation[1]).unwrap_or(Ordering::Equal))
            .then_with(|| a.translation[2].partial_cmp(&b.translation[2]).unwrap_or(Ordering::Equal))
    });
}

pub(crate) fn to_lowercase(arg: &TravelDestination) -> TravelDestination {
    TravelDestination {
        cell: arg.cell.to_lowercase(),
        ..*arg
    }
}

get_vec_element!(travel_destinations, TravelDestination, TravelDestination);
