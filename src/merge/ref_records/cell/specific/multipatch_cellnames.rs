use super::{print_as, NonOptMapColor, NonOptRegion, OptionRecordMergeLog, SpecificFlags};
use crate::{CellRecordMap, Cfg};
use anyhow::{Context, Result};
use tes3::esp::Cell;

const MULTIPATCH_KIND: &str = "cellnames";

pub(super) fn multipatch_cellnames(
    merged: &mut Cell,
    last: &Cell,
    map: &CellRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    merged.references.clear();
    if !cfg.meta.silent {
        specific_flags.multipatched = Some(MULTIPATCH_KIND);
        option_log
            .equal_to_the_last_or_clear(&map, cfg)
            .with_context(|| "Bug: failed to succeed equal_to_the_last_or_clear()")?;
        if cfg.meta.debug_multipatch_attempt {
            option_log.multipatch_attempt(&map, cfg)?;
        }
        macro_rules! log_changed_field {
            ($kind:expr, $field:ident$(:$subfield:ident)?$(::print_as::$print_as:ident)?) => {
                option_log.field_changed_or_multipatched(
                    $kind,
                    stringify!($field$(.$subfield)?),
                    format_args!(
                        "{:?} -> {:?}",
                        print_as!($($print_as:)?&last.$field$(.$subfield)?),
                        print_as!($($print_as:)?&merged.$field$(.$subfield)?)
                    ),
                    &map,
                    cfg,
                )?;
            };
        }
        log_changed_field!(MULTIPATCH_KIND, name);
        macro_rules! compare_fields {
            ($($field:ident$(:$subfield:ident)?$(::print_as::$print_as:ident)?),+) => {
                $(if merged.$field$(.$subfield)? != last.$field$(.$subfield)? {
                    log_changed_field!("", $field$(:$subfield)?$(::print_as::$print_as)?);
                })+
            };
        }
        compare_fields!(flags, data:flags, data:grid, region::print_as::NonOptRegion, map_color::print_as::NonOptMapColor);
    }
    Ok(())
}
