mod test_basic;
mod test_debug;
mod test_flags;
mod test_init;
mod test_log;
mod test_merge;
pub(super) use test_basic::test_basic;
pub(super) use test_debug::{
    test_debug, test_debug_all_equal, test_debug_compare_to_the_last, test_debug_equal_to_the_last, test_debug_list_all_plugins,
    test_debug_single,
};
pub(super) use test_flags::test_flags;
pub(super) use test_init::test_init;
pub(super) use test_log::{test_log, test_log_flags};
pub(super) use test_merge::test_merge;

macro_rules! assert_eq_inner {
    ($long:ident, $outer:expr, $dst:ident[$index:expr]) => {
        let TES3Object::$long(ref inner) = $dst.objects[$index] else {
            unreachable!()
        };
        assert_eq!(&$outer, inner);
    };
}

pub(crate) use assert_eq_inner;
