macro_rules! is_distinct {
    ($self:ident, $short_ai:ident, $long_ai:ident, $field:ident$(, method = $method:ident)?$(, index = $index:expr)?) => {
        !$self.$short_ai.vec.iter().any(|element| {
            if let AiPackage::$long_ai(ai_package) = element.0 {
                ai_package.$field$(.$method())?$([$index])? == $short_ai.$field$(.$method())?$([$index])?
            } else {
                false
            }
        })
    };
}

macro_rules! clone_subrecord {
    ($self:ident, $short_ai:ident, $plugin_info:ident, $option_log:ident, $map:ident, $cfg:ident, $long_ai:ident, $field:ident) => {
        if let AiPackage::$long_ai(ref mut record) = $self.$short_ai.record[0] {
            if !$cfg.meta.silent {
                $option_log.field_changed(
                    "ai_packages",
                    format_args!("({}), \"{}\": {:?}", stringify!($long_ai), stringify!($field), record.$field),
                    format_args!("{:?}", $short_ai.$field),
                    &$plugin_info.name,
                    $map,
                    $cfg,
                )?;
            }
            record.$field = $short_ai.$field.clone();
        } else {
            return Err(anyhow!(
                "Bug: failed to extract record and clone subrecord because self.{}.record[0] is not AiPackage::{}(record)",
                stringify!($short_ai),
                stringify!($long_ai)
            ));
        }
    };
    ($self:ident, $short_ai:ident, $plugin_info:ident, $option_log:ident, $map:ident, $cfg:ident, $long_ai:ident, $field:ident, index = $index:expr) => {
        if let AiPackage::$long_ai(ref mut record) = $self.$short_ai.record[0] {
            if !$cfg.meta.silent {
                $option_log.field_changed(
                    "ai_packages",
                    format_args!(
                        "({}), \"{}.{}\": {:?}",
                        stringify!($long_ai),
                        stringify!($field),
                        $index,
                        record.$field[$index]
                    ),
                    format_args!("{:?}", $short_ai.$field[$index]),
                    &$plugin_info.name,
                    $map,
                    $cfg,
                )?;
            }
            record.$field[$index] = $short_ai.$field[$index].clone();
        } else {
            return Err(anyhow!(
                "Bug: failed to extract record and clone subrecord because self.{}.record[0] is not AiPackage::{}(record)",
                stringify!($short_ai),
                stringify!($long_ai)
            ));
        }
    };
}

macro_rules! merge_if_distinct_basic {
    ($self:ident, $short_ai:ident, $plugin_info:ident, $option_log:ident, $map:ident, $cfg:ident, $($field:ident),+) => {
        $(paste! {
            if is_distinct!($self, $short_ai, [<$short_ai:camel>], $field) {
                clone_subrecord!($self, $short_ai, $plugin_info, $option_log, $map, $cfg, [<$short_ai:camel>], $field)
            }
        })+
    }
}

macro_rules! merge_if_distinct_string {
    ($self:ident, $short_ai:ident, $plugin_info:ident, $option_log:ident, $map:ident, $cfg:ident, $($field:ident),*) => {
        $(paste! {
            if is_distinct!($self, $short_ai, [<$short_ai:camel>], $field) && is_distinct!($self, $short_ai, [<$short_ai:camel>], $field, method = to_lowercase) {
                clone_subrecord!($self, $short_ai, $plugin_info, $option_log, $map, $cfg, [<$short_ai:camel>], $field)
            }
        })*
    }
}

macro_rules! merge_if_distinct_location {
    ($self:ident, $short_ai:ident, $plugin_info:ident, $option_log:ident, $map:ident, $cfg:ident, $($field:ident),*) => {
        $(paste! {
            for axis in 0..3 {
                if is_distinct!($self, $short_ai, [<$short_ai:camel>], $field, index = axis) {
                    clone_subrecord!($self, $short_ai, $plugin_info, $option_log, $map, $cfg, [<$short_ai:camel>], $field, index = axis)
                };
            }
        })*
    };
}

macro_rules! ai_packages_equal_lowercased {
    ($short_ai:ident, $ai_package:expr, ($($basic:ident),*), ($($string:ident),*)) => {
        paste! {
            if let AiPackage::[<$short_ai:camel>](source) = $ai_package {
                $short_ai.reset == source.reset
                    $(&& $short_ai.$basic == source.$basic)*
                    && $short_ai.target.to_lowercase() == source.target.to_lowercase()
                    $(&& $short_ai.$string.to_lowercase() == source.$string.to_lowercase())*
            } else {
                return Err(anyhow!(
                    "Bug: failed to compare source with lowercased ai_package {1} because {0} is not AiPackage::{1}(source)",
                    stringify!($ai_package),
                    stringify!($short_ai)
                ));
            }
        }
    };
}

pub(super) use {
    ai_packages_equal_lowercased, clone_subrecord, is_distinct, merge_if_distinct_basic, merge_if_distinct_location,
    merge_if_distinct_string,
};
