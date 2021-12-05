mod expr;

use crate::expr::Expr;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn efg(args: TokenStream, input: TokenStream) -> TokenStream {
    let expr = expr::parse(args);
    render(expr, input)
}

fn render(expr: Expr, input: TokenStream) -> TokenStream {
    let _ = expr;
    let _ = input;
    unimplemented!()
}
