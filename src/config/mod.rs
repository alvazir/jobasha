use crate::{create_dir_early, msg, Log, MsgTone};
use anyhow::{anyhow, Context, Result};
use confique::toml::{template, FormatOptions};
use console::Style;
use fs_err::write;
use std::{
    ffi::OsString,
    fmt::{Arguments, Write as _},
    path::PathBuf,
};
mod options;
mod settings;
mod util;
use options::{get_options, Options};
use settings::{get_settings, Settings};
use util::{
    append_default_to_skip, backup_settings_file, check_settings_version, check_verboseness, get_color, get_compare_only,
    get_delev_to, get_exe_name_and_dir, get_kind_delev_segment, get_kind_delev_to, get_log_file, get_output_file,
    get_progress_frequency, get_settings_file, prepare_delev_skip_patterns, prepare_plugin_extensions_to_ignore,
    show_configuration_add_header,
};

pub(crate) struct Cfg {
    pub(crate) config: String,
    pub(crate) output: OutputFile,
    pub(crate) dry_run: bool,
    pub(crate) log: Option<PathBuf>,
    pub(crate) no_log: bool,
    pub(crate) settings_file: SettingsFile,
    pub(crate) no_backup: bool,
    pub(crate) ignore_errors: bool,
    pub(crate) all_lists: bool,
    pub(crate) skip_last: usize,
    pub(crate) skip: Vec<String>,
    pub(crate) skip_unexpected_tags: bool,
    pub(crate) no_skip_unexpected_tags_default: bool,
    pub(crate) creatures: ListKind,
    pub(crate) items: ListKind,
    pub(crate) no_delete: bool,
    pub(crate) extended_delete: bool,
    pub(crate) always_delete: Vec<String>,
    pub(crate) never_delete: Vec<String>,
    pub(crate) no_threshold_warnings: bool,
    pub(crate) delev: bool,
    pub(crate) delev_distinct: bool,
    pub(crate) delev_output: OutputFile,
    pub(crate) delev_random: bool,
    pub(crate) delev_segment_progressive: bool,
    pub(crate) delev_segment_ratio: f64,
    pub(crate) delev_skip_list: DelevSkipPatterns,
    pub(crate) delev_no_skip_list: DelevSkipPatterns,
    pub(crate) delev_skip_subrecord: DelevSkipPatterns,
    pub(crate) delev_no_skip_subrecord: DelevSkipPatterns,
    pub(crate) no_compare: bool,
    pub(crate) compare_only: bool,
    pub(crate) compare_only_name: String,
    pub(crate) compare_with: String,
    pub(crate) compare_delev_with: String,
    pub(crate) compare_common: bool,
    pub(crate) verbose: u8,
    pub(crate) quiet: bool,
    pub(crate) progress: bool,
    pub(crate) progress_bar: bool,
    pub(crate) color: bool,
    pub(crate) no_summary: bool,
    pub(crate) guts: Guts,
}

pub(crate) struct SettingsFile {
    pub(crate) path: PathBuf,
    pub(crate) version_message: Option<String>,
    pub(crate) write: bool,
    pub(crate) backup_path: PathBuf,
    pub(crate) backup_written: bool,
    pub(crate) backup_overwritten: bool,
}

pub(crate) struct DelevSkipPatterns {
    pub(crate) is_empty: bool,
    pub(crate) exact: Vec<String>,
    pub(crate) prefix: Vec<String>,
    pub(crate) infix: Vec<String>,
    pub(crate) suffix: Vec<String>,
}

pub(crate) struct ListKind {
    pub(crate) skip: bool,
    pub(crate) threshold: f64,
    pub(crate) log_t: String,
    pub(crate) skip_delev: bool,
    pub(crate) delev_to: u16,
    pub(crate) delev_segment: u16,
    pub(crate) delev_segment_ceil: u16,
}

pub(crate) enum PluginKind {
    Merge,
    Delev,
}

