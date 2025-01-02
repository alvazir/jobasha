use super::{count_changes, print_as, NonOptScale, OptionRecordMergeLog, SpecificFlags, SUMMONS};
use crate::{Cfg, CreatureRecordMap};
use anyhow::{Context, Result};
use tes3::esp::{Creature, ObjectFlags};
mod mark_equal_count_changes;
mod multipatch_crea;
use mark_equal_count_changes::mark_equal_count_changes;
use multipatch_crea::multipatch_crea;

const MULTIPATCH_KIND: &str = "summon";

pub(crate) fn specific(
    id_low: &str,
    merged: &mut Creature,
    last: &Creature,
    map: &CreatureRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    if cfg.multipatch.summons {
        if cfg.meta.debug_multipatch_attempt {
            option_log.multipatch_attempt(&map, cfg)?;
        }
        if SUMMONS.contains(&id_low)
            && ((cfg.merge.crea && !merged.flags.contains(ObjectFlags::PERSISTENT))
                || (!cfg.merge.crea && !last.flags.contains(ObjectFlags::PERSISTENT)))
        {
            if cfg.multipatch.primitive && !cfg.merge.crea {
                *merged = last.clone();
            }
            multipatch_crea(merged, last, map, option_log, specific_flags, cfg)
                .with_context(|| format!("Bug: failed to multipatch CREA record: {id_low:?}"))?;
        } else if !cfg.merge.crea {
            mark_equal_count_changes(merged, last, specific_flags, cfg);
        }
    }
    Ok(())
}

pub(crate) fn specific_multipatch_check(id_low: &str, merged: &Creature, _cfg: &Cfg) -> bool {
    SUMMONS.contains(&id_low) && !merged.flags.contains(ObjectFlags::PERSISTENT)
}

pub(crate) fn specific_multipatch(
    merged: &mut Creature,
    map: &CreatureRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    let from = merged.flags;
    merged.flags.insert(ObjectFlags::PERSISTENT);
    if !cfg.meta.silent {
        let to = merged.flags;
        specific_flags.multipatched = Some(MULTIPATCH_KIND);
        if cfg.merge.interdependent_flags {
            option_log.field_changed_or_multipatched(MULTIPATCH_KIND, "flags", format_args!("{from:?} -> {to:?}"), &map, cfg)?;
        } else {
            option_log.field_changed_or_multipatched(MULTIPATCH_KIND, "flags", format_args!("+ PERSISTENT"), &map, cfg)?;
        };
    }
    Ok(())
}
