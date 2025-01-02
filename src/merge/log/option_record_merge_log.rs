use super::RecordMergeLog;
use crate::{Cfg, RecordMap};
use anyhow::{anyhow, Result};
use paste::paste;
use std::fmt::Arguments;

#[derive(Default)]
pub(crate) struct OptionRecordMergeLog(Option<RecordMergeLog>);

macro_rules! insert_header {
    ($header:ident, $log:ident, $map:ident, $cfg:ident) => {
        paste! {
            if !$log.headers.$header {
                $log.[<$header _header>]($map, $cfg)?;
            }
        }
    };
    ($header:ident, $log:ident, $map:ident, $cfg:ident, $($other_headers:ident),+) => {
        paste! {
            if !$log.headers.$header $(&& !$log.headers.$other_headers)+ {
                $log.[<$header _header>]($map, $cfg)?;
            }
        }
    };
}

macro_rules! make_self {
    ($self:ident, $map:ident) => {
        if $self.0.is_none() {
            $self.0 = Some(RecordMergeLog::new($map));
        }
    };
    ($self:ident, $verboseness:expr, $map:ident, $cfg:ident) => {
        if $self.0.is_none() {
            $self.0 = Some(RecordMergeLog::with_capacity($verboseness, $map, $cfg));
        }
    };
}

impl OptionRecordMergeLog {
    pub(super) fn into_inner(self) -> Option<RecordMergeLog> {
        self.0
    }

    pub(crate) fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub(crate) fn warn<'a, T: RecordMap<'a>>(&mut self, args: Arguments<'_>, map: &'a T, cfg: &Cfg) -> Result<()> {
        make_self!(self, map);
        if let Some(log) = &mut self.0 {
            log.push_warn(format_args!("{tab}Warning: {}", args, tab = cfg.guts.tab_l1))?;
        }
        Ok(())
    }

    pub(crate) fn single_instance<'a, T: RecordMap<'a>>(map: &'a T, cfg: &Cfg) -> Result<OptionRecordMergeLog> {
        Ok(OptionRecordMergeLog(Some(RecordMergeLog::single_instance(map, cfg)?)))
    }

    pub(crate) fn equal_to_the_last_or_clear<'a, T: RecordMap<'a>>(&mut self, map: &'a T, cfg: &Cfg) -> Result<()> {
        if !cfg.meta.debug_equal {
            if self.0.is_some() {
                self.0 = None;
            }
            Ok(())
        } else if let Some(log) = &mut self.0 {
            log.push(
                format_args!(
                    "{tab}Skipped {kind} record: {id:?} {{ Equal to the last instance }}",
                    tab = cfg.guts.tab_l1,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?,
                ),
                cfg.guts.debug_level_merge_skipped_equal_to_the_last,
                cfg,
            )
        } else {
            Err(anyhow!(
                "Bug: OptionRecordMergeLog should not be empty by this moment(record_equal_or_clear)"
            ))
        }
    }

