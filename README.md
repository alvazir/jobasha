<!-- markdownlint-disable MD013 -->
<!-- markdownlint-disable MD033 -->

# Jobasha

TES3 leveled list tool.  

## Description

Jobasha will perfectly organize all your lists in his library. It is a [command line](https://en.wikipedia.org/wiki/Command-line_interface) tool for [leveled list](https://en.uesp.net/wiki/Morrowind:Leveled_Lists) merging, deleveling and comparison.  

## Features

* [Merge leveled lists](https://en.uesp.net/wiki/Morrowind_Mod:Leveled_Lists#List_Merging)  
* Delete subrecords from merged leveled lists  
* Delevel subrecords  
* Compare leveled lists between plugins  
* Process both **Morrowind.ini** and **openmw.cfg**  
* Show detailed information  

## Usage

<!-- markdownlint-disable MD007 -->
* Most basic usage: `jobasha` to merge leveled lists
  <details>

  <summary>The program is flexible and allows complicated scenarios, though basic usage is simple</summary>

    * To provide non-default game configuration file:
      * `jobasha -c C:\another\profile\openmw.cfg`  
    * To delevel subrecords to level 1 in addition to merging leveled lists:
      * `jobasha -d`  
    * To delevel subrecords to level 5 in addition to merging leveled lists:
      * `jobasha -dt 5`
    * Almost everything may be done with command-line arguments, though settings file is the better way for advanced usage:  
      * `jobasha --settings-write` to create settings file  
      * `jobasha` afterwards to use it  

  </details>
* Type command `jobasha -h` for brief help
* Type command `jobasha --help` for extended help
* Type command `jobasha -? <OPTION>` to get extended help for a specific option
* Example outputs:  
  <details>
  
  <summary>Brief help</summary>

  ```text
  Jobasha - TES3 leveled list tool
  
  Usage: jobasha [OPTIONS]
  
  Options:
    -c, --config <PATH>         Path to the game configuration file
    -o, --output <PATH>         Name of the output plugin
    -O, --output-dir <PATH>     Name of the output plugin directory
    -n, --no-date               Do not add date to the output plugin name
        --dry-run               Do not write output plugin
    -l, --log <PATH>            Name of the log file
    -L, --no-log                Do not write log
    -s, --settings <PATH>       Name of the program settings file
        --settings-write        Write default program settings file and exit
        --no-backup             Do not make backups
        --ignore-errors         Ignore non-critical errors
    -?, --help-option <OPTION>  Print help for the specific option
    -h, --help                  Print help (see more with '--help')
    -V, --version               Print version
  
  Filters:
    -a, --all-lists                        Place all leveled lists into the output plugin
    -k, --skip-last <0>                    Do not process last <N> plugins
    -K, --skip <PLUGIN(S)>                 Do not process these plugins
        --no-skip-default                  Do not skip plugins defined by default
        --skip-unexpected-tags             Skip plugins that contain unexpected record types
        --no-skip-unexpected-tags-default  Do not skip plugins that contain known unexpected record types
        --skip-creatures                   Do not process creature leveled lists
        --skip-items                       Do not process item leveled lists
  
  Subrecord deletion:
    -D, --no-delete                  Do not delete subrecords from leveled lists
    -e, --extended-delete            Enable extended delete mode
    -A, --always-delete <PLUGIN(S)>  List of plugins to delete subrecords
    -N, --never-delete <PLUGIN(S)>   Do not delete subrecords from these plugins
        --threshold-creatures <67>   Threshold for % of deleted/initial creatures per list
        --threshold-items <49>       Threshold for % of deleted/initial items per list
    -T, --no-threshold-warnings      Do not show threshold warnings
  
  Delev:
    -d, --delev                     Delevel subrecords mode
    -t, --delev-to <1>              Set level to delevel subrecords to
        --delev-creatures-to <LVL>  Set level to delevel creature subrecords to
        --delev-items-to <LVL>      Set level to delevel item subrecords to
        --delev-distinct            Place deleveled lists into the additional output plugin
        --delev-output <PATH>       Name of the distinct delev output plugin
  
  Delev filters:
        --delev-skip-creatures                    Do not delevel creature subrecords
    -I, --delev-skip-items                        Do not delevel item subrecords
        --delev-skip-list <LIST(S)>               Do not delevel these lists
        --delev-no-skip-list <LIST(S)>            Delevel these lists even if they match --delev-skip-list
        --delev-skip-subrecord <SUBRECORD(S)>     Do not delevel these subrecords
        --delev-no-skip-subrecord <SUBRECORD(S)>  Delevel these subrecords even if they match --delev-skip-subrecord
  
  Compare:
        --no-compare                 Do not compare plugins
        --compare-with <PATH>        Plugin to compare output plugin with
        --compare-delev-with <PATH>  Plugin to compare delev output plugin with
  
  Display output:
    -v, --verbose...    Show more information
    -q, --quiet         Do not show anything
    -p, --progress      Show plugins reading progress
    -b, --progress-bar  Show plugins reading progress bar
    -C, --color         Show colored output
    -S, --no-summary    Do not show summary

  ```

  </details>
  <details>
  
  <summary>Program run display output</summary>

  ```text
  $./jobasha -d
  Log is being written into "/home/alvazir/__OMW/jobasha.log"
  Found game configuration file "/home/alvazir/.config/openmw/openmw.cfg"
  Plugin "MergedLeveledLists - 2023-11-13.esp" will be skipped, because it's name matches output plugin name pattern "mergedleveledlists - "
  555 subrecords from 143 leveled lists were deleted, add -v or check log for details
  97 merged leveled lists were identical to last loaded lists hence not placed into the output plugin, add -vv or check log for details
  6813 subrecords from 925 leveled lists were deleveled, add -vvv or check log for details
  Plugin "MergedLeveledLists - 2023-11-18.esp" was written to "MergedLeveledLists - 2023-11-18.esp"
  Performance: 1.891s duration, 19 plugins(420917 records) read at 10/s(229701/s)
  Lists stats: 6329 total, 5842 unique, 988 placed, 14 masters
  Merge stats: 341 merged, 210 untouched, 131 placed, 555 subrecords deleted
  Delev stats: 925 deleveled, 925 placed, 6813 subrecords deleveled
  
  Place "MergedLeveledLists - 2023-11-18.esp" last in load order and activate
  
  ```

  </details>

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

## Releases

[Binary downloads](https://www.nexusmods.com/morrowind/mods/52707) are available for GNU/Linux(x86-64), Android(AArch64), Windows(x86-64(MSVC, GNU)), macOS(x86-64, AArch64).

## System requirements

* OS: non-ancient(10-15 years old or younger): Linux kernel 3.2+, Android 4.4+, Windows 7+, macOS 10.12+.
* Memory: ~5x the size of the largest plugin(1GB should be enough for everything), e.g. ~350MB for Morrowind.esm(77MB), ~850MB for TR_Mainland.esm from Tamriel Rebuilt v23.10(167MB).

## Building

<details>

<summary>Jobasha is written in Rust, so you'll need to grab https://www.rust-lang.org in order to compile it. Jobasha compiles with Rust 1.74.0(stable) or newer</summary>

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

Licensed under the [GNU GPLv3](LICENSE).  

## Acknowledgments

* This project came to life thanks to the awesome [tes3 library](https://github.com/Greatness7/tes3) by [Greatness7](https://github.com/Greatness7)  
* Tools that also work with leveled lists(and do much more):  
  * [tes3cmd](https://github.com/john-moonsugar/tes3cmd)  
  * [OMWLLF](https://github.com/jmelesky/omwllf)  
  * [TES3Merge](https://github.com/NullCascade/TES3Merge)  
  * [DeltaPlugin](https://gitlab.com/bmwinger/delta-plugin)  
