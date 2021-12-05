mod expr;

use crate::expr::Expr;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::{self, FromIterator};

#[proc_macro_attribute]
pub fn efg(args: TokenStream, input: TokenStream) -> TokenStream {
    let expr = expr::parse(args);
    render(expr, input)
}

fn render(expr: Expr, input: TokenStream) -> TokenStream {
    vec![
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            TokenStream::from_iter(vec![
                TokenTree::Ident(Ident::new("cfg", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, render_expr(expr))),
            ]),
        )),
    ]
    .into_iter()
    .chain(input)
    .collect()
}

fn render_expr(expr: Expr) -> TokenStream {
    let mut tokens = TokenStream::new();
    match expr {
        Expr::Ident(ident) => {
            tokens.extend(iter::once(TokenTree::Ident(ident)));
        }
        Expr::Equal(ident, punct, literal) => {
            tokens.extend(iter::once(TokenTree::Ident(ident)));
            tokens.extend(iter::once(TokenTree::Punct(punct)));
            tokens.extend(iter::once(TokenTree::Literal(literal)));
        }
        Expr::Not(expr) => {
            let not = Ident::new("not", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(not)));
            let group = Group::new(Delimiter::Parenthesis, render_expr(*expr));
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
        Expr::Or(exprs) => {
            let mut inner = TokenStream::new();
            for (i, expr) in exprs.into_iter().enumerate() {
                if i > 0 {
                    let comma = Punct::new(',', Spacing::Alone);
                    inner.extend(iter::once(TokenTree::Punct(comma)));
                }
                inner.extend(render_expr(expr));
            }
            let any = Ident::new("any", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(any)));
            let group = Group::new(Delimiter::Parenthesis, inner);
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
        Expr::And(exprs) => {
            let mut inner = TokenStream::new();
            for (i, expr) in exprs.into_iter().enumerate() {
                if i > 0 {
                    let comma = Punct::new(',', Spacing::Alone);
                    inner.extend(iter::once(TokenTree::Punct(comma)));
                }
                inner.extend(render_expr(expr));
            }
            let all = Ident::new("all", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(all)));
            let group = Group::new(Delimiter::Parenthesis, inner);
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
    }
    tokens
}
