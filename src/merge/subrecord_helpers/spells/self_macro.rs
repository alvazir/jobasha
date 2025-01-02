macro_rules! process {
    ($($short:ident, $long:ident),+) => {
        $(paste! {
            fn [<delete_ $short>](&mut self, map: &'a [<$long RecordMap>], next_index: usize, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<()> {
                for (subindex, subrecord) in self.lowercased[0].iter().enumerate() {
                    if !self.deleted.iter().any(|deleted| deleted.0 == subindex) && !self.lowercased[next_index].contains(subrecord) {
                            let base_record = &map.record(0)?;
                            self.deleted.push((subindex, base_record.plugin_info));
                            log_field_shorten!(base_record, spells, next_index, subindex, option_log, map, $short, cfg);
                    }
                }
                Ok(())
            }

            fn [<add_ $short>](&mut self, map: &'a [<$long RecordMap>], next_index: usize, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<()> {
                for (subindex, subrecord) in self.lowercased[next_index].iter().enumerate() {
                    if !self.lowercased[0].contains(subrecord) &&
                        !self.added.iter().any(|added| &added.0 == subrecord) {
                            add_and_log_field_lengthen!(self.added, subrecord, spells, next_index, subindex, option_log, map, $short, cfg);
                    }
                }
                Ok(())
            }
        })+
    };
}

pub(crate) use process;
