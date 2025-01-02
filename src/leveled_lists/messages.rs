use crate::{append_for_details_or_check_log, msg, plural, Cfg, ListCounts, LlElement, Log, MsgTone, PluginName, ResponsiblePlugins};
use anyhow::{anyhow, Result};
use std::{cmp::max, fmt::Write as _};

pub(crate) struct LlMessages<'a> {
    pub(crate) threshold_resolved: ThresholdMessages<'a>,
    pub(crate) threshold_skipped: ThresholdMessages<'a>,
    pub(crate) threshold_warnings: ThresholdMessages<'a>,
    pub(crate) untouched_lists: Vec<UntouchedList<'a>>,
    pub(crate) deleted_subrecords: Vec<DeletedSubrecords<'a>>,
    pub(crate) deleveled_subrecords: Vec<DeleveledSubrecords<'a>>,
}

impl<'a> LlMessages<'a> {
    pub(crate) fn new() -> LlMessages<'a> {
        LlMessages {
            threshold_resolved: ThresholdMessages::new(ThresholdMessageKind::Resolved),
            threshold_skipped: ThresholdMessages::new(ThresholdMessageKind::Skipped),
            threshold_warnings: ThresholdMessages::new(ThresholdMessageKind::Warning),
            untouched_lists: Vec::new(),
            deleted_subrecords: Vec::new(),
            deleveled_subrecords: Vec::new(),
        }
    }

    pub(crate) fn exit_code(&self) -> i32 {
        if self.threshold_warnings.is_empty() {
            0
        } else {
            2
        }
    }

    pub(crate) fn show(&self, counts: &ListCounts, cfg: &Cfg, log: &mut Log) -> Result<()> {
        let mut text = match self.create_text_with_enough_capacity(counts, cfg) {
            Ok(Some(text)) => text,
            Ok(None) => return Ok(()),
            Err(_) => return Err(anyhow!("Bug: messages list contains nothing")),
        };
        if !self.deleted_subrecords.is_empty() {
            show_deleted_subrecords(&mut text, &self.deleted_subrecords, counts.merge.deleted_subrecord, cfg, log)?;
        }
        if !self.untouched_lists.is_empty() {
            show_untouched_lists(&mut text, &self.untouched_lists, cfg, log)?;
        }
        if !self.threshold_resolved.is_empty() {
            show_threshold_messages(&mut text, &self.threshold_resolved, cfg, log)?;
        }
        if !self.threshold_skipped.is_empty() {
            show_threshold_messages(&mut text, &self.threshold_skipped, cfg, log)?;
        }
        if !self.threshold_warnings.is_empty() {
            show_threshold_messages(&mut text, &self.threshold_warnings, cfg, log)?;
        }
        if !self.deleveled_subrecords.is_empty() {
            show_deleveled_subrecords(&mut text, &self.deleveled_subrecords, counts.delev.deleveled_subrecord, cfg, log)?;
        }
        Ok(())
    }

    fn create_text_with_enough_capacity(&self, counts: &ListCounts, cfg: &Cfg) -> Result<Option<String>> {
        match [
            counts.merge.deleted_subrecord,
            self.untouched_lists.len(),
            self.threshold_resolved.messages.len(),
            self.threshold_skipped.messages.len(),
            self.threshold_warnings.messages.len(),
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
}

pub(crate) struct ThresholdMessages<'a> {
    pub(crate) kind: ThresholdMessageKind,
    pub(crate) messages: Vec<ThresholdMessageRaw<'a>>,
}

impl<'a> ThresholdMessages<'a> {
    fn new(kind: ThresholdMessageKind) -> ThresholdMessages<'a> {
        ThresholdMessages {
            kind,
            messages: Vec::new(),
        }
    }

    pub(crate) fn push(
        &mut self,
        ratio: f64,
        threshold: f64,
        log_t: &'a str,
        id: String,
        initial_plugin: PluginName<'a>,
        delete: &[(LlElement, LlElement, ResponsiblePlugins<'a>)],
    ) {
        let mut plugins = Vec::new();
        for (_, _, responsible_plugins) in delete {
            for responsible_plugin in responsible_plugins.iter() {
                if !plugins.contains(&responsible_plugin) {
                    plugins.push(responsible_plugin);
                }
            }
        }
        self.messages.push(ThresholdMessageRaw {
            ratio,
            threshold,
            log_t,
            id,
            initial_plugin,
            responsible_plugins_str: plugins.into_iter().map(|x| x.as_str()).collect(),
        });
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

pub(crate) enum ThresholdMessageKind {
    Resolved,
    Skipped,
    Warning,
}

type ResponsiblePluginsStr<'a> = Vec<&'a str>;

pub(crate) struct ThresholdMessageRaw<'a> {
    pub(crate) ratio: f64,
    pub(crate) threshold: f64,
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: PluginName<'a>,
    pub(crate) responsible_plugins_str: ResponsiblePluginsStr<'a>,
}

pub(crate) struct DeletedSubrecords<'a> {
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: PluginName<'a>,
    pub(crate) subrecords: Vec<(LlElement, ResponsiblePluginsStr<'a>)>,
}

pub(crate) struct UntouchedList<'a> {
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: &'a str,
    pub(crate) last_plugin: &'a str,
}

type NewLevel = u16;

#[derive(Debug)]
pub(crate) struct DeleveledSubrecords<'a> {
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: PluginName<'a>,
    pub(crate) subrecords: Vec<(LlElement, NewLevel)>,
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
                "{} leveled list{} {} automatically excluded from subrecord deletion mode",
                list.messages.len(),
                plural("s", list.messages.len())?,
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
    append_for_details_or_check_log(text, details, cfg)?;
    msg_and_clear(text, tone, verbose, cfg, log)
}

fn msg_and_clear(text: &mut String, tone: MsgTone, verbose: u8, cfg: &Cfg, log: &mut Log) -> Result<()> {
    msg(&text, tone, verbose, cfg, log)?;
    text.clear();
    Ok(())
}
