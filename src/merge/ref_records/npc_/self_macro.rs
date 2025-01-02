macro_rules! merge_data_stats_fields_inner_loop {
    ($merged:ident, $stats:ident, $record:ident, $subsubfield:ident$(:$id:ident)?) => {
        if let Some(ref mut merged_stats) = $merged.data.stats {
            merged_stats.$subsubfield$([$id])? = $stats.$subsubfield$([$id])?;
        } else {
            $merged.data.stats = $record.npc_.data.stats.clone();
        }
    }
}

macro_rules! merge_data_stats_fields {
    ($option_merged:ident, $map:ident, $option_log:ident, $cfg:ident, $subsubfield:ident$(:$id:ident)?) => {
        paste! {
        for (record, next_index) in $map.records.iter().skip(1).zip(1usize..) {
            if let Some(ref stats) = record.npc_.data.stats {
                if !$map.records[..next_index].iter().any(|prev_record|
                    if let Some(ref prev_stats) = prev_record.npc_.data.stats {
                        if stats.$subsubfield$([$id])? == prev_stats.$subsubfield$([$id])? {
                            true
                        } else { false }
                    } else { false }
                    ) {
                        if $option_merged.is_none() {
                            $option_merged = Some(Npc_Ref::new(&$map.records[0].npc_));
                        }
                        if let Some(ref mut merged) = $option_merged {
                            let empty = "";
                            let last = stats.$subsubfield$([$id])?;
                            macro_rules! log_field_changed {
                                ($old:ident) => {
                                    if !$cfg.meta.silent {
                                        $option_log.field_changed([<$subsubfield:upper>]$([$id])?, format_args!("{:?}", $old), format_args!("{last:?}"), &record.plugin_info().name, &$map, $cfg)?;
                                    }
                                }
                            }
                            if let Some(merged_stats) = &merged.data.stats {
                                let merged = merged_stats.$subsubfield$([$id])?;
                                if merged == last {
                                    // COMMENT: cloned when processing previous field
                                    log_field_changed!(empty);
                                } else {
                                    log_field_changed!(merged);
                                }
                            } else {
                                    log_field_changed!(empty);
                            }
                            merge_data_stats_fields_inner_loop!(merged, stats, record, $subsubfield$(:$id)?);
                        }
                    }
                }
            }
        }
    }
}

macro_rules! npc_process_data_stats {
    ($option_merged:ident, $map:ident, $option_log:ident, $cfg:ident, $($subsubfield:ident),+) => {
        let mut record_changed_indexes: Vec<usize> = Vec::new();
        let mut last_stats = &$map.records[0].npc_.data.stats;
        let mut last_plugin = &$map.records[0].plugin_info;
        if last_stats.is_some() {
            record_changed_indexes.push(0);
        }
        for (record, next_index) in $map.records.iter().skip(1).zip(1usize..) {
            if !$map.records[..next_index].iter().any(|prev_record| prev_record.npc_.data.stats == record.npc_.data.stats) {
                last_stats = &record.npc_.data.stats;
                last_plugin = &record.plugin_info;
                if last_stats.is_some() && !record_changed_indexes.contains(&(next_index)) {
                    record_changed_indexes.push(next_index);
                }
            };
        }
        if last_stats.is_none() {
            if let Some(ref mut merged) = $option_merged {
                if merged.data.stats.is_some() {
                    merged.data.stats = None;
                    if !$cfg.meta.silent {
                        $option_log.field_changed_simple("\"data.stats\": \"Manual\" -> \"Auto\"", &last_plugin.name, &$map, $cfg)?;
                    }
                }
            } else if $map.records[0].npc_.data.stats.is_some() {
                let mut merged = Npc_Ref::new(&$map.records[0].npc_);
                merged.data.stats = None;
                if !$cfg.meta.silent {
                    let npc_flags = merged.npc_flags;
                    $option_log.warn(
                        format_args!(
                            "Invalid NPC_ record: {id:?}: \"npc_flags\": {npc_flags:?} [\"{last_plugin_name}\"] {{ AUTO_CALCULATE flag is missing despite \"data.stats\" not set }}",
                            id = merged.id,
                            last_plugin_name = &last_plugin.name
                        ),
                        &$map,
                        $cfg,
                    )?;
                    $option_log.field_changed_simple("\"data.stats\": \"Manual\" -> \"Auto\"", &last_plugin.name, &$map, $cfg)?;
                }
                $option_merged = Some(merged);
            }
        } else if record_changed_indexes.len() == 1 {
            let npc_init = &$map.record(0)?.npc_;
            if npc_init.data.stats.is_none() {
                if $option_merged.is_none() {
                    $option_merged = Some(Npc_Ref::new(npc_init));
                }
                if let Some(ref mut merged) = $option_merged {
                    merged.data.stats = $map.records[record_changed_indexes[0]].npc_.data.stats.clone();
                    $option_log.field_changed_simple("\"data.stats\": \"Auto\" -> \"Manual\"", &last_plugin.name, &$map, $cfg)?;
                }
            }
        } else {
            $(merge_data_stats_fields!($option_merged, $map, $option_log, $cfg, $subsubfield);)+
            for id in 0..8 {
                merge_data_stats_fields!($option_merged, $map, $option_log, $cfg, attributes:id);
            }
            for id in 0..27 {
                merge_data_stats_fields!($option_merged, $map, $option_log, $cfg, skills:id);
            }
        }
    };
}

macro_rules! npc_compare_data_stats {
    ($merged:ident, $last:ident, $option_log:ident, $map:ident, $ignored:expr, $cfg:ident, $($field:ident),+) => {
        if let Some(merged) = &$merged.data.stats {
            if let Some(last) = &$last.data.stats {
                macro_rules! iterate_over_kind{
                    ($kind:ident, $limit:expr) => {
                        paste! {
                            for id in 0..$limit {
                                if merged.$kind[id] != last.$kind[id] {
                                    $option_log.compare_to_the_last(
                                        [<$kind:upper>][id],
                                        format_args!("{:?}", merged.$kind[id]),
                                        format_args!("{:?}", last.$kind[id]),
                                        &$map,
                                        $ignored,
                                        $cfg
                                    )?;
                                }
                            }
                        }
                    }
                }
                $(if merged.$field != last.$field {
                    paste! {
                        $option_log.compare_to_the_last(
                            [<$field:upper>],
                            format_args!("{:?}", merged.$field),
                            format_args!("{:?}", last.$field),
                            &$map,
                            $ignored,
                            $cfg
                        )?;
                    }
                })+
                iterate_over_kind!(attributes, 8);
                iterate_over_kind!(skills, 27);
            } else {
                $option_log.compare_to_the_last("data.stats", format_args!("\"Manual\""), format_args!("\"Auto\""), &$map, $ignored, $cfg)?;
            }
        } else {
            if $last.data.stats.is_some() {
                $option_log.compare_to_the_last("data.stats", format_args!("\"Auto\""), format_args!("\"Manual\""), &$map, $ignored, $cfg)?;
            }
        }
    };
}

pub(super) use {merge_data_stats_fields, merge_data_stats_fields_inner_loop, npc_compare_data_stats, npc_process_data_stats};