pub(crate) struct OutputFile {
    pub(crate) kind: PluginKind,
    pub(crate) name: String,
    pub(crate) name_lowercased_starts_with: String,
    pub(crate) path: PathBuf,
    pub(crate) dir_path: PathBuf,
    pub(crate) backup_path: PathBuf,
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
    pub(crate) plugin_extensions_to_ignore: Vec<String>,
    pub(crate) skip_default_reasons: Vec<Vec<String>>,
    pub(crate) skip_unexpected_tags_default: Vec<String>,
    pub(crate) omw_cs_data_path_suffix_linux_macos: String,
    pub(crate) omw_cs_data_path_suffix_windows: String,
    pub(crate) omw_cs_data_paths_list: Vec<String>,
    pub(crate) header_version: f32,
    pub(crate) header_author: String,
    pub(crate) header_description_merge: String,
    pub(crate) header_description_delev: String,
    pub(crate) header_description_merge_and_delev: String,
    pub(crate) log_backup_suffix: String,
    pub(crate) progress_frequency: u8,
    pub(crate) progress_prefix: String,
    pub(crate) progress_template: String,
    pub(crate) progress_bar_template: String,
    pub(crate) progress_bar_chars: String,
    pub(crate) auto_resolve_lower_limit: f64,
    pub(crate) prefix_ignored_error_message: String,
    pub(crate) suffix_add_ignore_errors_suggestion: String,
    pub(crate) details_line_approximate_length: usize,
    pub(crate) verboseness_details_deleted_subrecords: u8,
    pub(crate) verboseness_details_untouched_lists: u8,
    pub(crate) verboseness_details_threshold_resolved: u8,
    pub(crate) verboseness_details_threshold_skipped: u8,
    pub(crate) verboseness_details_threshold_warnings: u8,
    pub(crate) verboseness_details_deleveled_subrecords: u8,
    pub(crate) verboseness_details_compare_plugins: u8,
    pub(crate) verboseness_show_configuration: u8,
    pub(crate) compare_tab_l1: String,
    pub(crate) compare_tab_l2: String,
    pub(crate) compare_tab_l3: String,
    pub(crate) show_configuration: ShowConfiguration,
    pub(crate) long_message_string_inital_capacity: usize,
}

pub(crate) struct ShowConfiguration {
    pub(crate) cmd: String,
    pub(crate) opt: String,
    pub(crate) set: String,
}

impl ShowConfiguration {
    fn new(capacity: usize) -> Result<ShowConfiguration> {
        let cmd = if std::env::args_os().len() == 0 {
            String::new()
        } else {
            let mut cmd = String::with_capacity(capacity);
            write!(cmd, "  Command-line arguments:\n    ")?;
            for (position, arg) in std::env::args_os().enumerate() {
                if position == 0 {
                    write!(cmd, "{:?}", arg.to_string_lossy())?;
                } else {
                    write!(cmd, ",{:?}", arg.to_string_lossy())?;
                }
            }
            cmd
        };
        Ok(ShowConfiguration {
            cmd,
            opt: String::with_capacity(capacity),
            set: String::with_capacity(capacity),
        })
    }

    fn add_bool(&mut self, opt: bool, name: &'static str) -> Result<()> {
        let string = if opt { &mut self.opt } else { &mut self.set };
        show_configuration_add_header(opt, string)?;
        write!(string, "\n    {} = true", name)?;
        Ok(())
    }

    fn add_some(&mut self, opt: bool, name: &'static str, format_args: Arguments<'_>) -> Result<()> {
        let string = if opt { &mut self.opt } else { &mut self.set };
        show_configuration_add_header(opt, string)?;
        write!(string, "\n    {} = {format_args}", name)?;
        Ok(())
    }
}

