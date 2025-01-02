macro_rules! show_flags {
    ($kind:ty, $($flag:ident),+) => {
        paste! {
        fn [<show_ $kind:snake>](flag: $kind) -> &'static str {
            match flag {
                $($kind::$flag => stringify!($flag),)+
                _ => "UNKNOWN_FLAG",
            }
        }
        }
    };
}

pub(crate) use show_flags;
