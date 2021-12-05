use proc_macro::{Ident, Literal, Punct, TokenStream};

pub enum Expr {
    Ident(Ident),
    Equal(Ident, Punct, Literal),
    Not(Box<Expr>),
    Or(Vec<Expr>),
    And(Vec<Expr>),
}

pub fn parse(args: TokenStream) -> Expr {
    let _ = args;
    unimplemented!()
}
