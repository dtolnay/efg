#![allow(clippy::manual_assert)]

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

macro_rules! with_ident {
    ($expr:expr) => {
        t! {
            #[efg(unix || $expr)]
            #[cfg(any(unix, windows))]
        }

        t! {
            #[efg($expr = "yes")]
            #[cfg(windows = "yes")]
        }
    };
}

with_ident!(windows);

macro_rules! with_literal {
    ($expr:expr) => {
        t! {
            #[efg(feature = $expr)]
            #[cfg(feature = "std")]
        }
    };
}

with_literal!("std");
