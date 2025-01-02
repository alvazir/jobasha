use crate::{Cfg, ComparePlugins, ListCounts, Log, OutputFile, PluginKind, RawPlugins};
use anyhow::{Context, Result};
mod compare_plugins;
mod write_plugins;
use compare_plugins::compare_plugins;
use write_plugins::write_plugins;

pub(super) fn process_output(
    mut plugins_to_compare: ComparePlugins,
    mut raw: RawPlugins,
    counts: &mut ListCounts,
    exit_code: &mut i32,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    update_headers(&mut raw, cfg).with_context(|| "Failed to update plugin's header")?;
    if !cfg.compare_only {
        write_plugins(&mut raw, &mut plugins_to_compare, counts, cfg, log).with_context(|| "Failed to write plugin")?;
    }
    compare_plugins(&raw, &plugins_to_compare, counts, exit_code, cfg, log).with_context(|| "Failed to compare plugins")?;
    Ok(())
}

fn update_headers(raw: &mut RawPlugins, cfg: &Cfg) -> Result<()> {
    if cfg.delev && !cfg.delev_distinct {
        raw.merge.update_header(&cfg.guts.header_description_merge_and_delev, cfg)?;
    } else {
        raw.merge.update_header(&cfg.guts.header_description_merge, cfg)?;
    }
    if cfg.delev_distinct {
        raw.delev.update_header(&cfg.guts.header_description_delev, cfg)?;
    }
    Ok(())
}

fn select_placed(output: &OutputFile, counts: &ListCounts, cfg: &Cfg) -> usize {
    match output.kind {
        PluginKind::Merge if !(cfg.delev && cfg.delev_distinct) => counts.total.placed,
        PluginKind::Merge => counts.merge.placed,
        PluginKind::Delev => counts.delev.placed,
    }
}
