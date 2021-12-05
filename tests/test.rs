use efg::efg;

macro_rules! t {
    (
        #[efg($($efg:tt)*)]
        #[cfg($($cfg:tt)*)]
    ) => {
        const _: () = {
            #[efg($($efg)*)]
            const EVAL: bool = true;

            #[efg(!($($efg)*))]
            const EVAL: bool = false;

            if EVAL != cfg!($($cfg)*) {
                panic!(concat!(
                    "efg=",
                    cfg!(not($($cfg)*)),
                    ", cfg=",
                    cfg!($($cfg)*),
                ));
            }
        };
    };
}

t! {
    #[efg(unix || windows)]
    #[cfg(any(unix, windows))]
}
