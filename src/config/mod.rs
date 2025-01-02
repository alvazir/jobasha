use crate::{create_dir_early, msg, Log, Merge, MsgTone};
use anyhow::{anyhow, Context, Result};
use confique::toml::{template, FormatOptions};
use console::Style;
use fs_err::write;
use std::{
    env::args_os,
    ffi::OsString,
    fmt::{Arguments, Write as _},
    io::stdin,
    path::PathBuf,
};
mod options;
mod settings;
mod util;
use options::{get_options, Options};
use settings::{get_settings, Settings};
use util::{
    append_default_to_skip, backup_settings_file, check_settings_version, check_verboseness, get_color, get_compare_only,
    get_delev_to, get_exe_name_and_dir, get_fogbug_fixed_value, get_interior_grid_change, get_kind_delev_segment, get_kind_delev_to,
    get_log_file, get_output_file, get_progress_frequency, get_settings_file, prepare_delev_skip_patterns,
    prepare_plugin_extensions_to_ignore, show_configuration_add_header,
};

const MERGE_TYPES: [&str; 30] = [
    "GMST", "CLAS", "RACE", "SOUN", "SKIL", "MGEF", "BSGN", "SPEL", "STAT", "DOOR", "MISC", "WEAP", "CONT", "CREA", "BODY", "LIGH",
    "ENCH", "NPC_", "ARMO", "CLOT", "REPA", "ACTI", "APPA", "LOCK", "PROB", "INGR", "BOOK", "ALCH", "CELL", "SNDG",
];
const ALWAYS_DELETE: [&str; 4] = ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm", "Tamriel_Data.esm"];
const NEVER_DELETE: [&str; 3] = ["Wares-base.esm", "abotWaterLife.esm", "RepopulatedMorrowind.ESM"];

#[derive(Default)]
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
    pub(crate) multipatch: Multipatch,
    pub(crate) merge: Merge,
    pub(crate) verbose: u8,
    pub(crate) quiet: bool,
    pub(crate) debug: u8,
    pub(crate) progress: bool,
    pub(crate) progress_bar: bool,
    pub(crate) color: bool,
    pub(crate) no_press_enter_to_exit: bool,
    pub(crate) press_enter_to_exit: bool,
    pub(crate) no_summary: bool,
    pub(crate) guts: Guts,
    pub(crate) meta: Meta,
}

impl Cfg {
    fn meta(&mut self) {
        if self.no_log && self.quiet {
            self.meta.silent = true;
        }
        if !self.meta.silent && self.debug >= self.guts.debug_level_merge_list_all_plugins {
            self.meta.debug_plugins = true;
        }
        if !self.meta.silent && self.debug >= self.guts.debug_level_merge_skipped_all_equal {
            self.meta.debug_all = true;
        }
        if !self.meta.silent && self.debug >= self.guts.debug_level_merge_skipped_single {
            self.meta.debug_single = true;
        }
        if !self.meta.silent && self.debug >= self.guts.debug_level_merge_compare_to_the_last {
            self.meta.debug_compare = true;
        }
        if !self.meta.silent && self.debug >= self.guts.debug_level_merge_skipped_equal_to_the_last {
            self.meta.debug_equal = true;
        }
        if self.meta.debug_compare && self.meta.debug_equal {
            self.meta.debug_compare_equal = true;
        }
        if !self.meta.silent && self.debug >= self.guts.debug_level_merge_multipatch_attempt {
            self.meta.debug_multipatch_attempt = true;
        }
        if !self.merge.cell && !self.multipatch.cellnames {
            self.meta.skip_exterior = true;
        }
        if !self.merge.cell && !self.multipatch.fogbug {
            self.meta.skip_interior = true;
        }
        if !self.merge.ignore_secondary_fog_density || self.multipatch.fogbug {
            self.meta.fix_fog = true;
        }
        if !self.merge.cell && self.multipatch.cellnames {
            self.meta.multipatch_cellnames = true;
        }
    }

    #[cfg(test)]
    pub(crate) fn reset_meta(&mut self) {
        self.meta = Meta::default();
        self.meta();
    }
}

#[derive(Default)]
pub(crate) struct SettingsFile {
    pub(crate) path: PathBuf,
    pub(crate) version_message: Option<String>,
    pub(crate) write: bool,
    pub(crate) backup_path: PathBuf,
    pub(crate) backup_written: bool,
    pub(crate) backup_overwritten: bool,
}

