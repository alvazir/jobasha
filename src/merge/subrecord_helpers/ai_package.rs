// COMMENT: used in [Creature, Npc].
use super::OptionRecordMergeLog;
use crate::{Cfg, PluginInfo, RecordMap};
use anyhow::{anyhow, Result};
use paste::paste;
use std::mem::discriminant;
use tes3::esp::AiPackage;
mod self_macro;
use self_macro::{
    ai_packages_equal_lowercased, clone_subrecord, is_distinct, merge_if_distinct_basic, merge_if_distinct_location,
    merge_if_distinct_string,
};

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::{test_basic_ai_packages, test_debug_compare_to_the_last_ai_packages, test_log_ai_packages};

#[derive(Default)]
pub struct AiPackagesKind<'a> {
    pub record: Vec<AiPackage>,
    pub vec: Vec<(&'a AiPackage, &'a PluginInfo)>,
}

macro_rules! ai_packages_helper {
    ($($field:ident),+) => {
        paste! {
            #[derive(Default)]
            pub(crate) struct AiPackagesHelper<'a> {
                processed: bool,
                reference: &'a [AiPackage],
                previous_reference: &'a [AiPackage],
                $($field: AiPackagesKind<'a>,)+
            }

            impl<'a> AiPackagesHelper<'a> {
                pub(crate) fn commit(&self, base: &Vec<AiPackage>) -> Vec<AiPackage> {
                    if self.processed {
                        if self.reference.len() == 1 {
                            match &self.reference[0] {
                                $(AiPackage::[<$field:camel>](_) => self.$field.record.clone(),)+
                            }
                        } else {
                            self.reference.to_owned()
                        }
                    } else {
                        base.to_owned()
                    }
                }

                fn previous_reference_kind(&self) -> &'static str {
                    match self.previous_reference.len() {
                        0 => "Empty",
                        1 => ai_package_variant(&self.previous_reference[0]),
                        _ => "Multiple",
                    }
                }
            }
        }
    };
}

ai_packages_helper!(travel, wander, escort, follow, activate);

impl<'a> AiPackagesHelper<'a> {
    pub(crate) fn process<T>(
        &mut self,
        base: &'a [AiPackage],
        reference: &'a [AiPackage],
        plugin_info: &'a PluginInfo,
        option_log: &mut OptionRecordMergeLog,
        map: &'a T,
        cfg: &Cfg,
    ) -> Result<()>
    where
        T: RecordMap<'a>,
    {
        macro_rules! log_field_changed {
            ($to:expr) => {
                if !cfg.meta.silent {
                    option_log.field_changed(
                        "ai_packages",
                        format_args!("({})", self.previous_reference_kind()),
                        format_args!("({})", $to),
                        &plugin_info.name,
                        map,
                        cfg,
                    )?;
                }
            };
        }
        macro_rules! merge {
            ($short:ident, ($($basic:ident),+), ($($string:ident),*), ($($location:ident),*)) => {{ paste! {
                if self.$short.record.is_empty() {
                    if !base.is_empty() {
                        if let AiPackage::[<$short:camel>](_) = &base[0] {
                            self.$short.vec.push((&base[0], plugin_info));
                        }
                    }
                    if self.$short.vec.is_empty() {
                        self.$short.record.push(reference[0].clone());
                        log_field_changed!(stringify!([<$short:camel>]));
                    } else {
                        self.$short.record.push(self.$short.vec[0].0.clone());
                        if self.$short.record[0] == reference[0] {
                            log_field_changed!(stringify!([<$short:camel>]));
                        }
                    }
                };
                if !self.$short.vec.is_empty() {
                    merge_if_distinct_basic!(self, $short, plugin_info, option_log, map, cfg, $($basic),+);
                    merge_if_distinct_string!(self, $short, plugin_info, option_log, map, cfg, $($string),*);
                    merge_if_distinct_location!(self, $short, plugin_info, option_log, map, cfg, $($location),*);
                };
                self.$short.vec.push((&reference[0], plugin_info));
            }
        }}}
        if self.processed {
            self.previous_reference = self.reference
        } else {
            self.processed = true;
            self.previous_reference = base
        };
        self.reference = reference;
        match reference.len() {
            0 => log_field_changed!("Empty"),
            1 => match &reference[0] {
                AiPackage::Travel(travel) => merge!(travel, (reset), (), (location)),
                AiPackage::Wander(wander) => {
                    merge!(
                        wander,
                        (distance, duration, game_hour, idle2, idle3, idle4, idle5, idle6, idle7, idle8, idle9, reset),
                        (),
                        ()
                    )
                }
                AiPackage::Escort(escort) => merge!(escort, (duration, reset), (target, cell), (location)),
                AiPackage::Follow(follow) => merge!(follow, (duration, reset), (target, cell), (location)),
                AiPackage::Activate(activate) => merge!(activate, (reset), (target), ()),
            },
            _ => log_field_changed!("Multiple"),
        }
        Ok(())
    }
}

pub(crate) fn ai_packages_equal(first: &AiPackage, second: &AiPackage) -> Result<bool> {
    let res = if discriminant(first) == discriminant(second) {
        match first {
            AiPackage::Travel(_) | AiPackage::Wander(_) => first == second,
            AiPackage::Escort(escort) => ai_packages_equal_lowercased!(escort, second, (duration, location), (cell)),
            AiPackage::Follow(follow) => ai_packages_equal_lowercased!(follow, second, (duration, location), (cell)),
            AiPackage::Activate(activate) => ai_packages_equal_lowercased!(activate, second, (), ()),
        }
    } else {
        false
    };
    Ok(res)
}

pub(crate) fn ai_package_variant(ai_package: &AiPackage) -> &'static str {
    match ai_package {
        AiPackage::Travel(_) => "Travel",
        AiPackage::Wander(_) => "Wander",
        AiPackage::Escort(_) => "Escort",
        AiPackage::Follow(_) => "Follow",
        AiPackage::Activate(_) => "Activate",
    }
}
