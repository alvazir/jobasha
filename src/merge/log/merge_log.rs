use super::OptionRecordMergeLog;
use crate::{append_for_details_or_check_log, msg, msg_thread_safe, plural, Cfg, Log, MsgTone};
use anyhow::Result;

#[derive(Default)]
pub(crate) struct MergeLog {
    file: String,
    text: String,
    warn: String,
    merged_count: usize,
    warn_count: usize,
}

impl MergeLog {
    pub(crate) fn push(&mut self, option_record_merge_logs: Vec<OptionRecordMergeLog>, merged_count: usize, cfg: &Cfg) {
        let capacity = cfg.guts.merge_log_string_allocation;
        macro_rules! push_kind {
            ($log:ident, $kind:ident, $reserve:expr) => {
                if !$log.$kind.is_empty() {
                    if $reserve && self.$kind.capacity() < capacity {
                        self.$kind.reserve(capacity);
                    }
                    self.$kind.push_str(&$log.$kind);
                }
            };
        }
        let mut record_merge_logs = option_record_merge_logs
            .into_iter()
            .filter_map(|option_log| option_log.into_inner())
            .collect::<Vec<_>>();
        record_merge_logs.sort_by_key(|element| element.init_id);
        for log in record_merge_logs {
            push_kind!(log, warn, false);
            push_kind!(log, file, true);
            push_kind!(log, text, true);
            self.warn_count += log.warn_count;
        }
        self.merged_count += merged_count;
    }

    pub(crate) fn msg(&self, cfg: &Cfg, log: &mut Log) -> Result<()> {
        if !self.warn.is_empty() {
            let mut text = format!(
                "{} warning{} {} generated while merging records",
                self.warn_count,
                plural("s", self.warn_count)?,
                plural("were", self.warn_count)?
            );
            append_for_details_or_check_log(&mut text, cfg.guts.verboseness_details_merge_warnings, cfg)?;
            msg(text, MsgTone::Ugly, 0, cfg, log)?;
            msg("", MsgTone::Neutral, cfg.guts.verboseness_details_merge_warnings, cfg, log)?;
            msg(&self.warn, MsgTone::Neutral, cfg.guts.verboseness_details_merge_warnings, cfg, log)?;
        }
        if self.merged_count == 0 && !cfg.merge.skip {
            msg("Nothing to merge", MsgTone::Neutral, 0, cfg, log)?;
        }
        if self.merged_count > 0 || (cfg.debug > 0 && (!self.file.is_empty() || !self.text.is_empty())) {
            let mut text = format!(
                "{} record{} {} merged",
                self.merged_count,
                plural("s", self.merged_count)?,
                plural("were", self.merged_count)?
            );
            append_for_details_or_check_log(&mut text, cfg.guts.verboseness_details_merge_record_merged, cfg)?;
            msg(text, MsgTone::Good, 0, cfg, log)?;
            if !self.file.is_empty() {
                msg("", MsgTone::Neutral, u8::MAX, cfg, log)?;
                msg(&self.file, MsgTone::Neutral, u8::MAX, cfg, log)?;
            }
            if !self.text.is_empty() {
                msg_thread_safe("", MsgTone::Neutral, 0, cfg)?;
                msg_thread_safe(&self.text, MsgTone::Neutral, 0, cfg)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl MergeLog {
    pub(crate) fn test_warn(&self) -> &str {
        &self.warn
    }

    pub(crate) fn test_file(&self) -> &str {
        &self.file
    }

    pub(crate) fn test_text(&self) -> &str {
        &self.text
    }
}
