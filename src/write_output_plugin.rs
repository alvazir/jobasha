use crate::{msg, Cfg, ListCounts, Log, MsgTone};
use anyhow::{Context, Result};
use std::{fs::create_dir_all, path::Path};
use tes3::esp::Plugin;

pub(crate) fn write_output_plugin(plugin: &mut Plugin, counts: &ListCounts, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if cfg.dry_run {
        return Ok(());
    } else if counts.placed == 0 {
        let text = "Everything is great right now, there is nothing to place into the output plugin";
        msg(text, MsgTone::Good, 0, cfg, log)?;
        return Ok(());
    }
    if cfg.output.dir_path != Path::new("") && !cfg.output.dir_path.exists() {
        create_dir_all(&cfg.output.dir_path)
            .with_context(|| format!("Failed to create output plugin directory \"{}\"", cfg.output.dir_path.display()))?;
        let text = format!("Output plugin directory \"{}\" was created", cfg.output.dir_path.display());
        msg(text, MsgTone::Good, 0, cfg, log)?;
    }
    plugin
        .save_path(&cfg.output.path)
        .with_context(|| format!("Failed to write output plugin to \"{}\"", cfg.output.path.display()))?;
    let text = format!("Output plugin was written to \"{}\"", cfg.output.path.display());
    msg(text, MsgTone::Good, 0, cfg, log)?;
    Ok(())
}
