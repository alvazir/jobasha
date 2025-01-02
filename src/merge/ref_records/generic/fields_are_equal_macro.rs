macro_rules! fields_are_equal {
    ($fst:ident, $snd:ident, $field:ident$(:$subfield:ident$(:$tuple_index:tt)?)?) => {
        $fst.$field$(.$subfield$(.$tuple_index)?)? == $snd.$field$(.$subfield$(.$tuple_index)?)?
    };
    ($fst:ident, $snd:ident, $field:ident$(:$subfield:ident$(:$tuple_index:tt)?)?, $($fields:ident$(:$subfields:ident$(:$tuple_indexes:tt)?)?),+) => {
        fields_are_equal!($fst, $snd, $field$(:$subfield$(:$tuple_index)?)?) && fields_are_equal!($fst, $snd, $($fields$(:$subfields$(:$tuple_indexes)?)?),+)
    }
}

pub(crate) use fields_are_equal;
