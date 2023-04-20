use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    verbatim_doc_comment,
    after_long_help = "Notes:
  - Display/log output looks better with monospaced font.
  - Don't clean the output plugin. Cleaning may rarely lead to removal of some leveled lists that should be there."
)]
/// Jobasha - Yet Another TES3 Leveled List Tool
///
/// Author: alvazir
/// License: Unlicense OR MIT
/// GitHub: https://github.com/alvazir/jobasha
/// Nexus Mods: https://www.nexusmods.com/morrowind/mods/52707
pub(crate) struct Options {
    /// Path to the game configuration file, e.g.: "C:\Users\Username\Documents\My Games\OpenMW\openmw.cfg"(absolute), "../Morrowind.ini"(relative). May be used to provide alternative game configuration file or in case the game configuration file was not found automatically.
    ///
    /// Default value: ""(automatically search for the game configuration file).
    #[arg(
        conflicts_with = "settings_write",
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::FilePath,
        help = "Path to the game configuration file"
    )]
    pub(crate) config: Option<String>,
    /// Name of the output plugin. May be provided as a path, e.g.: "C:\Morrowind\mods\LeveledLists.esp"(absolute), "mods/LeveledLists.esp"(relative). Non-existent directories will be created.
    ///
    /// Date is added to the output plugin name by default, e.g. "MergedLeveledLists - YYYY-mm-dd.esp". Use --no-date to disable this behaviour.
    ///
    /// Default value: "MergedLeveledLists.esp"(will be placed into the current directory).
    #[arg(
        conflicts_with = "settings_write",
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        help = "Name of the output plugin"
    )]
    pub(crate) output: Option<String>,
    /// Name of the output plugin directory. May be provided as a path, e.g.: "C:\Morrowind\mods"(absolute), "mods"(relative). Non-existent directory will be created.
    ///
    /// Default output plugin name will be used if --output is not provided. This option takes precedence when both --output and --output-dir provide directory path.
    ///
    /// Default value: ""(current directory).
    #[arg(
        conflicts_with = "settings_write",
        short = 'O',
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        help = "Name of the output plugin directory"
    )]
    pub(crate) output_dir: Option<String>,
    /// Do not add date to the output plugin name.
    #[arg(
        conflicts_with = "settings_write",
        short,
        long,
        help = "Do not add date to the output plugin name"
    )]
    pub(crate) no_date: bool,
    /// Do not write output plugin.
    #[arg(
        conflicts_with = "settings_write",
        short,
        long,
        help = "Do not write output plugin"
    )]
    pub(crate) dry_run: bool,
    /// Name of the log file. May be provided as a path. Non-existent directories will be created.
    ///
    /// Log contains display output of the program as if it was run with maximum verboseness. It is enabled by default, use --no-log to disable.
    ///
    /// Default value: "<program_name>.log"(file will be created in program directory).
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        help = "Name of the log file"
    )]
    pub(crate) log: Option<String>,
    /// Do not write log.
    #[arg(short = 'L', long, help = "Do not write log")]
    pub(crate) no_log: bool,
    /// Name of the program settings file. May be provided as a path. Non-existent directories will be created. Extension will be replaced with ".toml".
    ///
    /// Default value: "<program_name>.toml"(file will be created in program directory).
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::FilePath,
        help = "Name of the program settings file"
    )]
    pub(crate) settings: Option<String>,
    /// Write default program settings file and exit.
    ///
    /// Use this option if you keep using the same arguments. Modify default settings to suit your needs. Allows modifiying program behaviour even more, e.g. changing output plugin header, colors of messages or paths used for game configuration file auto-discovery.
    ///
    /// File will be created in program directory with name "<program_name>.toml" by default. Use --settings to provide another path. Keep in mind that non-default settings file path should be explicitly provided every time you want to use it.
    ///
    /// This flag conflicts with everything except --settings, --no-color, --log, --no-log.
    #[arg(long, help = "Write default program settings file and exit")]
    pub(crate) settings_write: bool,
    /// Ignore non-critical errors, e.g. missing plugin. May be useful, though it's better to fix underlying problems.
    #[arg(
        conflicts_with = "settings_write",
        long,
        help = "Ignore non-critical errors"
    )]
    pub(crate) ignore_errors: bool,
    /// Place all leveled lists into the output plugin.
    ///
    /// Only merged leveled lists that differ from the last loaded instance of leveled list are placed into the output plugin by default. See --no-summary for details.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        short = 'a',
        long,
        help = "Place all leveled lists into the output plugin"
    )]
    pub(crate) all_lists: bool,
    /// Do not process last <N> plugins from load order.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        short = 'k',
        long,
        help = "Do not process last <N> plugins",
        value_name = "0",
        value_parser = clap::value_parser!(usize)
    )]
    pub(crate) skip_last: Option<usize>,
    /// Do not process these plugins.
    ///
    /// Use it if you want to skip something from processing. For example plugins produced by delevel or merging tools. Program will automatically try to skip it's previous output plugin from processing. Use this option if it fails for some reason.
    ///
    /// May take either one or multiple comma-separated plugin names, see --always-delete for examples.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        short = 'K',
        long,
        help = "Do not process these plugins",
        value_name = "PLUGIN(S)",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true,
        value_delimiter = ','
    )]
    pub(crate) skip: Option<Vec<String>>,
    /// Do not process creature leveled lists.
    ///
    /// This flag conflicts with --no-items.
    #[arg(
        help_heading = "Filters",
        conflicts_with_all = ["settings_write", "no_items"],
        long,
        help = "Do not process creature leveled lists"
    )]
    pub(crate) no_creatures: bool,
    /// Do not process item leveled lists.
    ///
    /// This flag conflicts with --no-creatures.
    #[arg(
        help_heading = "Filters",
        conflicts_with_all = ["settings_write", "no_creatures"],
        long,
        help = "Do not process item leveled lists"
    )]
    pub(crate) no_items: bool,
    /// Do not delete subrecords from leveled lists.
    ///
    /// This flag conflicts with --extended-delete.
    #[arg(
        help_heading = "Subrecord deletion",
        conflicts_with_all = ["settings_write", "extended_delete", "always_delete", "never_delete", "threshold_creatures", "threshold_items", "no_threshold_warnings"],
        short = 'D',
        long,
        help = "Do not delete subrecords from leveled lists"
    )]
    pub(crate) no_delete: bool,
    /// Enable extended delete mode.
    ///
    /// Program only deletes subrecords from leveled lists originating from base game plugins by default, see --always-delete. With --extended-delete subrecords from any leveled list may be deleted. Threshold checks help to identify potential problems. Warning will be displayed when ratio of deleted/initial subrecords per each leveled list exceeds threshold. Then you may adjust thresholds or add plugin name to --never-delete. Or disable warnings completely with --no-threshold-warnings.
    ///
    /// This flag conflicts with --no-delete. It is required by --never-delete, --threshold_creatures, --threshold_items, --no-threshold-warnings.
    #[arg(
        help_heading = "Subrecord deletion",
        conflicts_with_all = ["settings_write", "no_delete"],
        short,
        long,
        help = "Enable extended delete mode"
    )]
    pub(crate) extended_delete: bool,
    /// List of plugins to delete subrecords. Subrecords from leveled lists originating from these plugins may be deleted. It's made specifically the base game plugins. Tamriel_Data is also considered base game in this case.
    ///
    /// This is the only "delete" option that's used by default. With --extended-delete it skips threshold checks for base game plugins. Threshold checks' purpose is to prevent problem presented in --never-delete, but base game leveled lists should be free of this problem.
    ///
    /// Pass empty string "" to disable. May take either one or multiple comma-separated plugin names, e.g.: "Morrowind.esm"(one), Morrowind.esm,Tribunal.esm(many). Pay attention, that there is no space after comma. Use double-quotes around plugin names with spaces. Case-insensitive. May be used multiple times instead of providing comma-separated list, e.g.: --always-delete Morrowind.esm --always-delete Tribunal.esm.
    ///
    /// Default value: "Morrowind.esm","Tribunal.esm","Bloodmoon.esm","Tamriel_Data.esm"
    ///
    /// This flag conflicts with --no-delete.
    #[arg(
        help_heading = "Subrecord deletion",
        conflicts_with_all = ["settings_write", "no_delete"],
        short = 'A',
        long,
        value_name = "PLUGIN(S)",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true,
        value_delimiter = ',',
        help = "List of plugins to delete subrecords"
    )]
    pub(crate) always_delete: Option<Vec<String>>,
    /// Do not delete subrecords from leveled lists introduced by these plugins.
    ///
    /// Some rare plugins were not designed for deletion of subrecords in merged leveled lists. For example, plugin "abotWaterLife" has item leveled list "ab01random_ingredient" with 66 ingredients. Plugin "abotWaterLifeTRaddon" also contains the same list with 5 ingredients only(TR specific). This list was clearly designed to be merged together to produce 71 ingredients. Common approach(that this tool relies on) is to have those 5 ingredients added to previously introduced 66 items in a subsequent list.
    ///
    /// Pass empty string "" to disable. May take either one or multiple comma-separated plugin names, see --always-delete for examples.
    ///
    /// Default value: "Wares-base.esm","abotWaterLife.esm","RepopulatedMorrowind.ESP"
    ///
    /// This flag requires --extended-delete.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete"],
        short = 'N',
        long,
        value_name = "PLUGIN(S)",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true,
        value_delimiter = ',',
        help = "Do not delete subrecords from these plugins"
    )]
    pub(crate) never_delete: Option<Vec<String>>,
    /// Threshold for percentage of deleted/initial creature subrecords per each leveled list. Will print warnings when threshold exceeded.
    ///
    /// Default value: 67(%).
    ///
    /// This flag requires --extended-delete. Conflicts with --creatures-off.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete", "no_creatures"],
        long,
        help = "Threshold for % of deleted/initial creatures per list",
        value_name = "67",
        value_parser = clap::value_parser!(u64).range(0..100)
    )]
    pub(crate) threshold_creatures: Option<u64>,
    /// Threshold for percentage of deleted/initial item subrecords per each leveled list. Will print warnings when threshold exceeded.
    ///
    /// Default value: 49(%).
    ///
    /// This flag requires --extended-delete. Conflicts with --items-off.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete", "no_items"],
        long,
        help = "Threshold for % of deleted/initial items per list",
        value_name = "49",
        value_parser = clap::value_parser!(u64).range(0..100)
    )]
    pub(crate) threshold_items: Option<u64>,
    /// Do not show threshold warnings.
    ///
    /// Warnings are shown when threshold of deleted/initial subrecords is exceeded for leveled list by default.
    ///
    /// This flag requires --extended-delete.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete"],
        short = 'T',
        long,
        help = "Do not show threshold warnings"
    )]
    pub(crate) no_threshold_warnings: bool,
    /// Show more information. May be provided twice for extra effect.
    ///
    /// This flag conflicts with --quiet.
    #[arg(
        help_heading = "Display output",
        conflicts_with_all = ["settings_write", "quiet"],
        short,
        long,
        action = clap::ArgAction::Count,
        help = "Show more information"
    )]
    pub(crate) verbose: u8,
    /// Do not show anything.
    ///
    /// This flag conflicts with --verbose.
    #[arg(
        help_heading = "Display output",
        conflicts_with_all = ["settings_write", "verbose"],
        short,
        long,
        help = "Do not show anything"
    )]
    pub(crate) quiet: bool,
    /// Do not show colored output.
    #[arg(
        help_heading = "Display output",
        short = 'C',
        long,
        help = "Do not show colored output"
    )]
    pub(crate) no_color: bool,
    /// Do not show plugins reading progress.
    ///
    /// This flag conflicts with --no-progress-bar.
    #[arg(
        help_heading = "Display output",
        conflicts_with_all = ["settings_write", "no_progress_bar"],
        short = 'P',
        long,
        help = "Do not show plugins reading progress"
    )]
    pub(crate) no_progress: bool,
    /// Do not show plugins reading progress bar. Progress is shown, but progress bar is hidden.
    ///
    /// This flag conflicts with --no-progress.
    #[arg(
        help_heading = "Display output",
        conflicts_with_all = ["settings_write", "no_progress"],
        short = 'B',
        long,
        help = "Do not show plugins reading progress bar"
    )]
    pub(crate) no_progress_bar: bool,
    /// Do not show summary.
    ///
    /// Summary's field names are mostly self explanatory though not all:
    /// [duration]: Program execution duration in seconds measured from inside the program.
    /// [records read]: Program has to read all the records to get leveled lists.
    /// [unique]: Merging is only required for (total - unique) number of leveled lists.
    /// [placed]: Amount of merged leveled lists placed into the output plugin.
    /// [untouched]: Lists that are identical to last loaded instance of itself, thus there is no need to place them into the output plugin.
    /// [masters]: Master subrecords are placed into the output plugin header for every plugin that has leveled list merged and placed.
    #[arg(
        help_heading = "Display output",
        conflicts_with = "settings_write",
        short = 'S',
        long,
        verbatim_doc_comment,
        help = "Do not show summary"
    )]
    pub(crate) no_summary: bool,
}

pub(crate) fn get_options() -> Result<Options> {
    let options = Options::try_parse()?;
    Ok(options)
}
