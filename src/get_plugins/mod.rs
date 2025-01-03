use crate::{err_or_ignore, err_or_ignore_thread_safe, msg, read_lines, Cfg, Log, MsgTone};
use anyhow::{anyhow, Context, Result};
use dirs::{data_dir, document_dir};
use fs_err::read_dir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Write as _,
    path::{Path, PathBuf},
};
mod get_game_config;
use get_game_config::get_game_config;
mod get_plugins_to_compare;
pub(crate) use get_plugins_to_compare::get_plugins_to_compare;

#[derive(Default, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) struct PluginInfo {
    pub(crate) name: String,
    pub(crate) name_lowercased: String,
    pub(crate) path: PathBuf,
}

#[derive(Default)]
struct Helper {
    mor_found: bool,
    omw_found: bool,
    omw_data_ended: bool,
    omw_data_counter: usize,
    mor_data_files_dir: PathBuf,
    mor_data_files_dir_found: bool,
    omw_all_plugins_found: bool,
    // Make plugins defined in both plugins_skip and plugins_skip_last be skipped once
    preskipped_plugins_number: usize,
    skipped_plugin_numbers: Vec<usize>,
    skip_default_reasons_processed: bool,
    skip_default_reasons: HashMap<String, String>,
}

pub(super) fn get_plugins(cfg: &Cfg, log: &mut Log) -> Result<Vec<PluginInfo>> {
    let mut res: Vec<PluginInfo> = Vec::new();
    if cfg.compare_only {
        return Ok(res);
    }
    let config_path = get_game_config(cfg, log).with_context(|| "Failed to get game configuration file")?;
    let text = format!("Gathering plugins from game configuration file \"{}\"", &config_path.display());
    msg(text, MsgTone::Neutral, 1, cfg, log)?;
    let config_lines =
        read_lines(&config_path).with_context(|| format!("Failed to read game configuration file \"{}\"", &config_path.display()))?;
    let mut helper: Helper = Helper::default();
    let mut omw_data_dirs: Vec<(usize, PathBuf)> = Vec::new();
    let mut omw_all_plugins: HashMap<String, PathBuf> = HashMap::new();
    for line in config_lines.map_while(Result::ok) {
        if !helper.omw_found && line.starts_with(&cfg.guts.mor_line_beginning_content) {
            if !helper.mor_data_files_dir_found {
                mor_get_data_files_dir(&config_path, &mut helper, cfg)
                    .with_context(|| "Failed to find Morrowind's \"Data Files\" directory")?;
            }
            mor_get_plugin(&line, &mut res, &mut helper, cfg, log).with_context(|| "Failed to find Morrowind's plugin")?;
        }
        if !helper.mor_found {
            if !helper.omw_data_ended && line.starts_with(&cfg.guts.omw_line_beginning_data) {
                omw_get_data_dir(&line, &mut omw_data_dirs, &mut helper, cfg, log)
                    .with_context(|| "Failed to get OpenMW's data directory")?;
            }
            if line.starts_with(&cfg.guts.omw_line_beginning_content) {
                if !helper.omw_all_plugins_found {
                    omw_get_cs_data_dir(&mut omw_data_dirs, &mut helper, cfg, log)
                        .with_context(|| "Failed to find \"hidden\" OpenMW-CS data directory path")?;
                    omw_all_plugins =
                        get_all_plugins(&omw_data_dirs, &mut helper, cfg).with_context(|| "Failed to find all OpenMW's plugins")?;
                };
                omw_get_plugin(&line, &mut res, &omw_all_plugins, &mut helper, cfg, log)
                    .with_context(|| "Failed to find OpenMW's plugin")?;
            }
        }
    }
    if res.is_empty() {
        error_none_listed(&config_path)?;
    }
    if cfg.skip_last > 0 {
        skip_last_plugins(&mut res, &helper, cfg, log).with_context(|| format!("Failed to skip last {} plugins", cfg.skip_last))?;
    }
    if res.is_empty() {
        error_none_listed(&config_path)
    } else {
        Ok(res)
    }
}

