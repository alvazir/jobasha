use crate::create_dir_early;
use anyhow::{anyhow, Context, Result};
use console::Style;
use std::{ffi::OsString, fs, path::PathBuf};
mod options;
mod settings;
mod util;
use confique::toml::{template, FormatOptions};
use options::{get_options, Options};
use settings::{get_settings, Settings};
use util::{
    get_color, get_exe_name_and_dir, get_log_file, get_output_file, get_progress_frequency,
    get_settings_file,
};

pub(crate) struct Cfg {
    pub(crate) config: String,
    pub(crate) output: OutputFile,
    pub(crate) dry_run: bool,
    pub(crate) log: Option<PathBuf>,
    pub(crate) no_log: bool,
    pub(crate) settings: PathBuf,
    pub(crate) settings_write: bool,
    pub(crate) ignore_errors: bool,
    pub(crate) all_lists: bool,
    pub(crate) skip_last: usize,
    pub(crate) skip: Vec<String>,
    pub(crate) creatures: Kind,
    pub(crate) items: Kind,
    pub(crate) no_delete: bool,
    pub(crate) extended_delete: bool,
    pub(crate) always_delete: Vec<String>,
    pub(crate) never_delete: Vec<String>,
    pub(crate) no_threshold_warnings: bool,
    pub(crate) verbose: u8,
    pub(crate) quiet: bool,
    pub(crate) no_color: bool,
    pub(crate) no_progress: bool,
    pub(crate) no_progress_bar: bool,
    pub(crate) no_summary: bool,
    pub(crate) guts: Guts,
}

pub(crate) struct Kind {
    pub(crate) no: bool,
    pub(crate) threshold: f64,
    pub(crate) log_t: String,
}

pub(crate) struct OutputFile {
    pub(crate) name: String,
    pub(crate) name_lowercased_starts_with: String,
    pub(crate) path: PathBuf,
    pub(crate) dir_path: PathBuf,
}

pub(crate) struct Guts {
    pub(crate) color_suggestion: Style,
    pub(crate) color_success: Style,
    pub(crate) color_warning: Style,
    pub(crate) color_ignored_error: Style,
    pub(crate) config_path_suffix_linux_macos: String,
    pub(crate) config_path_suffix_windows: String,
    pub(crate) config_paths_list: Vec<String>,
    pub(crate) mor_line_beginning_content: String,
    pub(crate) mor_data_files_dir: String,
    pub(crate) omw_line_beginning_content: String,
    pub(crate) omw_line_beginning_data: String,
    pub(crate) omw_plugin_extensions: Vec<OsString>,
    pub(crate) header_version: f32,
    pub(crate) header_author: String,
    pub(crate) header_description: String,
    pub(crate) progress_frequency: u8,
    pub(crate) progress_prefix: String,
    pub(crate) progress_template: String,
    pub(crate) progress_bar_template: String,
    pub(crate) progress_bar_chars: String,
    pub(crate) auto_resolve_lower_limit: f64,
    pub(crate) prefix_ignored_error_message: String,
    pub(crate) suffix_add_ignore_errors_suggestion: String,
    pub(crate) suffix_add_v_suggestion: String,
    pub(crate) suffix_add_2v_suggestion: String,
    pub(crate) suffix_add_v_suggestion_no_log: String,
    pub(crate) suffix_add_2v_suggestion_no_log: String,
}