    pub(crate) fn record_fixed<'a, T: RecordMap<'a>>(
        &mut self,
        field: &'static str,
        from: Arguments<'_>,
        to: Arguments<'_>,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        if let Some(log) = &mut self.0 {
            log.push(
                format_args!(
                    "{tab1}Fixed {kind} record: {id:?}:\n{tab2}\"{field}\": {from} -> {to}",
                    tab1 = cfg.guts.tab_l1,
                    tab2 = cfg.guts.tab_l2,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?,
                ),
                cfg.guts.verboseness_details_merge_field_changed,
                cfg,
            )
        } else {
            Err(anyhow!(
                "Bug: OptionRecordMergeLog should not be empty by this moment(record_fixed)"
            ))
        }
    }

    pub(crate) fn all_equal<'a, T: RecordMap<'a>>(&mut self, map: &'a T, cfg: &Cfg) -> Result<()> {
        make_self!(self, map);
        if let Some(log) = &mut self.0 {
            let quantity = map.records_quantity();
            if cfg.meta.debug_plugins && quantity > 2 {
                log.push(
                    format_args!(
                        "{tab}Skipped {kind} record: {id:?} {{ All {quantity} instances are equal in [\"{plugin_names}\"] }}",
                        tab = cfg.guts.tab_l1,
                        kind = map.kind_short_upper(),
                        id = map.record_id_debug()?,
                        plugin_names = map.all_plugin_names().join("\", \""),
                    ),
                    cfg.guts.debug_level_merge_skipped_all_equal,
                    cfg,
                )?;
            } else {
                log.push(
                    format_args!(
                        "{tab}Skipped {kind} record: {id:?} {{ All {quantity} instances are equal in [\"{init}\"{separator}\"{last}\"] }}",
                        tab = cfg.guts.tab_l1,
                        kind = map.kind_short_upper(),
                        id = map.record_id_debug()?,
                        init = map.init_plugin_name()?,
                        separator = if quantity == 2 { ", " } else { "..." },
                        last = map.last_plugin_name()?,
                    ),
                    cfg.guts.debug_level_merge_skipped_all_equal,
                    cfg,
                )?;
            }
        };
        Ok(())
    }

    pub(crate) fn multipatch_attempt<'a, T: RecordMap<'a>>(&mut self, map: &'a T, cfg: &Cfg) -> Result<()> {
        make_self!(self, map);
        if let Some(log) = &mut self.0 {
            log.push(
                format_args!(
                    "{tab}Trying to multipatch {kind} record: {id:?}",
                    tab = cfg.guts.tab_l1,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?,
                ),
                cfg.guts.debug_level_merge_multipatch_attempt,
                cfg,
            )?;
        };
        Ok(())
    }

    pub(crate) fn record_merged<'a, T: RecordMap<'a>>(&mut self, map: &'a T, cfg: &Cfg) -> Result<()> {
        make_self!(self, map);
        if let Some(log) = &mut self.0 {
            log.push(
                format_args!(
                    "{tab}Merged {kind} record: {id:?}",
                    tab = cfg.guts.tab_l1,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?
                ),
                cfg.guts.verboseness_details_merge_record_merged,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn record_multipatched<'a, T: RecordMap<'a>>(
        &mut self,
        multipatched: Option<&'static str>,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        make_self!(self, map);
        if let Some(log) = &mut self.0 {
            let multipatch_kind = match multipatched {
                Some("") => return Err(anyhow!("Bug: multipatched is empty")),
                Some(kind) => kind,
                None => return Err(anyhow!("Bug: multipatched is None")),
            };
            log.push(
                format_args!(
                    "{tab}Multipatched({multipatch_kind}) {kind} record: {id:?}",
                    tab = cfg.guts.tab_l1,
                    kind = map.kind_short_upper(),
                    id = map.record_id_debug()?
                ),
                cfg.guts.verboseness_details_merge_record_multipatched,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_extend<'a, T: RecordMap<'a>>(
        &mut self,
        field: &'static str,
        shorten: bool,
        args: Arguments<'_>,
        plugin_name: &'a str,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        let verboseness = cfg.guts.verboseness_details_merge_field_changed;
        make_self!(self, verboseness, map, cfg);
        if let Some(log) = &mut self.0 {
            insert_header!(merging, log, map, cfg);
            log.push(
                format_args!(
                    "{tab}\"{field}\": {sign} {} [\"{plugin_name}\"]",
                    args,
                    tab = cfg.guts.tab_l2,
                    sign = if shorten { "-" } else { "+" },
                ),
                verboseness,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_changed<'a, T: RecordMap<'a>>(
        &mut self,
        field: &'static str,
        from: Arguments<'_>,
        to: Arguments<'_>,
        plugin_name: &'a str,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        let verboseness = cfg.guts.verboseness_details_merge_field_changed;
        make_self!(self, verboseness, map, cfg);
        if let Some(log) = &mut self.0 {
            insert_header!(merging, log, map, cfg);
            log.push(
                format_args!("{tab}\"{field}\": {from} -> {to} [\"{plugin_name}\"]", tab = cfg.guts.tab_l2),
                verboseness,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_changed_custom<'a, T: RecordMap<'a>>(
        &mut self,
        field: &'static str,
        args: Arguments<'_>,
        plugin_name: &'a str,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        let verboseness = cfg.guts.verboseness_details_merge_field_changed;
        make_self!(self, verboseness, map, cfg);
        if let Some(log) = &mut self.0 {
            insert_header!(merging, log, map, cfg);
            log.push(
                format_args!("{tab}\"{field}\": {args} [\"{plugin_name}\"]", tab = cfg.guts.tab_l2),
                verboseness,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_changed_simple<'a, T: RecordMap<'a>>(
        &mut self,
        text: &str,
        plugin_name: &'a str,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        let verboseness = cfg.guts.verboseness_details_merge_field_changed;
        make_self!(self, verboseness, map, cfg);
        if let Some(log) = &mut self.0 {
            insert_header!(merging, log, map, cfg);
            log.push(
                format_args!("{tab}{text} [\"{plugin_name}\"]", tab = cfg.guts.tab_l2),
                verboseness,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_changed_fog_density<'a, T: RecordMap<'a>>(
        &mut self,
        from: Arguments<'_>,
        to: Arguments<'_>,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        let verboseness = cfg.guts.verboseness_details_merge_field_changed;
        make_self!(self, verboseness, map, cfg);
        if let Some(log) = &mut self.0 {
            if cfg.merge.cell {
                insert_header!(multipatching, log, map, cfg, merging);
            }
            log.push(
                format_args!(
                    "{tab}\"data.grid\": {from} -> {to} (synced) {{ secondary fog density }}",
                    tab = cfg.guts.tab_l2,
                ),
                verboseness,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_changed_redundant(
        &mut self,
        field: &'static str,
        from: Arguments<'_>,
        to: Arguments<'_>,
        cfg: &Cfg,
    ) -> Result<()> {
        if let Some(log) = &mut self.0 {
            log.push(
                format_args!(
                    "{tab}\"{field}\": {from} -> {to} (omitted change) {{ redundant value }}",
                    tab = cfg.guts.tab_l2,
                ),
                cfg.guts.verboseness_details_merge_field_changed,
                cfg,
            )
        } else {
            Err(anyhow!(
                "Bug: OptionRecordMergeLog should not be empty by this moment(field_changed_redundant)"
            ))
        }
    }

    pub(crate) fn field_changed_or_multipatched<'a, T: RecordMap<'a>>(
        &mut self,
        multipatch_kind: &'static str,
        field: &'static str,
        args: Arguments<'_>,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()> {
        let verboseness = cfg.guts.verboseness_details_merge_field_changed;
        make_self!(self, verboseness, map, cfg);
        if let Some(log) = &mut self.0 {
            insert_header!(multipatching, log, map, cfg);
            log.push(
                format_args!(
                    "{tab}\"{field}\": {args}{multipatch_prefix}{multipatch_kind}{multipatch_suffix}",
                    tab = cfg.guts.tab_l2,
                    multipatch_prefix = if multipatch_kind.is_empty() { "" } else { " (multipatched) { " },
                    multipatch_suffix = if multipatch_kind.is_empty() { "" } else { " }" },
                ),
                verboseness,
                cfg,
            )?;
        }
        Ok(())
    }

    pub(crate) fn field_multipatched(
        &mut self,
        multipatch_kind: &'static str,
        field: &'static str,
        args: Arguments<'_>,
        cfg: &Cfg,
    ) -> Result<()> {
        if let Some(log) = &mut self.0 {
            log.push(
                format_args!(
                    "{tab}\"{field}\": {args} (multipatched) {{ {multipatch_kind} }}",
                    tab = cfg.guts.tab_l2,
                ),
                cfg.guts.verboseness_details_merge_field_changed,
                cfg,
            )
        } else {
            Err(anyhow!(
                "Bug: OptionRecordMergeLog should not be empty by this moment(field_multipatched)"
            ))
        }
    }

    pub(crate) fn compare_to_the_last<'a, T: RecordMap<'a>>(
        &mut self,
        field_name: &'static str,
        merged: Arguments<'_>,
        last: Arguments<'_>,
        map: &'a T,
        ignored: bool,
        cfg: &Cfg,
    ) -> Result<()> {
        if let Some(log) = &mut self.0 {
            insert_header!(compare_to_the_last, log, map, cfg);
            log.push(
                format_args!(
                    "{tab}\"{field_name}\": {merged} != {last}{ignored_msg}",
                    tab = cfg.guts.tab_l2,
                    ignored_msg = if ignored { " (ignored)" } else { "" }
                ),
                cfg.guts.debug_level_merge_compare_to_the_last,
                cfg,
            )
        } else {
            Err(anyhow!(
                "Bug: OptionRecordMergeLog should not be empty by this moment(compare_to_the_last)"
            ))
        }
    }

    pub(crate) fn compare_to_the_last_simple<'a, T: RecordMap<'a>>(
        &mut self,
        field_name: &'static str,
        text: &str,
        map: &'a T,
        ignored: bool,
        cfg: &Cfg,
    ) -> Result<()> {
        if let Some(log) = &mut self.0 {
            insert_header!(compare_to_the_last, log, map, cfg);
            log.push(
                format_args!(
                    "{tab}\"{field_name}\": {text}{ignored_msg}",
                    tab = cfg.guts.tab_l2,
                    ignored_msg = if ignored { " (ignored)" } else { "" }
                ),
                cfg.guts.debug_level_merge_compare_to_the_last,
                cfg,
            )
        } else {
            Err(anyhow!(
                "Bug: OptionRecordMergeLog should not be empty by this moment(compare_to_the_last_simple)"
            ))
        }
    }
}
