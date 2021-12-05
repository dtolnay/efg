use proc_macro::{
    token_stream, Delimiter, Ident, Literal, Punct, Spacing, TokenStream, TokenTree,
};
use std::iter::Peekable;

pub enum Expr {
    Ident(Ident),
    Equal(Ident, Punct, Literal),
    Not(Box<Expr>),
    Or(Vec<Expr>),
    And(Vec<Expr>),
}

pub fn parse(args: TokenStream) -> Expr {
    let mut iter = args.into_iter().peekable();
    parse_disjunction(&mut iter)
}

type Iter<'a> = &'a mut Peekable<token_stream::IntoIter>;

fn parse_disjunction(iter: Iter) -> Expr {
    let conjunction = parse_conjunction(iter);
    let mut vec = vec![conjunction];
    loop {
        match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '|' => {
                let spacing = punct.spacing();
                let _ = iter.next().unwrap();
                if spacing != Spacing::Joint
                    || match iter.next() {
                        Some(TokenTree::Punct(second)) => second.as_char() != '|',
                        _ => true,
                    }
                {
                    panic!("expected ||");
                }
                let conjunction = parse_conjunction(iter);
                vec.push(conjunction);
            }
            None => break,
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => break,
            Some(_unexpected) => panic!("unexpected token"),
        }
    }
    if vec.len() == 1 {
        vec.remove(0)
    } else {
        Expr::Or(vec)
    }
}

fn parse_conjunction(iter: Iter) -> Expr {
    let atom = parse_atom(iter);
    let mut vec = vec![atom];
    loop {
        match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '&' => {
                let spacing = punct.spacing();
                let _ = iter.next().unwrap();
                if spacing != Spacing::Joint
                    || match iter.next() {
                        Some(TokenTree::Punct(second)) => second.as_char() != '&',
                        _ => true,
                    }
                {
                    panic!("expected &&");
                }
                let atom = parse_atom(iter);
                vec.push(atom);
            }
            _ => break,
        }
    }
    if vec.len() == 1 {
        vec.remove(0)
    } else {
        Expr::And(vec)
    }
}

fn parse_atom(iter: Iter) -> Expr {
    match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            let mut iter = group.stream().into_iter().peekable();
            let expr = parse_disjunction(&mut iter);
            if iter.next().is_some() {
                panic!("unexpected token");
            }
            expr
        }
        Some(TokenTree::Ident(ident)) => match iter.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
                let punct = punct.clone();
                let _ = iter.next().unwrap();
                match iter.next() {
                    Some(TokenTree::Literal(literal)) => Expr::Equal(ident, punct, literal),
                    Some(_unexpected) => panic!("unexpected token"),
                    None => panic!("expected a literal"),
                }
            }
            _ => Expr::Ident(ident),
        },
        Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
            let atom = parse_atom(iter);
            Expr::Not(Box::new(atom))
        }
        Some(_unexpected) => panic!("expected an identifier"),
        None => panic!("unexpected end of input"),
    }
}
