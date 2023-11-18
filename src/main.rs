/*
 *  Jobasha - TES3 leveled list merging and deleveling tool
 *
 *  Copyright (C) 2023 alvazir
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::{Context, Result};
use std::{process::exit, time::Instant};
mod config;
mod get_lists;
mod get_plugins;
mod output;
mod show_result;
mod util;
use config::{get_self_config, Cfg, OutputFile, PluginKind};
use get_lists::{get_lists, Creature, Item, LastCreature, LastItem, PluginName, ReadStats, ResponsiblePlugins, Subrecord};
use get_plugins::{get_plugins, PluginInfo};
use output::process_output;
use show_result::show_result;
use util::{
    create_dir_early, err_or_ignore, err_or_ignore_thread_safe, get_plugin_size, msg, plural, read_lines, show_log_path,
    show_settings_version_message, show_settings_written, ListCounts, Log, MsgTone, Progress,
};

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
    if cfg.settings_file.write {
        show_settings_written(&cfg, &mut log)?;
        return Ok(false);
    }
    show_settings_version_message(&cfg, &mut log)?;
    let plugins = get_plugins(&cfg, &mut log).with_context(|| "Failed to get plugins")?;
    let (creatures, items, record_read_stats) = get_lists(&plugins, &cfg).with_context(|| "Failed to get leveled lists")?;
    let (counts, warning) = process_output(creatures, items, &cfg, &mut log).with_context(|| "Failed to process leveled lists")?;
    show_result(timer, record_read_stats, counts, &cfg, &mut log)?;
    Ok(warning)
}
