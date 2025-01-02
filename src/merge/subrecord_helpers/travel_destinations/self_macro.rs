macro_rules! process {
    ($($short:ident, $long:ident),+) => {
        $(paste! {
            fn [<delete_ $short>](&mut self, map: &'a [<$long RecordMap>], next_index: usize, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<()> {
                for (subindex, subrecord) in self.lowercased[0].iter().enumerate() {
                    if !self.lowercased[next_index].contains(subrecord) {
                        if !self.deleted.iter().any(|deleted| deleted.0 == subindex) {
                            let base_record = &map.record(0)?;
                            self.deleted.push((subindex, base_record.plugin_info, next_index));
                            log_field_shorten!(base_record, travel_destinations, next_index, subindex, option_log, map, $short, cfg);
                        }
                    }
                }
                Ok(())
            }

            fn [<add_ $short>](&mut self, map: &'a [<$long RecordMap>], next_index: usize, option_log: &mut OptionRecordMergeLog, cfg: &Cfg) -> Result<()> {
                for (subindex, subrecord) in self.lowercased[next_index].iter().enumerate() {
                    if !self.lowercased[0].contains(subrecord) {
                        if !self.added.iter().any(|x| &x[0].0 == subrecord) {
                            let mut to_add = true;
                            for added_vec in &mut self.added {
                                if is_replacement(subrecord, &added_vec[0].0, cfg) {
                                    if !added_vec.iter().any(|added| &added.0 == subrecord) {
                                        add_and_log_field_lengthen!(added_vec, subrecord, travel_destinations, next_index, subindex, option_log, map, $short, cfg);
                                    }
                                    to_add = false;
                                    break;
                                }
                            }
                            if to_add {
                                for deleted in self.deleted.iter().filter(|x| x.2 != next_index) {
                                    if is_replacement(subrecord, &self.lowercased[0][deleted.0], cfg) {
                                        to_add = false;
                                        break;
                                    }
                                }
                                if to_add {
                                    add_and_log_field_lengthen!(self.added:vec, subrecord, travel_destinations, next_index, subindex, option_log, map, $short, cfg);
                                }
                            }
                        }
                    }
                }
                Ok(())
            }
        })+
    };
}

pub(super) use process;
