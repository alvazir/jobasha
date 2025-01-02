macro_rules! print_as {
    ($expr:expr) => {
        $expr
    };
    ($new_type:ident:$expr:expr) => {
        $new_type($expr)
    };
}

pub(crate) use print_as;
