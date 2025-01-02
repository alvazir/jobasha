macro_rules! test_merge {
    ($short:ident, $src:ident, $plugins:ident, $cfg:ident, $log:ident, $im:ident, $res:ident, $dst:ident:$dst_len:expr) => {
        paste! {
            $cfg.reset_meta();
            let mut raw_dst = RawPlugin::default();
            let mut $im = IntermediateRecords::default();
            let mut $log = MergeLog::default();
            for (object, plugin) in $src.iter().zip(&$plugins) {
                $im.[<get_ $short>](object.clone(), plugin);
            }
            let $res = [<merge_ $short>](&$im, &mut raw_dst, &$cfg, &mut $log);
            let $dst = raw_dst.plugin;
            println!("res = {:?}", $res);
            assert!($res.is_ok());
            for (count, i) in $src.iter().enumerate() {
                println!("src[{count}] = {i:?}");
            }
            println!("log = {}", $log.test_file());
            assert_eq!($dst.objects.len(), $dst_len);
        }
    };
}

pub(crate) use test_merge;
