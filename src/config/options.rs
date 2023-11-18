use anyhow::{anyhow, Result};
use clap::{builder::StyledStr, Arg, CommandFactory, Parser};

#[derive(Parser)]
#[command(
    author,
    version,
    verbatim_doc_comment,
    after_long_help = "Notes:
  - Display/log output looks better with monospaced font.
  - Don't clean the output plugin. Cleaning may rarely lead to removal of some leveled lists that should be there."
)]
/// Jobasha - TES3 leveled list merging and deleveling tool
///
/// Author: alvazir
/// License: GNU GPLv3
/// GitHub: https://github.com/alvazir/jobasha
/// Nexus Mods: https://www.nexusmods.com/morrowind/mods/52707
pub(super) struct Options {
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
    pub(super) config: Option<String>,
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
    pub(super) output: Option<String>,
    /// Name of the output plugin directory. May be provided as a path, e.g.: "C:\Morrowind\mods"(absolute), "mods"(relative). Non-existent directory will be created.
    ///
    /// Default output plugin name will be used if --output is not provided. This option takes precedence when both --output and --output-dir provide directory path.
    ///
    /// Default value: ""(current directory).
    #[arg(
        conflicts_with = "settings_write",
        short = 'O',
        long,
        aliases = ["output_dir", "dir-output", "dir_output"],
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        help = "Name of the output plugin directory"
    )]
    pub(super) output_dir: Option<String>,
    /// Do not add date to the output plugin name.
    #[arg(
        conflicts_with = "settings_write",
        short,
        long,
        aliases = ["no_date", "date-no", "date_no"],
        help = "Do not add date to the output plugin name"
    )]
    pub(super) no_date: bool,
    /// Do not write output plugin.
    #[arg(conflicts_with = "settings_write", long, aliases = ["dry_run", "run-dry", "run_dry"], help = "Do not write output plugin")]
    pub(super) dry_run: bool,
    /// Name of the log file. May be provided as a path. Non-existent directories will be created.
    ///
    /// Log contains display output of the program as if it was run with maximum verboseness. It is enabled by default, use --no-log to disable. Previous log will be saved with ".previous" extension.
    ///
    /// Default value: "<program_name>.log"(file will be created in program directory).
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        help = "Name of the log file"
    )]
    pub(super) log: Option<String>,
    /// Do not write log.
    #[arg(short = 'L', long, aliases = ["no_log", "log-no", "log_no"], help = "Do not write log")]
    pub(super) no_log: bool,
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
    pub(super) settings: Option<String>,
    /// Write default program settings file and exit.
    ///
    /// Use this option if you keep using the same arguments. Modify default settings to suit your needs. Allows modifiying program behaviour even more, e.g. changing output plugin header, colors of messages or paths used for game configuration file auto-discovery.
    ///
    /// File will be created in program directory with name "<program_name>.toml" by default. Backup of old settings file will be saved with ".backup" extension. Use --settings to provide another path. Keep in mind that non-default settings file path should be explicitly provided every time you want to use it.
    ///
    /// This flag conflicts with everything except --settings, --no-color, --log, --no-log.
    #[arg(long, aliases = ["settings_write", "write-settings", "write_settings"], help = "Write default program settings file and exit")]
    pub(super) settings_write: bool,
    /// Ignore non-critical errors, e.g. missing plugin. May be useful, though it's better to fix underlying problems.
    #[arg(
        conflicts_with = "settings_write",
        long,
        aliases = ["ignore_errors", "ignore-error", "ignore_error", "errors-ignore", "errors_ignore", "error-ignore", "error_ignore"],
        help = "Ignore non-critical errors"
    )]
    pub(super) ignore_errors: bool,
    /// Print help for the specific option. Accepts both short and long option names.
    ///
    /// Short help(-h) combined with this option(-?) is a convenient alternative to long help(--help).
    #[arg(
        short = '?',
        aliases = ["help_option", "option-help", "option_help"],
        long,
        help = "Print help for the specific option",
        value_name = "OPTION",
        allow_hyphen_values = true
    )]
    pub(super) help_option: Option<String>,
    /// Place all leveled lists into the output plugin.
    ///
    /// Only merged leveled lists that differ from the last loaded instance of leveled list are placed into the output plugin by default. See --no-summary for details.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        short = 'a',
        long,
        aliases = ["all_lists", "all", "lists-all", "lists_all"],
        help = "Place all leveled lists into the output plugin"
    )]
    pub(super) all_lists: bool,
    /// Do not process last <N> plugins from load order.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        short = 'k',
        long,
        aliases = ["skip_last", "last-skip", "last_skip"],
        help = "Do not process last <N> plugins",
        value_name = "0",
        value_parser = clap::value_parser!(usize)
    )]
    pub(super) skip_last: Option<usize>,
    /// Do not process these plugins. Use it if you want to skip something from processing.
    ///
    /// Plugins produced by delevel or merging tools are a good example to skip. By default "Merged Objects.esp"(TES3Merge) and "merged.omwaddon"(Delta) are added to this list automatically. Use --no-skip-default if you don't want to skip those for some reason.
    ///
    /// Program's own previous output plugins are also automatically skipped from processing.
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
    pub(super) skip: Option<Vec<String>>,
    /// Do not skip plugins defined by default: "Merged Objects.esp"(TES3Merge) and "merged.omwaddon"(Delta).
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        long,
        alias = "no_skip_default",
        help = "Do not skip plugins defined by default"
    )]
    pub(super) no_skip_default: bool,
    /// Do not process creature leveled lists.
    ///
    /// This flag conflicts with --no-items.
    #[arg(
        help_heading = "Filters",
        conflicts_with_all = ["settings_write", "no_items"],
        long,
        aliases = ["no_creatures", "creatures-no", "creatures_no", "no-creature", "no_creature", "creature-no", "creature_no"],
        help = "Do not process creature leveled lists"
    )]
    pub(super) no_creatures: bool,
    /// Do not process item leveled lists.
    ///
    /// This flag conflicts with --no-creatures.
    #[arg(
        help_heading = "Filters",
        conflicts_with_all = ["settings_write", "no_creatures"],
        long,
        aliases = ["no_items", "items-no", "items_no", "no-item", "no_item", "item-no", "item_no"],
        help = "Do not process item leveled lists"
    )]
    pub(super) no_items: bool,
    /// Do not delete subrecords from leveled lists.
    ///
    /// This flag conflicts with --extended-delete.
    #[arg(
        help_heading = "Subrecord deletion",
        conflicts_with_all = ["settings_write", "extended_delete", "always_delete", "never_delete", "threshold_creatures", "threshold_items", "no_threshold_warnings"],
        short = 'D',
        long,
        aliases = ["no_delete", "delete-no", "delete_no"],
        help = "Do not delete subrecords from leveled lists"
    )]
    pub(super) no_delete: bool,
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
        aliases = ["extended_delete", "delete-extended", "delete_extended"],
        help = "Enable extended delete mode"
    )]
    pub(super) extended_delete: bool,
    /// List of plugins to delete subrecords. Subrecords from leveled lists originating from these plugins may be deleted. It's made specifically for the base game plugins. Tamriel_Data is also considered base game in this case.
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
        aliases = ["always_delete", "delete-always", "delete_always"],
        value_name = "PLUGIN(S)",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true,
        value_delimiter = ',',
        help = "List of plugins to delete subrecords"
    )]
    pub(super) always_delete: Option<Vec<String>>,
    /// Do not delete subrecords from leveled lists introduced by these plugins.
    ///
    /// Some rare plugins were not designed for deletion of subrecords in merged leveled lists. For example, plugin "abotWaterLife" has item leveled list "ab01random_ingredient" with 66 ingredients. Plugin "abotWaterLifeTRaddon" also contains the same list with 5 ingredients only(TR specific). This list was clearly designed to be merged together to produce 71 ingredients. Common approach(that this tool relies on) is to have those 5 ingredients added to previously introduced 66 items in a subsequent list.
    ///
    /// Pass empty string "" to disable. May take either one or multiple comma-separated plugin names, see --always-delete for examples.
    ///
    /// Default value: "Wares-base.esm","abotWaterLife.esm","RepopulatedMorrowind.ESM"
    ///
    /// This flag requires --extended-delete.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete"],
        short = 'N',
        long,
        aliases = ["never_delete", "delete-never", "delete_never"],
        value_name = "PLUGIN(S)",
        value_hint = clap::ValueHint::FilePath,
        use_value_delimiter = true,
        value_delimiter = ',',
        help = "Do not delete subrecords from these plugins"
    )]
    pub(super) never_delete: Option<Vec<String>>,
    /// Threshold for percentage of deleted/initial creature subrecords per each leveled list. Will print warnings when threshold exceeded.
    ///
    /// Default value: 67(%).
    ///
    /// This flag requires --extended-delete. Conflicts with --no-creatures.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete", "no_creatures"],
        long,
        aliases = ["threshold_creatures", "creatures-threshold", "creatures_threshold", "threshold-creature", "threshold_creature", "creature-threshold", "creature_threshold"],
        help = "Threshold for % of deleted/initial creatures per list",
        value_name = "67",
        value_parser = clap::value_parser!(u64).range(0..100)
    )]
    pub(super) threshold_creatures: Option<u64>,
    /// Threshold for percentage of deleted/initial item subrecords per each leveled list. Will print warnings when threshold exceeded.
    ///
    /// Default value: 49(%).
    ///
    /// This flag requires --extended-delete. Conflicts with --no-items.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete", "no_items"],
        long,
        aliases = ["threshold_items", "items-threshold", "items_threshold", "threshold-item", "threshold_item", "item-threshold", "item_threshold"],
        help = "Threshold for % of deleted/initial items per list",
        value_name = "49",
        value_parser = clap::value_parser!(u64).range(0..100)
    )]
    pub(super) threshold_items: Option<u64>,
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
        alias = "no_threshold_warnings",
        help = "Do not show threshold warnings"
    )]
    pub(super) no_threshold_warnings: bool,
    /// Delevel subrecords mode.
    ///
    /// By default it delevels everything to level 1, deleveled lists are placed into the output plugin. Use --delev-to to set different level to delevel to. Use --delev-distinct to place deleveled lists into different output plugin.
    ///
    /// This flag is required by all other --delev-* flags.
    #[arg(
        help_heading = "Delev",
        conflicts_with = "settings_write",
        short,
        long,
        alias = "delevel",
        help = "Delevel subrecords mode"
    )]
    pub(super) delev: bool,
    /// Set level to delevel subrecords to.
    ///
    /// Subrecords level will be set to this value if it's higher. Use --delev-creatures-to or --delev-items-to in addition(or alternatively) if you need to set different values for creatures and items.
    ///
    /// Default value: 1.
    ///
    /// This flag requires --delev.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with = "settings_write",
        short = 't',
        long,
        aliases = ["delev_to", "delevel-to", "delevel_to"],
        help = "Set level to delevel subrecords to",
        value_name = "1",
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    pub(super) delev_to: Option<u16>,
    /// Set level to delevel creature subrecords to.
    ///
    /// This flag requires --delev. Conflicts with --no-creatures, --delev-no-creatures.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write", "no_creatures", "delev_no_creatures"],
        long,
        aliases = ["delev_creatures_to", "delev-creature-to", "delev_creature_to", "delevel-creatures-to", "delevel-creature-to", "delevel_creatures_to", "delevele_creature_to"],
        help = "Set level to delevel creature subrecords to",
        value_name = "LVL",
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    pub(super) delev_creatures_to: Option<u16>,
    /// Set level to delevel item subrecords to.
    ///
    /// This flag requires --delev. Conflicts with --no-items, --delev-no-items.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write", "no_items", "delev_no_items"],
        long,
        aliases = ["delev_items_to", "delev-item-to", "delev_item_to", "delevel-items-to", "delevel-item-to", "delevel_items_to", "delevele_item_to"],
        help = "Set level to delevel item subrecords to",
        value_name = "LVL",
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    pub(super) delev_items_to: Option<u16>,
    /// Do not delevel creature subrecords.
    ///
    /// This flag requires --delev. Conflicts with --delev-no-items.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write", "delev_no_items"],
        long,
        aliases = ["delev_no_creatures", "delev-creatures-no", "delev_creatures_no", "delev-no-creature", "delev_no_creature", "delev-creature-no", "delev_creature_no", "delevel-no-creatures", "delevel_no_creatures", "delevel-creatures-no", "delevel_creatures_no", "delevel-no-creature", "delevel_no_creature", "delevel-creature-no", "delevel_creature_no"],
        help = "Do not delevel creature subrecords"
    )]
    pub(super) delev_no_creatures: bool,
    /// Do not delevel item subrecords.
    ///
    /// This flag requires --delev. Conflicts with --delev-no-creatures.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write", "delev_no_creatures"],
        short = 'I',
        long,
        aliases = ["delev_no_items", "delev-items-no", "delev_items_no", "delev-no-item", "delev_no_item", "delev-item-no", "delev_item_no", "delevel-no-items", "delevel_no_items", "delevel-items-no", "delevel_items_no", "delevel-no-item", "delevel_no_item", "delevel-item-no", "delevel_item_no"],
        help = "Do not delevel item subrecords"
    )]
    pub(super) delev_no_items: bool,
    /// Place deleveled lists into the additional output plugin.
    ///
    /// Deleveled lists are placed into the output plugin by default. Use this option to separate merged and deleveled lists. By default additional plugin has the same name as the output plugin with added infix " - Delev", e.g. "MergedLeveledLists - Delev.esp". Use --delev-output to set custom name.
    ///
    /// This flag requires --delev.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write"],
        long,
        aliases = ["delev_distinct", "delevel-distinct", "delevel_distinct"],
        help = "Place deleveled lists into the additional output plugin"
    )]
    pub(super) delev_distinct: bool,
    /// Name of the distinct delev output plugin.
    ///
    /// Same as --output option, see --output for details. The only difference is that the default value is empty, so the file has the same name as the output plugin with added infix " - Delev", e.g. "MergedLeveledLists - Delev.esp".
    ///
    /// Default value: "".
    ///
    /// This flag requires --delev-distinct.
    #[arg(
        help_heading = "Delev",
        requires = "delev_distinct",
        conflicts_with = "settings_write",
        long,
        aliases = ["delev_output", "delevel-output", "delevel_output"],
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        help = "Name of the distinct delev output plugin"
    )]
    pub(super) delev_output: Option<String>,
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
    pub(super) verbose: u8,
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
    pub(super) quiet: bool,
    /// Show plugins reading progress.
    #[arg(
        help_heading = "Display output",
        conflicts_with = "settings_write",
        short = 'p',
        long,
        alias = "progress",
        help = "Show plugins reading progress"
    )]
    pub(super) progress: bool,
    /// Show plugins reading progress bar.
    ///
    /// This flag implicitly sets --progress.
    #[arg(
        help_heading = "Display output",
        conflicts_with = "settings_write",
        short = 'b',
        long,
        aliases = ["bar", "progress_bar", "bar-progress", "bar_progress"],
        help = "Show plugins reading progress bar"
    )]
    pub(super) progress_bar: bool,
    /// Show colored output.
    #[arg(help_heading = "Display output", short = 'C', long, alias = "color", help = "Show colored output")]
    pub(super) color: bool,
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
        alias = "no_summary",
        verbatim_doc_comment,
        help = "Do not show summary"
    )]
    pub(super) no_summary: bool,
}

