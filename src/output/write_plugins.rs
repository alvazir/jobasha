use crate::{get_plugin_size, msg, Cfg, ListCounts, Log, MsgTone, OutputFile, PluginKind};
use anyhow::{anyhow, Context, Result};
use fs_err::create_dir_all;
use std::path::Path;
use tes3::esp::{Plugin, TES3Object};

pub(super) fn write_plugins(
    merge_plugin: Plugin,
    delev_plugin: Plugin,
    counts: &mut ListCounts,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if cfg.dry_run {
        return Ok(());
    }
    let mut merge_was_written = false;
    write_plugin(merge_plugin, counts, &cfg.output, &mut merge_was_written, cfg, log)?;
    if cfg.delev_distinct {
        write_plugin(delev_plugin, counts, &cfg.delev_output, &mut merge_was_written, cfg, log)?;
    }
    Ok(())
}

fn write_plugin(
    mut plugin: Plugin,
    counts: &mut ListCounts,
    output: &OutputFile,
    merge_was_written: &mut bool,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    let placed = match output.kind {
        PluginKind::Merge if !(cfg.delev && cfg.delev_distinct) => counts.total.placed,
        PluginKind::Merge => counts.merge.placed,
        PluginKind::Delev => counts.delev.placed,
    };
    if placed == 0 {
        let text = format!(
            "Everything is great right now, there is nothing to place into plugin \"{}\"",
            output.name
        );
        msg(text, MsgTone::Good, 0, cfg, log)?;
        return Ok(());
    }
    if output.dir_path != Path::new("") && !output.dir_path.exists() {
        create_dir_all(&output.dir_path).with_context(|| {
            format!(
                "Failed to create directory \"{}\" for plugin \"{}\"",
                output.dir_path.display(),
                output.name
            )
        })?;
        let text = format!("Directory \"{}\" was created", output.dir_path.display());
        msg(text, MsgTone::Good, 0, cfg, log)?;
    }
    if matches!(output.kind, PluginKind::Delev) && *merge_was_written {
        delev_append_master(&mut plugin, counts, cfg, log)?;
    }
    plugin
        .save_path(&output.path)
        .with_context(|| format!("Failed to write plugin \"{}\" to \"{}\"", output.name, output.path.display()))?;
    if matches!(output.kind, PluginKind::Merge) {
        *merge_was_written = true;
    }
    let text = format!("Plugin \"{}\" was written to \"{}\"", output.name, output.path.display());
    msg(text, MsgTone::Good, 0, cfg, log)?;
    Ok(())
}

fn delev_append_master(deleveled_plugin: &mut Plugin, counts: &mut ListCounts, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let size = get_plugin_size(&cfg.output.path, cfg, log)?;
    match &mut deleveled_plugin.objects[0] {
        TES3Object::Header(header) => header.masters.push((cfg.output.name.clone(), size)),
        _ => return Err(anyhow!("omg")),
    };
    counts.delev.master += 1;
    Ok(())
}
