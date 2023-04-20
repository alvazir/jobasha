use crate::{msg, Cfg, Log, MsgTone};
use anyhow::{anyhow, Context, Result};
use dirs::{document_dir, preference_dir};
use std::path::PathBuf;

pub(crate) fn get_game_config(cfg: &Cfg, log: &mut Log) -> Result<PathBuf> {
    match cfg.config.is_empty() {
        true => find_config(cfg, log).with_context(|| "Failed to find game configuration file"),
        false => check_config(cfg).with_context(|| "Failed to read game configuration file"),
    }
}

fn find_config(cfg: &Cfg, log: &mut Log) -> Result<PathBuf> {
    let mut checked_paths: Vec<PathBuf> = Vec::new();

    macro_rules! check_config_path {
        ($config_path:expr) => {
            if $config_path.exists() {
                let text = format!(
                    "Found game configuration file \"{}\"",
                    $config_path.display()
                );
                msg(text, MsgTone::Good, 0, cfg, log)?;
                return Ok($config_path);
            } else {
                checked_paths.push($config_path);
            }
        };
    }
    if let Some(dir) = preference_dir() {
        check_config_path!(dir.join(&cfg.guts.config_path_suffix_linux_macos));
    } else {
        checked_paths.push(PathBuf::from(format!(
            "Failed to get __preference_dir__ to check \"__preference_dir__/{}\"",
            &cfg.guts.config_path_suffix_linux_macos
        )));
    };
    if let Some(dir) = document_dir() {
        check_config_path!(dir.join(&cfg.guts.config_path_suffix_windows));
    } else {
        checked_paths.push(PathBuf::from(format!(
            "Failed to get __document_dir__ to check \"__document_dir__/{}\"",
            &cfg.guts.config_path_suffix_windows
        )));
    };
    for path in &cfg.guts.config_paths_list {
        check_config_path!(PathBuf::new().join(path));
    }
    Err(anyhow!(
        "Failed to find game configuration file. Consider using --config-file option. Checked following paths:\n{}",
        checked_paths
            .iter()
            .map(|path| format!("\t{}", path.display()))
            .collect::<Vec<String>>()
            .join("\n")
    ))
}

fn check_config(cfg: &Cfg) -> Result<PathBuf> {
    let config_path = PathBuf::from(&cfg.config);
    match config_path != PathBuf::new() && config_path.exists() {
        true => Ok(config_path),
        false => Err(anyhow!(
            "Failed to find game configuration file at path \"{}\". Consider using --config-file option.",
            cfg.config
        )),
    }
}
