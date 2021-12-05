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

macro_rules! with_expr {
    ($expr:expr) => {
        t! {
            #[efg(unix || $expr)]
            #[cfg(any(unix, windows))]
        }
    };
}

with_expr!(windows);
