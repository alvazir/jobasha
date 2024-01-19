use super::{DeletedSubrecords, DeleveledSubrecords, Messages, ThresholdMessageKind, ThresholdMessages, UntouchedList};
use crate::{msg, plural, Cfg, ListCounts, Log, MsgTone};
use anyhow::{anyhow, Result};
use std::{cmp::max, fmt::Write as _};

pub(crate) fn show_messages(m: &Messages, counts: &ListCounts, exit_code: &mut i32, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let mut text = match create_text_with_enough_capacity(m, counts, cfg) {
        Ok(Some(text)) => text,
        Ok(None) => return Ok(()),
        Err(_) => return Err(anyhow!("Bug: messages list contains nothing")),
    };
    if !m.deleted_subrecords.is_empty() {
        show_deleted_subrecords(&mut text, &m.deleted_subrecords, counts.merge.deleted_subrecord, cfg, log)?;
    }
    if !m.untouched_lists.is_empty() {
        show_untouched_lists(&mut text, &m.untouched_lists, cfg, log)?;
    }
    if !m.threshold_resolved.is_empty() {
        show_threshold_messages(&mut text, &m.threshold_resolved, cfg, log)?;
    }
    if !m.threshold_skipped.is_empty() {
        show_threshold_messages(&mut text, &m.threshold_skipped, cfg, log)?;
    }
    if !m.threshold_warnings.is_empty() {
        show_threshold_messages(&mut text, &m.threshold_warnings, cfg, log)?;
        *exit_code = 2;
    }
    if !m.deleveled_subrecords.is_empty() {
        show_deleveled_subrecords(&mut text, &m.deleveled_subrecords, counts.delev.deleveled_subrecord, cfg, log)?;
    }
    Ok(())
}

fn show_deleted_subrecords(
    text: &mut String,
    list: &[DeletedSubrecords],
    subrecords_count: usize,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    let details = cfg.guts.verboseness_details_deleted_subrecords;
    write!(
        text,
        "{} subrecord{} from {} leveled list{} {} deleted",
        subrecords_count,
        plural("s", subrecords_count)?,
        list.len(),
        plural("s", list.len())?,
        plural("were", subrecords_count)?,
    )?;
    msg_with_details_suggestion(text, MsgTone::Good, 0, details, cfg, log)?;
    if !(cfg.no_log && cfg.verbose < details) {
        writeln!(
            text,
            "\n\t{:>2} {:<32} {:<1} {:<32} {:<16} RESPONSIBLE PLUGINS",
            "LV", "DELETED SUBRECORD", "T", "LEVELED LIST", "INITIAL PLUGIN"
        )?;
        for list_item in list.iter() {
            for subrecord in list_item.subrecords.iter() {
                writeln!(
                    text,
                    "\t{:>2} {:<32} {:<1} {:<32} {:<16} {}",
                    subrecord.0 .1,
                    subrecord.0 .0,
                    list_item.log_t,
                    list_item.id,
                    list_item.initial_plugin,
                    subrecord.1.join(", "),
                )?;
            }
        }
        msg_and_clear(text, MsgTone::Neutral, details, cfg, log)?;
    }
    Ok(())
}

fn show_untouched_lists(text: &mut String, list: &[UntouchedList], cfg: &Cfg, log: &mut Log) -> Result<()> {
    let details = cfg.guts.verboseness_details_untouched_lists;
    text.clear();
    write!(
        text,
        "{} merged leveled list{} {} identical to last loaded list{} hence not placed into the output plugin",
        list.len(),
        plural("s", list.len())?,
        plural("were", list.len())?,
        plural("s", list.len())?,
    )?;
    msg_with_details_suggestion(text, MsgTone::Good, 0, details, cfg, log)?;
    if !(cfg.no_log && cfg.verbose < details) {
        writeln!(text, "\n\t{:<1} {:<32} {:<32} LAST PLUGIN", "T", "LEVELED LIST", "INITIAL PLUGIN")?;
        for list_item in list.iter() {
            writeln!(
                text,
                "\t{:<1} {:<32} {:<32} {}",
                list_item.log_t, list_item.id, list_item.initial_plugin, list_item.last_plugin
            )?;
        }
        msg_and_clear(text, MsgTone::Neutral, details, cfg, log)?;
    }
    Ok(())
}

