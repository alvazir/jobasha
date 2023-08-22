use anyhow::{Context, Result};
use std::{process::exit, time::Instant};
mod get_game_config;
mod get_lists;
mod get_plugins;
mod get_self_config;
mod make_output_plugin;
mod show_summary;
mod util;
mod write_output_plugin;
use get_game_config::get_game_config;
use get_lists::{get_lists, Creature, Item, LastList, PluginName, ReadStats, ResponsiblePlugins, Subrecord};
use get_plugins::{get_plugins, PluginInfo};
use get_self_config::{get_self_config, Cfg};
use make_output_plugin::{make_output_plugin, ListCounts};
use show_summary::show_summary;
use util::{
    create_dir_early, err_or_ignore, err_or_ignore_thread_safe, msg, show_log_path, show_output_plugin_suggestion,
    show_settings_written, Log, MsgTone, Progress,
};
use write_output_plugin::write_output_plugin;

fn main() {
    match run() {
        Ok(false) => exit(0),
        Err(error) => {
            eprintln!("{error:?}");
            exit(1);
        }
        // Some leveled lists should probably be excluded from subrecords deletion mode
        Ok(true) => exit(2),
    }
}

fn run() -> Result<bool> {
    let timer = Instant::now();
    let cfg = get_self_config()?;
    let mut log = Log::new(&cfg)?;
    show_log_path(&cfg, &mut log)?;
    if cfg.settings_write {
        show_settings_written(&cfg, &mut log)?;
        return Ok(false);
    }
    let config_path = get_game_config(&cfg, &mut log).with_context(|| "Failed to get game configuration file")?;
    let plugins = get_plugins(&config_path, &cfg, &mut log).with_context(|| "Failed to get plugins")?;
    let (creatures, items, record_read_stats) = get_lists(&plugins, &cfg).with_context(|| "Failed to get leveled lists")?;
    let (mut output_plugin, counts, warning) =
        make_output_plugin(creatures, items, &cfg, &mut log).with_context(|| "Failed to process leveled lists")?;
    write_output_plugin(&mut output_plugin, &counts, &cfg, &mut log).with_context(|| "Failed to write output plugin")?;
    show_summary(timer, record_read_stats, &counts, &cfg, &mut log)?;
    show_output_plugin_suggestion(&counts, &cfg, &mut log)?;
    Ok(warning)
}
