<!-- markdownlint-disable MD013 -->
<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD036 -->
# Changelog

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
  * By default all lists are deleveled to level 1 with `--delev` flag(short `-d`).
  * You may set another level to delevel to with `--delev-to` flag(short `-t`):
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