impl Cfg {
    fn new(opt: Options, set: Settings, settings_file: SettingsFile, exe: Option<String>, dir: Option<PathBuf>) -> Result<Cfg> {
        let long_message_string_inital_capacity = set.guts.long_message_string_inital_capacity;
        let mut show_configuration = ShowConfiguration::new(long_message_string_inital_capacity)?;
        macro_rules! opt_or_set_bool {
            ($name:ident) => {
                match opt.$name {
                    true => {
                        show_configuration.add_bool(true, stringify!($name))?;
                        opt.$name
                    }
                    false => {
                        if set.options.$name {
                            show_configuration.add_bool(false, stringify!($name))?;
                        };
                        set.options.$name
                    }
                }
            };
        }
        macro_rules! opt_or_set_some {
            ($name:ident, $default:expr) => {
                match opt.$name {
                    Some(value) => {
                        show_configuration.add_some(true, stringify!($name), format_args!("{:?}", value))?;
                        value
                    }
                    None => {
                        if set.options.$name != $default {
                            show_configuration.add_some(false, stringify!($name), format_args!("{:?}", &set.options.$name))?;
                        }
                        set.options.$name
                    }
                }
            };
        }
        macro_rules! opt_or_set_vec_lowercase {
            ($name:ident, $default:expr) => {
                match opt.$name {
                    Some(value) => {
                        show_configuration.add_some(true, stringify!($name), format_args!("{:?}", value))?;
                        value.iter().map(|x| x.to_lowercase()).collect()
                    }
                    None => {
                        if &set.options.$name[..] != &$default[..] {
                            show_configuration.add_some(false, stringify!($name), format_args!("{:?}", &set.options.$name))?;
                        }
                        set.options.$name.iter().map(|x| x.to_lowercase()).collect()
                    }
                }
            };
        }
        macro_rules! opt_or_set_threshold {
            ($name_ident:ident, $name_string:expr, $default:expr) => {
                match opt.$name_ident {
                    Some(num) => {
                        show_configuration.add_some(true, stringify!($name_ident), format_args!("{:?}", num))?;
                        num as f64
                    }
                    None => {
                        if set.options.$name_ident != $default {
                            show_configuration.add_some(false, stringify!($name), format_args!("{:?}", &set.options.$name_ident))?;
                        }
                        match set.options.$name_ident <= 100 {
                            true => set.options.$name_ident as f64,
                            false => {
                                return Err(anyhow!(format!(
                                    "Value of {} should be in range 0-100",
                                    set.options.$name_ident
                                )))
                            }
                        }
                    }
                }
            };
        }
        macro_rules! set_ext {
            ($name:expr) => {
                $name.iter().map(|ext| ext.to_lowercase().into()).collect()
            };
        }
        macro_rules! get_verbose {
            ($name:expr) => {
                check_verboseness($name, stringify!($name))?
            };
        }
        let no_log = opt_or_set_bool!(no_log);
        let delev_to = get_delev_to(opt_or_set_some!(delev_to, 1))?;
        let delev_creatures_to = get_kind_delev_to(delev_to, opt_or_set_some!(delev_creatures_to, 0));
        let delev_items_to = get_kind_delev_to(delev_to, opt_or_set_some!(delev_items_to, 0));
        let delev_segment = opt_or_set_some!(delev_segment, 0);
        let delev_segment_ratio = opt_or_set_threshold!(delev_segment_ratio, "delev_segment_ratio", 50);
        let (creatures_delev_segment, creatures_delev_segment_ceil) = get_kind_delev_segment(
            "Creatures",
            delev_creatures_to,
            delev_segment_ratio,
            delev_segment,
            opt_or_set_some!(delev_creatures_segment, 0),
        )?;
        let (items_delev_segment, items_delev_segment_ceil) = get_kind_delev_segment(
            "Items",
            delev_items_to,
            delev_segment_ratio,
            delev_segment,
            opt_or_set_some!(delev_items_segment, 0),
        )?;
        let no_skip_default = opt_or_set_bool!(no_skip_default);
        let (compare_only, compare_only_name) = get_compare_only(&opt.compare_only, &mut show_configuration)?;
        Ok(Cfg {
            output: get_output_file(&opt, &set, PluginKind::Merge, &compare_only_name, &mut show_configuration)?,
            delev_output: get_output_file(&opt, &set, PluginKind::Delev, "", &mut show_configuration)?,
            config: opt_or_set_some!(config, ""),
            dry_run: opt_or_set_bool!(dry_run),
            no_log,
            log: get_log_file(no_log, opt_or_set_some!(log, ""), exe, dir)?,
            settings_file,
            no_backup: opt_or_set_bool!(no_backup),
            ignore_errors: opt_or_set_bool!(ignore_errors),
            all_lists: opt_or_set_bool!(all_lists),
            skip_last: opt_or_set_some!(skip_last, 0),
            skip: if no_skip_default {
                opt_or_set_vec_lowercase!(skip, Vec::<String>::new())
            } else {
                append_default_to_skip(opt_or_set_vec_lowercase!(skip, Vec::<String>::new()), &set.guts.skip_default)
            },
            skip_unexpected_tags: opt_or_set_bool!(skip_unexpected_tags),
            no_skip_unexpected_tags_default: opt_or_set_bool!(no_skip_unexpected_tags_default),
            creatures: ListKind {
                skip: opt_or_set_bool!(skip_creatures),
                threshold: opt_or_set_threshold!(threshold_creatures, "threshold_creatures", 67),
                log_t: set.guts.log_t_creature,
                skip_delev: opt_or_set_bool!(delev_skip_creatures),
                delev_to: delev_creatures_to,
                delev_segment: creatures_delev_segment,
                delev_segment_ceil: creatures_delev_segment_ceil,
            },
            items: ListKind {
                skip: opt_or_set_bool!(skip_items),
                threshold: opt_or_set_threshold!(threshold_items, "threshold_items", 49),
                log_t: set.guts.log_t_item,
                skip_delev: opt_or_set_bool!(delev_skip_items),
                delev_to: delev_items_to,
                delev_segment: items_delev_segment,
                delev_segment_ceil: items_delev_segment_ceil,
            },
            no_delete: opt_or_set_bool!(no_delete),
            extended_delete: opt_or_set_bool!(extended_delete),
            always_delete: opt_or_set_vec_lowercase!(
                always_delete,
                ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm", "Tamriel_Data.esm"]
            ),
            never_delete: opt_or_set_vec_lowercase!(never_delete, ["Wares-base.esm", "abotWaterLife.esm", "RepopulatedMorrowind.ESM"]),
            no_threshold_warnings: opt_or_set_bool!(no_threshold_warnings),
            delev: opt_or_set_bool!(delev),
            delev_distinct: opt_or_set_bool!(delev_distinct),
            delev_random: opt_or_set_bool!(delev_random),
            delev_segment_progressive: opt_or_set_bool!(delev_segment_progressive),
            delev_segment_ratio,
            delev_skip_list: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_skip_list, Vec::<String>::new())),
            delev_no_skip_list: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_no_skip_list, Vec::<String>::new())),
            delev_skip_subrecord: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_skip_subrecord, Vec::<String>::new())),
            delev_no_skip_subrecord: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(
                delev_no_skip_subrecord,
                Vec::<String>::new()
            )),
            no_compare: opt_or_set_bool!(no_compare),
            compare_only,
            compare_only_name,
            compare_with: opt_or_set_some!(compare_with, ""),
            compare_delev_with: opt_or_set_some!(compare_delev_with, ""),
            compare_common: opt_or_set_bool!(compare_common),
            verbose: if opt.verbose == 0 {
                if set.options.verbose != 0 {
                    show_configuration.add_some(false, "verbose", format_args!("{:?}", &set.options.verbose))?;
                }
                get_verbose!(set.options.verbose)
            } else {
                show_configuration.add_some(true, "verbose", format_args!("{:?}", &opt.verbose))?;
                get_verbose!(opt.verbose)
            },
            quiet: opt_or_set_bool!(quiet),
            progress: opt_or_set_bool!(progress) || opt_or_set_bool!(progress_bar),
            progress_bar: opt_or_set_bool!(progress_bar),
            color: opt_or_set_bool!(color),
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
                plugin_extensions_to_ignore: prepare_plugin_extensions_to_ignore(set.guts.plugin_extensions_to_ignore),
                skip_default_reasons: if no_skip_default {
                    Vec::new()
                } else {
                    set.guts.skip_default_reasons
                },
                skip_unexpected_tags_default: set.guts.skip_unexpected_tags_default.iter().map(|tag| tag.to_lowercase()).collect(),
                omw_cs_data_path_suffix_linux_macos: set.guts.omw_cs_data_path_suffix_linux_macos,
                omw_cs_data_path_suffix_windows: set.guts.omw_cs_data_path_suffix_windows,
                omw_cs_data_paths_list: set.guts.omw_cs_data_paths_list,
                header_version: set.guts.header_version,
                header_author: set.guts.header_author,
                header_description_merge: set.guts.header_description_merge,
                header_description_delev: set.guts.header_description_delev,
                header_description_merge_and_delev: set.guts.header_description_merge_and_delev,
                log_backup_suffix: set.guts.log_backup_suffix,
                progress_frequency: get_progress_frequency(set.guts.progress_frequency)?,
                progress_prefix: set.guts.progress_prefix,
                progress_template: set.guts.progress_template,
                progress_bar_template: set.guts.progress_bar_template,
                progress_bar_chars: set.guts.progress_bar_chars,
                auto_resolve_lower_limit: set.guts.auto_resolve_lower_limit,
                prefix_ignored_error_message: set.guts.prefix_ignored_error_message,
                suffix_add_ignore_errors_suggestion: set.guts.suffix_add_ignore_errors_suggestion,
                details_line_approximate_length: set.guts.details_line_approximate_length,
                verboseness_details_deleted_subrecords: get_verbose!(set.guts.verboseness_details_deleted_subrecords),
                verboseness_details_untouched_lists: get_verbose!(set.guts.verboseness_details_untouched_lists),
                verboseness_details_threshold_resolved: get_verbose!(set.guts.verboseness_details_threshold_resolved),
                verboseness_details_threshold_skipped: get_verbose!(set.guts.verboseness_details_threshold_skipped),
                verboseness_details_threshold_warnings: get_verbose!(set.guts.verboseness_details_threshold_warnings),
                verboseness_details_deleveled_subrecords: get_verbose!(set.guts.verboseness_details_deleveled_subrecords),
                verboseness_details_compare_plugins: get_verbose!(set.guts.verboseness_details_compare_plugins),
                verboseness_show_configuration: get_verbose!(set.guts.verboseness_show_configuration),
                compare_tab_l1: set.guts.compare_tab_l1,
                compare_tab_l2: set.guts.compare_tab_l2,
                compare_tab_l3: set.guts.compare_tab_l3,
                show_configuration,
                long_message_string_inital_capacity,
            },
        })
    }

    pub(super) fn show_configuration(&self, log: &mut Log) -> Result<()> {
        let show_cfg = &self.guts.show_configuration;
        if !(show_cfg.cmd.is_empty() && show_cfg.opt.is_empty() && show_cfg.set.is_empty()) {
            let verbosity = self.guts.verboseness_show_configuration;
            let text = "\nProgram run with the following configuration:";
            msg(text, MsgTone::Neutral, verbosity, self, log)?;
            macro_rules! show_cfg {
                ($($field:ident),+) => {
                    $(if !show_cfg.$field.is_empty() {
                        msg(&show_cfg.$field, MsgTone::Neutral, verbosity, self, log)?;
                    })+
                }
            }
            show_cfg!(cmd, opt, set);
            msg(String::new(), MsgTone::Neutral, verbosity, self, log)?;
        }
        Ok(())
    }

    pub(super) fn show_log_path(&self, log: &mut Log) -> Result<()> {
        if self.no_log {
            Ok(())
        } else {
            let log_path = match &self.log {
                None => return Err(anyhow!("Failed to show log path because it's empty")),
                Some(log_path) => log_path,
            };
            msg(format!("Log is written to \"{}\"", log_path.display()), MsgTone::Good, 0, self, log)
        }
    }

    pub(super) fn show_settings_written(&self, log: &mut Log) -> Result<()> {
        let mut text = String::with_capacity(self.guts.long_message_string_inital_capacity);
        if self.settings_file.backup_written {
            text.push_str(&format!(
                "Previous settings file was renamed to \"{}\"{}",
                self.settings_file.backup_path.display(),
                if self.settings_file.backup_overwritten {
                    ", previous backup was overwritten"
                } else {
                    ""
                },
            ));
            msg(text, MsgTone::Warm, 0, self, log)?;
        }
        text = format!("Wrote default program settings into \"{}\"", self.settings_file.path.display());
        msg(text, MsgTone::Good, 0, self, log)
    }

    pub(super) fn show_settings_version_message(&self, log: &mut Log) -> Result<()> {
        if let Some(message) = &self.settings_file.version_message {
            msg(message, MsgTone::Bad, 0, self, log)
        } else {
            Ok(())
        }
    }
}

pub(super) fn get_self_config() -> Result<Cfg> {
    let options = get_options()?;
    let (exe, dir) = get_exe_name_and_dir();
    let mut settings_file = get_settings_file(&exe, &dir, &options).with_context(|| "Failed to get program settings file path")?;
    let settings = get_settings(&mut settings_file).with_context(|| "Failed to get default or provided settings")?;
    if options.settings_write {
        let toml = template::<Settings>(FormatOptions::default());
        create_dir_early(&settings_file.path, "settings")?;
        backup_settings_file(&mut settings_file, &settings.guts.settings_backup_suffix, options.no_backup)?;
        write(&settings_file.path, toml)
            .with_context(|| format!("Failed to write default program settings into \"{}\"", settings_file.path.display()))?;
    }
    let configuration = Cfg::new(options, settings, settings_file, exe, dir).with_context(|| "Failed to configure program")?;
    Ok(configuration)
}
