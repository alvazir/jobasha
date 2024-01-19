use crate::{ComparePlugins, ListCounts, PluginName, ResponsiblePlugins, Subrecord};
use std::collections::HashMap;
use tes3::esp::{LeveledCreature, LeveledItem, Plugin, TES3Object};

pub(crate) struct Messages<'a> {
    pub(crate) threshold_resolved: ThresholdMessages<'a>,
    pub(crate) threshold_skipped: ThresholdMessages<'a>,
    pub(crate) threshold_warnings: ThresholdMessages<'a>,
    pub(crate) untouched_lists: Vec<UntouchedList<'a>>,
    pub(crate) deleted_subrecords: Vec<DeletedSubrecords<'a>>,
    pub(crate) deleveled_subrecords: Vec<DeleveledSubrecords<'a>>,
}

impl<'a> Messages<'a> {
    fn new() -> Messages<'a> {
        Messages {
            threshold_resolved: ThresholdMessages::new(ThresholdMessageKind::Resolved),
            threshold_skipped: ThresholdMessages::new(ThresholdMessageKind::Skipped),
            threshold_warnings: ThresholdMessages::new(ThresholdMessageKind::Warning),
            untouched_lists: Vec::new(),
            deleted_subrecords: Vec::new(),
            deleveled_subrecords: Vec::new(),
        }
    }
}

#[derive(Default)]
pub(crate) struct RawOutputPlugin<'a> {
    pub(crate) plugin: Plugin,
    pub(crate) lists: Vec<TES3Object>,
    pub(crate) masters: Masters<'a>,
}

pub(crate) struct Helper<'a> {
    pub(crate) merge: RawOutputPlugin<'a>,
    pub(crate) delev: RawOutputPlugin<'a>,
    pub(crate) compare: ComparePlugins,
    pub(crate) counts: ListCounts,
    pub(crate) messages: Messages<'a>,
    pub(crate) exit_code: i32,
}

impl<'a> Helper<'a> {
    pub(crate) fn new(plugins_to_compare: ComparePlugins) -> Helper<'a> {
        Helper {
            merge: RawOutputPlugin::default(),
            delev: RawOutputPlugin::default(),
            compare: plugins_to_compare,
            counts: ListCounts::default(),
            messages: Messages::new(),
            exit_code: 0,
        }
    }
}

pub(crate) type Masters<'a> = HashMap<PluginName<'a>, (usize, PluginName<'a>, u64)>;

pub(crate) struct ThresholdMessages<'a> {
    pub(crate) kind: ThresholdMessageKind,
    pub(crate) messages: Vec<ThresholdMessageRaw<'a>>,
}

impl<'a> ThresholdMessages<'a> {
    fn new(kind: ThresholdMessageKind) -> ThresholdMessages<'a> {
        ThresholdMessages {
            kind,
            messages: Vec::new(),
        }
    }

    pub(crate) fn push(
        &mut self,
        ratio: f64,
        threshold: f64,
        log_t: &'a str,
        id: String,
        initial_plugin: PluginName<'a>,
        delete: &[(Subrecord, Subrecord, ResponsiblePlugins<'a>)],
    ) {
        let mut plugins = Vec::new();
        for (_, _, responsible_plugins) in delete {
            for responsible_plugin in responsible_plugins.iter() {
                if !plugins.contains(&responsible_plugin) {
                    plugins.push(responsible_plugin);
                }
            }
        }
        self.messages.push(ThresholdMessageRaw {
            ratio,
            threshold,
            log_t,
            id,
            initial_plugin,
            responsible_plugins_str: plugins.into_iter().map(|x| x.as_str()).collect(),
        });
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

pub(crate) enum ThresholdMessageKind {
    Resolved,
    Skipped,
    Warning,
}

type ResponsiblePluginsStr<'a> = Vec<&'a str>;

pub(crate) struct ThresholdMessageRaw<'a> {
    pub(crate) ratio: f64,
    pub(crate) threshold: f64,
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: PluginName<'a>,
    pub(crate) responsible_plugins_str: ResponsiblePluginsStr<'a>,
}

pub(crate) struct DeletedSubrecords<'a> {
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: PluginName<'a>,
    pub(crate) subrecords: Vec<(Subrecord, ResponsiblePluginsStr<'a>)>,
}

pub(crate) struct UntouchedList<'a> {
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: &'a str,
    pub(crate) last_plugin: &'a str,
}

type NewLevel = u16;

#[derive(Debug)]
pub(crate) struct DeleveledSubrecords<'a> {
    pub(crate) log_t: &'a str,
    pub(crate) id: String,
    pub(crate) initial_plugin: PluginName<'a>,
    pub(crate) subrecords: Vec<(Subrecord, NewLevel)>,
}

pub(super) type Levc<'a> = HashMap<String, (usize, &'a LeveledCreature)>;
pub(super) type Levi<'a> = HashMap<String, (usize, &'a LeveledItem)>;

#[derive(Default)]
pub(super) struct ListDiffStats {
    pub(super) added: usize,
    pub(super) removed: usize,
    pub(super) changed: usize,
}