#[derive(Default)]
pub(crate) struct DelevSkipPatterns {
    pub(crate) is_empty: bool,
    pub(crate) exact: Vec<String>,
    pub(crate) prefix: Vec<String>,
    pub(crate) infix: Vec<String>,
    pub(crate) suffix: Vec<String>,
}

#[derive(Default)]
pub(crate) struct ListKind {
    pub(crate) skip: bool,
    pub(crate) threshold: f64,
    pub(crate) log_t: String,
    pub(crate) skip_delev: bool,
    pub(crate) delev_to: u16,
    pub(crate) delev_segment: u16,
    pub(crate) delev_segment_ceil: u16,
}

#[derive(Default)]
pub(crate) enum PluginKind {
    #[default]
    Merge,
    Delev,
}

#[derive(Default)]
pub(crate) struct OutputFile {
    pub(crate) kind: PluginKind,
    pub(crate) name: String,
    pub(crate) name_lowercased_starts_with: String,
    pub(crate) path: PathBuf,
    pub(crate) dir_path: PathBuf,
    pub(crate) backup_path: PathBuf,
}

#[derive(Default)]
pub(crate) struct Multipatch {
    pub(crate) skip: bool,
    pub(crate) cellnames: bool,
    pub(crate) fogbug: bool,
    pub(crate) summons: bool,
    pub(crate) primitive: bool,
}

