macro_rules! count_changes {
    ($merged:ident, $last:ident, $specific_flags:ident, $($field:ident$(:$subfield:ident$(:$tuple_index:tt)?)?),+) => {
        $(if $merged.$field$(.$subfield$(.$tuple_index)?)? != $last.$field$(.$subfield$(.$tuple_index)?)? {
            $specific_flags.changes += 1;
        })+
    };
}

pub(crate) use count_changes;
