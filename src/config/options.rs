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
/// Jobasha - TES3 leveled list tool
///
/// Author: alvazir
/// License: GNU GPLv3
/// GitHub: https://github.com/alvazir/jobasha
/// Nexus Mods: https://www.nexusmods.com/morrowind/mods/52707
pub(super) struct Options {
    /// Path to the game configuration file, e.g.: "C:\Users\Username\Documents\My Games\OpenMW\openmw.cfg"(absolute), "../Morrowind.ini"(relative). May be used to provide alternative game configuration file or in case the game configuration file is not found automatically.
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
    /// Log contains display output of the program as if it was run with maximum verboseness. It is enabled by default, use --no-log to disable. Previous log will be saved with ".backup" extension.
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
    /// This flag conflicts with everything except --settings, --log, --no-log, --color, --no-backup.
    #[arg(long, aliases = ["settings_write", "write-settings", "write_settings"], help = "Write default program settings file and exit")]
    pub(super) settings_write: bool,
    /// Do not make backups.
    ///
    /// By default output plugins, log file and settings file are backed up before rewriting.
    #[arg(long, aliases = ["no_backup", "backup-no", "backup_no"], help = "Do not make backups")]
    pub(super) no_backup: bool,
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
    /// Skip plugins that contain unexpected record types.
    ///
    /// Some new plugins may contain record types not defined in TES3 plugin "specification". You will encounter "Unexpected Tag: ..." error then. Use this option to skip plugins with unexpected tags. Consider reporting newly appeared record types so that they'd be added to the list of unexpected tags to skip by default.
    ///
    /// By default program skips plugins that contain "LUAL" records. See --no-skip-unexpected-tags-default.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        long,
        alias = "skip_unexpected_tags",
        help = "Skip plugins that contain unexpected record types"
    )]
    pub(super) skip_unexpected_tags: bool,
    /// Do not skip plugins that contain known unexpected record types.
    ///
    /// By default program skips plugins that contain "LUAL" records. See --skip-unexpected-tags for details.
    #[arg(
        help_heading = "Filters",
        conflicts_with = "settings_write",
        long,
        alias = "no_skip_unexpected_tags_default",
        help = "Do not skip plugins that contain known unexpected record types"
    )]
    pub(super) no_skip_unexpected_tags_default: bool,
    /// Do not process creature leveled lists.
    ///
    /// This flag conflicts with --skip-items.
    #[arg(
        help_heading = "Filters",
        conflicts_with_all = ["settings_write", "skip_items"],
        long,
        aliases = ["skip_creatures", "creatures-skip", "creatures_skip", "skip-creature", "skip_creature", "creature-skip", "creature_skip"],
        help = "Do not process creature leveled lists"
    )]
    pub(super) skip_creatures: bool,
    /// Do not process item leveled lists.
    ///
    /// This flag conflicts with --skip-creatures.
    #[arg(
        help_heading = "Filters",
        conflicts_with_all = ["settings_write", "skip_creatures"],
        long,
        aliases = ["skip_items", "items-skip", "items_skip", "skip-item", "skip_item", "item-skip", "item_skip"],
        help = "Do not process item leveled lists"
    )]
    pub(super) skip_items: bool,
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
    /// This flag requires --extended-delete. Conflicts with --skip-creatures.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete", "skip_creatures"],
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
    /// This flag requires --extended-delete. Conflicts with --skip-items.
    #[arg(
        help_heading = "Subrecord deletion",
        requires = "extended_delete",
        conflicts_with_all = ["settings_write", "no_delete", "skip_items"],
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
    /// This flag requires --delev. Conflicts with --skip-creatures, --delev-skip-creatures.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write", "skip_creatures", "delev_skip_creatures"],
        long,
        aliases = ["delev_creatures_to", "delev-creature-to", "delev_creature_to", "delevel-creatures-to", "delevel-creature-to", "delevel_creatures_to", "delevele_creature_to"],
        help = "Set level to delevel creature subrecords to",
        value_name = "LVL",
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    pub(super) delev_creatures_to: Option<u16>,
    /// Set level to delevel item subrecords to.
    ///
    /// This flag requires --delev. Conflicts with --skip-items, --delev-skip-items.
    #[arg(
        help_heading = "Delev",
        requires = "delev",
        conflicts_with_all = ["settings_write", "skip_items", "delev_skip_items"],
        long,
        aliases = ["delev_items_to", "delev-item-to", "delev_item_to", "delevel-items-to", "delevel-item-to", "delevel_items_to", "delevele_item_to"],
        help = "Set level to delevel item subrecords to",
        value_name = "LVL",
        value_parser = clap::value_parser!(u16).range(1..)
    )]
    pub(super) delev_items_to: Option<u16>,
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
    /// Do not delevel creature subrecords.
    ///
    /// This flag requires --delev. Conflicts with --delev-skip-items.
    #[arg(
        help_heading = "Delev filters",
        requires = "delev",
        conflicts_with_all = ["settings_write", "delev_skip_items"],
        long,
        aliases = ["delev_skip_creatures", "delev-creatures-skip", "delev_creatures_skip", "delev-skip-creature", "delev_skip_creature", "delev-creature-skip", "delev_creature_skip", "delevel-skip-creatures", "delevel_skip_creatures", "delevel-creatures-skip", "delevel_creatures_skip", "delevel-skip-creature", "delevel_skip_creature", "delevel-creature-skip", "delevel_creature_skip"],
        help = "Do not delevel creature subrecords"
    )]
    pub(super) delev_skip_creatures: bool,
    /// Do not delevel item subrecords.
    ///
    /// This flag requires --delev. Conflicts with --delev-skip-creatures.
    #[arg(
        help_heading = "Delev filters",
        requires = "delev",
        conflicts_with_all = ["settings_write", "delev_skip_creatures"],
        short = 'I',
        long,
        aliases = ["delev_skip_items", "delev-items-skip", "delev_items_skip", "delev-skip-item", "delev_skip_item", "delev-item-skip", "delev_item_skip", "delevel-skip-items", "delevel_skip_items", "delevel-items-skip", "delevel_items_skip", "delevel-skip-item", "delevel_skip_item", "delevel-item-skip", "delevel_item_skip"],
        help = "Do not delevel item subrecords"
    )]
    pub(super) delev_skip_items: bool,
    /// Do not delevel these lists.
    ///
    /// Use this option to skip leveled lists from deleveling. Additionaly use --delev-no-skip-list to further refine your rules with lists you want to delevel even if they fit skip patterns.
    ///
    /// Following examples are illustrated with Bloodmoon's wolfpack and werewolf leveled lists:
    ///     - bm_ex_wolfpack, bm_ex_wolfpack_20, bm_ex_wolfpack_40, bm_ex_wolfpack_60
    ///     - bm_werewolf_wilderness01 - bm_werewolf_wilderness09
    ///
    /// There are 4 pattern types, they are processed in the following order:
    ///
    ///   "exact"
    ///     - Default type. Leveled list matches if it's name is exactly the same as the pattern.
    ///     - Example: --delev-skip-list "bm_ex_wolfpack" would only skip that exact list.
    ///
    ///   "prefix"
    ///     - Defined by prepending with "prefix:". Leveled list matches if it's name starts with the pattern.
    ///     - Example: --delev-skip-list "prefix:bm_ex_" would skip all bm_ex_wolfpack* leveled lists.
    ///
    ///   "suffix"
    ///     - Defined by prepending with "suffix:". Leveled list matches if it's name ends with the pattern.
    ///     - Example: --delev-skip-list "suffix:_40" would only skip bm_ex_wolfpack_40.
    ///
    ///   "infix"
    ///     - Defined by prepending with "infix:". Leveled list matches if it's name contains the pattern.
    ///     - Example: --delev-skip-list "infix:wolf" would skip all those lists.
    ///
    /// May take either one or multiple comma-separated plugin names, e.g.: "bm_ex_wolfpack"(one), prefix:bm_ex_,suffix:_40(many). Pay attention that there is no space after comma. Use double-quotes around list names with spaces. Case-insensitive. May be used multiple times instead of providing comma-separated list, e.g.: --delev-skip-list prefix:bm_ex_ --delev-skip-list suffix:-40.
    ///
    /// This flag requires --delev.
    #[arg(
        help_heading = "Delev filters",
        requires = "delev",
        conflicts_with = "settings_write",
        long,
        aliases = ["delev_skip_list", "delev-list-skip", "delev_list_skip", "delev-skip-lists", "delev_skip_lists", "delev-lists-skip", "delev_lists_skip", "delevel-skip-list", "delevel_skip_list", "delevel-list-skip", "delevel_list_skip", "delevel-skip-lists", "delevel_skip_lists", "delevel-lists-skip", "delevel_lists_skip"],
        help = "Do not delevel these lists",
        value_name = "LIST(S)",
        use_value_delimiter = true,
        value_delimiter = ',',
        verbatim_doc_comment
    )]
    pub(super) delev_skip_list: Option<Vec<String>>,
    /// Delevel these lists even if they match --delev-skip-list.
    ///
    /// The opposite of --delev-skip-list. Patterns work the same.
    ///
    /// Following examples are illustrated with Bloodmoon's wolfpack and werewolf leveled lists:
    ///     - bm_ex_wolfpack, bm_ex_wolfpack_20, bm_ex_wolfpack_40, bm_ex_wolfpack_60
    ///     - bm_werewolf_wilderness01 - bm_werewolf_wilderness09
    ///
    /// The best way to describe is to continue examples started in --delev-skip-list:
    ///
    ///  --delev-skip-list "prefix:bm_" --delev-no-skip-list "prefix:bm_ex_" would skip werewolf lists
    ///
    ///  --delev-skip-list "infix:wolf" --delev-no-skip-list "suffix:09" would skip everything except bm_werewolf_wilderness09
    ///
    /// This flag requires --delev-skip-list.
    #[arg(
        help_heading = "Delev filters",
        requires = "delev_skip_list",
        conflicts_with = "settings_write",
        long,
        aliases = ["delev_no_skip_list", "delev-list-no-skip", "delev_list_no_skip", "delev-no-skip-lists", "delev_no_skip_lists", "delev-lists-no-skip", "delev_lists_no_skip", "delevel-no-skip-list", "delevel_no_skip_list", "delevel-list-no-skip", "delevel_list_no_skip", "delevel-no-skip-lists", "delevel_no_skip_lists", "delevel-lists-no-skip", "delevel_lists_no_skip", "delev-skip-no-list", "delev_skip_no_list", "delev-list-skip-no", "delev_list_skip_no", "delev-skip-no-lists", "delev_skip_no_lists", "delev-lists-skip-no", "delev_lists_skip_no", "delevel-skip-no-list", "delevel_skip_no_list", "delevel-list-skip-no", "delevel_list_skip_no", "delevel-skip-no-lists", "delevel_skip_no_lists", "delevel-lists-skip-no", "delevel_lists_skip_no"],
        help = "Delevel these lists even if they match --delev-skip-list",
        value_name = "LIST(S)",
        use_value_delimiter = true,
        value_delimiter = ',',
        verbatim_doc_comment
    )]
    pub(super) delev_no_skip_list: Option<Vec<String>>,
    /// Do not delevel these subrecords. Works exactly as --delev-skip-list, but filters out subrecords instead of lists.
    ///
    /// This flag requires --delev.
    #[arg(
        help_heading = "Delev filters",
        requires = "delev",
        conflicts_with = "settings_write",
        long,
        aliases = ["delev_skip_subrecord", "delev-subrecord-skip", "delev_subrecord_skip", "delev-skip-subrecords", "delev_skip_subrecords", "delev-subrecords-skip", "delev_subrecords_skip", "delevel-skip-subrecord", "delevel_skip_subrecord", "delevel-subrecord-skip", "delevel_subrecord_skip", "delevel-skip-subrecords", "delevel_skip_subrecords", "delevel-subrecords-skip", "delevel_subrecords_skip"],
        help = "Do not delevel these subrecords",
        value_name = "SUBRECORD(S)",
        use_value_delimiter = true,
        value_delimiter = ',',
        verbatim_doc_comment
    )]
    pub(super) delev_skip_subrecord: Option<Vec<String>>,
    /// Delevel these lists even if they match --delev-skip-subrecord. Works exactly as --delev-no-skip-list, but filters out subrecords instead of lists.
    ///
    /// This flag requires --delev-skip-subrecord.
    #[arg(
        help_heading = "Delev filters",
        requires = "delev_skip_subrecord",
        conflicts_with = "settings_write",
        long,
        aliases = ["delev_no_skip_subrecord", "delev-subrecord-no-skip", "delev_subrecord_no_skip", "delev-no-skip-subrecords", "delev_no_skip_subrecords", "delev-subrecords-no-skip", "delev_subrecords_no_skip", "delevel-no-skip-subrecord", "delevel_no_skip_subrecord", "delevel-subrecord-no-skip", "delevel_subrecord_no_skip", "delevel-no-skip-subrecords", "delevel_no_skip_subrecords", "delevel-subrecords-no-skip", "delevel_subrecords_no_skip", "delev-skip-no-subrecord", "delev_skip_no_subrecord", "delev-subrecord-skip-no", "delev_subrecord_skip_no", "delev-skip-no-subrecords", "delev_skip_no_subrecords", "delev-subrecords-skip-no", "delev_subrecords_skip_no", "delevel-skip-no-subrecord", "delevel_skip_no_subrecord", "delevel-subrecord-skip-no", "delevel_subrecord_skip_no", "delevel-skip-no-subrecords", "delevel_skip_no_subrecords", "delevel-subrecords-skip-no", "delevel_subrecords_skip_no"],
        help = "Delevel these subrecords even if they match --delev-skip-subrecord",
        value_name = "SUBRECORD(S)",
        use_value_delimiter = true,
        value_delimiter = ',',
        verbatim_doc_comment
    )]
    pub(super) delev_no_skip_subrecord: Option<Vec<String>>,
    /// Do not compare plugins.
    ///
    /// By default output plugin is compared with previous version if there is one(same filename). It's not written if previous version is the same.
    #[arg(
        help_heading = "Compare",
        conflicts_with = "settings_write",
        long, aliases = ["no_compare", "compare-no", "compare_no"], help = "Do not compare plugins")]
    pub(super) no_compare: bool,
    /// Plugin to compare output plugin with.
    ///
    /// This flag allows to compare output plugin with any other plugin. By default output plugin is compared with previous version if there is one(same filename).
    ///
    /// This flag conflicts with --no-compare.
    #[arg(
        help_heading = "Compare",
        conflicts_with_all = ["settings_write", "no_compare"],
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        alias = "compare_with",
        help = "Plugin to compare output plugin with"
    )]
    pub(super) compare_with: Option<String>,
    /// Plugin to compare delev output plugin with.
    ///
    /// This flag allows to compare delev plugin with any other plugin. By default delev plugin is compared with previous version if there is one(same filename).
    ///
    /// This flag requires --delev-distinct. Conflicts with --no-compare.
    #[arg(
        help_heading = "Compare",
        requires = "delev_distinct",
        conflicts_with_all = ["settings_write", "no_compare"],
        long,
        value_name = "PATH",
        value_hint = clap::ValueHint::Other,
        aliases = ["compare_delev_with", "delev-compare-with", "delev_compare_with"],
        help = "Plugin to compare delev output plugin with"
    )]
    pub(super) compare_delev_with: Option<String>,
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