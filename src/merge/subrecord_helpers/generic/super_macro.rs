macro_rules! log_field_shorten {
    ($record:ident, $vec_field:ident, $index:ident, $subindex:ident, $option_log:ident, $map:ident, $short:ident, $cfg:ident) => {
        paste! {
            if !$cfg.meta.silent {
                log_field_extend!($option_log, true, [<get_ $vec_field _element>](&$record.$short.$vec_field, $subindex)?, &$map.record($index)?, $map, $cfg);
            }
        }
    };
}

macro_rules! add_and_log_field_lengthen {
    ($vec_name:ident$(.$vec_subname:ident)?$(:$vec:ident)?, $subrecord:ident, $vec_field:ident, $index:ident, $subindex:ident, $option_log:ident, $map:ident, $short:ident, $cfg:ident) => {
        macro_rules! selector {
            ($member:ident) => {
                $member
            };
            ($member:ident, $macro:ident) => {
                $macro![$member]
            };
        }
        paste! {
            let record = &$map.record($index)?;
            let element = [<get_ $vec_field _element>](&record.$short.$vec_field, $subindex)?;
            let member = ($subrecord.to_owned(), element, record.plugin_info);
            $vec_name$(.$vec_subname)?.push(selector!(member$(, $vec)?));
            if !$cfg.meta.silent {
                log_field_extend!($option_log, false, element, record, $map, $cfg);
            }
        }
    };
}

macro_rules! get_vec_element {
    ($vec_field:ident, $in_kind:ty, $out_kind:ty) => {
        paste! {
            fn [<get_ $vec_field _element>]($vec_field: &[$in_kind], subindex: usize) -> Result<&$out_kind> {
                Ok(&$vec_field[subindex])
            }
        }
    };
}

pub(crate) use {add_and_log_field_lengthen, get_vec_element, log_field_shorten};
