use crate::{Cfg, PluginName};
use anyhow::{anyhow, Context, Result};
use hashbrown::HashMap;
use tes3::esp::{FileType, FixedString, Header, Plugin, TES3Object};

type Masters<'a> = HashMap<PluginName<'a>, (usize, PluginName<'a>, u64)>;

#[derive(Default)]
pub(crate) struct RawPlugins<'a> {
    pub(crate) merge: RawPlugin<'a>,
    pub(crate) delev: RawPlugin<'a>,
}

impl RawPlugins<'_> {
    pub(crate) fn new(cfg: &Cfg) -> Self {
        Self {
            merge: RawPlugin::new(),
            delev: if cfg.delev_distinct {
                RawPlugin::new()
            } else {
                RawPlugin::default()
            },
        }
    }
}
#[derive(Default)]
pub(crate) struct RawPlugin<'a> {
    pub(crate) plugin: Plugin,
    pub(crate) masters: Masters<'a>,
}

impl RawPlugin<'_> {
    fn new() -> Self {
        let mut res = Self::default();
        res.plugin.objects.push(TES3Object::Header(Header::default()));
        res
    }

    pub(crate) fn update_header(&mut self, description: &str, cfg: &Cfg) -> Result<()> {
        let num_objects = self
            .plugin
            .objects
            .len()
            .checked_sub(1)
            .with_context(|| format!("Bug: there are no records in a plugin with description \"{description}\""))?
            as u32;
        let Some(TES3Object::Header(ref mut header)) = self.plugin.objects.get_mut(0) else {
            return Err(anyhow!(
                "Bug: first record it not a header in a plugin with description \"{description}\""
            ));
        };
        header.version = cfg.guts.header_version;
        header.file_type = FileType::Esp;
        header.author = FixedString(cfg.guts.header_author.to_owned());
        header.description = FixedString(description.to_owned());
        header.num_objects = num_objects;
        header.masters = Self::make_masters(&self.masters);
        Ok(())
    }

    fn make_masters(masters: &Masters) -> Vec<(String, u64)> {
        let mut masters_sorted: Vec<(usize, &String, u64)> = masters.values().cloned().collect();
        masters_sorted.sort();
        masters_sorted.into_iter().map(|(_, name, size)| (name.to_owned(), size)).collect()
    }
}
