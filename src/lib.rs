//! [![github]](https://github.com/dtolnay/efg)&ensp;[![crates-io]](https://crates.io/crates/efg)&ensp;[![docs-rs]](https://docs.rs/efg)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! **Conditional compilation using boolean expression syntax, rather than
//! *any()*, *all()*, *not()*.**
//!
//! <br>
//!
//! Rust's `cfg` and `cfg_attr` conditional compilation attributes use a
//! restrictive domain-specific language for specifying configuration
//! predicates. The syntax is described in the *[Conditional compilation]* page
//! of the Rust reference. The reason for this syntax as opposed to ordinary
//! boolean expressions was to accommodate restrictions that old versions of
//! rustc used to have on the grammar of attributes.
//!
//! However, all restrictions on the attribute grammar were lifted in Rust
//! 1.18.0 by [rust-lang/rust#40346]. This crate explores implementing
//! conditional compilation using ordinary boolean expressions
//! instead:&ensp;`&&`,&ensp;`||`,&ensp;`!`&ensp;as usual in Rust syntax.
//!
//! [Conditional compilation]: https://doc.rust-lang.org/1.57.0/reference/conditional-compilation.html
//! [rust-lang/rust#40346]: https://github.com/rust-lang/rust/pull/40346
//!
//! <table>
//! <tr><th><center>built into rustc</center></th><th><center>this crate</center></th></tr>
//! <tr><td><code>#[cfg(any(<i>thing1</i>, <i>thing2</i>, &hellip;))]</code></td><td><code>#[efg(<i>thing1</i> || <i>thing2</i> || &hellip;)]</code></td></tr>
//! <tr><td><code>#[cfg(all(<i>thing1</i>, <i>thing2</i>, &hellip;))]</code></td><td><code>#[efg(<i>thing1</i> &amp;&amp; <i>thing2</i> &amp;&amp; &hellip;)]</code></td></tr>
//! <tr><td><code>#[cfg(not(<i>thing</i>))]</code></td><td><code>#[efg(!<i>thing</i>)]</code></td></tr>
//! </table>
//!
//! <br>
//!
//! # Examples
//!
//! A real-world example from the `quote` crate:
//!
//! ```
//! # use efg::efg;
//! #
//! #[efg(feature = "proc-macro" && !(target_arch = "wasm32" && target_os = "unknown"))]
//! extern crate proc_macro;
//! ```
//!
//! and from the `proc-macro2` crate:
//!
//! ```
//! # use efg::efg;
//! #
//! # struct Span;
//! # struct LineColumn;
//! #
//! # impl Span {
//! #[efg(super_unstable || feature = "span-locations")]
//! pub fn start(&self) -> LineColumn {
//! # unimplemented!()
//! # }
//! # }
//! ```

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
