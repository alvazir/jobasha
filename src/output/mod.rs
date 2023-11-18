use crate::{Cfg, Creature, Item, ListCounts, Log};
use anyhow::{Context, Result};
use tes3::esp::{FileType, FixedString, Header, ObjectFlags, TES3Object};
mod make_lists;
mod show_messages;
mod structs;
mod write_plugins;
use make_lists::make_lists;
use show_messages::show_messages;
use structs::{
    DeletedSubrecords, DeleveledSubrecords, Helper, Masters, Messages, RawOutputPlugin, ThresholdMessageKind, ThresholdMessages,
    UntouchedList,
};
use write_plugins::write_plugins;

pub(super) fn process_output(creatures: Vec<Creature>, items: Vec<Item>, cfg: &Cfg, log: &mut Log) -> Result<(ListCounts, bool)> {
    let mut h: Helper = make_lists(creatures, items, cfg, log)?;
    if !(cfg.no_log && cfg.quiet) {
        show_messages(&h.messages, &h.counts, &mut h.warning, cfg, log)?;
    }
    if cfg.delev && !cfg.delev_distinct {
        make_plugin(
            &mut h.merge,
            h.counts.merge.placed,
            &cfg.guts.header_description_merge_and_delev,
            cfg,
        );
    } else {
        make_plugin(&mut h.merge, h.counts.merge.placed, &cfg.guts.header_description_merge, cfg);
    }
    if cfg.delev_distinct {
        make_plugin(&mut h.delev, h.counts.delev.placed, &cfg.guts.header_description_delev, cfg);
    }
    write_plugins(h.merge.plugin, h.delev.plugin, &mut h.counts, cfg, log).with_context(|| "Failed to write plugin")?;
    Ok((h.counts, h.warning))
}

fn make_plugin(raw: &mut RawOutputPlugin, counts_placed: usize, description: &str, cfg: &Cfg) {
    raw.plugin
        .objects
        .push(TES3Object::Header(make_header(cfg, &raw.masters, counts_placed, description)));
    raw.plugin.objects.append(&mut raw.lists);
}

fn make_header(cfg: &Cfg, masters: &Masters, counts_placed: usize, description: &str) -> Header {
    Header {
        flags: ObjectFlags::default(),
        version: cfg.guts.header_version,
        file_type: FileType::Esp,
        author: FixedString(cfg.guts.header_author.to_owned()),
        description: FixedString(description.to_owned()),
        num_objects: counts_placed as u32,
        masters: make_masters(masters),
    }
}

fn make_masters(masters: &Masters) -> Vec<(String, u64)> {
    let mut masters_sorted: Vec<(usize, &String, u64)> = masters.values().cloned().collect();
    masters_sorted.sort();
    masters_sorted.into_iter().map(|(_, b, c)| (b.to_owned(), c)).collect()
}
