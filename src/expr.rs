use crate::error::Error;
use proc_macro::{
    token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};
use std::iter::Peekable;

pub struct Expr {
    pub node: Node,
    pub rest: Peekable<token_stream::IntoIter>,
}

pub enum Node {
    Ident(Ident),
    Equal(Ident, Punct, Literal),
    Not(Box<Node>),
    Or(Vec<Node>),
    And(Vec<Node>),
}

pub fn parse(args: TokenStream) -> Result<Expr, Error> {
    let mut iter = args.into_iter().peekable();
    let node = parse_disjunction(&mut iter, None)?;
    Ok(Expr { node, rest: iter })
}

type Iter<'a> = &'a mut Peekable<token_stream::IntoIter>;
type Ctx<'a> = Option<&'a Group>;

fn parse_disjunction(iter: Iter, ctx: Ctx) -> Result<Node, Error> {
    let conjunction = parse_conjunction(iter, ctx)?;
    let mut vec = vec![conjunction];
    loop {
        match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '|' => {
                let spacing = punct.spacing();
                let span = punct.span();
                let _ = iter.next().unwrap();
                if spacing != Spacing::Joint
                    || match iter.next() {
                        Some(TokenTree::Punct(second)) => second.as_char() != '|',
                        _ => true,
                    }
                {
                    return Err(Error::new(span, "expected ||"));
                }
                let conjunction = parse_conjunction(iter, ctx)?;
                vec.push(conjunction);
            }
            None => break,
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => break,
            Some(unexpected) => return Err(unexpected_token(unexpected, "unexpected token")),
        }
    }
    let node = if vec.len() == 1 {
        vec.remove(0)
    } else {
        Node::Or(vec)
    };
    Ok(node)
}

fn parse_conjunction(iter: Iter, ctx: Ctx) -> Result<Node, Error> {
    let atom = parse_atom(iter, ctx)?;
    let mut vec = vec![atom];
    loop {
        match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '&' => {
                let spacing = punct.spacing();
                let span = punct.span();
                let _ = iter.next().unwrap();
                if spacing != Spacing::Joint
                    || match iter.next() {
                        Some(TokenTree::Punct(second)) => second.as_char() != '&',
                        _ => true,
                    }
                {
                    return Err(Error::new(span, "expected &&"));
                }
                let atom = parse_atom(iter, ctx)?;
                vec.push(atom);
            }
            _ => break,
        }
    }
    let node = if vec.len() == 1 {
        vec.remove(0)
    } else {
        Node::And(vec)
    };
    Ok(node)
}

fn parse_atom(iter: Iter, ctx: Ctx) -> Result<Node, Error> {
    match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            let mut iter = group.stream().into_iter().peekable();
            let node = parse_disjunction(&mut iter, Some(&group))?;
            if let Some(unexpected) = iter.next() {
                return Err(unexpected_token(&unexpected, "unexpected token"));
            }
            Ok(node)
        }
        Some(TokenTree::Ident(ident)) => match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                let punct = punct.clone();
                let _ = iter.next().unwrap();
                match iter.next() {
                    Some(TokenTree::Literal(literal)) => Ok(Node::Equal(ident, punct, literal)),
                    Some(unexpected) => Err(unexpected_token(
                        &unexpected,
                        "unexpected token, expected a literal",
                    )),
                    None => {
                        let span = Span::call_site();
                        Err(Error::new(span, "expected a literal"))
                    }
                }
            }
            _ => Ok(Node::Ident(ident)),
        },
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
            let atom = parse_atom(iter, ctx)?;
            Ok(Node::Not(Box::new(atom)))
        }
        Some(unexpected) => Err(unexpected_token(
            &unexpected,
            "unexpected token, expected an identifier",
        )),
        None => {
            if let Some(group) = ctx {
                let span = group.span_close();
                Err(Error::new(span, "expected an identifier"))
            } else {
                let span = Span::call_site();
                Err(Error::new(span, "unexpected end of input"))
            }
        }
    }
}

fn unexpected_token(unexpected: &TokenTree, msg: &str) -> Error {
    let span = unexpected.span();
    Error::new(span, msg)
}