fn get_all_plugins(omw_data_dirs: &[(usize, PathBuf)], helper: &mut Helper, cfg: &Cfg) -> Result<HashMap<String, PathBuf>> {
    let mut found_plugins: Vec<(usize, String, PathBuf)> = omw_data_dirs
        .par_iter()
        .map(|(id, dir_path)| -> Result<Vec<(usize, String, PathBuf)>, _> {
            let mut res: Vec<(usize, String, PathBuf)> = Vec::new();
            match read_dir(dir_path) {
                Ok(dir_contents) => {
                    for entry in dir_contents.flatten() {
                        let path = entry.path();
                        if let Some(plugin_extension) = path.extension() {
                            if cfg.guts.omw_plugin_extensions.contains(&plugin_extension.to_ascii_lowercase()) {
                                res.push((*id, entry.file_name().to_string_lossy().into_owned(), path));
                            }
                        }
                    }
                }
                Err(error) => {
                    let text = format!("Failed to open directory \"{}\" with error \"{}\"", dir_path.display(), error);
                    err_or_ignore_thread_safe(text, cfg)?;
                }
            }

            Ok(res)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|vec| !vec.is_empty())
        .flatten()
        .collect();
    found_plugins.sort();
    let mut all_plugins: HashMap<String, PathBuf> = HashMap::new();
    found_plugins.into_iter().rev().for_each(|(_, plugin, path)| {
        if let Entry::Vacant(v) = all_plugins.entry(plugin) {
            v.insert(path);
        }
    });
    if !helper.omw_all_plugins_found {
        helper.omw_all_plugins_found = true;
    }
    Ok(all_plugins)
}

fn mor_get_data_files_dir(config_path: &Path, helper: &mut Helper, cfg: &Cfg) -> Result<()> {
    helper.mor_data_files_dir = match config_path.canonicalize()?.parent() {
        Some(path) => Path::new(path).join(&cfg.guts.mor_data_files_dir),
        None => {
            return Err(anyhow!(
                "Failed to find Morrowind's \"Data Files\" directory at expected location \"{}\"",
                &cfg.guts.mor_data_files_dir
            ))
        }
    };
    if !helper.mor_data_files_dir_found {
        helper.mor_data_files_dir_found = true;
    }
    Ok(())
}

fn mor_get_plugin(line: &str, res: &mut Vec<PluginInfo>, helper: &mut Helper, cfg: &Cfg, log: &mut Log) -> Result<()> {
    if let Some(raw_name) = line.split('=').nth(1) {
        if let Some((name, name_lowercased)) = skip_filtered_plugins(raw_name, helper, cfg, log)? {
            let path = helper.mor_data_files_dir.join(&name);
            if path.exists() {
                res.push(PluginInfo {
                    name,
                    name_lowercased,
                    path,
                });
            } else {
                let text = format!(
                    "Plugin \"{}\" not found at expected location \"{}\"",
                    name,
                    helper.mor_data_files_dir.join(&name).display()
                );
                err_or_ignore(text, cfg, log)?;
            }
        }
    } else {
        let text = format!("Failed to parse line \"{line}\"");
        err_or_ignore(text, cfg, log)?;
    }
    if !helper.mor_found {
        helper.mor_found = true;
    }
    Ok(())
}

fn omw_get_data_dir(
    line: &str,
    omw_data_dirs: &mut Vec<(usize, PathBuf)>,
    helper: &mut Helper,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if let Some(raw_data) = line.split('=').nth(1) {
        let data = PathBuf::from(&raw_data[1..raw_data.len() - 1]);
        omw_data_dirs.push((helper.omw_data_counter, data));
        helper.omw_data_counter += 1;
    } else {
        let text = format!("Failed to parse line \"{line}\"");
        err_or_ignore(text, cfg, log)?;
    }
    if !helper.omw_found {
        helper.omw_found = true;
    }
    Ok(())
}

fn omw_get_plugin(
    line: &str,
    res: &mut Vec<PluginInfo>,
    omw_all_plugins: &HashMap<String, PathBuf>,
    helper: &mut Helper,
    cfg: &Cfg,
    log: &mut Log,
) -> Result<()> {
    if let Some(raw_name) = line.split('=').nth(1) {
        if let Some((name, name_lowercased)) = skip_filtered_plugins(raw_name, helper, cfg, log)? {
            if !cfg
                .guts
                .plugin_extensions_to_ignore
                .iter()
                .any(|ext| name_lowercased.ends_with(ext))
            {
                if let Some(path) = omw_all_plugins.get(&name) {
                    res.push(PluginInfo {
                        name,
                        name_lowercased,
                        path: path.clone(),
                    });
                } else {
                    let text = format!("Failed to find plugin \"{name}\"");
                    err_or_ignore(text, cfg, log)?;
                }
            }
        }
    }
    if !helper.omw_data_ended {
        helper.omw_data_ended = true;
    }
    Ok(())
}

fn skip_filtered_plugins(raw_name: &str, helper: &mut Helper, cfg: &Cfg, log: &mut Log) -> Result<Option<(String, String)>> {
    let name = raw_name.trim().to_owned();
    let name_lowercased = name.to_lowercase();
    helper.preskipped_plugins_number += 1;
    let text = if name == cfg.output.name {
        format!(
            "Plugin \"{}\" will be skipped, because it has the same name as the output plugin",
            name
        )
    } else if name_lowercased.starts_with(&cfg.output.name_lowercased_starts_with) {
        format!(
            "Plugin \"{}\" will be skipped, because it's name matches output plugin name pattern \"{}\"",
            name, cfg.output.name_lowercased_starts_with
        )
    } else if cfg.delev && cfg.delev_distinct && name == cfg.delev_output.name {
        format!(
            "Plugin \"{}\" will be skipped, because it has the same name as the delev output plugin",
            name
        )
    } else if cfg.delev && cfg.delev_distinct && name_lowercased.starts_with(&cfg.delev_output.name_lowercased_starts_with) {
        format!(
            "Plugin \"{}\" will be skipped, because it's name matches delev output plugin name pattern \"{}\"",
            name, cfg.output.name_lowercased_starts_with
        )
    } else if cfg.skip.contains(&name_lowercased) {
        let mut reason = format!("Plugin \"{}\" will be skipped, because it's listed as a plugin to skip", name);
        // Message should not be displayed when "--all-lists" is specified
        if !cfg.all_lists {
            if !helper.skip_default_reasons_processed {
                process_skip_default_reasons(helper, cfg);
            }
            if let Some(reason_tail) = helper.skip_default_reasons.get(&name_lowercased) {
                write!(reason, "{reason_tail}")?;
            }
        }
        reason
    } else {
        String::new()
    };
    if text.is_empty() {
        Ok(Some((name, name_lowercased)))
    } else {
        helper.skipped_plugin_numbers.push(helper.preskipped_plugins_number);
        msg(&text, MsgTone::Neutral, 0, cfg, log)?;
        Ok(None)
    }
}

fn process_skip_default_reasons(helper: &mut Helper, cfg: &Cfg) {
    let separator = ":\n  ";
    for reason in &cfg.guts.skip_default_reasons {
        if reason.len() > 1 {
            helper
                .skip_default_reasons
                .insert(reason[0].to_lowercase(), format!("{separator}{}", reason[1..].join(separator)));
        }
    }
    helper.skip_default_reasons_processed = true;
}

fn skip_last_plugins(res: &mut Vec<PluginInfo>, helper: &Helper, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let res_len = res.len();
    if res_len > cfg.skip_last {
        // Make plugins defined in both plugins_skip and plugins_skip_last be skipped once
        let skip_last_border = helper.preskipped_plugins_number - cfg.skip_last;
        let mut skip_last_modifier = 0;
        let mut already_skipped_numbers = Vec::new();
        for number in helper.skipped_plugin_numbers.iter() {
            if number > &skip_last_border {
                skip_last_modifier += 1;
                already_skipped_numbers.push(helper.preskipped_plugins_number - number);
            }
        }
        let mut number_shift = 0;
        for (number, plugin) in res.drain(res_len + skip_last_modifier - cfg.skip_last..).rev().enumerate() {
            // Make plugins defined in both plugins_skip and plugins_skip_last be skipped once
            let mut check_number = number + number_shift;
            loop {
                if already_skipped_numbers.contains(&check_number) {
                    already_skipped_numbers.retain(|item| item != &check_number);
                    number_shift += 1;
                    check_number += 1;
                } else {
                    break;
                }
            }
            let text = format!(
                "Plugin \"{}\" will be skipped, because it's number {} from the end",
                plugin.name,
                number + number_shift + 1
            );
            msg(&text, MsgTone::Neutral, 0, cfg, log)?;
        }
        Ok(())
    } else {
        Err(anyhow!(
            "There are only {} plugins, --skip-last {} is too large",
            res_len,
            cfg.skip_last
        ))
    }
}

fn error_none_listed(config_path: &Path) -> Result<Vec<PluginInfo>> {
    Err(anyhow!(
        "None plugins listed in game configuration file: \"{}\"",
        config_path.display()
    ))
}

fn omw_get_cs_data_dir(omw_data_dirs: &mut Vec<(usize, PathBuf)>, helper: &mut Helper, cfg: &Cfg, log: &mut Log) -> Result<()> {
    let mut checked_paths: Vec<PathBuf> = Vec::new();
    macro_rules! check_omw_cs_data_path {
        ($omw_cs_data_path:expr) => {
            if $omw_cs_data_path.exists() {
                omw_data_dirs.push((helper.omw_data_counter, $omw_cs_data_path));
                helper.omw_data_counter += 1;
                let text = format!(
                    "Added \"hidden\" OpenMW-CS data path \"{}\" to the list of directories",
                    $omw_cs_data_path.display()
                );
                return msg(text, MsgTone::Neutral, 0, cfg, log);
            } else {
                checked_paths.push($omw_cs_data_path);
            }
        };
    }
    if let Some(dir) = data_dir() {
        check_omw_cs_data_path!(dir.join(&cfg.guts.omw_cs_data_path_suffix_linux_macos));
    } else {
        checked_paths.push(PathBuf::from(format!(
            "Failed to get __data_dir__ to check \"__data_dir__/{}\"",
            &cfg.guts.omw_cs_data_path_suffix_linux_macos
        )));
    };
    if let Some(dir) = document_dir() {
        check_omw_cs_data_path!(dir.join(&cfg.guts.omw_cs_data_path_suffix_windows));
    } else {
        checked_paths.push(PathBuf::from(format!(
            "Failed to get __document_dir__ to check \"__document_dir__/{}\"",
            &cfg.guts.omw_cs_data_path_suffix_windows
        )));
    };
    for path in &cfg.guts.omw_cs_data_paths_list {
        check_omw_cs_data_path!(PathBuf::from(path));
    }
    let text = format!(
        "Failed to find \"hidden\" OpenMW-CS data path. Probably none exists. Checked following paths:\n{}",
        checked_paths
            .iter()
            .map(|path| format!("\t{}", path.display()))
            .collect::<Vec<String>>()
            .join("\n")
    );
    msg(text, MsgTone::Neutral, 1, cfg, log)
}
