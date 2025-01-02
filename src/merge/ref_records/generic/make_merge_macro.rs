macro_rules! generic_make_merge {(
    $short:ident,
    ($ref_record:ident, $long:ident, $id:ident),
    ($($field:ident$(:$subfield:ident$(:$tuple_index:tt)?$(;$array_index:tt)?)?$(.$prefix:tt)?
       // COMMENT: print Option values as strings, e.g. None = "", Some(x) = x
       $(::print_as::$print_as:ident)?
       $(::print_compact::$print_compact:ident)?
       $(::debug_level_to_log::$debug_level_to_log:ident)?
       $(::revert_to_none::$revert_to_none:literal)?
       $(=$flags_type:ty)?
       ),+),
    ($($vec_field:ident),*),
    ($($ai_packages:ident)?),
    ($($npc_process_data_stats:ident, $npc_compare_data_stats:ident)?),
    ($($specific:ident)?)
) => { paste! {
pub(crate) fn [<merge_ $short>](intermediate_records: &IntermediateRecords, raw_plugin: &mut RawPlugin, cfg: &Cfg, merge_log: &mut MergeLog) -> Result<()> {
    let multipatch_equal_records = (stringify!($short) == "cell" && cfg.multipatch.fogbug)
        || (stringify!($short) == "crea" && cfg.multipatch.summons);
    let multipatch_or_debug_single = multipatch_equal_records || cfg.meta.debug_single;
    let (indexed_records, logs): (Vec<Option<(usize, $long)>>, Vec<OptionRecordMergeLog>) = intermediate_records
        .$short
        .par_iter()
        .map(|(_id_low, map)| -> Result<Option<(Option<(usize, $long)>, OptionRecordMergeLog)>> {
            macro_rules! multipatch_equal_records {
                ($option_log:ident) => {
                    $(
            if multipatch_equal_records {
                if cfg.meta.debug_multipatch_attempt {
                    $option_log.multipatch_attempt(&map, cfg)?;
                }
                let last = &map.last_record()?.$short;
                if [<$specific _multipatch_check>](_id_low, last, cfg) {
                    let mut merged_record = last.clone();
                    let id = &map.record_id_debug()?;
                    macro_rules! err_context { ($head:expr) => {
                        format!(
                            "Bug: failed to {head} {kind} record: {id:?}",
                            head = $head,
                            kind = stringify!([<$short:upper>])
                        )
                    }; }
                    let mut specific_flags = SpecificFlags::default();
                    [<$specific _multipatch>](&mut merged_record,&map, &mut $option_log, &mut specific_flags, cfg)
                        .with_context(|| err_context!(
                                format!("succeed {} while multipatching", stringify!($specific))
                        ))?;
                    let option_record_with_init_id_merged = Some((map.init_id, merged_record));
                    if !cfg.meta.silent {
                        if cfg.meta.debug_compare {
                            if let Some((_, ref merged)) = option_record_with_init_id_merged {
                                [<compare_to_the_last_ $short>](merged, last, &map, &mut $option_log, false, cfg)
                                    .with_context(|| err_context!("compare to the last while multipatching"))?;
                            };
                        }
                        $option_log.record_multipatched(specific_flags.multipatched, &map, cfg)?;
                    }
                    return Ok(Some((option_record_with_init_id_merged, $option_log)));
                }
            }
                    )?
                }
            }
            // COMMENT: merging only makes sense with 2 or more records
            if map.records.len() > 1 {
                let mut option_log: OptionRecordMergeLog = OptionRecordMergeLog::default();
                let mut option_record_with_init_id_merged: Option<(usize, $long)> = None;
                let mut option_ref_merged: Option<$ref_record> = None;

                // COMMENT: [MERGE START]
                $( // COMMENT: most fields(and subfields) are processed here
                    // COMMENT: independent flags require tracking changes
                    macro_rules! make_independent_flags_variables {
                        ($changed:ident) => {};
                        ($changed:ident, $flags:ty) => {
                            let mut $changed = <$flags>::default();
                        }
                    }
                    make_independent_flags_variables!(changed_flags$(, $flags_type)?);
                    for (record, next_index) in map.records.iter().skip(1).zip(1usize..) {
                        // COMMENT: if new field value is distinct from previous...
                        macro_rules! continue_if_none {
                            () => {};
                            (false) => {
                                if record.$short.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?.is_none() {
                                    continue;
                                }
                            }
                        }
                        // COMMENT: prevent reverting to none for some fields, e.g. WHGT
                        continue_if_none!($($revert_to_none)?);
                        if !map.records[..next_index].iter().any(|prev_record|
                            prev_record.$short.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?
                            == record.$short.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?)
                        {
                            if option_ref_merged.is_none() {
                                option_ref_merged = Some($ref_record::new(&map.records[0].$short));
                            }
                            if let Some(ref mut merged) = option_ref_merged {
                                let new = $($prefix)?record.$short.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?;
                                // COMMENT: independent flags diverge from the usual flow
                                macro_rules! merge_independent_flags {
                                    () => {};
                                    ($show_flags:ident) => {
                                        if !cfg.merge.interdependent_flags {
                                            let merged_flags =
                                                &mut merged.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?;
                                            macro_rules! change_flag {
                                                ($flag:expr, insert) => {
                                                    change_flag!($flag, insert, false)
                                                };
                                                ($flag:expr, remove) => {
                                                    change_flag!($flag, remove, true)
                                                };
                                                ($flag:expr, $method:ident, $shorten:literal) => {
                                                    if !changed_flags.contains($flag) {
                                                        changed_flags.insert($flag);
                                                        merged_flags.$method($flag);
                                                        if !cfg.meta.silent {
                                                            option_log.field_extend(
                                                                stringify!($field$(.$subfield$(.$tuple_index)?$([$array_index])?)?),
                                                                $shorten,
                                                                format_args!("{}", $show_flags($flag)),
                                                                &record.plugin_info.name,
                                                                &map,
                                                                cfg
                                                            )?;
                                                        }
                                                    }
                                                }
                                            }
                                            if cfg.merge.plus_before_minus {
                                                for flag in new.difference(*merged_flags).iter() {
                                                    change_flag!(flag, insert);
                                                }
                                            }
                                            for flag in merged_flags.difference(new).iter() {
                                                change_flag!(flag, remove);
                                            }
                                            if !cfg.merge.plus_before_minus {
                                                for flag in new.difference(*merged_flags).iter() {
                                                    change_flag!(flag, insert);
                                                }
                                            }
                                            continue;
                                        }
                                    }
                                }
                                merge_independent_flags!($([<show_ $flags_type:snake>])?);
                                if !cfg.meta.silent {
                                    macro_rules! print_normal {
                                        ($field_name:expr, $merged:expr) => {
                                            option_log.field_changed(
                                                $field_name,
                                                format_args!("{:?}", print_as!($($print_as:)?$merged)),
                                                format_args!("{:?}", print_as!($($print_as:)?new)),
                                                &record.plugin_info().name,
                                                &map,
                                                cfg
                                            )?;
                                        }
                                    }
                                    #[allow(unused_macros)]
                                    macro_rules! print_compact {
                                        ($field_name:expr, $merged:expr) => {
                                            [<show_compact_ $field$(_$subfield)?>](
                                                &mut option_log,
                                                $field_name,
                                                $merged,
                                                new,
                                                &record.plugin_info().name,
                                                &map,
                                                cfg
                                            )?;
                                        }
                                    }
                                    macro_rules! print_selector {
                                        ($field_name:expr, $merged:expr) => {
                                            print_normal!($field_name, $merged);
                                        };
                                        ($field_name:expr, $merged:expr, ::print_compact::$no_compact:ident, ::debug_level_to_log::$debug_level:ident) => {
                                            if cfg.debug >= cfg.guts.$debug_level {
                                                print_normal!($field_name, $merged);
                                            } else if !cfg.merge.$no_compact {
                                                print_compact!($field_name, $merged);
                                            }
                                        };
                                        ($field_name:expr, $merged:expr, ::print_compact::$no_compact:ident) => {
                                            if cfg.merge.$no_compact {
                                                print_normal!($field_name, $merged);
                                            } else {
                                                print_compact!($field_name, $merged);
                                            }
                                        }
                                    }
                                    print_selector!(
                                        stringify!($field$(.$subfield$(.$tuple_index)?$([$array_index])?)?),
                                        merged.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?
                                        $(, ::print_compact::$print_compact)?
                                        $(, ::debug_level_to_log::$debug_level_to_log)?
                                    );
                                }
                                // COMMENT: ... then it's the new merged value
                                merged.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)? = new;
                            }
                        };
                    };
                )+
                $( // COMMENT: npc_.data.stats are too different from other fields
                    $npc_process_data_stats!(option_ref_merged, map, option_log, cfg, health, magicka, fatigue);
                )?
                #[allow(unused_macros)] // COMMENT: common selector for next 2 field kinds
                macro_rules! process_helper_common {
                    ($record:ident, $next_index:ident, $common_field:ident) => {
                        // COMMENT: same approach - merge only if new field value is distinct from previous...
                        if map.records[..$next_index]
                            .iter()
                            .any(|prev_record| $record.$short.$common_field == prev_record.$short.$common_field)
                        {
                            continue;
                        } else if option_ref_merged.is_none() {
                            let merged = $ref_record::new(&map.records[0].$short);
                            option_ref_merged = Some(merged);
                        }
                    };
                }
                #[allow(unused_macros)]
                macro_rules! merge_context {
                    ($err_field:ident, $record:ident) => {
                        format!(
                            "Failed to merge \"{field}\" field for {kind} record: {id:?}",
                            field = stringify!($err_field),
                            kind = stringify!([<$short:upper>]),
                            id = $record.$short.id,
                        )
                    };
                }
                $( // COMMENT: process vector fields: invetory, spells, travel_destinations
                    let mut $vec_field = [<$vec_field:camel Helper>]::default();
                    for (record, next_index) in map.records.iter().skip(1).zip(1usize..) {
                        process_helper_common!(record, next_index, $vec_field);
                        $vec_field.[<make_ $short>](&map, next_index, &mut option_log, cfg)
                            .with_context(|| merge_context!($vec_field, record))?;
                    }
                )*
                $( // COMMENT: process ai_packages
                    let mut $ai_packages = AiPackagesHelper::default();
                    for (record, next_index) in map.records.iter().skip(1).zip(1usize..) {
                        process_helper_common!(record, next_index, $ai_packages);
                        $ai_packages.process(
                            &map.records[0].$short.$ai_packages,
                            &record.$short.$ai_packages,
                            record.plugin_info(),
                            &mut option_log,
                            &map,
                            cfg
                        ).with_context(|| merge_context!($ai_packages, record))?;
                    }
                )?
                // COMMENT: [MERGE FINISH]

                // COMMENT: if something's merged
                if let Some(mut merged) = option_ref_merged {
                    let id = &map.record_id_debug()?;
                    macro_rules! err_context { ($head:expr) => {
                        format!(
                            "Bug: failed to {head} {kind} record: {id:?}",
                            head = $head,
                            kind = stringify!([<$short:upper>])
                        )
                    }; }
                    let last = &map.last_record()
                        .with_context(|| err_context!("succeed map.last_record() while merging"))?
                        .$short;
                    $( // COMMENT: cook vector and ai_packages fields from half-baked state
                        merged.$vec_field = $vec_field.commit(&merged.base.$vec_field)
                            .with_context(|| err_context!(
                                    format!("succeed {}.commit while merging", stringify!($vec_field))
                            ))?;
                    )*
                    $(merged.$ai_packages = $ai_packages.commit(&merged.base.$ai_packages);)?
                    if merged.equal(last$(, &$vec_field)*) // COMMENT: ai_packages are implicitly included
                        .with_context(|| err_context!("check for equality between merged and last"))?
                    { // COMMENT: merged is equal to the last, drop log too if not debug
                        if !cfg.meta.silent {
                            option_log.equal_to_the_last_or_clear(&map, cfg)
                                .with_context(|| err_context!(
                                        "succeed equal_to_the_last_or_clear() after merged.equal() == true"
                                ))?;
                        }
                        multipatch_equal_records!(option_log);
                    } else {
                        #[allow(unused_mut)]
                        // COMMENT: turn whole record from references into the owned
                        let mut merged_record = merged.into_owned();
                        #[allow(unused_mut, unused_assignments)]
                        let mut specific_flags = SpecificFlags::default();
                        $(
                            $specific(_id_low, &mut merged_record, last, &map, &mut option_log, &mut specific_flags, cfg)
                                .with_context(|| err_context!(
                                        format!("succeed {} while merging", stringify!($specific))
                                ))?;
                        )?
                        if specific_flags.equal_after_specific {
                            // COMMENT: equal to the last after specific checks, drop log too if not debug
                            if !cfg.meta.silent {
                                if cfg.meta.debug_compare_equal {
                                    let changes = [<compare_to_the_last_ $short>](
                                        &merged_record,
                                        last,
                                        &map,
                                        &mut option_log,
                                        true,
                                        cfg
                                    ).with_context(|| err_context!("compare to the last after specific()"))?;
                                    if changes != specific_flags.changes {
                                        option_log.warn(
                                            format_args!(
                                                "Check {kind} record: {id:?}: fields ignored on comparison = {changes} >= expected = {expected} {{ should be merged possibly }}",
                                                kind = stringify!([<$short:upper>]),
                                                expected = specific_flags.changes,
                                            ),
                                            &map,
                                            cfg,
                                        )?;
                                    }
                                }
                                option_log.equal_to_the_last_or_clear(&map, cfg)
                                    .with_context(|| err_context!(
                                            "succeed equal_to_the_last_or_clear() after specific()"
                                    ))?;
                            }
                        } else {
                            // COMMENT: finally merged record is done
                            option_record_with_init_id_merged = Some((map.init_id, merged_record));
                            if !cfg.meta.silent {
                                // COMMENT: debug log showing difference between merged and last records
                                if cfg.meta.debug_compare {
                                    if let Some((_, ref merged)) = option_record_with_init_id_merged {
                                        [<compare_to_the_last_ $short>](merged, last, &map, &mut option_log, false, cfg)
                                            .with_context(|| err_context!("compare to the last"))?;
                                    };
                                }
                                if specific_flags.multipatched.is_some() {
                                    option_log.record_multipatched(specific_flags.multipatched, &map, cfg)?;
                                } else {
                                    option_log.record_merged(&map, cfg)?;
                                }
                            }
                        }
                    }
                // COMMENT: if nothing was merged
                } else {
                    if cfg.meta.debug_all {
                        option_log.all_equal(&map, cfg)?;
                    }
                    multipatch_equal_records!(option_log);
                }

                // COMMENT: if there is any result
                if option_record_with_init_id_merged.is_some() || option_log.is_some() {
                    return Ok(Some((option_record_with_init_id_merged, option_log)));
                }
            // COMMENT: there is nothing to merge with 1 record
            } else {
                if multipatch_or_debug_single {
                    #[allow(unused_mut)]
                    let mut option_log: OptionRecordMergeLog = if cfg.meta.debug_single {
                        OptionRecordMergeLog::single_instance(&map, cfg)?
                    } else {
                        OptionRecordMergeLog::default()
                    };
                    multipatch_equal_records!(option_log);
                    if option_log.is_some() {
                        return Ok(Some((None, option_log)));
                    }
                }
            }
            // COMMENT: if there is no result at all
            Ok(None)
        })
        .filter_map(|result_option| identity(result_option.transpose()))
        .collect::<Result<(Vec<Option<(usize, $long)>>, Vec<OptionRecordMergeLog>)>>()?;

    let mut sorted_records: Vec<(usize, $long)> = indexed_records.into_iter().filter_map(identity).collect();
    merge_log.push(logs, sorted_records.len(), cfg);
    if !sorted_records.is_empty() {
        sorted_records.sort_by_key(|element| element.0);
        raw_plugin.plugin.objects.extend(sorted_records.into_iter().map(|(_, record)| TES3Object::$long(record)));
    };
    Ok(())
}

pub(crate) fn [<compare_to_the_last_ $short>]<'a>(
    merged: &'a $long,
    last: &'a $long,
    map: &'a crate::input::merge::[<$long RecordMap>],
    option_log: &mut OptionRecordMergeLog,
    ignored: bool,
    cfg: &Cfg,
) -> Result<usize> {
    macro_rules! log_condition {
        ($merged:expr, $last:expr) => {
            $merged != $last
        };
        ($merged:expr, $last:expr, ::print_compact::$no_compact:ident) => {
            $merged != $last
        };
        ($merged:expr, $last:expr, ::print_compact::$no_compact:ident, ::debug_level_to_log::$debug_level:ident) => {
            log_condition!($merged, $last) && (cfg.debug >= cfg.guts.$debug_level || !cfg.merge.$no_compact)
        }
    }
    // COMMENT: changes count is only used for "simple" fields for now
    let mut changes = 0;
    $(if log_condition!(
            merged.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?,
            last.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?
            $(, ::print_compact::$print_compact)?
            $(, ::debug_level_to_log::$debug_level_to_log)?
            ) {
        changes += 1;
        option_log.compare_to_the_last(
            stringify!($field$(.$subfield$(.$tuple_index)?$([$array_index])?)?),
            format_args!("{:?}", merged.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?),
            format_args!("{:?}", last.$field$(.$subfield$(.$tuple_index)?$([$array_index])?)?),
            &map,
            ignored,
            cfg
        )?;
    })+
    $($npc_compare_data_stats!(merged, last, option_log, map, ignored, cfg, health, magicka, fatigue);)?
    #[allow(unused_macros)]
    macro_rules! compare_vec_field_to_the_last {
        ($vector_field:ident) => {
            if merged.$vector_field.len() != last.$vector_field.len() {
                changes += 1;
                option_log.compare_to_the_last(
                    stringify!($vector_field),
                    format_args!("quantity {}", merged.$vector_field.len()),
                    format_args!("{}", last.$vector_field.len()),
                    &map,
                    ignored,
                    cfg
                )?;
            } else if merged.$vector_field != last.$vector_field {
                changes += 1;
                option_log.compare_to_the_last_simple(
                    stringify!($vector_field), "different despite equal quantity", &map, ignored, cfg
                )?;
            }
        }
    }
    $(compare_vec_field_to_the_last!($vec_field);)*
    $(
        if merged.$ai_packages.len() != last.$ai_packages.len() {
            changes += 1;
            option_log.compare_to_the_last(
                "ai_packages",
                format_args!("quantity {}", merged.$ai_packages.len()),
                format_args!("{}", last.$ai_packages.len()),
                &map,
                ignored,
                cfg
            )?;
        } else if !merged.$ai_packages.is_empty() {
            if merged.$ai_packages.len() == 1 {
                if merged.$ai_packages != last.$ai_packages {
                    let merged0 = &merged.$ai_packages[0];
                    let last0 = &last.$ai_packages[0];
                    if discriminant(merged0) != discriminant(last0) {
                        changes += 1;
                        option_log.compare_to_the_last(
                            "ai_packages",
                            format_args!("type {:?}", ai_package_variant(merged0)),
                            format_args!("{:?}", ai_package_variant(last0)),
                            &map,
                            ignored,
                            cfg
                        )?;
                    } else {
                        changes += 1;
                        option_log.compare_to_the_last_simple(
                            "ai_packages", "different despite the same type", &map, ignored, cfg
                        )?;
                    }
                }
            } else if merged.$ai_packages != last.$ai_packages {
                changes += 1;
                option_log.compare_to_the_last_simple(
                    "ai_packages", "different despite equal quantity", &map, ignored, cfg
                )?;
            }
        }
    )?
    Ok(changes)
}}}}

pub(crate) use generic_make_merge;