impl Multipatch {
    pub(crate) fn new(skip: bool, cellnames: bool, fogbug: bool, summons: bool, primitive: bool) -> Self {
        if skip {
            Self {
                skip,
                ..Default::default()
            }
        } else if cellnames || fogbug || summons {
            Self {
                cellnames,
                fogbug,
                summons,
                primitive,
                ..Default::default()
            }
        } else {
            Self {
                cellnames: true,
                fogbug: true,
                summons: true,
                primitive,
                ..Default::default()
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct Meta {
    pub(crate) silent: bool,
    pub(crate) debug_plugins: bool,
    pub(crate) debug_all: bool,
    pub(crate) debug_single: bool,
    pub(crate) debug_compare: bool,
    pub(crate) debug_equal: bool,
    pub(crate) debug_compare_equal: bool,
    pub(crate) debug_multipatch_attempt: bool,
    pub(crate) skip_exterior: bool,
    pub(crate) skip_interior: bool,
    pub(crate) fix_fog: bool,
    pub(crate) multipatch_cellnames: bool,
}

#[derive(Default)]
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
    pub(crate) no_skip_unknown_cell_flags: bool,
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
    pub(crate) merge_log_string_allocation: usize,
    pub(crate) merge_log_string_allocation_per_record: usize,
    pub(crate) multipatch_fogbug_fixed_value: f32,
    pub(crate) auto_resolve_lower_limit: f64,
    pub(crate) verboseness_details_deleted_subrecords: u8,
    pub(crate) verboseness_details_untouched_lists: u8,
    pub(crate) verboseness_details_threshold_resolved: u8,
    pub(crate) verboseness_details_threshold_skipped: u8,
    pub(crate) verboseness_details_threshold_warnings: u8,
    pub(crate) verboseness_details_deleveled_subrecords: u8,
    pub(crate) verboseness_details_merge_warnings: u8,
    pub(crate) verboseness_details_merge_record_merged: u8,
    pub(crate) verboseness_details_merge_record_multipatched: u8,
    pub(crate) verboseness_details_merge_field_changed: u8,
    pub(crate) verboseness_details_compare_plugins: u8,
    pub(crate) verboseness_show_configuration: u8,
    pub(crate) debug_level_merge_list_all_plugins: u8,
    pub(crate) debug_level_merge_compare_to_the_last: u8,
    pub(crate) debug_level_merge_skipped_equal_to_the_last: u8,
    pub(crate) debug_level_merge_interior_grid_change: u8,
    pub(crate) debug_level_merge_skipped_all_equal: u8,
    pub(crate) debug_level_merge_skipped_single: u8,
    pub(crate) debug_level_merge_multipatch_attempt: u8,
    pub(crate) prefix_ignored_error_message: String,
    pub(crate) suffix_add_ignore_errors_suggestion: String,
    pub(crate) details_line_approximate_length: usize,
    pub(crate) tab_l1: String,
    pub(crate) tab_l2: String,
    pub(crate) tab_l3: String,
    pub(crate) show_configuration: ShowConfiguration,
    pub(crate) long_message_string_inital_capacity: usize,
}

#[derive(Default)]
pub(crate) struct ShowConfiguration {
    pub(crate) cmd_len: usize,
    pub(crate) cmd: String,
    pub(crate) opt: String,
    pub(crate) set: String,
}

impl ShowConfiguration {
    fn new(capacity: usize) -> Result<ShowConfiguration> {
        let args = args_os();
        let cmd_len = args.len();
        let cmd = if cmd_len == 0 {
            String::new()
        } else {
            let mut cmd = String::with_capacity(capacity);
            write!(cmd, "  Command-line arguments:\n    ")?;
            for (position, arg) in args.enumerate() {
                if position == 0 {
                    write!(cmd, "{:?}", arg.to_string_lossy())?;
                } else {
                    write!(cmd, ",{:?}", arg.to_string_lossy())?;
                }
            }
            cmd
        };
        Ok(ShowConfiguration {
            cmd_len,
            cmd,
            opt: String::with_capacity(capacity),
            set: String::with_capacity(capacity),
        })
    }

    fn add_bool(&mut self, opt: bool, name: &'static str) -> Result<()> {
        let string = if opt { &mut self.opt } else { &mut self.set };
        show_configuration_add_header(opt, string)?;
        write!(string, "\n    {name} = true")?;
        Ok(())
    }

    fn add_some(&mut self, opt: bool, name: &'static str, format_args: Arguments<'_>) -> Result<()> {
        let string = if opt { &mut self.opt } else { &mut self.set };
        show_configuration_add_header(opt, string)?;
        write!(string, "\n    {name} = {format_args}")?;
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
            ($name:ident) => {
                opt_or_set_vec_lowercase!($name, Vec::<String>::new())
            };
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
            ($name_ident:ident, $type:ty, $default:expr, $max:expr) => {
                match opt.$name_ident {
                    Some(num) => {
                        show_configuration.add_some(true, stringify!($name_ident), format_args!("{:?}", num))?;
                        num as $type
                    }
                    None => {
                        if set.options.$name_ident != $default {
                            show_configuration.add_some(
                                false,
                                stringify!($name_ident),
                                format_args!("{:?}", &set.options.$name_ident),
                            )?;
                        }
                        match set.options.$name_ident <= $max {
                            true => set.options.$name_ident as $type,
                            false => {
                                return Err(anyhow!(format!(
                                    "Value of {} should be in range 0-{}",
                                    set.options.$name_ident, $max
                                )));
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
        macro_rules! get_repeating_u8 {
            ($name:ident) => {
                if opt.$name == 0 {
                    if set.options.$name != 0 {
                        show_configuration.add_some(false, stringify!($name), format_args!("{:?}", &set.options.$name))?;
                    }
                    get_verbose!(set.options.$name)
                } else {
                    show_configuration.add_some(true, stringify!($name), format_args!("{:?}", &opt.$name))?;
                    get_verbose!(opt.$name)
                }
            };
        }
        macro_rules! get_verbose {
            ($name:expr) => {
                check_verboseness($name, stringify!($name))?
            };
        }
        let no_log = opt_or_set_bool!(no_log);
        let quiet = opt_or_set_bool!(quiet);
        let delev_to = get_delev_to(opt_or_set_some!(delev_to, 1))?;
        let delev_creatures_to = get_kind_delev_to(delev_to, opt_or_set_some!(delev_creatures_to, 0));
        let delev_items_to = get_kind_delev_to(delev_to, opt_or_set_some!(delev_items_to, 0));
        let delev_segment = opt_or_set_some!(delev_segment, 0);
        let delev_segment_ratio = opt_or_set_threshold!(delev_segment_ratio, f64, 50, 100);
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
        let mut cfg = Cfg {
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
                opt_or_set_vec_lowercase!(skip)
            } else {
                append_default_to_skip(opt_or_set_vec_lowercase!(skip), &set.guts.skip_default)
            },
            skip_unexpected_tags: opt_or_set_bool!(skip_unexpected_tags),
            no_skip_unexpected_tags_default: opt_or_set_bool!(no_skip_unexpected_tags_default),
            creatures: ListKind {
                skip: opt_or_set_bool!(skip_creatures),
                threshold: opt_or_set_threshold!(threshold_creatures, f64, 67, 100),
                log_t: set.guts.log_t_creature,
                skip_delev: opt_or_set_bool!(delev_skip_creatures),
                delev_to: delev_creatures_to,
                delev_segment: creatures_delev_segment,
                delev_segment_ceil: creatures_delev_segment_ceil,
            },
            items: ListKind {
                skip: opt_or_set_bool!(skip_items),
                threshold: opt_or_set_threshold!(threshold_items, f64, 49, 100),
                log_t: set.guts.log_t_item,
                skip_delev: opt_or_set_bool!(delev_skip_items),
                delev_to: delev_items_to,
                delev_segment: items_delev_segment,
                delev_segment_ceil: items_delev_segment_ceil,
            },
            no_delete: opt_or_set_bool!(no_delete),
            extended_delete: opt_or_set_bool!(extended_delete),
            always_delete: opt_or_set_vec_lowercase!(always_delete, ALWAYS_DELETE),
            never_delete: opt_or_set_vec_lowercase!(never_delete, NEVER_DELETE),
            no_threshold_warnings: opt_or_set_bool!(no_threshold_warnings),
            delev: opt_or_set_bool!(delev),
            delev_distinct: opt_or_set_bool!(delev_distinct),
            delev_random: opt_or_set_bool!(delev_random),
            delev_segment_progressive: opt_or_set_bool!(delev_segment_progressive),
            delev_segment_ratio,
            delev_skip_list: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_skip_list)),
            delev_no_skip_list: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_no_skip_list)),
            delev_skip_subrecord: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_skip_subrecord)),
            delev_no_skip_subrecord: prepare_delev_skip_patterns(opt_or_set_vec_lowercase!(delev_no_skip_subrecord)),
            no_compare: opt_or_set_bool!(no_compare),
            compare_only,
            compare_only_name,
            compare_with: opt_or_set_some!(compare_with, ""),
            compare_delev_with: opt_or_set_some!(compare_delev_with, ""),
            compare_common: opt_or_set_bool!(compare_common),
            multipatch: Multipatch::new(
                opt_or_set_bool!(no_multipatch),
                opt_or_set_bool!(cellnames),
                opt_or_set_bool!(fogbug),
                opt_or_set_bool!(summons),
                opt_or_set_bool!(primitive),
            ),
            //opt_or_set_bool!(no_merge),
            merge: Merge::new(
                true,
                opt_or_set_bool!(ignore_secondary_fog_density),
                opt_or_set_bool!(interdependent_flags),
                opt_or_set_bool!(keep_redundant_values),
                opt_or_set_bool!(plus_before_minus),
                opt_or_set_bool!(verbose_atmosphere_data),
                opt_or_set_threshold!(destination_similarity, f32, 1024, 8192),
                opt_or_set_vec_lowercase!(merge_types, MERGE_TYPES),
                opt_or_set_vec_lowercase!(merge_skip_types),
            ),
            verbose: get_repeating_u8!(verbose),
            quiet,
            debug: get_repeating_u8!(debug),
            progress: opt_or_set_bool!(progress) || opt_or_set_bool!(progress_bar),
            progress_bar: opt_or_set_bool!(progress_bar),
            color: opt_or_set_bool!(color),
            no_press_enter_to_exit: opt_or_set_bool!(no_press_enter_to_exit),
            press_enter_to_exit: opt_or_set_bool!(press_enter_to_exit),
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
                no_skip_unknown_cell_flags: set.guts.no_skip_unknown_cell_flags,
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
                merge_log_string_allocation: set.guts.merge_log_string_allocation,
                merge_log_string_allocation_per_record: set.guts.merge_log_string_allocation_per_record,
                multipatch_fogbug_fixed_value: get_fogbug_fixed_value(set.guts.multipatch_fogbug_fixed_value)?,
                auto_resolve_lower_limit: set.guts.auto_resolve_lower_limit,
                verboseness_details_deleted_subrecords: get_verbose!(set.guts.verboseness_details_deleted_subrecords),
                verboseness_details_untouched_lists: get_verbose!(set.guts.verboseness_details_untouched_lists),
                verboseness_details_threshold_resolved: get_verbose!(set.guts.verboseness_details_threshold_resolved),
                verboseness_details_threshold_skipped: get_verbose!(set.guts.verboseness_details_threshold_skipped),
                verboseness_details_threshold_warnings: get_verbose!(set.guts.verboseness_details_threshold_warnings),
                verboseness_details_deleveled_subrecords: get_verbose!(set.guts.verboseness_details_deleveled_subrecords),
                verboseness_details_merge_record_merged: get_verbose!(set.guts.verboseness_details_merge_record_merged),
                verboseness_details_merge_record_multipatched: get_verbose!(set.guts.verboseness_details_merge_record_multipatched),
                verboseness_details_merge_field_changed: get_verbose!(set.guts.verboseness_details_merge_field_changed),
                verboseness_details_merge_warnings: get_verbose!(set.guts.verboseness_details_merge_warnings),
                verboseness_details_compare_plugins: get_verbose!(set.guts.verboseness_details_compare_plugins),
                verboseness_show_configuration: get_verbose!(set.guts.verboseness_show_configuration),
                debug_level_merge_list_all_plugins: get_verbose!(set.guts.debug_level_merge_list_all_plugins),
                debug_level_merge_compare_to_the_last: get_verbose!(set.guts.debug_level_merge_compare_to_the_last),
                debug_level_merge_skipped_equal_to_the_last: get_verbose!(set.guts.debug_level_merge_skipped_equal_to_the_last),
                debug_level_merge_multipatch_attempt: get_verbose!(set.guts.debug_level_merge_multipatch_attempt),
                debug_level_merge_interior_grid_change: get_interior_grid_change(
                    get_verbose!(set.guts.debug_level_merge_interior_grid_change),
                    set.guts.debug_level_merge_skipped_equal_to_the_last,
                )?,
                debug_level_merge_skipped_all_equal: get_verbose!(set.guts.debug_level_merge_skipped_all_equal),
                debug_level_merge_skipped_single: get_verbose!(set.guts.debug_level_merge_skipped_single),
                prefix_ignored_error_message: set.guts.prefix_ignored_error_message,
                suffix_add_ignore_errors_suggestion: set.guts.suffix_add_ignore_errors_suggestion,
                details_line_approximate_length: set.guts.details_line_approximate_length,
                tab_l1: set.guts.tab_l1,
                tab_l2: set.guts.tab_l2,
                tab_l3: set.guts.tab_l3,
                show_configuration,
                long_message_string_inital_capacity,
            },
            meta: Meta::default(),
        };
        cfg.meta();
        Ok(cfg)
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
            msg("", MsgTone::Neutral, verbosity, self, log)?;
        }
        Ok(())
    }

    // pub(super) fn show_merge_types(&self, log: &mut Log) -> Result<()> {
    //     if !self.merge.skip {
    //         msg(format!("Merge types: {}", self.merge), MsgTone::Neutral, 0, self, log)
    //     } else {
    //         Ok(())
    //     }
    // }

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

    pub(super) fn press_enter_to_exit(&self, log: &mut Log) -> Result<()> {
        if !self.quiet && ((!self.no_press_enter_to_exit && self.guts.show_configuration.cmd_len <= 1) || self.press_enter_to_exit) {
            msg("Press enter to exit...", MsgTone::Neutral, 0, self, log)?;
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).with_context(|| "Failed to read input")?;
        }
        Ok(())
    }
}

pub(super) fn get_self_config() -> Result<Cfg> {
    let options = get_options()?;
    let (exe, dir) = get_exe_name_and_dir();
    let mut settings_file = get_settings_file(&exe, &dir, &options).with_context(|| "Failed to get program settings file path")?;
    let settings = get_settings(&mut settings_file).with_context(|| "Failed to get default or provided settings")?;
    if options.settings_write {
        let mut format = FormatOptions::default();
        format.general.leaf_field_gap = Some(0);
        format.general.comments = options.settings_comments;
        let toml = template::<Settings>(format);
        create_dir_early(&settings_file.path, "settings")?;
        backup_settings_file(&mut settings_file, &settings.guts.settings_backup_suffix, options.no_backup)?;
        write(&settings_file.path, toml)
            .with_context(|| format!("Failed to write default program settings into \"{}\"", settings_file.path.display()))?;
    }
    let configuration = Cfg::new(options, settings, settings_file, exe, dir).with_context(|| "Failed to configure program")?;
    Ok(configuration)
}
