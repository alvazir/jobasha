macro_rules! make {
    ($($short:ident, $long:ident),+) => {
        $(paste! {
            pub(crate) fn [<make_ $short>](&mut self, map: &'a [<$long RecordMap>], next_index: usize, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<()> {
                if !self.generated {
                    self.lowercased = map.records.iter().map(|record| record.$short.inventory.iter().map(|element| to_lowercase(element)).collect::<Vec<InventoryRecordLow>>()).collect::<Vec<Vec<InventoryRecordLow>>>();
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
    }
}

macro_rules! process {
    ($($short:ident, $long:ident),+) => {
        $(paste! {
            fn [<delete_ $short>](
                &mut self,
                map: &'a [<$long RecordMap>],
                next_index: usize,
                option_log: &mut OptionRecordMergeLog,
                cfg: &Cfg
            ) -> Result<()> {
                for (subindex, subrecord) in self.lowercased[0].iter().enumerate() {
                    let first_count = self.lowercased[0].iter().filter(|low| low == &subrecord).count();
                    let object_count = self.lowercased[next_index].iter().filter(|low| low == &subrecord).count();
                    if object_count < first_count
                        && self
                            .deleted
                            .iter()
                            .filter(|(low, _, _)| low == subrecord)
                            .count()
                            < (first_count - object_count)
                    {
                        let base_record = &map.record(0)?;
                        let to_delete = (subrecord.clone(), subindex, base_record.plugin_info);
                        if !self.deleted.contains(&to_delete) {
                            self.deleted.push(to_delete);
                            log_field_shorten!(base_record, inventory, next_index, subindex, option_log, map, $short, cfg);
                        }
                    }
                }
                Ok(())
            }

            fn [<add_ $short>](
                &mut self,
                map: &'a [<$long RecordMap>],
                next_index: usize,
                option_log: &mut OptionRecordMergeLog,
                cfg: &Cfg
            ) -> Result<()> {
                for (subindex, subrecord) in self.lowercased[next_index].iter().enumerate() {
                    if !(self.lowercased[0].contains(subrecord) || self.added.iter().any(|added| &added.0 == subrecord))
                        || self.lowercased[next_index].iter().filter(|low| low == &subrecord).count()
                            > (self.lowercased[0].iter().filter(|low| low == &subrecord).count() + self.added.iter().filter(|added| &added.0 == subrecord).count())
                    {
                        add_and_log_field_lengthen!(self.added, subrecord, inventory, next_index, subindex, option_log, map, $short, cfg);
                    }
                }
                Ok(())
            }
        })+
    }
}

pub(super) use {make, process};
