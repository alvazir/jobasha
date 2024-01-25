use super::{check_settings_version, SettingsFile};
use anyhow::{Context, Result};
use confique::Config;

#[derive(Config)]
pub(super) struct Settings {
    #[config(nested)]
    pub(super) options: Options,
    #[config(nested)]
    pub(super) guts: Guts,
}

#[derive(Config)]
pub(super) struct Options {
    /// Description of all the options is provided with --help. There are two lines per each option: default value and set value. Uncomment second line for the needed option and set the value.
    ///
    /// [Filters]
    ///
    #[config(default = "")]
    pub(super) config: String,
    #[config(default = "MergedLeveledLists.esp")]
    pub(super) output: String,
    #[config(default = "")]
    pub(super) output_dir: String,
    #[config(default = false)]
    pub(super) date: bool,
    #[config(default = false)]
    pub(super) dry_run: bool,
    #[config(default = "")]
    pub(super) log: String,
    #[config(default = false)]
    pub(super) no_log: bool,
    #[config(default = false)]
    pub(super) no_backup: bool,
    #[config(default = false)]
    pub(super) ignore_errors: bool,
    ///
    /// [Filters]
    ///
    #[config(default = false)]
    pub(super) all_lists: bool,
    #[config(default = 0)]
    pub(super) skip_last: usize,
    #[config(default = [])]
    pub(super) skip: Vec<String>,
    #[config(default = false)]
    pub(super) no_skip_default: bool,
    #[config(default = false)]
    pub(super) skip_unexpected_tags: bool,
    #[config(default = false)]
    pub(super) no_skip_unexpected_tags_default: bool,
    #[config(default = false)]
    pub(super) skip_creatures: bool,
    #[config(default = false)]
    pub(super) skip_items: bool,
    ///
    /// [Subrecord deletion]
    ///
    #[config(default = false)]
    pub(super) no_delete: bool,
    #[config(default = false)]
    pub(super) extended_delete: bool,
    #[config(default = ["Morrowind.esm","Tribunal.esm","Bloodmoon.esm","Tamriel_Data.esm"])]
    pub(super) always_delete: Vec<String>,
    #[config(default = ["Wares-base.esm", "abotWaterLife.esm", "RepopulatedMorrowind.ESM"])]
    pub(super) never_delete: Vec<String>,
    #[config(default = 67)]
    pub(super) threshold_creatures: u64,
    #[config(default = 49)]
    pub(super) threshold_items: u64,
    #[config(default = false)]
    pub(super) no_threshold_warnings: bool,
    ///
    /// [Delev]
    ///
    #[config(default = false)]
    pub(super) delev: bool,
    #[config(default = 1)]
    pub(super) delev_to: u16,
    /// Following 2 sections are effectively disabled by default with 0 values.
    #[config(default = 0)]
    pub(super) delev_creatures_to: u16,
    #[config(default = 0)]
    pub(super) delev_items_to: u16,
    #[config(default = false)]
    pub(super) delev_distinct: bool,
    /// By default the value is empty so that "guts.delev_output_infix_default" is added to the output plugin name.
    #[config(default = "")]
    pub(super) delev_output: String,
    #[config(default = false)]
    pub(super) delev_random: bool,
    /// Following 3 sections are effectively disabled by default with 0 values.
    #[config(default = 0)]
    pub(super) delev_segment: u16,
    #[config(default = 0)]
    pub(super) delev_creatures_segment: u16,
    #[config(default = 0)]
    pub(super) delev_items_segment: u16,
    #[config(default = false)]
    pub(super) delev_segment_progressive: bool,
    #[config(default = 50)]
    pub(super) delev_segment_ratio: u8,
    ///
    /// [Delev filters]
    ///
    #[config(default = false)]
    pub(super) delev_skip_creatures: bool,
    #[config(default = false)]
    pub(super) delev_skip_items: bool,
    #[config(default = [])]
    pub(super) delev_skip_list: Vec<String>,
    #[config(default = [])]
    pub(super) delev_no_skip_list: Vec<String>,
    #[config(default = [])]
    pub(super) delev_skip_subrecord: Vec<String>,
    #[config(default = [])]
    pub(super) delev_no_skip_subrecord: Vec<String>,
    ///
    /// [Compare]
    ///
    #[config(default = false)]
    pub(super) no_compare: bool,
    #[config(default = "")]
    pub(super) compare_with: String,
    #[config(default = "")]
    pub(super) compare_delev_with: String,
    #[config(default = false)]
    pub(super) compare_common: bool,
    ///
    /// [Display output]
    ///
    #[config(default = 0)]
    pub(super) verbose: u8,
    #[config(default = false)]
    pub(super) quiet: bool,
    #[config(default = false)]
    pub(super) progress: bool,
    #[config(default = false)]
    pub(super) progress_bar: bool,
    #[config(default = false)]
    pub(super) color: bool,
    #[config(default = false)]
    pub(super) no_summary: bool,
}

