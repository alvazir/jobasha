use anyhow::{Context, Result};
use confique::Config;
use std::path::PathBuf;

#[derive(Config)]
pub(crate) struct Settings {
    #[config(nested)]
    pub(crate) options: Options,
    #[config(nested)]
    pub(crate) guts: Guts,
}

#[derive(Config)]
pub(crate) struct Options {
    /// Description of all the options is provided with --help. There are two lines per each option: default value and set value. Uncomment second line for the needed option and set the value.
    #[config(default = "")]
    pub(crate) config: String,
    #[config(default = "MergedLeveledLists.esp")]
    pub(crate) output: String,
    #[config(default = "")]
    pub(crate) output_dir: String,
    #[config(default = false)]
    pub(crate) no_date: bool,
    #[config(default = false)]
    pub(crate) dry_run: bool,
    #[config(default = "")]
    pub(crate) log: String,
    #[config(default = false)]
    pub(crate) no_log: bool,
    #[config(default = false)]
    pub(crate) ignore_errors: bool,
    #[config(default = false)]
    pub(crate) all_lists: bool,
    #[config(default = 0)]
    pub(crate) skip_last: usize,
    #[config(default = [])]
    pub(crate) skip: Vec<String>,
    #[config(default = false)]
    pub(crate) no_creatures: bool,
    #[config(default = false)]
    pub(crate) no_items: bool,
    #[config(default = false)]
    pub(crate) no_delete: bool,
    #[config(default = false)]
    pub(crate) extended_delete: bool,
    #[config(default = ["Morrowind.esm","Tribunal.esm","Bloodmoon.esm","Tamriel_Data.esm"])]
    pub(crate) always_delete: Vec<String>,
    #[config(default = ["Wares-base.esm", "abotWaterLife.esm", "RepopulatedMorrowind.ESP"])]
    pub(crate) never_delete: Vec<String>,
    #[config(default = 67)]
    pub(crate) threshold_creatures: u64,
    #[config(default = 49)]
    pub(crate) threshold_items: u64,
    #[config(default = false)]
    pub(crate) no_threshold_warnings: bool,
    #[config(default = 0)]
    pub(crate) verbose: u8,
    #[config(default = false)]
    pub(crate) quiet: bool,
    #[config(default = false)]
    pub(crate) no_color: bool,
    #[config(default = false)]
    pub(crate) no_progress: bool,
    #[config(default = false)]
    pub(crate) no_progress_bar: bool,
    #[config(default = false)]
    pub(crate) no_summary: bool,
}

