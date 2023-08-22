use crate::{Cfg, PluginInfo, Progress};
use anyhow::{Context, Result};
use std::collections::{hash_map::Entry, HashMap};
use tes3::esp::{LeveledCreature, LeveledItem, ObjectFlags, Plugin, TES3Object};

#[derive(Clone, PartialEq)]
pub(crate) struct Creature<'a> {
    pub(crate) flags: Vec<ObjectFlags>,
    pub(crate) id: String,
    pub(crate) list_flags: Vec<u32>,
    pub(crate) chance_nones: Vec<u8>,
    pub(crate) list: Vec<Subrecord>,
    pub(crate) list_lowercased: Vec<Subrecord>,
    pub(crate) first: Vec<(Subrecord, Subrecord)>,
    pub(crate) delete: Vec<(Subrecord, Subrecord, ResponsiblePlugins<'a>)>,
    pub(crate) count: usize,
    pub(crate) plugin_name_lowercased: PluginName<'a>,
    pub(crate) masters: Vec<&'a PluginInfo>,
    pub(crate) last: LastList,
    pub(crate) last_plugin_name: Option<PluginName<'a>>,
}

pub(crate) type Subrecord = (String, u16);
pub(crate) type PluginName<'a> = &'a String;
pub(crate) type ResponsiblePlugins<'a> = Vec<PluginName<'a>>;

#[derive(Default, Clone, PartialEq)]
pub(crate) struct LastList {
    pub(crate) flag: ObjectFlags,
    pub(crate) list_flag: u32,
    pub(crate) chance_none: u8,
    pub(crate) list: Vec<Subrecord>,
}

pub(crate) type Item<'a> = Creature<'a>;

struct Helper<'a> {
    cfg: &'a Cfg,
    plugin_info: &'a PluginInfo,
    plugin_content: Plugin,
    creatures: HelperCreatures,
    items: HelperItems,
}

impl<'a> Helper<'a> {
    fn new(cfg: &'a Cfg, plugin_info: &'a PluginInfo) -> Helper<'a> {
        Helper {
            cfg,
            plugin_info,
            plugin_content: Plugin::new(),
            creatures: HelperCreatures::default(),
            items: HelperItems::default(),
        }
    }
}

#[derive(Default)]
struct HelperCreatures {
    counter: usize,
    ids: HashMap<String, usize>,
}

type HelperItems = HelperCreatures;

#[derive(Default)]
pub(crate) struct PluginReadStats {
    pub(crate) total: u32,
    pub(crate) speed: f64,
}

pub(crate) type RecordReadStats = PluginReadStats;

#[derive(Default)]
pub(crate) struct ReadStats {
    pub(crate) plugins: PluginReadStats,
    pub(crate) records: RecordReadStats,
}

impl ReadStats {
    fn get_plugins(&mut self, total: usize) {
        self.plugins.total = total as u32;
    }
    fn get_records(&mut self, tes3object: &TES3Object) {
        self.records.total += match tes3object {
            TES3Object::Header(header) => header.num_objects,
            _ => 0,
        };
    }
    fn get_speed(&mut self, seconds: f64) {
        self.records.speed = self.records.total as f64 / seconds;
        self.plugins.speed = self.plugins.total as f64 / seconds;
    }
}

macro_rules! get_lists {
    ($name:ident, $helper:ident, $my_kind:ident, $tes3_kind:ident) => {
        for object in $helper.plugin_content.objects_of_type::<$tes3_kind>() {
            match $helper.$name.ids.entry(object.id.to_lowercase()) {
                Entry::Vacant(v) => {
                    $name.push($my_kind {
                        flags: vec![object.flags.clone()],
                        id: object.id.clone(),
                        list_flags: vec![object.list_flags],
                        chance_nones: vec![object.chance_none],
                        list: object.$name.clone(),
                        list_lowercased: Vec::new(),
                        first: Vec::new(),
                        delete: Vec::new(),
                        count: 1,
                        plugin_name_lowercased: &$helper.plugin_info.name_lowercased,
                        masters: vec![&$helper.plugin_info],
                        last: LastList::default(),
                        last_plugin_name: None,
                    });
                    v.insert($helper.$name.counter);
                    $helper.$name.counter += 1;
                }
                Entry::Occupied(o) => {
                    let o = &mut $name[*o.get()];
                    let mut add_master = false;
                    if !o.flags.contains(&object.flags) {
                        o.flags.push(object.flags.clone());
                        add_master = true;
                    }
                    if !o.list_flags.contains(&object.list_flags) {
                        o.list_flags.push(object.list_flags);
                        add_master = true;
                    }
                    if !o.chance_nones.contains(&object.chance_none) {
                        o.chance_nones.push(object.chance_none);
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
                    let object_list_lowercased = object
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
                            o.list.push(object.$name[index].clone());
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
                    o.last.list = object.$name.clone();
                    o.last.flag = object.flags.clone();
                    o.last.list_flag = object.list_flags;
                    o.last.chance_none = object.chance_none;
                    o.last_plugin_name = Some(&$helper.plugin_info.name);
                    o.count += 1;
                }
            };
        }
    };
}

pub(crate) fn get_lists<'a>(plugins: &'a [PluginInfo], cfg: &'a Cfg) -> Result<(Vec<Creature<'a>>, Vec<Item<'a>>, ReadStats)> {
    let mut creatures: Vec<Creature> = Vec::new();
    let mut items: Vec<Item> = Vec::new();
    let mut helper = Helper::new(cfg, &plugins[0]);
    let mut stats = ReadStats::default();
    let plugins_len = plugins.len();
    let mut progress = Progress::new(plugins_len, cfg);
    for (plugin_info, count) in plugins.iter().zip(1u64..) {
        if !progress.off {
            progress.tick(count);
        }
        helper.plugin_info = plugin_info;
        helper
            .plugin_content
            .load_path(&plugin_info.path)
            .with_context(|| format!("Failed to read plugin \"{}\"", &plugin_info.name))?;
        stats.get_records(&helper.plugin_content.objects[0]);
        if !cfg.creatures.no {
            get_lists!(creatures, helper, Creature, LeveledCreature);
        }
        if !cfg.items.no {
            get_lists!(items, helper, Item, LeveledItem);
        }
    }
    stats.get_plugins(plugins_len);
    stats.get_speed(progress.finish());
    Ok((creatures, items, stats))
}
