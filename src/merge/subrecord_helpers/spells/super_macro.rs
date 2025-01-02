macro_rules! make {
    ($field:ident, $($short:ident$(::add_second_deleted_index::$add_second_deleted_index:literal)?, $long:ident),+) => {
        $(paste! {
            pub(crate) fn [<make_ $short>](&mut self, map: &'a [<$long RecordMap>], next_index: usize, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<()> {
                if !self.generated {
                    for (record_index, record) in map.records.iter().enumerate() {
                        let mut lowercased_element = Vec::with_capacity(record.$short.$field.len());
                        for (spell_index, spell) in record.$short.$field.iter().enumerate() {
                            let spell_lowercased = to_lowercase(spell);
                            if lowercased_element.contains(&spell_lowercased) {
                                if record_index == 0 {
                                    // COMMENT: second index helps with travel_destinations and different order of plus and minus
                                    macro_rules! add_second_deleted_index {
                                        () => {
                                            self.deleted.push((spell_index, record.plugin_info()));
                                        };
                                        (true) => {
                                            self.deleted.push((spell_index, record.plugin_info(), next_index));
                                        }
                                    }
                                    add_second_deleted_index!($($add_second_deleted_index)?);
                                };
                                if !cfg.meta.silent {
                                    option_log.warn(
                                        format_args!(
                                            "Fixed {kind} record: {id:?}: \"{field}\": - {spell:?} (ignored) {{ duplicate list element defined in [\"{plugin_name}\"] }}",
                                            kind = stringify!($short).to_uppercase(),
                                            id = &map.record(0).unwrap().$short.id,
                                            field = stringify!($field),
                                            plugin_name = &map.record(record_index).unwrap().plugin_info.name
                                        ),
                                        &map,
                                        cfg,
                                    )?;
                                    option_log.field_extend("spells", true,
                                        format_args!("{spell:?} (ignored)"),
                                        &map.record(record_index).unwrap().plugin_info.name,
                                        &map, cfg
                                    )?;
                                }
                            }
                            lowercased_element.push(spell_lowercased);
                        }
                        self.lowercased.push(lowercased_element);
                    }
                    self.generated = true;
                }
                if cfg.merge.plus_before_minus {
                    self.[<add_ $short>](map, next_index, option_log, cfg)?;
                };
                self.[<delete_ $short>](map, next_index, option_log, cfg)?;
                if !cfg.merge.plus_before_minus {
                    self.[<add_ $short>](map, next_index, option_log, cfg)?;
                }
                Ok(())
            }
        })+
    };
}

pub(crate) use make;
