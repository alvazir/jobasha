use crate::{Cfg, RecordMap};
use anyhow::{Context, Result};
use paste::paste;
use std::fmt::{Arguments, Write as _};

#[derive(Default)]
pub(super) struct RecordMergeLogHeaders {
    pub(super) merging: bool,
    pub(super) multipatching: bool,
    pub(super) compare_to_the_last: bool,
}

#[derive(Default)]
pub(super) struct RecordMergeLog {
    pub(super) init_id: usize,
    pub(super) file: String,
    pub(super) text: String,
    pub(super) warn: String,
    pub(super) warn_count: usize,
    pub(super) headers: RecordMergeLogHeaders,
}

impl RecordMergeLog {
    pub(crate) fn new<'a, T: RecordMap<'a>>(map: &'a T) -> Self {
        Self {
            init_id: map.init_id(),
            ..Default::default()
        }
    }

    pub(crate) fn with_capacity<'a, T: RecordMap<'a>>(verboseness: u8, map: &'a T, cfg: &Cfg) -> Self {
        Self {
            init_id: map.init_id(),
            file: if cfg.no_log {
                String::new()
            } else {
                String::with_capacity(cfg.guts.merge_log_string_allocation_per_record)
            },
            text: if cfg.quiet || verboseness > cfg.verbose {
                String::new()
            } else {
                String::with_capacity(cfg.guts.merge_log_string_allocation_per_record)
            },
            ..Default::default()
        }
    }

    pub(crate) fn single_instance<'a, T: RecordMap<'a>>(map: &'a T, cfg: &Cfg) -> Result<Self> {
        let text = format!(
            "{tab}Skipped {kind} record: {id:?} {{ The only instance is in [\"{init}\"] }}\n",
            tab = cfg.guts.tab_l1,
            kind = map.kind_short_upper(),
            id = map.record_id_debug()?,
            init = map.init_plugin_name()?
        );
        Ok(Self {
            init_id: map.init_id(),
            text: if cfg.quiet || cfg.guts.debug_level_merge_skipped_single > cfg.verbose {
                String::new()
            } else {
                text.clone()
            },
            file: if cfg.no_log { String::new() } else { text },
            ..Default::default()
        })
    }

    pub(super) fn push(&mut self, args: Arguments<'_>, verboseness: u8, cfg: &Cfg) -> Result<()> {
        if !cfg.no_log {
            writeln!(self.file, "{}", args).with_context(|| "Failed to write to file log buffer")?;
        }
        if !(cfg.quiet || verboseness > cfg.verbose) {
            writeln!(self.text, "{}", args).with_context(|| "Failed to write to text log buffer")?;
        }
        Ok(())
    }

    pub(super) fn push_warn(&mut self, args: Arguments<'_>) -> Result<()> {
        self.warn_count += 1;
        writeln!(self.warn, "{}", args).with_context(|| "Failed to write to warn log buffer")
    }

    pub(super) fn compare_to_the_last_header<'a, T: RecordMap<'a>>(&mut self, map: &'a T, cfg: &Cfg) -> Result<()> {
        self.push(
            format_args!(
                "{tab}Comparing to the last instance of {kind} record: {id:?} [\"{plugin_name}\"]",
                tab = cfg.guts.tab_l1,
                kind = map.kind_short_upper(),
                id = map.record_id_debug()?,
                plugin_name = map.last_plugin_name()?
            ),
            cfg.guts.debug_level_merge_compare_to_the_last,
            cfg,
        )?;
        self.headers.compare_to_the_last = true;
        Ok(())
    }
}

macro_rules! merging_header {
    ($($low:ident, $camel:literal),+) => {
        paste! {
            $(
impl RecordMergeLog {
    pub(super) fn [<$low _header>]<'a, T: RecordMap<'a>>(&mut self, map: &'a T, cfg: &Cfg) -> Result<()> {
        let quantity = map.records_quantity();
        if cfg.meta.debug_plugins && quantity > 2 {
            self.push(
                format_args!(
                    "{tab}{merging} {kind} record: {id:?} [\"{plugin_names}\"]",
                    tab = cfg.guts.tab_l1,
                    merging = $camel,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?,
                    plugin_names = map.all_plugin_names().join("\", \""),
                ),
                cfg.guts.verboseness_details_merge_field_changed,
                cfg,
            )?;
        } else if quantity == 1 {
            self.push(
                format_args!(
                    "{tab}{merging} {kind} record: {id:?} [\"{init}\"]",
                    tab = cfg.guts.tab_l1,
                    merging = $camel,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?,
                    init = map.init_plugin_name()?,
                ),
                cfg.guts.verboseness_details_merge_field_changed,
                cfg,
            )?;
        } else {
            self.push(
                format_args!(
                    "{tab}{merging} {kind} record: {id:?} [\"{init}\"{separator}\"{last}\"]",
                    tab = cfg.guts.tab_l1,
                    merging = $camel,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?,
                    init = map.init_plugin_name()?,
                    separator = if quantity == 2 { ", " } else { "..." },
                    last = map.last_plugin_name()?,
                ),
                cfg.guts.verboseness_details_merge_field_changed,
                cfg,
            )?;
        }
        self.headers.$low = true;
        Ok(())
    }
}
            )+
        }
    }
}

merging_header!(merging, "Merging", multipatching, "Multipatching");
