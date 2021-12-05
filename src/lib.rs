mod error;
mod expr;

use crate::expr::{Expr, Node};
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::iter::{self, FromIterator};

#[proc_macro_attribute]
pub fn efg(args: TokenStream, input: TokenStream) -> TokenStream {
    match expr::parse(args) {
        Ok(expr) => render("cfg", expr, input),
        Err(error) => error.compile_error(),
    }
}

#[proc_macro_attribute]
pub fn efg_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    match expr::parse(args) {
        Ok(expr) => render("cfg_attr", expr, input),
        Err(error) => error.compile_error(),
    }
}

fn render(head: &'static str, expr: Expr, input: TokenStream) -> TokenStream {
    vec![
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            TokenStream::from_iter(vec![
                TokenTree::Ident(Ident::new(head, Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, render_expr(expr))),
            ]),
        )),
    ]
    .into_iter()
    .chain(input)
    .collect()
}

fn render_expr(expr: Expr) -> TokenStream {
    let mut tokens = render_node(expr.node);
    tokens.extend(expr.rest);
    tokens
}

fn render_node(node: Node) -> TokenStream {
    let mut tokens = TokenStream::new();
    match node {
        Node::Ident(ident) => {
            tokens.extend(iter::once(TokenTree::Ident(ident)));
        }
        Node::Equal(ident, punct, literal) => {
            tokens.extend(iter::once(TokenTree::Ident(ident)));
            tokens.extend(iter::once(TokenTree::Punct(punct)));
            tokens.extend(iter::once(TokenTree::Literal(literal)));
        }
        Node::Not(node) => {
            let not = Ident::new("not", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(not)));
            let group = Group::new(Delimiter::Parenthesis, render_node(*node));
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
        Node::Or(nodes) => {
            let mut inner = TokenStream::new();
            for (i, node) in nodes.into_iter().enumerate() {
                if i > 0 {
                    let comma = Punct::new(',', Spacing::Alone);
                    inner.extend(iter::once(TokenTree::Punct(comma)));
                }
                inner.extend(render_node(node));
            }
            let any = Ident::new("any", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(any)));
            let group = Group::new(Delimiter::Parenthesis, inner);
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
        Node::And(nodes) => {
            let mut inner = TokenStream::new();
            for (i, node) in nodes.into_iter().enumerate() {
                if i > 0 {
                    let comma = Punct::new(',', Spacing::Alone);
                    inner.extend(iter::once(TokenTree::Punct(comma)));
                }
                inner.extend(render_node(node));
            }
            let all = Ident::new("all", Span::call_site());
            tokens.extend(iter::once(TokenTree::Ident(all)));
            let group = Group::new(Delimiter::Parenthesis, inner);
            tokens.extend(iter::once(TokenTree::Group(group)));
        }
    }
    tokens
}
