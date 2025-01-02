use crate::PluginInfo;
use anyhow::{Context, Result};
use hashbrown::{hash_map::Entry, HashMap};
use paste::paste;
use std::fmt;
use tes3::esp::{
    Activator, Alchemy, Apparatus, Armor, Birthsign, Bodypart, Book, Cell, CellFlags, Class, Clothing, Container, Creature, Door,
    EffectId, Enchanting, GameSetting, Ingredient, Light, Lockpick, MagicEffect, MiscItem, Npc, Probe, Race, RepairItem, Skill,
    SkillId, Sound, SoundGen, Spell, Static, Weapon,
};

pub(crate) trait RecordMap<'a> {
    fn init_id(&self) -> usize;
    fn records_quantity(&self) -> usize;
    fn kind_short_upper(&self) -> &'static str;
    fn all_plugin_names(&'a self) -> Vec<&'a str>;
    fn init_plugin_name(&'a self) -> anyhow::Result<&'a str>;
    fn last_plugin_name(&'a self) -> anyhow::Result<&'a str>;
    fn record_id_debug(&'a self) -> anyhow::Result<impl 'a + std::fmt::Debug>;
    #[allow(dead_code)]
    fn record_id_display(&'a self) -> anyhow::Result<impl 'a + std::fmt::Display>;
}

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum CellKey {
    Interior(String),
    Exterior((i32, i32)),
}

macro_rules! set_merge_types {
    ($list:ident, $struct:ident, $bool:expr, $($short:ident),+) => {
        for arg in $list {
            for word_with_comma in arg.split_whitespace() {
                for word in word_with_comma.split(',') {
                    if !word.is_empty() {
                        match word {
                            $(stringify!($short) => { $struct.$short = $bool; },)+
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

macro_rules! select_id_debug {
    ($self:ident, $record:ident, $short:ident, cell, $name:ident) => {
        if $record.$short.data.flags.contains(CellFlags::IS_INTERIOR) {
            Ok(&$record.$short.name)
        } else {
            Ok(&$self.$name)
        }
    };
    ($self:ident, $record:ident, $short:ident, $key:ident) => {
        Ok(&$record.$short.$key)
    };
}

macro_rules! make_intermediate_records {
    ($($short:ident:$key:ident:$key_type:ty:$long:ident$(:$cell_name:ident)?),+) => {
        paste! {
$(
pub(crate) struct [<$long Record>]<'a> {
    pub(crate) $short: $long,
    pub(crate) plugin_info: &'a PluginInfo,
}

impl<'a> [<$long Record>]<'a> {
    fn new($short: $long, plugin_info: &'a PluginInfo) -> [<$long Record>]<'a> {
        [<$long Record>] { $short, plugin_info }
    }

    #[allow(dead_code)]
    pub(crate) fn plugin_info(&self) -> &PluginInfo {
        self.plugin_info
    }
}

pub(crate) struct [<$long RecordMap>]<'a> {
    pub(crate) init_id: usize,
    $(pub(crate) $cell_name: String,)?
    pub(crate) records: Vec<[<$long Record>]<'a>>,
}

impl<'a> [<$long RecordMap>]<'a> {
    fn new(init_id: usize, $short: $long, plugin_info: &'a PluginInfo) -> [<$long RecordMap>]<'a> {
        [<$long RecordMap>] {
            init_id,
            $($cell_name: String::new(),)?
            records: vec![[<$long Record>]::new($short, plugin_info)],
        }
    }

    fn push(&mut self, $short: $long, plugin_info: &'a PluginInfo) {
        self.records.push([<$long Record>]::new($short, plugin_info))
    }

    pub(crate) fn record(&self, index:usize) -> Result<&[<$long Record>]> {
        self.records.get(index).with_context(|| format!("Bug: failed to get record({index})"))
    }

    #[allow(dead_code)]
    pub(crate) fn record_id_debug(&'a self) -> Result<impl 'a + std::fmt::Debug> {
        let record = self.record(0)?;
        select_id_debug!(self, record, $short, $key$(, $cell_name)?)
    }

    pub(crate) fn last_record(&self) -> Result<&[<$long Record>]> {
        self.records.last().with_context(|| format!("Bug: failed to get last record"))
    }
}

impl<'a> RecordMap<'a> for &[<$long RecordMap>]<'a> {
    fn init_id(&self) -> usize {
        self.init_id
    }

    fn records_quantity(&self) -> usize {
        self.records.len()
    }

    fn kind_short_upper(&self) -> &'static str {
        stringify!([<$short:upper>])
    }

    fn all_plugin_names(&'a self) -> Vec<&'a str> {
        self.records.iter().map(|x| x.plugin_info.name.as_ref()).collect()
    }

    fn init_plugin_name(&'a self) -> anyhow::Result<&'a str> {
        Ok(&self.record(0)?.plugin_info.name)
    }

    fn last_plugin_name(&'a self) -> anyhow::Result<&'a str> {
        Ok(&self.last_record()?.plugin_info.name)
    }

    fn record_id_debug(&'a self) -> Result<impl 'a + std::fmt::Debug> {
        let record = self.record(0)?;
        select_id_debug!(self, record, $short, $key$(, $cell_name)?)
    }

    fn record_id_display(&'a self) -> Result<impl 'a + std::fmt::Display> {
        let record = self.record(0)?;
        macro_rules! select_id_display {
            (id) => { &record.$short.id };
            (cell, $name:ident) => {
            if record.$short.data.flags.contains(CellFlags::IS_INTERIOR) {
                &record.$short.name
            } else {
                &self.$name
            }};
            ($key_field:ident) => { record.$short.$key_field as i32 };
        }
        Ok(select_id_display!($key$(, $cell_name)?))
    }
}
)+

#[derive(Default)]
pub(crate) struct IntermediateRecords<'a> {
    counter: usize,
    $(pub(crate) $short: HashMap<$key_type, [<$long RecordMap>]<'a>>,)+
}

impl<'a> IntermediateRecords<'a> {
    $(
    pub(crate) fn [<get_ $short>](&mut self, $short: $long, plugin_info: &'a PluginInfo) {
        self.counter += 1;
        macro_rules! select_hashmap_entry {
            (id) => { $short.id.to_lowercase() };
            (cell) => {
                if $short.data.flags.contains(CellFlags::IS_INTERIOR) {
                    CellKey::Interior($short.name.to_lowercase())
                } else {
                    CellKey::Exterior($short.data.grid)
                }
            };
            ($key_field:ident) => { $short.$key_field };
        }
        match self.$short.entry(select_hashmap_entry!($key)) {
            Entry::Vacant(v) => {
                v.insert([<$long RecordMap>]::new(self.counter, $short, plugin_info));
            }
            Entry::Occupied(mut o) => {
                o.get_mut().push($short, plugin_info);
            }
        };
    }
    )+
}

#[derive(Default)]
pub(crate) struct Merge {
    pub(crate) skip: bool,
    pub(crate) ignore_secondary_fog_density: bool,
    pub(crate) interdependent_flags: bool,
    pub(crate) keep_redundant_values: bool,
    pub(crate) plus_before_minus: bool,
    pub(crate) verbose_atmosphere_data: bool,
    pub(crate) destination_similarity: f32,
    pub(crate) references: bool,
    $(pub(crate) $short: bool,)+
}

impl fmt::Display for Merge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut empty = true;
        $(if self.$short {
                write!(f, "{}{}", if empty { "" } else { ", " }, stringify!([<$short:upper>]))?;
                empty = false;
        })+
        if empty {
                write!(f, "NONE")?;
        }
        Ok(())
    }
}

impl Merge {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        no_merge: bool,
        ignore_secondary_fog_density: bool,
        interdependent_flags: bool,
        keep_redundant_values: bool,
        plus_before_minus: bool,
        verbose_atmosphere_data: bool,
        destination_similarity: f32,
        merge_types: Vec<String>,
        merge_skip_types: Vec<String>,
    ) -> Self {
        let mut res = Self {
            skip: no_merge,
            ignore_secondary_fog_density,
            interdependent_flags,
            keep_redundant_values,
            plus_before_minus,
            verbose_atmosphere_data,
            destination_similarity,
            ..Default::default()
        };
        if !no_merge {
            set_merge_types!(merge_types, res, true, $($short),+);
            set_merge_types!(merge_skip_types, res, false, $($short),+);
        }
        res
    }
}
        }
    };
}

make_intermediate_records!(
    gmst:id:String:GameSetting,
    clas:id:String:Class,
    race:id:String:Race,
    soun:id:String:Sound,
    skil:skill_id:SkillId:Skill,
    mgef:effect_id:EffectId:MagicEffect,
    bsgn:id:String:Birthsign,
    spel:id:String:Spell,
    stat:id:String:Static,
    door:id:String:Door,
    misc:id:String:MiscItem,
    weap:id:String:Weapon,
    cont:id:String:Container,
    crea:id:String:Creature,
    body:id:String:Bodypart,
    ligh:id:String:Light,
    ench:id:String:Enchanting,
    npc_:id:String:Npc,
    armo:id:String:Armor,
    clot:id:String:Clothing,
    repa:id:String:RepairItem,
    acti:id:String:Activator,
    appa:id:String:Apparatus,
    lock:id:String:Lockpick,
    prob:id:String:Probe,
    ingr:id:String:Ingredient,
    book:id:String:Book,
    alch:id:String:Alchemy,
    cell:cell:CellKey:Cell:name,
    sndg:id:String:SoundGen
);