impl Cfg {
    fn new(
        opt: Options,
        set: Settings,
        settings_file: PathBuf,
        exe: Option<String>,
        dir: Option<PathBuf>,
    ) -> Result<Cfg> {
        macro_rules! opt_or_set_bool {
            ($name:ident) => {
                match opt.$name {
                    true => opt.$name,
                    false => set.options.$name,
                }
            };
        }
        macro_rules! opt_or_set_some {
            ($name:ident) => {
                match opt.$name {
                    Some(value) => value,
                    None => set.options.$name,
                }
            };
        }
        macro_rules! opt_or_set_vec_lowercase {
            ($name:ident) => {
                match opt.$name {
                    Some(value) => value.iter().map(|x| x.to_lowercase()).collect(),
                    None => set.options.$name.iter().map(|x| x.to_lowercase()).collect(),
                }
            };
        }
        macro_rules! opt_or_set_threshold {
            ($name_ident:ident, $name_string:expr) => {
                match opt.$name_ident {
                    Some(num) => num as f64,
                    None => match set.options.$name_ident <= 100 {
                        true => set.options.$name_ident as f64,
                        false => {
                            return Err(anyhow!(format!(
                                "Value of {} should be in range 0-100",
                                $name_string
                            )))
                        }
                    },
                }
            };
        }
        macro_rules! set_ext {
            ($name:expr) => {
                $name.iter().map(|ext| ext.to_lowercase().into()).collect()
            };
        }
        let no_log = opt_or_set_bool!(no_log);
        Ok(Cfg {
            output: get_output_file(&opt, &set)?,
            config: opt_or_set_some!(config),
            dry_run: opt_or_set_bool!(dry_run),
            no_log,
            log: get_log_file(no_log, opt_or_set_some!(log), exe, dir)?,
            settings: settings_file,
            settings_write: opt.settings_write,
            ignore_errors: opt_or_set_bool!(ignore_errors),
            all_lists: opt_or_set_bool!(all_lists),
            skip_last: opt_or_set_some!(skip_last),
            skip: opt_or_set_vec_lowercase!(skip),
            creatures: Kind {
                no: opt_or_set_bool!(no_creatures),
                threshold: opt_or_set_threshold!(threshold_creatures, "threshold_creatures"),
                log_t: set.guts.log_t_creature,
            },
            items: Kind {
                no: opt_or_set_bool!(no_items),
                threshold: opt_or_set_threshold!(threshold_items, "threshold_items"),
                log_t: set.guts.log_t_item,
            },
            no_delete: opt_or_set_bool!(no_delete),
            extended_delete: opt_or_set_bool!(extended_delete),
            always_delete: opt_or_set_vec_lowercase!(always_delete),
            never_delete: opt_or_set_vec_lowercase!(never_delete),
            no_threshold_warnings: opt_or_set_bool!(no_threshold_warnings),
            verbose: if opt.verbose == 0 {
                set.options.verbose
            } else {
                opt.verbose
            },
            quiet: opt_or_set_bool!(quiet),
            no_color: opt_or_set_bool!(no_color),
            no_progress: opt_or_set_bool!(no_progress),
            no_progress_bar: opt_or_set_bool!(no_progress_bar),
            no_summary: opt_or_set_bool!(no_summary),
            guts: Guts {
                color_suggestion: get_color(&set.guts.color_suggestion)?,
                color_success: get_color(&set.guts.color_success)?,
                color_warning: get_color(&set.guts.color_warning)?,
                color_ignored_error: get_color(&set.guts.color_ignored_error)?,
                config_path_suffix_linux_macos: set.guts.config_path_suffix_linux_macos,
                config_path_suffix_windows: set.guts.config_path_suffix_windows,
                config_paths_list: set.guts.config_paths_list,
                mor_line_beginning_content: set.guts.mor_line_beginning_content,
                mor_data_files_dir: set.guts.mor_data_files_dir,
                omw_line_beginning_content: set.guts.omw_line_beginning_content,
                omw_line_beginning_data: set.guts.omw_line_beginning_data,
                omw_plugin_extensions: set_ext!(set.guts.omw_plugin_extensions),
                header_version: set.guts.header_version,
                header_author: set.guts.header_author,
                header_description: set.guts.header_description,
                progress_frequency: get_progress_frequency(set.guts.progress_frequency)?,
                progress_prefix: set.guts.progress_prefix,
                progress_template: set.guts.progress_template,
                progress_bar_template: set.guts.progress_bar_template,
                progress_bar_chars: set.guts.progress_bar_chars,
                auto_resolve_lower_limit: set.guts.auto_resolve_lower_limit,
                prefix_ignored_error_message: set.guts.prefix_ignored_error_message,
                suffix_add_ignore_errors_suggestion: set.guts.suffix_add_ignore_errors_suggestion,
                suffix_add_v_suggestion: set.guts.suffix_add_v_suggestion,
                suffix_add_2v_suggestion: set.guts.suffix_add_2v_suggestion,
                suffix_add_v_suggestion_no_log: set.guts.suffix_add_v_suggestion_no_log,
                suffix_add_2v_suggestion_no_log: set.guts.suffix_add_2v_suggestion_no_log,
            },
        })
    }
}

pub(crate) fn get_self_config() -> Result<Cfg> {
    let options = get_options()?;
    let (exe, dir) = get_exe_name_and_dir();
    let settings_file = get_settings_file(&exe, &dir, &options.settings)
        .with_context(|| "Failed to get program settings file path")?;
    let settings = get_settings(&settings_file)
        .with_context(|| "Failed to get default or provided settings")?;
    if options.settings_write {
        let toml = template::<Settings>(FormatOptions::default());
        create_dir_early(&settings_file, "settings")?;
        fs::write(&settings_file, toml).with_context(|| {
            format!(
                "Failed to write default program settings into \"{}\"",
                settings_file.display()
            )
        })?;
    }
    let configuration = Cfg::new(options, settings, settings_file, exe, dir)
        .with_context(|| "Failed to configure program")?;
    Ok(configuration)
}
