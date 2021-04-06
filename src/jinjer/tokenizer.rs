use std::io::{BufRead, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //  single character misc tokens
    LParen, RParen,
    LBrace, RBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    //  logical operator tokens
    Bang, Equal, BangEqual, EqualEqual,
    Greater, GreaterEqual,
    Lesser, LesserEqual,
    //  literals
    Ident(String), Str(String), Float(f32), Int(i32),
    //  keywords
    And, Or, Else, If, False, True, Let, Eof,
    //  Error token
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
    pub length: u32,
}

pub struct Tokenizer<R> {
    reader: R,
    line: u32,
    col: u32,
    line_chars: Vec<char>,
}

fn read_line_chars<R: BufRead>(reader: &mut R) -> Result<Vec<char>> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    Ok(buf.chars().collect())
}

impl<R: BufRead> Tokenizer<R> {
    fn from_reader(mut reader: R) -> Result<Self> {
        let line_chars = read_line_chars(&mut reader)?;
        Ok(Self { reader, line: 0, col: 0, line_chars })
    }

    fn advance_line(&mut self) -> Result<()> {
        self.col = 0;
        self.line += 1;
        self.line_chars = read_line_chars(&mut self.reader)?;
        Ok(())
    }

    fn advance_char(&mut self) -> Result<()> {
        assert!((self.col as usize) < self.line_chars.len());
        self.col += 1;
        while (self.col as usize) >= self.line_chars.len() {
            self.advance_line()?;
        }
        Ok(())
    }

    fn peek_char(&self) -> Option<char> { 
        self.line_chars.get(self.col as usize).cloned()
    }

    fn pop_char(&mut self) -> Result<Option<char>> {
        let ret = self.peek_char();
        if let Some(_) = ret { self.advance_char()?; }
        Ok(ret)
    }

    fn pop_char_if_eq(&mut self, expected: char) -> Result<bool> {
        let eq = self.peek_char() == Some(expected); 
        if eq { self.advance_char()?; }
        Ok(eq)
    }

    fn token(&self, kind: TokenKind, length: u32) -> Token {
        Token { kind, line: self.line, col: self.col - length, length}
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        while self.peek_char().map(char::is_whitespace).unwrap_or(false) {
            self.advance_char()?;
        }
        Ok(())
    }

    fn read_string(&mut self) -> Result<Token> {
        let mut length = 1;
        let mut acc = String::new();
        while self.peek_char().map(|ch| ch != '"').unwrap_or(false) {
            acc.push(self.pop_char()?.unwrap());
            length += 1;
        }


        let kind = if self.peek_char().is_none() {
            TokenKind::Error(String::from("Unterminated string"))
        } else {
            self.advance_char();  //  skip '"'
            TokenKind::Str(acc)
        };
        Ok(self.token(kind, length))
    }

    pub fn pop(&mut self) -> Result<Token> {
        use TokenKind::*;

        self.skip_whitespace()?;

        Ok(match self.pop_char()? {
            None => self.token(Eof, 0),
            Some(c) => {
                match c {
                    '(' => self.token(LParen, 1),
                    ')' => self.token(RParen, 1),
                    '{' => self.token(LBrace, 1),
                    '}' => self.token(RBrace, 1),
                    ';' => self.token(Semicolon, 1),
                    ',' => self.token(Comma, 1),
                    '.' => self.token(Dot, 1),
                    '-' => self.token(Minus, 1),
                    '+' => self.token(Plus, 1),
                    '/' => if self.pop_char_if_eq('/')? { 
                        self.advance_line()?;   //  skip the whole line (because comments are skipped)
                        self.pop()? // return the token afterwards
                    } else { self.token(Slash, 1) },
                    '*' => self.token(Star, 1),

                    '!' => if self.pop_char_if_eq('=')? { self.token(BangEqual, 2) } else { self.token(Bang, 1) }
                    '=' => if self.pop_char_if_eq('=')? { self.token(EqualEqual, 2) } else { self.token(Equal, 1) }
                    '<' => if self.pop_char_if_eq('=')? { self.token(LesserEqual, 2) } else { self.token(Lesser, 1) }
                    '>' => if self.pop_char_if_eq('=')? { self.token(GreaterEqual, 2) } else { self.token(Greater, 1) }

                    '"' => self.read_string()?,

                    _ => self.token(Error(String::from("Unrecognizable token")), 1),
                }
            }
        })
    }
}
