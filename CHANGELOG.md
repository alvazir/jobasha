<!-- markdownlint-disable MD013 -->
<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD036 -->
# Changelog

## 0.6.0 (2025-01-02)

New features

* Add multipatch similar to tes3cmd, TES3Merge etc:
  * `--no-multipatch`(short `-P`) to disable.
  * Individual patches may be selected with corresponding options:
    * `--cellnames` makes sure that renamed cells are not renamed back by accident.
    * `--fogbug` sets fog density to a small value for fogbugged cells(0 fog density).
    * `--summons` sets persistent flag for known summons.
  * Add several options that slightly change logic of multipatching or log format.
* Add `--debug`(short `-u`) option to show debug information. It may be provided multiple times similar to `--verbose`:
  * `-uuuuuu` would put a lot of debug information to a log file.
* Add `--settings-comments` to add comments to a settings file when creating it with `--settings-write`.

Feature enhancements

* Allow ignoring empty or corrupted plugins with `--ignore-errors`(thanks to Falc77 for report).
* Make settings file much shorter by default(new option `--settings-comments` should be used to return comments).

Fixes

* Show error and quit when settings file passed with `--settings` option doesn't exist. Previously program silently ignored the error.

Miscellaneous

* Improve performance by ~20% by offloading dropping unused objects to another thread.
* Update dependencies, settings version.
* Code refactoring.
* State proper MSRV in Cargo.toml and README instead of current Rust version.
* Binary downloads page:
  * Add `Win7` Windows build to be used with Windows 7+ because Rust [1.78+ requires Windows 10+](https://releases.rs/docs/1.78.0/#compiler).
  * Drop `GNU` Windows build because it's not needed.
  * Remove `-msvc` suffix from Windows 10+ build's folder.

## 0.5.0 (2024-01-30)

**Breaking changes**

* Several short option names has been changed(--no-delete, --extended-delete).

New features

* Add `--delev-random`(short `-r`) to delevel to a random level between original and target levels.
* Skip multipatch.esp(tes3cmd) by default(optional).
* Make program halt at the end and wait for keypress when run without arguments. That's primarily done to prevent terminal window immediately closing when run in Windows by double-click(thanks to Walkihr for feedback).

Fixes

* Fix formatting and bug in threshold messages output.

Miscellaneous

* Add `Docs` folder to release with settings(jobasha.toml) and help files(help_brief.txt, help_extended.txt). That's to be used when program is run in Windows by double-click(thanks to Walkihr for feedback).
* Print suggestion to add `--all-lists` when multipatch.esp is skipped.
* Add program run configuration output to log file(or to display with very verbose output -vvvv), showing used command-line arguments, non-default options and settings.

## 0.4.0 (2024-01-19)

**Breaking changes**

* Option to add datestamp to the output plugin name is now opt-in(`--date`) instead of opt-out(`--no-date`).
* Several short option names has been changed(--color, --progress, --progress-bar).

New features

* Add "hidden" OpenMW-CS data directory to the list of plugin directories.
* Add `--compare-only` option to only compare 2 plugins without merging anything.
* Add `--compare-common` option to only show changes to common Masters and Lists.
* Add several options to allow more complicated deleveling. Check help for examples:
  * `--delev-segment`(short `-g`) to make partial deleveling for subrecords with level greater or equal to the value. Example usage: you want to delevel almost everything to level 1, but don't want to encounter anything 21+ at level 1. Passing `--delev-segment 21` would make anything 21+ to be deleveled to 11 minimum by default(roughly halved). Formula is:
    * new-level = delev-to + (delev-segment - delev-to) * (delev-segment-ratio / 100%)
      * default value for delev-to is 1
      * default value for delev-segment-ratio is 50%
  * `--delev-segment-progressive`(short `-G`) to make multiple equal segments, e.g. 21+, 41+ etc.
  * `--delev-segment-ratio`(defaults to 50%) to set minimal level to delevel to for the segment.

Fixes

* Ignore plugins that contain non-TES3 record types(CELL::XSCL, TES3::FORM).

Miscellaneous

* Print suggestion to add `--all-lists` when merged.omwaddon is skipped.
* Print suggestion to add `--all-lists` or set `LEVC = false` and `LEVI = false` in TES3Merge.ini when Merged Objects.esp is skipped.
* Provide exit code 3 when plugins differ in --compare-only mode.
* Add `musl` Linux build to be used when glibc shipped with OS is old.

## 0.3.0 (2023-12-05)

**Breaking changes**

* Rename `--no-creatures` to `--skip-creatures` to be in line with similar options.
* Rename `--no-items` to `--skip-items` to be in line with similar options.

New features

* Compare plugins:
  * Enabled by default and compares output plugin with it's previous version(if there is one).
  * Add `--no-compare` option to disable comparison.
  * Add `--compare-with` option to compare with another plugin instead of previous version.
* New delevel filters:
  * Add `--delev-skip-list` option to skip leveled lists by name.
  * Add `--delev-skip-subrecords` option to skip subrecords(items, creatures) by name.
* Auto-backup of previous output plugin(s):
  * Add `--no-backup` option to stop making backups of output plugins, log file and settings file.

Miscellaneous

* Rename instead of copy for backups to preserve timestamps.
* Increase verbosity level required to display details on deleted subrecords.
* All backups now have `.backup` file extension.
* Reword several messages.

## 0.2.1 (2023-11-20)

Fixes

* Ignore plugins that contain non-TES3 record types(LUAL) automatically thanks to GeneralUlfric's report.

## 0.2.0 (2023-11-18)

**Breaking changes**

* Several option names has been changed.

New features

* Delevel leveled lists:
  * By default all lists are deleveled to level 1 with `--delev` option(short `-d`).
  * You may set another level to delevel to with `--delev-to` option(short `-t`):
    * `jobasha -dt 5` to delevel everything higher than level 5 to level 5.
  * You may set different levels to delevel to for creatures and items.
  * You may opt out from deleveling either creatures or items:
    * `jobasha --delev --delev-no-items` to delevel only creatures.
  * You may place deleveled list into a separate output plugin.
* Skip Merged Objects.esp(TES3Merge) and merged.omwaddon(Delta) by default(optional).
* Ignore omwscripts plugins automatically(thanks to Zerafall's report).
* `-?` option to get help for individual option, because extended help `jobasha --help` became too long to quickly find something.

Feature enhancements

* Change summary format to adapt for delev stats.
* Auto-backup of previous log and settings files.
* More forgiving argument names processing. For example `--run-dry` would be treated as correct form of `--dry-run`.

Fixes

* Display proper error when encountering empty/corrupted game configuration file(thanks to Zerafall's report).

Miscellaneous

* Change license from dual MIT and UNLICENSE to GNU GPLv3.
* Rename program from "Jobasha - yet another TES3 leveled list tool" to "Jobasha - TES3 leveled list merging and deleveling tool".
* Add system requirements to description.
* Update versions of rust and all dependencies, notably tes3 library to latest commit(2fae07a0).
* Improve filesystem-related error messages.
* Switch --color, --progress and --progress-bar behaviour from opt-out to opt-in for maximum compatibility with Windows.

## 0.1.0 (2023-04-20)

Features

* Merge leveled lists
* Delete subrecords from merged leveled lists
* Show detailed information
