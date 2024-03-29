use crate::{msg, Cfg, Log, MsgTone, PluginInfo, Progress};
use anyhow::{anyhow, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    io::ErrorKind,
};
use tes3::esp::{LeveledCreature, LeveledCreatureFlags, LeveledItem, LeveledItemFlags, ObjectFlags, Plugin, TES3Object};

#[derive(Clone, PartialEq)]
pub(crate) struct Creature<'a> {
    pub(crate) flags: Vec<ObjectFlags>,
    pub(crate) id: String,
    pub(crate) leveled_creature_flags: Vec<LeveledCreatureFlags>,
    pub(crate) chance_nones: Vec<u8>,
    pub(crate) list: Vec<Subrecord>,
    pub(crate) list_lowercased: Vec<Subrecord>,
    pub(crate) first: Vec<(Subrecord, Subrecord)>,
    pub(crate) delete: Vec<(Subrecord, Subrecord, ResponsiblePlugins<'a>)>,
    pub(crate) count: usize,
    pub(crate) plugin_name_lowercased: PluginName<'a>,
    pub(crate) masters: Vec<&'a PluginInfo>,
    pub(crate) last: LastCreature,
    pub(crate) last_plugin_name: Option<PluginName<'a>>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct Item<'a> {
    pub(crate) flags: Vec<ObjectFlags>,
    pub(crate) id: String,
    pub(crate) leveled_item_flags: Vec<LeveledItemFlags>,
    pub(crate) chance_nones: Vec<u8>,
    pub(crate) list: Vec<Subrecord>,
    pub(crate) list_lowercased: Vec<Subrecord>,
    pub(crate) first: Vec<(Subrecord, Subrecord)>,
    pub(crate) delete: Vec<(Subrecord, Subrecord, ResponsiblePlugins<'a>)>,
    pub(crate) count: usize,
    pub(crate) plugin_name_lowercased: PluginName<'a>,
    pub(crate) masters: Vec<&'a PluginInfo>,
    pub(crate) last: LastItem,
    pub(crate) last_plugin_name: Option<PluginName<'a>>,
}

pub(crate) type Subrecord = (String, u16);
pub(crate) type PluginName<'a> = &'a String;
pub(crate) type ResponsiblePlugins<'a> = Vec<PluginName<'a>>;

#[derive(Clone, PartialEq)]
pub(crate) struct LastCreature {
    pub(crate) flag: ObjectFlags,
    pub(crate) list_flag: LeveledCreatureFlags,
    pub(crate) chance_none: u8,
    pub(crate) list: Vec<Subrecord>,
}

impl Default for LastCreature {
    fn default() -> Self {
        LastCreature {
            flag: ObjectFlags::default(),
            list_flag: LeveledCreatureFlags::CALCULATE_FROM_ALL_LEVELS,
            chance_none: u8::default(),
            list: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct LastItem {
    pub(crate) flag: ObjectFlags,
    pub(crate) list_flag: LeveledItemFlags,
    pub(crate) chance_none: u8,
    pub(crate) list: Vec<Subrecord>,
}

impl Default for LastItem {
    fn default() -> Self {
        LastItem {
            flag: ObjectFlags::default(),
            list_flag: LeveledItemFlags::CALCULATE_FROM_ALL_LEVELS,
            chance_none: u8::default(),
            list: Vec::new(),
        }
    }
}

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
    ($name:ident, $helper:ident, $my_kind:ident, $tes3_kind:ident, $flags_kind:ident, $last:ident) => {
        for object in $helper.plugin_content.objects_of_type::<$tes3_kind>() {
            match $helper.$name.ids.entry(object.id.to_lowercase()) {
                Entry::Vacant(v) => {
                    $name.push($my_kind {
                        flags: vec![object.flags.clone()],
                        id: object.id.clone(),
                        $flags_kind: vec![object.$flags_kind],
                        chance_nones: vec![object.chance_none],
                        list: object.$name.clone(),
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
                    let o = &mut $name[*o.get()];
                    let mut add_master = false;
                    if !o.flags.contains(&object.flags) {
                        o.flags.push(object.flags.clone());
                        add_master = true;
                    }
                    if !o.$flags_kind.contains(&object.$flags_kind) {
                        o.$flags_kind.push(object.$flags_kind);
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
                    o.last.list_flag = object.$flags_kind;
                    o.last.chance_none = object.chance_none;
                    o.last_plugin_name = Some(&$helper.plugin_info.name);
                    o.count += 1;
                }
            };
        }
    };
}

pub(super) fn get_lists<'a>(
    plugins: &'a [PluginInfo],
    cfg: &'a Cfg,
    log: &mut Log,
) -> Result<(Vec<Creature<'a>>, Vec<Item<'a>>, ReadStats)> {
    let mut creatures: Vec<Creature> = Vec::new();
    let mut items: Vec<Item> = Vec::new();
    let mut stats = ReadStats::default();
    if cfg.compare_only {
        return Ok((creatures, items, stats));
    }
    let mut helper = Helper::new(cfg, &plugins[0]);
    let plugins_len = plugins.len();
    let mut progress = Progress::new(plugins_len, cfg);
    let mut skipped_plugins: Vec<String> = Vec::new();
    for (plugin_info, count) in plugins.iter().zip(1u64..) {
        if !progress.off {
            progress.tick(count);
        }
        helper.plugin_info = plugin_info;
        if let Err(error) = helper.plugin_content.load_path(&plugin_info.path) {
            if matches!(error.kind(), ErrorKind::InvalidData) {
                if let Some(tag) = error.to_string().strip_prefix("Unexpected Tag: ") {
                    if cfg.skip_unexpected_tags
                        || (!cfg.no_skip_unexpected_tags_default
                            && cfg.guts.skip_unexpected_tags_default.contains(&tag.to_lowercase()))
                    {
                        skipped_plugins.push(format!(
                            "Plugin \"{}\" will be skipped, because it contains known unexpected record type: {}",
                            &plugin_info.name, tag
                        ));
                        continue;
                    } else {
                        return Err(anyhow!("Failed to read plugin \"{}\"\n{}\nUse either --skip \"{0}\" to skip this plugin or --skip-unexpected-tags to skip all similar plugins\nConsider reporting the error to add this tag to the list of unexpected tags to skip by default", &plugin_info.name, error));
                    }
                }
            };
            return Err(anyhow!("Failed to read plugin \"{}\"\n{}", &plugin_info.name, error));
        };
        stats.get_records(&helper.plugin_content.objects[0]);
        if !cfg.creatures.skip {
            get_lists!(creatures, helper, Creature, LeveledCreature, leveled_creature_flags, LastCreature);
        }
        if !cfg.items.skip {
            get_lists!(items, helper, Item, LeveledItem, leveled_item_flags, LastItem);
        }
    }
    stats.get_plugins(plugins_len - skipped_plugins.len());
    stats.get_speed(progress.finish());
    if !skipped_plugins.is_empty() {
        msg(skipped_plugins.join("\n"), MsgTone::Neutral, 0, cfg, log)?;
    }
    Ok((creatures, items, stats))
}
