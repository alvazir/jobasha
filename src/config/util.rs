use super::{DelevSkipPatterns, Options, OutputFile, PluginKind, Settings, SettingsFile, ShowConfiguration};
use crate::{get_delev_segment_ceil, read_lines};
use anyhow::{anyhow, Context, Result};
use chrono::Local;
use console::Style;
use fs_err::rename;
use std::{
    env::current_exe,
    fmt::Write as _,
    path::{Path, PathBuf},
};

pub(super) fn get_exe_name_and_dir() -> (Option<String>, Option<PathBuf>) {
    match current_exe() {
        Ok(path) => (
            path.file_stem().map(|exe| exe.to_string_lossy().into_owned()),
            path.parent().map(|dir| dir.to_owned()),
        ),
        Err(_) => (None, None),
    }
}

pub(super) fn get_settings_file(exe: &Option<String>, dir: &Option<PathBuf>, options: &Options) -> Result<SettingsFile> {
    let extension = "toml";
    let fallback_filename = "settings.toml";
    let name = &options.settings;
    let filename = match name {
        Some(name) => match Path::new(name).file_stem() {
            Some(filename) => format!("{}.{extension}", filename.to_string_lossy()),
            None => return Err(anyhow!("Failed to get settings filename without extension from \"{}\"", name)),
        },
        None => match exe {
            Some(file_stem) => format!("{file_stem}.{extension}"),
            None => {
                eprintln!("Failed to get program name: falling back to default name \"{fallback_filename}\" for settings");
                fallback_filename.into()
            }
        },
    };
    let settings_file_path = match name {
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
    if !options.settings_write && options.settings.is_some() && !settings_file_path.exists() {
        return Err(anyhow!("Settings file \"{}\" not found", settings_file_path.display()));
    }
    let settings_file = SettingsFile {
        path: settings_file_path,
        version_message: None,
        write: options.settings_write,
        backup_path: PathBuf::new(),
        backup_written: false,
        backup_overwritten: false,
    };
    Ok(settings_file)
}

pub(super) fn backup_settings_file(settings_file: &mut SettingsFile, backup_suffix: &str, no_backup: bool) -> Result<()> {
    if !no_backup && settings_file.path.exists() {
        let mut backup_path = settings_file.path.clone().into_os_string();
        backup_path.push(backup_suffix);
        settings_file.backup_path = backup_path.into();
        settings_file.backup_overwritten = settings_file.backup_path.exists();
        settings_file.backup_written = true;
        rename(&settings_file.path, &settings_file.backup_path).with_context(|| {
            format!(
                "Failed to rename previous program settings \"{}\" to \"{}\"",
                &settings_file.path.display(),
                &settings_file.backup_path.display()
            )
        })
    } else {
        Ok(())
    }
}

pub(super) fn get_log_file(no_log: bool, name: String, exe: Option<String>, dir: Option<PathBuf>) -> Result<Option<PathBuf>> {
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

pub(super) fn get_color(color: &str) -> Result<Style> {
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

pub(super) fn get_progress_frequency(frequency: u8) -> Result<u8> {
    match frequency {
        f if f > 0 && f <= 10 => Ok(frequency),
        _ => Err(anyhow!("Progress frequency must be between 1 and 10 Hz")),
    }
}

pub(super) fn get_interior_grid_change(level: u8, equal_to_the_last: u8) -> Result<u8> {
    match level {
        f if f <= equal_to_the_last => Ok(level),
        _ => Err(anyhow!("Setting \"guts.debug_level_merge_interior_grid_change\"({level}) should be less or equal to \"guts.debug_level_merge_skipped_equal_to_the_last\"({equal_to_the_last})")),
    }
}

pub(super) fn get_output_file(
    opt: &Options,
    set: &Settings,
    kind: PluginKind,
    compare_only_name: &str,
    show_configuration: &mut ShowConfiguration,
) -> Result<OutputFile> {
    macro_rules! name_parse_error {
        ($name:ident, $part:expr) => {
            return Err(anyhow!("Failed to parse {} from output plugin name: \"{}\"", $part, $name,))
        };
    }
    let (opt_output, set_options_output, option_name) = match kind {
        PluginKind::Merge => (&opt.output, &set.options.output, "output"),
        PluginKind::Delev => (&opt.delev_output, &set.options.delev_output, "delev_output"),
    };
    let mut raw_path = match opt_output {
        Some(name) => {
            show_configuration.add_some(true, option_name, format_args!("{:?}", &name))?;
            name
        }
        None => {
            if (matches!(kind, PluginKind::Merge) && set_options_output != "MergedLeveledLists.esp")
                || (matches!(kind, PluginKind::Delev) && !set_options_output.is_empty())
            {
                show_configuration.add_some(false, option_name, format_args!("{:?}", &set_options_output))?;
            }
            set_options_output
        }
    };
    let mut path = PathBuf::from(&raw_path);
    if raw_path.is_empty() && matches!(kind, PluginKind::Delev) {
        raw_path = match &opt.output {
            Some(name) => name,
            None => &set.options.output,
        };
        path = PathBuf::from(&raw_path);
        let stem = match path.file_stem() {
            Some(stem) => stem.to_string_lossy(),
            None => name_parse_error!(raw_path, "file name without extension"),
        };
        path.set_file_name(format!(
            "{}{}{}",
            stem, &set.guts.output_date_separators[0], &set.guts.delev_output_infix_default
        ));
    };
    let dir_path = match &opt.output_dir {
        Some(path) => {
            if matches!(kind, PluginKind::Merge) {
                show_configuration.add_some(true, "output_dir", format_args!("{:?}", &path))?;
            }
            PathBuf::from(&path)
        }
        None => match &set.options.output_dir.is_empty() {
            false => {
                if matches!(kind, PluginKind::Merge) {
                    show_configuration.add_some(false, "output_dir", format_args!("{:?}", &set.options.output_dir))?;
                }
                PathBuf::from(&set.options.output_dir)
            }
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
    let date = match opt.date {
        true => opt.date,
        false => set.options.date,
    };
    let mut name: String;
    if date {
        let separator_default = &set.guts.output_date_separators[0];
        let date_infix = format!("{}{}", separator_default, Local::now().format(&set.guts.output_date_format));
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
    let backup_path = dir_path.join(format!("{name}{}", &set.guts.output_backup_suffix));
    if !compare_only_name.is_empty() {
        name = compare_only_name.to_owned();
    }
    Ok(OutputFile {
        kind,
        name,
        name_lowercased_starts_with,
        path,
        dir_path,
        backup_path,
    })
}

pub(super) fn get_delev_to(lvl: u16) -> Result<u16> {
    match lvl {
        0 => Err(anyhow!("Level to delevel to should be larger than 0")),
        _ => Ok(lvl),
    }
}

pub(super) fn get_kind_delev_to(lvl: u16, kind_lvl: u16) -> u16 {
    if kind_lvl != 0 {
        kind_lvl
    } else {
        lvl
    }
}

pub(super) fn get_kind_delev_segment(
    kind_str: &str,
    delev_to: u16,
    delev_segment_ratio: f64,
    lvl: u16,
    kind_lvl: u16,
) -> Result<(u16, u16)> {
    let segment = if kind_lvl != 0 { kind_lvl } else { lvl };
    let mut ceil = 0;
    if segment != 0 {
        if segment < delev_to {
            return Err(anyhow!(
                "{kind_str} delev_segment({segment}) should be larger or equal to the corresponding level to delevel to({delev_to})"
            ));
        } else {
            ceil = get_delev_segment_ceil(&segment, segment, delev_to, delev_segment_ratio);
        }
    }
    Ok((segment, ceil))
}

pub(super) fn get_fogbug_fixed_value(fog: f32) -> Result<f32> {
    if fog <= 0.0 {
        Err(anyhow!("Fog value to fix fogbug should be larger than 0"))
    } else {
        Ok(fog)
    }
}

pub(super) fn check_verboseness(level: u8, name: &str) -> Result<u8> {
    let limit = 10;
    match level {
        f if f <= limit => Ok(level),
        _ => {
            if let Some(suffix) = name.strip_prefix("opt.") {
                Err(anyhow!("Option \"{suffix}\" should be passed no more than {limit} times"))
            } else {
                Err(anyhow!("Option \"{}\"[{level}] should be less than or equal to {limit}", name))
            }
        }
    }
}

pub(super) fn prepare_plugin_extensions_to_ignore(list: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    for extension in list.iter() {
        let mut prepared = extension.to_lowercase();
        prepared.insert(0, '.');
        res.push(prepared)
    }
    res
}

pub(super) fn append_default_to_skip(mut skip: Vec<String>, default: &[String]) -> Vec<String> {
    skip.extend(default.iter().map(|x| x.to_lowercase()));
    skip
}

pub(super) fn check_settings_version(settings_file: &mut SettingsFile) -> Result<()> {
    if settings_file.path.exists() {
        let expected_settings_version = String::from("0.6.0");
        let mut detected_settings_version = String::new();
        macro_rules! find_version {
            ($prefix:expr) => {
                for line in read_lines(&settings_file.path)
                    .with_context(|| {
                        format!(
                            "Failed to read program configuration file \"{}\"",
                            &settings_file.path.display()
                        )
                    })?
                    .map_while(Result::ok)
                {
                    if line.starts_with($prefix) {
                        let version_raw = &line.strip_prefix($prefix);
                        if let Some(version_raw) = version_raw {
                            detected_settings_version = version_raw.trim().replace('"', "").to_owned();
                            break;
                        }
                    }
                }
            };
        }
        find_version!("#settings_version = ");
        if detected_settings_version.is_empty() {
            find_version!("# # Settings version: ");
        }
        if detected_settings_version != expected_settings_version {
            settings_file.version_message =  Some(
                format!("Attention: Program configuration file \"{}\" version differs from expected:\n  Expected version = \"{}\", detected version = \"{}\".\n  Consider recreating it with \"--settings-write\".\n  File will be backed up and then overwritten, though better make backup yourself if you need it.", &settings_file.path.display(), expected_settings_version, detected_settings_version),
            );
        }
    }
    Ok(())
}

pub(super) fn prepare_delev_skip_patterns(raw_patterns: Vec<String>) -> DelevSkipPatterns {
    let (mut exact, mut prefix, mut infix, mut suffix) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let is_empty = raw_patterns.is_empty();
    if !is_empty {
        for pattern in raw_patterns {
            if let Some(remainder) = pattern.strip_prefix("prefix:") {
                prefix.push(remainder.to_owned());
            } else if let Some(remainder) = pattern.strip_prefix("infix:") {
                infix.push(remainder.to_owned());
            } else if let Some(remainder) = pattern.strip_prefix("suffix:") {
                suffix.push(remainder.to_owned());
            } else {
                exact.push(pattern);
            }
        }
    }
    DelevSkipPatterns {
        is_empty,
        exact,
        prefix,
        infix,
        suffix,
    }
}

pub(super) fn get_compare_only(compare_only: &Option<String>, show_configuration: &mut ShowConfiguration) -> Result<(bool, String)> {
    match compare_only {
        None => Ok((false, String::new())),
        Some(value) => {
            show_configuration.add_some(true, "compare_only", format_args!("{:?}", value))?;
            Ok((true, value.to_owned()))
        }
    }
}

pub(super) fn show_configuration_add_header(opt: bool, string: &mut String) -> Result<()> {
    if string.is_empty() {
        let text = match opt {
            true => "Options:",
            false => "Settings:",
        };
        write!(string, "  {text}",)?;
    }
    Ok(())
}