#[derive(Config)]
pub(super) struct Guts {
    /// Guts of the program. Use at your own risk ;-)
    ///
    /// # Following line is used to determine version of used settings to warn about outdated version:
    /// # Settings version: 0.4.0
    ///
    /// [Colors]
    /// Available colors are: blue, cyan, green, magenta, red, yellow, none.
    ///
    #[config(default = "cyan")]
    pub(super) color_suggestion: String,
    #[config(default = "green")]
    pub(super) color_success: String,
    #[config(default = "red")]
    pub(super) color_warning: String,
    #[config(default = "yellow")]
    pub(super) color_ignored_error: String,
    ///
    /// [Game configuration file paths]
    /// For both Morrowind.ini and openmw.cfg. Actually file may have any name or extension. "/" is used as separator for all platforms.
    ///
    /// Path that is appended to the "preference_dir": "$HOME/.config|$HOME/Library/Preferences" + config_path_suffix_linux_macos
    #[config(default = "openmw/openmw.cfg")]
    pub(super) config_path_suffix_linux_macos: String,
    ///
    /// Path that is appended to the "document_dir": "C:\Users\Username\Documents" + config_path_suffix_windows
    #[config(default = "My Games/OpenMW/openmw.cfg")]
    pub(super) config_path_suffix_windows: String,
    ///
    /// All other relative/absolute paths to check:
    ///  "/storage/emulated/0/omw/config/openmw.cfg": android openmw.cfg absolute path
    ///  "openmw.cfg": all platforms, looks for openmw.cfg in the directory where it's run
    ///  "Morrowind.ini": all platforms, looks for Morrowind.ini in the directory where it's run
    ///  "../Morrowind.ini": all platforms, looks for Morrowind.ini in the parent directory from where it's run(e.g. "Data Files")
    #[config(default = ["/storage/emulated/0/omw/config/openmw.cfg", "openmw.cfg", "Morrowind.ini", "../Morrowind.ini"])]
    pub(super) config_paths_list: Vec<String>,
    ///
    /// [Game configuration file processing]
    /// These are used to parse Morrowind.ini and openmw.cfg.
    ///
    #[config(default = "GameFile")]
    pub(super) mor_line_beginning_content: String,
    #[config(default = "Data Files")]
    pub(super) mor_data_files_dir: String,
    #[config(default = "content=")]
    pub(super) omw_line_beginning_content: String,
    #[config(default = "data=")]
    pub(super) omw_line_beginning_data: String,
    #[config(default = ["esm", "esp", "omwaddon", "omwscripts"])]
    pub(super) omw_plugin_extensions: Vec<String>,
    /// Plugins with following extensions will not be processed. It's made to ignore .omwscripts, though may be used for anything else.
    #[config(default = ["omwscripts"])]
    pub(super) plugin_extensions_to_ignore: Vec<String>,
    /// Following plugins are skipped unless --no-skip-default is set.
    #[config(default = ["Merged Objects.esp", "merged.omwaddon"])]
    pub(super) skip_default: Vec<String>,
    /// Reason to display when skipping these plugins unless --all-lists is specified.
    #[config(default = [
        ["Merged Objects.esp", "This plugin was probably created by TES3Merge. Add \"--all-lists\" to override leveled lists in it.\n  Or set \"LEVC = false\" and \"LEVI = false\" in TES3Merge.ini."],
        ["merged.omwaddon", "This plugin was probably created by DeltaPlugin. Add \"--all-lists\" to override leveled lists in it."],
    ])]
    pub(super) skip_default_reasons: Vec<Vec<String>>,
    /// Plugins with the following record types are skipped unless --no-skip-unexpected-tags-default is set.
    #[config(default = ["LUAL", "TES3::FORM", "CELL::XSCL"])]
    pub(super) skip_unexpected_tags_default: Vec<String>,
    /// [Section: "Hidden" OpenMW-CS data directory]
    ///
    /// Path that is appended to the "data_dir": "$HOME/.local/share|$HOME/Library/Application Support" + omw_cs_data_path_suffix_linux_macos
    #[config(default = "openmw/data")]
    pub(super) omw_cs_data_path_suffix_linux_macos: String,
    /// Path that is appended to the "document_dir": "C:\Users\Username\Documents" + omw_cs_data_path_suffix_windows
    #[config(default = "My Games/OpenMW/data")]
    pub(super) omw_cs_data_path_suffix_windows: String,
    #[config(default = [])]
    pub(super) omw_cs_data_paths_list: Vec<String>,
    ///
    /// [Date]
    ///
    /// Format of date string added to output plugin name.
    #[config(default = "%Y-%m-%d")]
    pub(super) output_date_format: String,
    /// List of date separators to check for excluding previous output plugin from list of plugins to process. First separator in list is used between output plugin base name and added date string.
    #[config(default = [" - ", ".", "-", "_", " "])]
    pub(super) output_date_separators: Vec<String>,
    ///
    /// [Output plugin]
    ///
    /// Output plugin default extension.
    #[config(default = "esp")]
    pub(super) output_extension_default: String,
    /// Delev output plugin default infix.
    #[config(default = "Delev")]
    pub(super) delev_output_infix_default: String,
    ///
    /// [Header]
    /// Output plugin will have these values placed into header.
    ///
    #[config(default = 1.3)]
    pub(super) header_version: f32,
    #[config(default = "Jobasha")]
    pub(super) header_author: String,
    #[config(default = "Auto-generated merged leveled lists")]
    pub(super) header_description_merge: String,
    #[config(default = "Auto-generated deleveled leveled lists")]
    pub(super) header_description_delev: String,
    #[config(default = "Auto-generated merged and deleveled leveled lists")]
    pub(super) header_description_merge_and_delev: String,
    ///
    /// [Backup file suffixes]
    ///
    #[config(default = ".backup")]
    pub(super) settings_backup_suffix: String,
    #[config(default = ".backup")]
    pub(super) log_backup_suffix: String,
    #[config(default = ".backup")]
    pub(super) output_backup_suffix: String,
    ///
    /// [Progress]
    /// Configuration of progress/progress bar. Do not set frequency higher than 15 - it slows everything due to locks etc.
    ///
    #[config(default = 5)]
    pub(super) progress_frequency: u8,
    #[config(default = "Reading plugins:")]
    pub(super) progress_prefix: String,
    #[config(default = "{prefix} {pos}/{len}")]
    pub(super) progress_template: String,
    #[config(default = "{prefix} {wide_bar} {pos}/{len}")]
    pub(super) progress_bar_template: String,
    #[config(default = "# ")]
    pub(super) progress_bar_chars: String,
    ///
    /// [Auto-resolve lower limit]
    /// By default non-base game originating leveled lists will be set to no-delete mode if 100% of their original subrecords would be deleted.
    ///
    #[config(default = 100.0)]
    pub(super) auto_resolve_lower_limit: f64,
    ///
    /// [Messages]
    /// Unsorted parts of messages used in multiple places.
    ///
    #[config(default = "Ignored error: ")]
    pub(super) prefix_ignored_error_message: String,
    #[config(default = "\n\tAdd --ignore-errors to ignore this error")]
    pub(super) suffix_add_ignore_errors_suggestion: String,
    #[config(default = 128)]
    pub(super) details_line_approximate_length: usize,
    #[config(default = 2)]
    pub(super) verboseness_details_deleted_subrecords: u8,
    #[config(default = 2)]
    pub(super) verboseness_details_untouched_lists: u8,
    #[config(default = 1)]
    pub(super) verboseness_details_threshold_resolved: u8,
    #[config(default = 2)]
    pub(super) verboseness_details_threshold_skipped: u8,
    #[config(default = 0)]
    pub(super) verboseness_details_threshold_warnings: u8,
    #[config(default = 3)]
    pub(super) verboseness_details_deleveled_subrecords: u8,
    #[config(default = 1)]
    pub(super) verboseness_details_compare_plugins: u8,
    #[config(default = "  ")]
    pub(super) compare_tab_l1: String,
    #[config(default = "    ")]
    pub(super) compare_tab_l2: String,
    #[config(default = "      ")]
    pub(super) compare_tab_l3: String,
    /// These 2 are used in log/output in a leveled list type column. Only first letter is used.
    #[config(default = "C")]
    pub(super) log_t_creature: String,
    #[config(default = "I")]
    pub(super) log_t_item: String,
}

pub(super) fn get_settings(settings_file: &mut SettingsFile) -> Result<Settings> {
    let settings = Settings::builder()
        .file(&settings_file.path)
        .load()
        .with_context(|| "Failed to load settings")?;
    check_settings_version(settings_file)?;
    Ok(settings)
}
