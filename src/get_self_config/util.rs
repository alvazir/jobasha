use super::{Options, OutputFile, Settings};
use anyhow::{anyhow, Result};
use chrono::Local;
use console::Style;
use std::{
    env::current_exe,
    path::{Path, PathBuf},
};

pub(crate) fn get_exe_name_and_dir() -> (Option<String>, Option<PathBuf>) {
    match current_exe() {
        Ok(path) => (
            path.file_stem()
                .map(|exe| exe.to_string_lossy().into_owned()),
            path.parent().map(|dir| dir.to_owned()),
        ),
        Err(_) => (None, None),
    }
}

pub(crate) fn get_settings_file(
    exe: &Option<String>,
    dir: &Option<PathBuf>,
    name: &Option<String>,
) -> Result<PathBuf> {
    let extension = "toml";
    let fallback_filename = "settings.toml";
    let filename = match name {
        Some(name) => match Path::new(name).file_stem() {
            Some(filename) => format!("{}.{extension}", filename.to_string_lossy()),
            None => {
                return Err(anyhow!(
                    "Failed to get settings filename without extension from \"{}\"",
                    name
                ))
            }
        },
        None => match exe {
            Some(file_stem) => format!("{file_stem}.{extension}"),
            None => {
                eprintln!("Failed to get program name: falling back to default name \"{fallback_filename}\" for settings");
                fallback_filename.into()
            }
        },
    };
    let settings_file = match name {
        Some(name) => match Path::new(name).parent() {
            Some(path) => path.join(filename),
            None => PathBuf::from(&filename),
        },
        None => match dir {
            Some(path) => path.join(filename),
            None => {
                eprintln!("Failed to get program directory: falling back to checking \"{filename}\" in current directory");
                PathBuf::from(filename)
            }
        },
    };
    Ok(settings_file)
}

pub(crate) fn get_log_file(
    no_log: bool,
    name: String,
    exe: Option<String>,
    dir: Option<PathBuf>,
) -> Result<Option<PathBuf>> {
    if no_log {
        return Ok(None);
    }
    let extension = "log";
    let fallback_filename = "log.log";
    let filename = match name.is_empty() {
        false => match Path::new(&name).file_name() {
            Some(filename) => filename.to_string_lossy().into_owned(),
            None => return Err(anyhow!("Failed to get log file name \"{}\"", name)),
        },
        true => match exe {
            Some(file_stem) => format!("{file_stem}.{extension}"),
            None => {
                eprintln!("Failed to get program name: falling back to default name \"{fallback_filename}\" for log");
                fallback_filename.into()
            }
        },
    };
    let log = match name.is_empty() {
        false => match Path::new(&name).parent() {
            Some(path) => path.join(filename),
            None => PathBuf::from(&filename),
        },
        true => match dir {
            Some(path) => path.join(filename),
            None => {
                eprintln!("Failed to get program directory: falling back to writing log into \"{filename}\" in current directory");
                PathBuf::from(filename)
            }
        },
    };
    Ok(Some(log))
}

pub(crate) fn get_color(color: &str) -> Result<Style> {
    let style = match color {
        "blue" => Style::new().blue(),
        "cyan" => Style::new().cyan(),
        "green" => Style::new().green(),
        "magenta" => Style::new().magenta(),
        "red" => Style::new().red(),
        "yellow" => Style::new().yellow(),
        "none" => Style::new(),
        _ => return Err(anyhow!("Wrong color \"{color}\" defined")),
    };
    Ok(style)
}

pub(crate) fn get_progress_frequency(frequency: u8) -> Result<u8> {
    match frequency {
        f if f > 0 && f <= 10 => Ok(frequency),
        _ => Err(anyhow!("Progress frequency must be between 1 and 10 Hz")),
    }
}

pub(crate) fn get_output_file(opt: &Options, set: &Settings) -> Result<OutputFile> {
    macro_rules! name_parse_error {
        ($name:ident, $part:expr) => {
            return Err(anyhow!(
                "Failed to parse {} from output plugin name: \"{}\"",
                $part,
                $name,
            ))
        };
    }
    let raw_path = match &opt.output {
        Some(name) => name,
        None => &set.options.output,
    };
    let mut path = PathBuf::from(&raw_path);
    let dir_path = match &opt.output_dir {
        Some(path) => PathBuf::from(&path),
        None => match &set.options.output_dir.is_empty() {
            false => PathBuf::from(&set.options.output_dir),
            true => match path.parent() {
                Some(path) => PathBuf::from(path),
                None => name_parse_error!(raw_path, "directory path"),
            },
        },
    };
    if path.file_name().is_none() {
        name_parse_error!(raw_path, "file name");
    };
    let stem = match path.file_stem() {
        Some(stem) => stem.to_string_lossy(),
        None => name_parse_error!(raw_path, "file name without extension"),
    };
    let extension = match path.extension() {
        Some(extension) => extension.to_string_lossy().into_owned(),
        None => set.guts.output_extension_default.clone(),
    };
    let mut name_lowercased_starts_with = String::new();
    let no_date = match opt.no_date {
        true => opt.no_date,
        false => set.options.no_date,
    };
    let name: String;
    if !no_date {
        let separator_default = &set.guts.output_date_separators[0];
        let date_infix = format!(
            "{}{}",
            separator_default,
            Local::now().format(&set.guts.output_date_format)
        );
        name_lowercased_starts_with = format!("{}{}", stem, separator_default).to_lowercase();
        name = format!("{}{}.{}", stem, date_infix, extension);
        path = dir_path.join(&name);
    } else {
        for separator in &set.guts.output_date_separators {
            let prefix = stem.split(separator).next().unwrap();
            if stem != prefix {
                name_lowercased_starts_with = format!("{}{}", prefix, separator).to_lowercase();
                break;
            }
            name_lowercased_starts_with = stem.to_lowercase();
        }
        name = format!("{}.{}", stem, extension);
        path = dir_path.join(&name);
    };
    Ok(OutputFile {
        name,
        name_lowercased_starts_with,
        path,
        dir_path,
    })
}
