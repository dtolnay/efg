use crate::error::Error;
use proc_macro::{
    token_stream, Delimiter, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
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
    let node = parse_disjunction(&mut iter)?;
    Ok(Expr { node, rest: iter })
}

type Iter<'a> = &'a mut Peekable<token_stream::IntoIter>;

fn parse_disjunction(iter: Iter) -> Result<Node, Error> {
    let conjunction = parse_conjunction(iter)?;
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
                let conjunction = parse_conjunction(iter)?;
                vec.push(conjunction);
            }
            None => break,
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => break,
            Some(unexpected) => {
                let span = unexpected.span();
                return Err(Error::new(span, "unexpected token"));
            }
        }
    }
    let node = if vec.len() == 1 {
        vec.remove(0)
    } else {
        Node::Or(vec)
    };
    Ok(node)
}

fn parse_conjunction(iter: Iter) -> Result<Node, Error> {
    let atom = parse_atom(iter)?;
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
                let atom = parse_atom(iter)?;
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

fn parse_atom(iter: Iter) -> Result<Node, Error> {
    match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            let mut iter = group.stream().into_iter().peekable();
            let node = parse_disjunction(&mut iter)?;
            if let Some(unexpected) = iter.next() {
                let span = unexpected.span();
                return Err(Error::new(span, "unexpected token"));
            }
            Ok(node)
        }
        Some(TokenTree::Ident(ident)) => match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                let punct = punct.clone();
                let _ = iter.next().unwrap();
                match iter.next() {
                    Some(TokenTree::Literal(literal)) => Ok(Node::Equal(ident, punct, literal)),
                    Some(unexpected) => {
                        let span = unexpected.span();
                        Err(Error::new(span, "unexpected token, expected a literal"))
                    }
                    None => {
                        let span = Span::call_site();
                        Err(Error::new(span, "expected a literal"))
                    }
                }
            }
            _ => Ok(Node::Ident(ident)),
        },
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
            let atom = parse_atom(iter)?;
            Ok(Node::Not(Box::new(atom)))
        }
        Some(unexpected) => {
            let span = unexpected.span();
            Err(Error::new(span, "expected an identifier"))
        }
        None => {
            let span = Span::call_site();
            Err(Error::new(span, "unexpected end of input"))
        }
    }
}
