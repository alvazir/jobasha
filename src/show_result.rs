use crate::{msg, plural, Cfg, ListCounts, Log, MsgTone, ReadStats};
use anyhow::Result;
use std::{fmt::Write as _, time::Instant};

pub(super) fn show_result(timer: Instant, read_stats: ReadStats, counts: ListCounts, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if cfg.compare_only {
        return Ok(());
    }
    let mut text = String::with_capacity(cfg.guts.long_message_string_inital_capacity * 4);
    writeln!(
        text,
        "Performance: {:.3}s duration, {} plugin{}({} record{}) read at {:.0}/s({:.0}/s)",
        timer.elapsed().as_secs_f64(),
        read_stats.plugins.total,
        plural("s", read_stats.plugins.total as usize)?,
        read_stats.records.total,
        plural("s", read_stats.records.total as usize)?,
        read_stats.plugins.speed,
        read_stats.records.speed,
    )?;
    write!(text, "Lists stats: {} total, {} unique", counts.total.total, counts.total.unique)?;
    if counts.total.placed > 0 && !(cfg.delev && cfg.delev_distinct) {
        write!(text, ", {} placed", counts.total.placed)?;
    }
    if counts.total.master > 0 && !(cfg.delev && cfg.delev_distinct) {
        write!(text, ", {} masters", counts.total.master)?;
    }
    write!(text, "\nMerge stats: {} merged", counts.merge.merged)?;
    if counts.merge.untouched > 0 {
        write!(text, ", {} untouched", counts.merge.untouched)?;
    }
    if counts.merge.placed > 0 && cfg.delev {
        write!(text, ", {} placed", counts.merge.placed)?;
    }
    if counts.merge.deleted_subrecord > 0 {
        write!(text, ", {} subrecords deleted", counts.merge.deleted_subrecord)?;
    }
    if counts.merge.master > 0 && cfg.delev && cfg.delev_distinct {
        write!(text, ", {} masters", counts.merge.master)?;
    }
    if cfg.delev {
        write!(text, "\nDelev stats: {} deleveled", counts.delev.deleveled)?;
        if counts.delev.placed > 0 {
            write!(text, ", {} placed", counts.delev.placed)?;
        }
        if counts.delev.deleveled_subrecord > 0 {
            write!(text, ", {} subrecords deleveled", counts.delev.deleveled_subrecord)?;
        }
        if counts.delev.master > 0 && cfg.delev_distinct {
            write!(text, ", {} masters", counts.delev.master)?;
        }
    }
    let show = if cfg.no_summary { u8::MAX } else { 0 };
    msg(&text, MsgTone::Neutral, show, cfg, log)?;
    show_output_plugin_suggestion(text, counts, cfg, log)
}

fn show_output_plugin_suggestion(mut text: String, counts: ListCounts, cfg: &Cfg, log: &mut Log) -> Result<()> {
    text.clear();
    if !cfg.dry_run {
        write!(text, "\nPlace ")?;
        let mut merge_placed = false;
        let mut delev_placed = false;
        if counts.merge.placed > 0 {
            merge_placed = true;
            write!(text, "\"{}\"", cfg.output.name)?;
        }
        if counts.delev.placed > 0 && cfg.delev && cfg.delev_distinct {
            if merge_placed {
                write!(text, " and ")?;
            }
            delev_placed = true;
            write!(text, "\"{}\"", cfg.delev_output.name)?;
        }
        writeln!(text, " last in load order and activate")?;
        if merge_placed || delev_placed {
            msg(text, MsgTone::Warm, 0, cfg, log)?;
        }
    }
    Ok(())
}