#[derive(Config)]
pub(crate) struct Guts {
    /// Guts of the program. Use at your own risk ;-)
    ///
    /// [Colors]
    /// Available colors are: blue, cyan, green, magenta, red, yellow, none.
    ///
    #[config(default = "cyan")]
    pub(crate) color_suggestion: String,
    #[config(default = "green")]
    pub(crate) color_success: String,
    #[config(default = "red")]
    pub(crate) color_warning: String,
    #[config(default = "yellow")]
    pub(crate) color_ignored_error: String,
    ///
    /// [Game configuration file paths]
    /// For both Morrowind.ini and openmw.cfg. Actually file may have any name or extension. "/" is used as separator for all platforms.
    ///
    /// Path that is appended to the "preference_dir": "$HOME/.config|$HOME/Library/Preferences" + config_path_suffix_linux_macos
    #[config(default = "openmw/openmw.cfg")]
    pub(crate) config_path_suffix_linux_macos: String,
    ///
    /// Path that is appended to the "document_dir": "C:\Users\Username\Documents" + config_path_suffix_windows
    #[config(default = "My Games/OpenMW/openmw.cfg")]
    pub(crate) config_path_suffix_windows: String,
    ///
    /// All other relative/absolute paths to check:
    ///  "/storage/emulated/0/omw/config/openmw.cfg": android openmw.cfg absolute path
    ///  "openmw.cfg": all platforms, looks for openmw.cfg in the directory where it's run
    ///  "Morrowind.ini": all platforms, looks for Morrowind.ini in the directory where it's run
    ///  "../Morrowind.ini": all platforms, looks for Morrowind.ini in the parent directory from where it's run(e.g. "Data Files")
    #[config(default = ["/storage/emulated/0/omw/config/openmw.cfg", "openmw.cfg", "Morrowind.ini", "../Morrowind.ini"])]
    pub(crate) config_paths_list: Vec<String>,
    ///
    /// [Game configuration file processing]
    /// These are used to parse Morrowin.ini and openmw.cfg.
    ///
    #[config(default = "GameFile")]
    pub(crate) mor_line_beginning_content: String,
    #[config(default = "Data Files")]
    pub(crate) mor_data_files_dir: String,
    #[config(default = "content=")]
    pub(crate) omw_line_beginning_content: String,
    #[config(default = "data=")]
    pub(crate) omw_line_beginning_data: String,
    #[config(default = ["esm", "esp", "omwaddon"])]
    pub(crate) omw_plugin_extensions: Vec<String>,
    ///
    /// [Date]
    ///
    /// Format of date string added to output plugin name.
    #[config(default = "%Y-%m-%d")]
    pub(crate) output_date_format: String,
    /// List of date separators to check for excluding previous output plugin from list of plugins to process. First separator in list is used between output plugin base name and added date string.
    #[config(default = [" - ", ".", "-", "_", " "])]
    pub(crate) output_date_separators: Vec<String>,
    ///
    /// [Output plugin default extension]
    ///
    #[config(default = "esp")]
    pub(crate) output_extension_default: String,
    ///
    /// [Header]
    /// Output plugin will have these values placed into header.
    ///
    #[config(default = 1.3)]
    pub(crate) header_version: f32,
    #[config(default = "Jobasha")]
    pub(crate) header_author: String,
    #[config(default = "Auto-generated merged leveled lists")]
    pub(crate) header_description: String,
    ///
    /// [Progress]
    /// Configuration of progress/progress bar. Do not set frequency higher than 15 - it slows everything due to locks etc.
    ///
    #[config(default = 5)]
    pub(crate) progress_frequency: u8,
    #[config(default = "Reading plugins:")]
    pub(crate) progress_prefix: String,
    #[config(default = "{prefix} {pos}/{len}")]
    pub(crate) progress_template: String,
    #[config(default = "{prefix} {wide_bar} {pos}/{len}")]
    pub(crate) progress_bar_template: String,
    #[config(default = "# ")]
    pub(crate) progress_bar_chars: String,
    ///
    /// [Auto-resolve lower limit]
    /// By default non-base game originating leveled lists will be set to no-delete mode if 100% of their original subrecords would be deleted.
    ///
    #[config(default = 100.0)]
    pub(crate) auto_resolve_lower_limit: f64,
    ///
    /// [Messages]
    /// Unsorted parts of messages used in multiple places.
    ///
    #[config(default = "Ignored error: ")]
    pub(crate) prefix_ignored_error_message: String,
    #[config(default = "\n\tAdd --ignore-errors to ignore this error")]
    pub(crate) suffix_add_ignore_errors_suggestion: String,
    #[config(default = ", add --verbose or check log for details")]
    pub(crate) suffix_add_v_suggestion: String,
    #[config(default = ", add -vv or check log for details")]
    pub(crate) suffix_add_2v_suggestion: String,
    #[config(default = ", add --verbose for details")]
    pub(crate) suffix_add_v_suggestion_no_log: String,
    #[config(default = ", add -vv for details")]
    pub(crate) suffix_add_2v_suggestion_no_log: String,
    /// These 2 are used in log/output in a leveled list type column. Only first letter is used.
    #[config(default = "C")]
    pub(crate) log_t_creature: String,
    #[config(default = "I")]
    pub(crate) log_t_item: String,
}

pub(crate) fn get_settings(settings_toml: &PathBuf) -> Result<Settings> {
    let settings = Settings::builder()
        .file(settings_toml)
        .load()
        .with_context(|| "Failed to load settings")?;
    Ok(settings)
}
