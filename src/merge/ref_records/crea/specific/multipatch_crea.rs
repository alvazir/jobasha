use super::{print_as, NonOptScale, OptionRecordMergeLog, SpecificFlags, MULTIPATCH_KIND};
use crate::{Cfg, CreatureRecordMap};
use anyhow::{Context, Result};
use tes3::esp::{Creature, ObjectFlags};

pub(super) fn multipatch_crea(
    merged: &mut Creature,
    last: &Creature,
    map: &CreatureRecordMap,
    option_log: &mut OptionRecordMergeLog,
    specific_flags: &mut SpecificFlags,
    cfg: &Cfg,
) -> Result<()> {
    let from = merged.flags;
    merged.flags.insert(ObjectFlags::PERSISTENT);
    if merged == last {
        specific_flags.equal_after_specific = true;
    }
    if !cfg.meta.silent {
        let to = merged.flags;
        if cfg.merge.crea {
            if cfg.merge.interdependent_flags {
                option_log.field_multipatched(MULTIPATCH_KIND, "flags", format_args!("{from:?} -> {to:?}"), cfg)?;
            } else {
                option_log.field_multipatched(MULTIPATCH_KIND, "flags", format_args!("+ PERSISTENT"), cfg)?;
            };
        } else {
            specific_flags.multipatched = Some(MULTIPATCH_KIND);
            option_log
                .equal_to_the_last_or_clear(&map, cfg)
                .with_context(|| "Bug: failed to succeed equal_to_the_last_or_clear()")?;
            macro_rules! log_changed_field {
                ($kind:expr, $field:ident$(:$subfield:ident$(:$tuple_index:tt)?)?$(::print_as::$print_as:ident)?) => {
                    option_log.field_changed_or_multipatched(
                        $kind,
                        stringify!($field$(.$subfield$(.$tuple_index)?)?),
                        format_args!(
                            "{:?} -> {:?}",
                            print_as!($($print_as:)?&last.$field$(.$subfield$(.$tuple_index)?)?),
                            print_as!($($print_as:)?&merged.$field$(.$subfield$(.$tuple_index)?)?)
                        ),
                        &map,
                        cfg,
                    )?;
                };
            }
            macro_rules! compare_fields {
                ($($field:ident$(:$subfield:ident$(:$tuple_index:tt)?)?$(::print_as::$print_as:ident)?),+) => {
                    $(if merged.$field$(.$subfield$(.$tuple_index)?)? != last.$field$(.$subfield$(.$tuple_index)?)? {
                        log_changed_field!("", $field$(:$subfield$(:$tuple_index)?)?$(::print_as::$print_as)?);
                    })+
                };
            }
            log_changed_field!(MULTIPATCH_KIND, flags);
            compare_fields!(
                name,
                script,
                mesh,
                ai_data:hello,
                ai_data:fight,
                ai_data:flee,
                ai_data:alarm,
                ai_data:services,
                inventory,
                spells,
                ai_packages,
                travel_destinations,
                sound,
                scale::print_as::NonOptScale,
                creature_flags,
                blood_type,
                data:creature_type,
                data:level,
                data:strength,
                data:intelligence,
                data:willpower,
                data:agility,
                data:speed,
                data:endurance,
                data:personality,
                data:luck,
                data:health,
                data:magicka,
                data:fatigue,
                data:soul,
                data:combat,
                data:magic,
                data:stealth,
                data:gold,
                data:attack1:0,
                data:attack1:1,
                data:attack2:0,
                data:attack2:1,
                data:attack3:0,
                data:attack3:1
            );
        }
    }
    Ok(())
}
