<!-- markdownlint-disable MD013 -->
<!-- markdownlint-disable MD033 -->

# Jobasha

Yet another TES3 leveled list tool.  

## Description

Jobasha is a [command line](https://en.wikipedia.org/wiki/Command-line_interface) tool for [leveled lists](https://en.uesp.net/wiki/Morrowind:Leveled_Lists) manipulation.  

Features:  
[x] [Merge leveled lists](https://en.uesp.net/wiki/Morrowind_Mod:Leveled_Lists#List_Merging)  
[x] Delete subrecords from merged leveled lists  
[x] Process both **Morrowind.ini** and **openmw.cfg**  
[x] Show detailed information  
[ ] Delevel  
[ ] Diff  
[ ] (maybe) Multipatch  
[ ] (maybe) Shell completions  

## Usage

<details>

<summary>Type command `jobasha -h` for brief help</summary>

```text
Jobasha - Yet Another TES3 Leveled List Tool

Usage: jobasha [OPTIONS]

Options:
  -c, --config <PATH>      Path to the game configuration file
  -o, --output <PATH>      Name of the output plugin
  -O, --output-dir <PATH>  Name of the output plugin directory
  -n, --no-date            Do not add date to the output plugin name
  -d, --dry-run            Do not write output plugin
  -l, --log <PATH>         Name of the log file
  -L, --no-log             Do not write log
  -s, --settings <PATH>    Name of the program settings file
      --settings-write     Write default program settings file and exit
      --ignore-errors      Ignore non-critical errors
  -h, --help               Print help (see more with '--help')
  -V, --version            Print version

Filters:
  -a, --all-lists         Place all leveled lists into the output plugin
  -k, --skip-last <0>     Do not process last <N> plugins
  -K, --skip <PLUGIN(S)>  Do not process these plugins
      --no-creatures      Do not process creature leveled lists
      --no-items          Do not process item leveled lists

Subrecord deletion:
  -D, --no-delete                  Do not delete subrecords from leveled lists
  -e, --extended-delete            Enable extended delete mode
  -A, --always-delete <PLUGIN(S)>  List of plugins to delete subrecords
  -N, --never-delete <PLUGIN(S)>   Do not delete subrecords from these plugins
      --threshold-creatures <67>   Threshold for % of deleted/initial creatures per list
      --threshold-items <49>       Threshold for % of deleted/initial items per list
  -T, --no-threshold-warnings      Do not show threshold warnings

Display output:
  -v, --verbose...       Show more information
  -q, --quiet            Do not show anything
  -C, --no-color         Do not show colored output
  -P, --no-progress      Do not show plugins reading progress
  -B, --no-progress-bar  Do not show plugins reading progress bar
  -S, --no-summary       Do not show summary
```

</details>
<details>

<summary>Type command `jobasha --help` for extended help</summary>

```text
Jobasha - Yet Another TES3 Leveled List Tool

Author: alvazir
License: Unlicense OR MIT
GitHub: https://github.com/alvazir/jobasha
Nexus Mods: https://www.nexusmods.com/morrowind/mods/52707

Usage: jobasha [OPTIONS]

Options:
  -c, --config <PATH>
          Path to the game configuration file, e.g.: "C:\Users\Username\Documents\My Games\OpenMW\openmw.cfg"(absolute),
          "../Morrowind.ini"(relative). May be used to provide alternative game configuration file or in case the game configuration file was
          not found automatically.

          Default value: ""(automatically search for the game configuration file).

  -o, --output <PATH>
          Name of the output plugin. May be provided as a path, e.g.: "C:\Morrowind\mods\LeveledLists.esp"(absolute),
          "mods/LeveledLists.esp"(relative). Non-existent directories will be created.

          Date is added to the output plugin name by default, e.g. "MergedLeveledLists - YYYY-mm-dd.esp". Use --no-date to disable this
          behaviour.

          Default value: "MergedLeveledLists.esp"(will be placed into the current directory).

  -O, --output-dir <PATH>
          Name of the output plugin directory. May be provided as a path, e.g.: "C:\Morrowind\mods"(absolute), "mods"(relative). Non-existent
          directory will be created.

          Default output plugin name will be used if --output is not provided. This option takes precedence when both --output and --output-dir
          provide directory path.

          Default value: ""(current directory).

  -n, --no-date
          Do not add date to the output plugin name

  -d, --dry-run
          Do not write output plugin

  -l, --log <PATH>
          Name of the log file. May be provided as a path. Non-existent directories will be created.

          Log contains display output of the program as if it was run with maximum verboseness. It is enabled by default, use --no-log to
          disable.

          Default value: "<program_name>.log"(file will be created in program directory).

  -L, --no-log
          Do not write log

  -s, --settings <PATH>
          Name of the program settings file. May be provided as a path. Non-existent directories will be created. Extension will be replaced
          with ".toml".

          Default value: "<program_name>.toml"(file will be created in program directory).

      --settings-write
          Write default program settings file and exit.

          Use this option if you keep using the same arguments. Modify default settings to suit your needs. Allows modifiying program behaviour
          even more, e.g. changing output plugin header, colors of messages or paths used for game configuration file auto-discovery.

          File will be created in program directory with name "<program_name>.toml" by default. Use --settings to provide another path. Keep in
          mind that non-default settings file path should be explicitly provided every time you want to use it.

          This flag conflicts with everything except --settings, --no-color, --log, --no-log.

      --ignore-errors
          Ignore non-critical errors

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Filters:
  -a, --all-lists
          Place all leveled lists into the output plugin.

          Only merged leveled lists that differ from the last loaded instance of leveled list are placed into the output plugin by default. See
          --no-summary for details.

  -k, --skip-last <0>
          Do not process last <N> plugins

  -K, --skip <PLUGIN(S)>
          Do not process these plugins.

          Use it if you want to skip something from processing. For example plugins produced by delevel or merging tools. Program will
          automatically try to skip it's previous output plugin from processing. Use this option if it fails for some reason.

          May take either one or multiple comma-separated plugin names, see --always-delete for examples.

      --no-creatures
          Do not process creature leveled lists.

          This flag conflicts with --no-items.

      --no-items
          Do not process item leveled lists.

          This flag conflicts with --no-creatures.

Subrecord deletion:
  -D, --no-delete
          Do not delete subrecords from leveled lists.

          This flag conflicts with --extended-delete.

  -e, --extended-delete
          Enable extended delete mode.

          Program only deletes subrecords from leveled lists originating from base game plugins by default, see --always-delete. With
          --extended-delete subrecords from any leveled list may be deleted. Threshold checks help to identify potential problems. Warning will
          be displayed when ratio of deleted/initial subrecords per each leveled list exceeds threshold. Then you may adjust thresholds or add
          plugin name to --never-delete. Or disable warnings completely with --no-threshold-warnings.

          This flag conflicts with --no-delete. It is required by --never-delete, --threshold_creatures, --threshold_items,
          --no-threshold-warnings.

  -A, --always-delete <PLUGIN(S)>
          List of plugins to delete subrecords. Subrecords from leveled lists originating from these plugins may be deleted. It's made
          specifically the base game plugins. Tamriel_Data is also considered base game in this case.

          This is the only "delete" option that's used by default. With --extended-delete it skips threshold checks for base game plugins.
          Threshold checks' purpose is to prevent problem presented in --never-delete, but base game leveled lists should be free of this
          problem.

          Pass empty string "" to disable. May take either one or multiple comma-separated plugin names, e.g.: "Morrowind.esm"(one),
          Morrowind.esm,Tribunal.esm(many). Pay attention, that there is no space after comma. Use double-quotes around plugin names with
          spaces. Case-insensitive. May be used multiple times instead of providing comma-separated list, e.g.: --always-delete Morrowind.esm
          --always-delete Tribunal.esm.

          Default value: "Morrowind.esm","Tribunal.esm","Bloodmoon.esm","Tamriel_Data.esm"

          This flag conflicts with --no-delete.

  -N, --never-delete <PLUGIN(S)>
          Do not delete subrecords from leveled lists introduced by these plugins.

          Some rare plugins were not designed for deletion of subrecords in merged leveled lists. For example, plugin "abotWaterLife" has item
          leveled list "ab01random_ingredient" with 66 ingredients. Plugin "abotWaterLifeTRaddon" also contains the same list with 5 ingredients
          only(TR specific). This list was clearly designed to be merged together to produce 71 ingredients. Common approach(that this tool
          relies on) is to have those 5 ingredients added to previously introduced 66 items in a subsequent list.

          Pass empty string "" to disable. May take either one or multiple comma-separated plugin names, see --always-delete for examples.

          Default value: "Wares-base.esm","abotWaterLife.esm","RepopulatedMorrowind.ESP"

          This flag requires --extended-delete.

      --threshold-creatures <67>
          Threshold for percentage of deleted/initial creature subrecords per each leveled list. Will print warnings when threshold exceeded.

          Default value: 67(%).

          This flag requires --extended-delete. Conflicts with --creatures-off.

      --threshold-items <49>
          Threshold for percentage of deleted/initial item subrecords per each leveled list. Will print warnings when threshold exceeded.

          Default value: 49(%).

          This flag requires --extended-delete. Conflicts with --items-off.

  -T, --no-threshold-warnings
          Do not show threshold warnings.

          Warnings are shown when threshold of deleted/initial subrecords is exceeded for leveled list by default.

          This flag requires --extended-delete.

Display output:
  -v, --verbose...
          Show more information. May be provided twice for extra effect.

          This flag conflicts with --quiet.

  -q, --quiet
          Do not show anything.

          This flag conflicts with --verbose.

  -C, --no-color
          Do not show colored output

  -P, --no-progress
          Do not show plugins reading progress.

          This flag conflicts with --no-progress-bar.

  -B, --no-progress-bar
          Do not show plugins reading progress bar. Progress is shown, but progress bar is hidden.

          This flag conflicts with --no-progress.

  -S, --no-summary
          Do not show summary.

          Summary's field names are mostly self explanatory though not all:
          [duration]: Program execution duration in seconds measured from inside the program.
          [records read]: Program has to read all the records to get leveled lists.
          [unique]: Merging is only required for (total - unique) number of leveled lists.
          [placed]: Amount of merged leveled lists placed into the output plugin.
          [untouched]: Lists that are identical to last loaded instance of itself, thus there is no need to place them into the output plugin.
          [masters]: Master subrecords are placed into the output plugin header for every plugin that has leveled list merged and placed.

Notes:
  - Display/log output looks better with monospaced font.
  - Don't clean the output plugin. Cleaning may rarely lead to removal of some leveled lists that should be there.
```

</details>
<details>

<summary>Example display output</summary>

```text
$ ./jobasha --skip "delev.esp"
Log is being written into "/home/alvazir/__OMW/sbox/Data Files/jobasha.log"
Found game configuration file "/home/alvazir/.config/openmw/openmw.cfg"
Plugin "MergedLeveledLists - 2023-04-20.esp" will be skipped, because it has the same name as the output plugin
Plugin "delev.esp" will be skipped, because it's listed as a plugin to skip
Reading plugins: ############################################################################################################# 377/377
579 subrecords from 156 leveled lists were deleted, add --verbose or check log for details
292 merged leveled lists were identical to last loaded lists hence not placed into the output plugin, add -vv or check log for details
Output plugin was written to "MergedLeveledLists - 2023-04-20.esp"
Performance: 1.700s duration, 377 plugins read at 224/s, 412469 records read at 245380/s
Lists stats: 6062 total, 5485 unique, 430 merged, 138 placed / 292 untouched, 24 masters, 579 deleted subrecords

Place "MergedLeveledLists - 2023-04-20.esp" last in load order and activate
```

</details>

## Releases

[Binary downloads](https://www.nexusmods.com/morrowind/mods/52707) are available for GNU/Linux(x86-64), Android(AArch64), Windows(x86-64(MSVC, GNU)), macOS(x86-64, AArch64).

## Building

<details>

<summary>Jobasha is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org) in order to compile it. Jobasha compiles with Rust 1.69.0(stable) or newer</summary>

```shell
git clone https://github.com/alvazir/jobasha
cd jobasha
cargo build --release
./target/release/jobasha --version
```

</details>

## Links

* [Nexus Mods releases](https://www.nexusmods.com/morrowind/mods/52707)  
  * [Report a bug](https://www.nexusmods.com/morrowind/mods/52707/?tab=bugs)  
  * [File a feature request/suggestion](https://www.nexusmods.com/morrowind/mods/52707/?tab=posts)  
* [GitHub repository](https://github.com/alvazir/jobasha)  
  * [File an issue](https://github.com/alvazir/jobasha/issues)  

## License

[Dual-licensed](COPYING) under the [MIT License](LICENSE-MIT) or the [Unlicense](UNLICENSE).  

## Acknowledgments

* This project came to life thanks to the awesome [tes3 library](https://github.com/Greatness7/tes3) by [Greatness7](https://github.com/Greatness7)  
* Tools that also work with leveled lists(and do much more):  
  * [tes3cmd](https://github.com/john-moonsugar/tes3cmd)  
  * [OMWLLF](https://github.com/jmelesky/omwllf)  
  * [TES3Merge](https://github.com/NullCascade/TES3Merge)  
  * [DeltaPlugin](https://gitlab.com/bmwinger/delta-plugin)  
