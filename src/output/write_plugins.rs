use super::{select_placed, Helper};
use crate::{get_plugin_size, msg, Cfg, ComparePlugin, ListCounts, Log, MsgTone, OutputFile, PluginKind};
use anyhow::{anyhow, Context, Result};
use fs_err::{create_dir_all, rename};
use std::path::Path;
use tes3::esp::{Plugin, TES3Object};

pub(super) fn write_plugins(h: &mut Helper, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if cfg.dry_run {
        return Ok(());
    }
    let mut merge_was_written = false;
    write_plugin(
        &mut h.merge.plugin,
        &mut h.counts,
        &cfg.output,
        &mut h.compare.previous,
        &mut merge_was_written,
        cfg,
        log,
    )?;
    if cfg.delev_distinct {
        write_plugin(
            &mut h.delev.plugin,
            &mut h.counts,
            &cfg.delev_output,
            &mut h.compare.delev_previous,
            &mut merge_was_written,
            cfg,
            log,
        )?;
    }
    Ok(())
}

fn write_plugin(
    plugin: &mut Plugin,
    counts: &mut ListCounts,
    output: &OutputFile,
    previous: &mut ComparePlugin,
    merge_was_written: &mut bool,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if select_placed(output, counts, cfg) == 0 {
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
        delev_append_master(plugin, counts, cfg, log)?;
    }
    if previous.loaded {
        if previous.plugin.objects == plugin.objects {
            previous.compared = true;
            previous.equal = true;
            if matches!(output.kind, PluginKind::Merge) {
                *merge_was_written = true;
            }
            let text = format!(
                "Plugin \"{}\" was not written because it's equal to the existing plugin \"{}\"",
                output.name,
                output.path.display()
            );
            msg(text, MsgTone::Good, 0, cfg, log)?;
            return Ok(());
        } else {
            previous.compared = true;
        };
    }
    backup_previous_plugin(output, cfg, log)?;
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

fn backup_previous_plugin(output: &OutputFile, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if !cfg.no_backup && output.path.exists() {
        rename(&output.path, &output.backup_path).with_context(|| {
            format!(
                "Failed to rename previous plugin \"{}\" to \"{}\"",
                &output.path.display(),
                &output.backup_path.display()
            )
        })?;
        let text = format!("Previous plugin was renamed to \"{}\"", &output.backup_path.display());
        msg(text, MsgTone::Warm, 1, cfg, log)
    } else {
        Ok(())
    }
}
