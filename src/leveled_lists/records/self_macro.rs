macro_rules! ll_record_methods {
    ($short:ident, $long:ident, $ll_long:ident, $name:ident, $helper:ident, $flags_kind:ident, $last:ident, $kind_differs:ident) => {
        paste! {

        #[derive(Default)]
        pub(crate) struct [<$ll_long s>]<'a>(pub(crate) Vec<$ll_long<'a>>);

        impl<'a> [<$ll_long s>]<'a> {
            pub(crate) fn [<make_ $short>](
                self,
                raw: &mut RawPlugins<'a>,
                messages: &mut LlMessages<'a>,
                counts: &mut ListCounts,
                rng: &mut ThreadRng,
                cfg: &'a Cfg,
                log: &mut Log
            ) -> Result<()> {
                for mut o in self.0.into_iter() {
                    counts.total.total += o.count;
                    counts.total.unique += 1;
                    if o.count > 1 || cfg.all_lists || cfg.delev {
                        let mut $name = o.list;

                        if o.count > 1 {
                            if !cfg.no_delete && !o.delete.is_empty() {
                                delete_subrecords(
                                    &mut $name,
                                    &mut o.list_lowercased,
                                    &o.id,
                                    o.first,
                                    o.delete,
                                    cfg.$name.threshold,
                                    &cfg.$name.log_t,
                                    &o.masters[0],
                                    counts,
                                    messages,
                                    cfg,
                                )?;
                            }
                            $name.sort_by(|(name1, _), (name2, _)| name1.to_lowercase().cmp(&name2.to_lowercase()));
                            $name.sort_by_key(|level| level.1);
                        }

                        let is_merge = if o.count > 1
                            && !cfg.all_lists
                            && $kind_differs(&o.flags, &o.$flags_kind, &o.chance_nones, o.list_lowercased, &mut o.last)
                        {
                            true
                        } else {
                            false
                        };

                        let mut is_delev = false;
                        if cfg.delev && !cfg.$name.skip_delev {
                            let delev_list = delevel_list(&$name, &o.id, &cfg.$name, &o.masters[0], rng, cfg, counts, messages);
                            if !delev_list.is_empty() {
                                counts.delev.deleveled += 1;
                                if cfg.delev_distinct {
                                    let x = o.masters.clone();
                                    append_masters(x, &mut raw.delev.masters, &mut counts.delev.master, cfg, log)?;
                                    raw.delev.plugin.objects.push(TES3Object::$long($long {
                                        flags: o.flags.last().unwrap().clone(),
                                        id: o.id.clone(),
                                        $flags_kind: *o.$flags_kind.last().unwrap(),
                                        chance_none: *o.chance_nones.last().unwrap(),
                                        $name: delev_list,
                                    }));
                                    counts.delev.placed += 1;
                                } else {
                                    is_delev = true;
                                    $name = delev_list;
                                }
                            }
                        };

                        if cfg.all_lists || is_delev || is_merge {
                            append_masters(
                                o.masters,
                                &mut raw.merge.masters,
                                if cfg.delev && cfg.delev_distinct {
                                    &mut counts.merge.master
                                } else {
                                    &mut counts.total.master
                                },
                                cfg,
                                log,
                            )?;
                            raw.merge.plugin.objects.push(TES3Object::$long($long {
                                flags: o.flags.last().unwrap().clone(),
                                id: o.id,
                                $flags_kind: *o.$flags_kind.last().unwrap(),
                                chance_none: *o.chance_nones.last().unwrap(),
                                $name,
                            }));
                            if o.count > 1 {
                                counts.merge.merged += 1;
                                if is_merge || cfg.all_lists {
                                    counts.merge.placed += 1;
                                    if !is_delev || cfg.all_lists {
                                        counts.total.placed += 1;
                                    }
                                } else {
                                    counts.merge.untouched += 1;
                                }
                            } else if cfg.all_lists {
                                counts.merge.placed += 1;
                                counts.total.placed += 1;
                            }
                            if is_delev {
                                counts.delev.placed += 1;
                                if !cfg.all_lists {
                                    counts.total.placed += 1;
                                }
                            }
                        } else if o.count > 1 {
                            messages.untouched_lists.push(UntouchedList {
                                log_t: &cfg.$name.log_t,
                                id: o.id,
                                initial_plugin: &o.masters[0].name,
                                last_plugin: &o.last_plugin_name.unwrap(),
                            });
                            if !is_merge {
                                counts.merge.untouched += 1;
                            }
                            counts.merge.merged += 1;
                        }
                    }
                }
                Ok(())
            }

            pub(crate) fn [<get_ $short>](&mut self, $short: $long, $helper: &mut InputHelper<'a>) {
                match $helper.$name.ids.entry($short.id.to_lowercase()) {
                    Entry::Vacant(v) => {
                        self.0.push($ll_long {
                            flags: vec![$short.flags],
                            id: $short.id,
                            $flags_kind: vec![$short.$flags_kind],
                            chance_nones: vec![$short.chance_none],
                            list: $short.$name,
                            list_lowercased: Vec::new(),
                            first: Vec::new(),
                            delete: Vec::new(),
                            count: 1,
                            plugin_name_lowercased: &$helper.plugin_info.name_lowercased,
                            masters: vec![&$helper.plugin_info],
                            last: $last::default(),
                            last_plugin_name: None,
                        });
                        v.insert($helper.$name.counter);
                        $helper.$name.counter += 1;
                    }
                    Entry::Occupied(o) => {
                        let o = &mut self.0[*o.get()];
                        let mut add_master = false;
                        if !o.flags.contains(&$short.flags) {
                            o.flags.push($short.flags);
                            add_master = true;
                        }
                        if !o.$flags_kind.contains(&$short.$flags_kind) {
                            o.$flags_kind.push($short.$flags_kind);
                            add_master = true;
                        }
                        if !o.chance_nones.contains(&$short.chance_none) {
                            o.chance_nones.push($short.chance_none);
                            add_master = true;
                        }
                        if o.count == 1 {
                            o.list_lowercased = o.list.iter().map(|(name, level)| (name.to_lowercase(), *level)).collect();
                            o.first = o
                                .list_lowercased
                                .iter()
                                .zip(o.list.iter())
                                .map(|(a, b)| (a.clone(), b.clone()))
                                .collect();
                        };
                        let object_list_lowercased = $short
                            .$name
                            .iter()
                            .map(|(name, level)| (name.to_lowercase(), *level))
                            .collect::<Vec<_>>();
                        if !$helper.cfg.no_delete
                            && !($helper.cfg.extended_delete && $helper.cfg.never_delete.contains(&o.plugin_name_lowercased))
                        {
                            for (list_item_lowercased, list_item) in o.first.iter() {
                                let object_count = object_list_lowercased.iter().filter(|x| x == &list_item_lowercased).count();
                                let first_count = o.first.iter().filter(|(x, _)| x == list_item_lowercased).count();
                                if object_count < first_count {
                                    if o.delete
                                        .iter()
                                        .filter(|(subrecord_lowercased, _, _)| subrecord_lowercased == list_item_lowercased)
                                        .count()
                                        < (first_count - object_count)
                                    {
                                        o.delete.push((
                                            list_item_lowercased.clone(),
                                            list_item.clone(),
                                            vec![&$helper.plugin_info.name],
                                        ));
                                        add_master = true;
                                    } else {
                                        for (_, _, responsible_plugins) in o
                                            .delete
                                            .iter_mut()
                                            .filter(|(subrecord_lowercased, _, _)| subrecord_lowercased == list_item_lowercased)
                                        {
                                            if !responsible_plugins.contains(&&$helper.plugin_info.name) {
                                                responsible_plugins.push(&$helper.plugin_info.name)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        for (index, list_item_lowercased) in object_list_lowercased.iter().enumerate() {
                            if !o.list_lowercased.contains(list_item_lowercased)
                                || object_list_lowercased.iter().filter(|x| x == &list_item_lowercased).count()
                                    > o.list_lowercased.iter().filter(|x| x == &list_item_lowercased).count()
                            {
                                o.list_lowercased.push(list_item_lowercased.clone());
                                o.list.push($short.$name[index].clone());
                                add_master = true;
                            } else {
                                if o.first.iter().filter(|(x, _)| x == list_item_lowercased).count() == 0 {
                                    add_master = true;
                                }
                            }
                        }
                        if add_master {
                            o.masters.push(&$helper.plugin_info);
                        }
                        o.last.list = $short.$name;
                        o.last.flag = $short.flags;
                        o.last.list_flag = $short.$flags_kind;
                        o.last.chance_none = $short.chance_none;
                        o.last_plugin_name = Some(&$helper.plugin_info.name);
                        o.count += 1;
                    }
                };
            }
        }
        }
    };
}

pub(super) use ll_record_methods;
