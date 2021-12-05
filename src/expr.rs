use proc_macro::TokenStream;

pub struct Expr {}

pub fn parse(args: TokenStream) -> Expr {
    let _ = args;
    unimplemented!()
}
