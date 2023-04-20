use crate::{msg, Cfg, ListCounts, Log, MsgTone, ReadStats};
use anyhow::Result;
use std::time::Instant;

pub(crate) fn show_summary(
    timer: Instant,
    read_stats: ReadStats,
    counts: &ListCounts,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    let text = format!(
            "Performance: {:.3}s duration, {} plugins read at {:.0}/s, {} records read at {:.0}/s\nLists stats: {} total, {} unique, {} merged, {} placed / {} untouched, {} masters, {} deleted subrecords",
            timer.elapsed().as_secs_f64(),
            read_stats.plugins.total,
            read_stats.plugins.speed,
            read_stats.records.total,
            read_stats.records.speed,
            counts.total,
            counts.unique,
            counts.merged,
            counts.placed,
            counts.untouched,
            counts.master,
            counts.deleted_subrecord,
        );
    let show = if cfg.no_summary { 99 } else { 0 };
    msg(text, MsgTone::Neutral, show, cfg, log)
}
