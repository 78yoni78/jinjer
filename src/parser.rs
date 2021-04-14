use std::io::{self, BufRead};
use super::tokenizer::{Tokenizer, Token, TokenKind};

#[derive(Debug)]
pub enum Error {
    UnexpectedToken { expected: &'static str, actual: Token },
    Io(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiOper {
    Add, Sub, Mul, Div, Mod,
}

macro_rules! BiOper_fn {
    ($name: ident, $($p: pat => $ret: expr),+ $(,)?) => {
        pub fn $name(kind: &TokenKind) -> Option<Self> {
            use BiOper::*;
            use TokenKind::*;
    
            match kind {
                $( $p => Some($ret), )+
                _ => None,
            }
        }
    };
}

impl BiOper {
    BiOper_fn!{from_token_kind, 
        Plus => Add,
        Minus => Sub,
        Star => Mul,
        Slash => Div,
        Percent => Mod,
    }

    BiOper_fn!{add_sub, 
        Plus => Add,
        Minus => Sub,
    }

    BiOper_fn!{mul_div_mod, 
        Star => Mul,
        Slash => Div,
        Percent => Mod,
    }
}

//  let x = 1 + 1; 2 * x
//  Var("x", Add(Int(1), Int(1)), Mul(Int(2), Var("x")))
#[derive(Debug, Clone)]
pub enum Expr {
    IntLiteral(i32),
    BiOper(BiOper, Box<(Expr, Expr)>),
    Var(String),
    Let(String, Box<(Expr, Expr)>),
}

macro_rules! expect {
    ($tokenizer: expr, $p: pat) => {
        expect!($tokenizer, $p, ())
    };
    ($tokenizer: expr, $p: pat, $res: expr) => {
        {
            let token = $tokenizer.pop().map_err(Error::Io)?;
            match token.kind {
                $p => $res,
                _ => return Err(Error::UnexpectedToken {
                    expected: std::stringify!($p),
                    actual: token
                }),
            }
        }
    };
}

fn parse_atom<R: BufRead>(tokenizer: &mut Tokenizer<R>) -> Result<Expr> {
    use TokenKind::{Int, Ident, LParen, Let};
    
    let token = tokenizer.pop().map_err(Error::Io)?;
    match token.kind {
        Int(i) => Ok(Expr::IntLiteral(i)),
        Ident(name) => Ok(Expr::Var(name)),
        LParen => {
            let e = parse_expr(tokenizer)?;
            expect!(tokenizer, TokenKind::RParen);
            Ok(e)
        },
        Let => {
            let name: String = expect!(tokenizer, Ident(name), name);
            expect!(tokenizer, TokenKind::Equal);
            let e1 = parse_expr(tokenizer)?;
            expect!(tokenizer, TokenKind::In);
            let e2 = parse_expr(tokenizer)?;
            Ok(Expr::Let(name, Box::new((e1, e2))))
        }
        _ => Err(Error::UnexpectedToken { expected: "Anything", actual: token }),
    }
}

pub fn parse_expr<R: BufRead>(tokenizer: &mut Tokenizer<R>) -> Result<Expr> {
    let mut left = parse_atom(tokenizer)?;
    while let Some(op) = BiOper::add_sub(&tokenizer.peek().map_err(Error::Io)?.kind) {
        tokenizer.pop().map_err(Error::Io)?; //  skip th op
        let right = parse_atom(tokenizer)?;
        left = Expr::BiOper(op, Box::new((left, right)));
    }
    Ok(left)
}