fn arg_get_help(arg: &Arg) -> Result<StyledStr> {
    match arg.get_long_help() {
        Some(help) => Ok(help.clone()),
        None => match arg.get_help() {
            Some(help) => Ok(help.clone()),
            None => Err(anyhow!("Error: failed to find help for \"{}\" argument", arg.get_id())),
        },
    }
}

fn check_long_arg_names_and_aliases(string: &str, command: &clap::Command) -> Result<()> {
    let mut string = string.to_lowercase().replace('-', "_");
    if let Some(stripped) = string.strip_prefix("__") {
        string = stripped.to_owned();
    }
    match string.as_ref() {
        "help" => return Err(anyhow!("Print help (see a summary with '-h')")),
        "version" => return Err(anyhow!("Print version")),
        _ => {
            for arg in command.get_arguments() {
                if arg.get_id() == &string {
                    return Err(anyhow!(arg_get_help(arg)?));
                } else if let Some(vec) = arg.get_all_aliases() {
                    for alias in vec {
                        if alias.to_lowercase().replace('-', "_") == string {
                            return Err(anyhow!(arg_get_help(arg)?));
                        }
                    }
                }
            }
        }
    };
    Ok(())
}

fn check_short_arg_names_and_aliases(string: &str, command: &clap::Command) -> Result<()> {
    let string = match string.strip_prefix('-') {
        Some(stripped) => stripped.to_owned(),
        None => string.to_owned(),
    };
    if string.len() == 1 {
        let character = string.chars().next().expect("string is empty");
        match character {
            'h' => return Err(anyhow!("Print help (see more with '--help')")),
            'V' => return Err(anyhow!("Print version")),
            _ => {
                for arg in command.get_arguments() {
                    if let Some(short) = arg.get_short() {
                        if short == character {
                            return Err(anyhow!(arg_get_help(arg)?));
                        }
                    };
                    if let Some(vec) = arg.get_all_short_aliases() {
                        for alias in vec {
                            if alias == character {
                                return Err(anyhow!(arg_get_help(arg)?));
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(())
}

fn check_show_help_for_option(options: &Options) -> Result<()> {
    if let Some(string) = &options.help_option {
        let command = Options::command();
        check_long_arg_names_and_aliases(string, &command)?;
        check_short_arg_names_and_aliases(string, &command)?;
        Err(anyhow!(
            "Failed to find option \"{}\" to show help for it. Use \"-h\" to get list of available options.",
            string
        ))
    } else {
        Ok(())
    }
}

pub(super) fn get_options() -> Result<Options> {
    let options = Options::try_parse()?;
    check_show_help_for_option(&options)?;
    Ok(options)
}
