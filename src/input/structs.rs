use crate::{Cfg, PluginInfo};
use hashbrown::HashMap;
use tes3::esp::TES3Object;

pub(crate) type PluginName<'a> = &'a String;
pub(crate) type ResponsiblePlugins<'a> = Vec<PluginName<'a>>;

pub(crate) struct InputHelper<'a> {
    pub(crate) cfg: &'a Cfg,
    pub(crate) plugin_index: usize,
    pub(crate) plugin_info: &'a PluginInfo,
    pub(crate) creatures: HelperCreatures,
    pub(crate) items: HelperItems,
}

impl<'a> InputHelper<'a> {
    pub(super) fn new(cfg: &'a Cfg, plugin_info: &'a PluginInfo) -> InputHelper<'a> {
        InputHelper {
            cfg,
            plugin_index: 0,
            plugin_info,
            creatures: HelperCreatures::default(),
            items: HelperItems::default(),
        }
    }

    pub(super) fn set_plugin(&mut self, plugin_count: u64, plugin_info: &'a PluginInfo) {
        self.plugin_index = plugin_count as usize - 1;
        self.plugin_info = plugin_info;
    }
}

#[derive(Default)]
pub(crate) struct HelperCreatures {
    pub(crate) counter: usize,
    pub(crate) ids: HashMap<String, usize>,
}

pub(crate) type HelperItems = HelperCreatures;

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
    pub(super) fn get_plugins(&mut self, total: usize) {
        self.plugins.total = total as u32;
    }
    pub(super) fn get_records(&mut self, tes3object: &TES3Object) {
        self.records.total += match tes3object {
            TES3Object::Header(header) => header.num_objects,
            _ => 0,
        };
    }
    pub(super) fn get_speed(&mut self, seconds: f64) {
        self.records.speed = self.records.total as f64 / seconds;
        self.plugins.speed = self.plugins.total as f64 / seconds;
    }
}