fn show_threshold_messages(text: &mut String, list: &ThresholdMessages, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let warnings_off = if cfg.no_threshold_warnings { u8::MAX } else { 0 };
    let details = match list.kind {
        ThresholdMessageKind::Resolved => cfg.guts.verboseness_details_threshold_resolved,
        ThresholdMessageKind::Skipped => cfg.guts.verboseness_details_threshold_skipped,
        ThresholdMessageKind::Warning => cfg.guts.verboseness_details_threshold_warnings,
    };
    match list.kind {
        ThresholdMessageKind::Resolved => {
            write!(
                text,
                "{} leveled list{} automatically excluded from subrecord deletion mode",
                list.messages.len(),
                plural("were", list.messages.len())?,
            )?;
            msg_with_details_suggestion(text, MsgTone::Good, max(0, warnings_off), details, cfg, log)?;
        }
        ThresholdMessageKind::Skipped => {
            write!(
                text,
                "{} merged leveled list{} had subrecords auto-deleted due to always-delete rule",
                list.messages.len(),
                plural("s", list.messages.len())?,
            )?;
            msg_with_details_suggestion(text, MsgTone::Good, max(0, warnings_off), details, cfg, log)?;
        }
        ThresholdMessageKind::Warning => {
            write!(
                text,
                "{} list{} {} ratio of deleted/initial subrecords higher than threshold and subrecords were deleted",
                list.messages.len(),
                plural("s", list.messages.len())?,
                plural("have", list.messages.len())?,
            )?;
            msg_with_details_suggestion(text, MsgTone::Bad, max(0, warnings_off), details, cfg, log)?;
        }
    };
    if !(cfg.no_log && cfg.verbose < details) {
        writeln!(
            text,
            "\n\t{:>5} {:<5} {:<1} {:<32} {:<32} RESPONSIBLE PLUGINS",
            "RATIO", "THOLD", "T", "LEVELED LIST", "INITIAL PLUGIN"
        )?;
        let mut plugins: Vec<String> = Vec::new();
        for ratio_message_raw in list.messages.iter() {
            if !plugins.contains(ratio_message_raw.initial_plugin) {
                plugins.push(ratio_message_raw.initial_plugin.to_owned());
            }
            writeln!(
                text,
                "\t{:>5} {:<5} {:<1} {:<32} {:<32} {}",
                format!("{:.0}%", ratio_message_raw.ratio),
                format!("{}%", ratio_message_raw.threshold),
                ratio_message_raw.log_t,
                ratio_message_raw.id,
                ratio_message_raw.initial_plugin,
                ratio_message_raw.responsible_plugins_str.join(", "),
            )?
        }
        match list.kind {
            ThresholdMessageKind::Skipped => {}
            _ => {
                writeln!(
                    text,
                    "
Following plugin{} {} probably not designed for deletion from leveled lists:
\t\"{}\"

Consider performing any of these actions:
\t1. Disable subrecord deletion for leveled lists from {} plugin{} with --never-delete:
\t\t--never-delete \"{}\"
\t2. Increase ratio threshold with --threshold-creatures / --threshold-items
\t3. Disable subrecord deletion at all with --no-delete
\t4. Disable this warning with --no-threshold-warnings",
                    plural("s", list.messages.len())?,
                    plural("are", list.messages.len())?,
                    plugins.join("\", \""),
                    plural("these", list.messages.len())?,
                    plural("s", list.messages.len())?,
                    plugins.join(","),
                )?;
            }
        };
        msg_and_clear(text, MsgTone::Neutral, max(details, warnings_off), cfg, log)?;
    };
    Ok(())
}

fn show_deleveled_subrecords(
    text: &mut String,
    list: &[DeleveledSubrecords],
    subrecords_count: usize,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    let details = cfg.guts.verboseness_details_deleveled_subrecords;
    write!(
        text,
        "{} subrecord{} from {} leveled list{} {} deleveled",
        subrecords_count,
        plural("s", subrecords_count)?,
        list.len(),
        plural("s", list.len())?,
        plural("were", subrecords_count)?,
    )?;
    msg_with_details_suggestion(text, MsgTone::Good, 0, details, cfg, log)?;
    if !(cfg.no_log && cfg.verbose < details) {
        writeln!(
            text,
            "\n\t{:>3} {:>3} {:<32} {:<1} {:<32} {:<16}",
            "NEW", "OLD", "DELEVELED SUBRECORD", "T", "LEVELED LIST", "INITIAL PLUGIN"
        )?;
        for list_item in list.iter() {
            for subrecord in list_item.subrecords.iter() {
                writeln!(
                    text,
                    "\t{:>3} {:>3} {:<32} {:<1} {:<32} {:<16}",
                    subrecord.1, subrecord.0 .1, subrecord.0 .0, list_item.log_t, list_item.id, list_item.initial_plugin,
                )?;
            }
        }
        msg_and_clear(text, MsgTone::Neutral, details, cfg, log)?;
    }
    Ok(())
}

fn msg_with_details_suggestion(text: &mut String, tone: MsgTone, verbose: u8, details: u8, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let _ = if cfg.verbose >= details {
        write!(text, ":")
    } else {
        write!(
            text,
            ", add {:v<details$}{} for details",
            "-",
            if cfg.no_log { "" } else { " or check log" },
            details = details as usize + 1,
        )
    };
    msg_and_clear(text, tone, verbose, cfg, log)
}

fn msg_and_clear(text: &mut String, tone: MsgTone, verbose: u8, cfg: &Cfg, log: &mut Log) -> Result<()> {
    msg(&text, tone, verbose, cfg, log)?;
    text.clear();
    Ok(())
}

fn create_text_with_enough_capacity(m: &Messages, counts: &ListCounts, cfg: &Cfg) -> Result<Option<String>> {
    match [
        counts.merge.deleted_subrecord,
        m.untouched_lists.len(),
        m.threshold_resolved.messages.len(),
        m.threshold_skipped.messages.len(),
        m.threshold_warnings.messages.len(),
        counts.delev.deleveled_subrecord,
    ]
    .iter()
    .max()
    {
        Some(0) => Ok(None),
        Some(max) => Ok(Some(String::with_capacity(cfg.guts.details_line_approximate_length * max))),
        None => unreachable!(),
    }
}
