macro_rules! get_previous_cell_option_field {
    ($map:ident, $field:ident) => {
        $map.records
            .iter()
            .rev()
            .filter(|record| record.cell.$field.is_some())
            .map(|record| record.cell.$field.as_ref())
            .next()
            .unwrap_or(None)
    };
}

pub(crate) use get_previous_cell_option_field;
