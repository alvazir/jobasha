/*
 *  Jobasha - TES3 leveled list tool
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
mod get_plugins;
mod input;
mod leveled_lists;
mod merge;
mod output;
mod show_result;
mod util;
use config::{get_self_config, Cfg, DelevSkipPatterns, ListKind, OutputFile, PluginKind};
use get_plugins::{get_plugins, get_plugins_to_compare, PluginInfo};
use input::{
    get_records,
    merge::{
        BirthsignRecordMap, CellKey, CellRecordMap, ContainerRecordMap, CreatureRecordMap, IntermediateRecords, Merge, NpcRecordMap,
        RaceRecordMap, RecordMap,
    },
    structs::{InputHelper, PluginName, ReadStats, ResponsiblePlugins},
};
use leveled_lists::{
    messages::LlMessages,
    records::{LlCreatureRecords, LlElement, LlItemRecords},
};
use merge::{merge_records, RawPlugins};
use output::process_output;
use show_result::show_result;
use util::{
    append_for_details_or_check_log, create_dir_early, err_or_ignore, err_or_ignore_thread_safe, get_delev_segment_ceil,
    get_plugin_size, msg, msg_thread_safe, plural, read_lines, ComparePlugin, ComparePlugins, ListCounts, Log, MsgTone, Progress,
};

// use peak_alloc::PeakAlloc; // slows down the program too much
// #[global_allocator]
// static PEAK_ALLOC: PeakAlloc = PeakAlloc; // slows down the program too much

fn main() {
    match run() {
        // 0: Ok / Plugins are the same in --compare-only mode
        // 1: Error
        // 2: Some leveled lists should probably be excluded from subrecords deletion mode
        // 3: Plugins are different in --compare-only mode
        Ok(exit_code) => {
            // println!("PEAK MEMORY USAGE: {:.0}MB", PEAK_ALLOC.peak_usage_as_mb()); // slows down the program too much
            exit(exit_code)
        }
        Err(error) => {
            eprintln!("{error:?}");
            exit(1);
        }
    }
}

fn run() -> Result<i32> {
    let timer = Instant::now();
    let cfg = get_self_config()?;
    let mut log = Log::new(&cfg)?;
    cfg.show_log_path(&mut log)?;
    if cfg.settings_file.write {
        cfg.show_settings_written(&mut log)?;
        return Ok(0);
    }
    cfg.show_settings_version_message(&mut log)?;
    cfg.show_configuration(&mut log)?;
    // cfg.show_merge_types(&mut log)?;
    let plugins_to_compare = get_plugins_to_compare(&cfg, &mut log).with_context(|| "Failed to get plugins for comparison")?;
    let plugins = get_plugins(&cfg, &mut log).with_context(|| "Failed to get plugins")?;
    let (ll_creatures, ll_items, intermediate_records, record_read_stats) =
        get_records(&plugins, &cfg, &mut log).with_context(|| "Failed to get records")?;
    let (raw_plugins, mut counts, mut exit_code) =
        merge_records(ll_creatures, ll_items, intermediate_records, &cfg, &mut log).with_context(|| "Failed to merge records")?;
    process_output(plugins_to_compare, raw_plugins, &mut counts, &mut exit_code, &cfg, &mut log)
        .with_context(|| "Failed to process output")?;
    show_result(timer, record_read_stats, counts, &cfg, &mut log)?;
    cfg.press_enter_to_exit(&mut log)?;
    Ok(exit_code)
}
