use std::io::{BufRead, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //  single character misc tokens
    LParen, RParen,
    LBrace, RBrace,
    Comma, Dot, Minus, Plus, Percent,
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
    token_line: u32,
    token_col: u32,
    token_length: u32,
    line_chars: Vec<char>,
    peek: Option<Token>,
}

fn read_line_chars<R: BufRead>(reader: &mut R) -> Result<Vec<char>> {
    let mut buf = String::new();
    reader.read_line(&mut buf)?;
    Ok(buf.chars().collect())
}

impl<R: BufRead> Tokenizer<R> {
    pub fn from_reader(mut reader: R) -> Result<Self> {
        let line_chars = read_line_chars(&mut reader)?;
        Ok(Self { 
            reader, line_chars, 
            line: 0, col: 0, 
            peek: None, 
            token_line: 0, token_col: 0, token_length: 0,
        })
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
        self.token_length += 1;
        while (self.col as usize) >= self.line_chars.len() {
            self.advance_line()?;
            if self.line_chars.is_empty() { break; }    //  handle EOF
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

    fn begin_token(&mut self) {
        self.token_col = self.col;
        self.token_line = self.line;
    } 

    fn end_token(&self, kind: TokenKind) -> Token {
        Token { kind, line: self.token_line, col: self.token_col, length: self.token_length }
    }

    fn skip_whitespace(&mut self) -> Result<()> {
        while self.peek_char().map(char::is_whitespace).unwrap_or(false) {
            self.advance_char()?;
        }
        Ok(())
    }

    fn read_string(&mut self) -> Result<Token> {
        self.begin_token();
        self.advance_char();    //  skip first '"'

        let mut acc = String::new();
        while self.peek_char().map(|ch| ch != '"').unwrap_or(false) {
            acc.push(self.pop_char()?.unwrap());
        }

        let kind = if self.peek_char().is_none() {
            TokenKind::Error(String::from("Unterminated string"))
        } else {
            self.advance_char()?;  //  skip '"'
            TokenKind::Str(acc)
        };
        Ok(self.end_token(kind))
    }

    fn read_identifier(&mut self) -> Result<Token> {
        self.begin_token();

        let mut acc = String::new();
        while self.peek_char().filter(|x| x.is_alphanumeric() || *x == '_' ).is_some() {
            acc.push(self.pop_char()?.unwrap());
        }

        Ok(self.end_token(TokenKind::Ident(acc)))
    }

    fn read_number(&mut self) -> Result<Token> {
        self.begin_token();

        let mut acc = 0;
        
        while std::matches!(self.peek_char(), Some('0'..='9')) {
            let digit = char::to_digit(self.pop_char()?.unwrap(), 10).unwrap();
            acc = acc * 10 + digit as i32;
        }

        Ok(self.end_token(TokenKind::Int(acc)))
    }

    fn read_token(&mut self) -> Result<Token> {
        use TokenKind::*;

        self.skip_whitespace()?;

        Ok(match self.peek_char() {
            None => {
                self.begin_token();
                self.end_token(Eof)
            },
            Some('"') => self.read_string()?,
            Some('0'..='9') => self.read_number()?,
            Some('a'..='z') | Some('A'..='Z') | Some('_') => self.read_identifier()?,
            Some(c) => {
                self.begin_token();
                self.advance_char()?;
                match c {
                    '(' => self.end_token(LParen),
                    ')' => self.end_token(RParen),
                    '{' => self.end_token(LBrace),
                    '}' => self.end_token(RBrace),
                    ';' => self.end_token(Semicolon),
                    ',' => self.end_token(Comma),
                    '.' => self.end_token(Dot),
                    '-' => self.end_token(Minus),
                    '+' => self.end_token(Plus),
                    '/' => if self.pop_char_if_eq('/')? { 
                        self.advance_line()?;   //  skip the whole line (because comments are skipped)
                        self.pop()? // return the token afterwards
                    } else { self.end_token(Slash) },
                    '*' => self.end_token(Star),
                    '%' => self.end_token(Percent),

                    '!' => if self.pop_char_if_eq('=')? { self.end_token(BangEqual) } else { self.end_token(Bang) }
                    '=' => if self.pop_char_if_eq('=')? { self.end_token(EqualEqual) } else { self.end_token(Equal) }
                    '<' => if self.pop_char_if_eq('=')? { self.end_token(LesserEqual) } else { self.end_token(Lesser) }
                    '>' => if self.pop_char_if_eq('=')? { self.end_token(GreaterEqual) } else { self.end_token(Greater) }

                    _ => self.end_token(Error(String::from("Unrecognizable token"))),
                }
            }
        })
    }

    fn initialize_peek(&mut self) -> Result<()> {
        if self.peek.is_none() {
            self.peek = Some(self.read_token()?);
        };
        Ok(())
    }

    pub fn peek(&mut self) -> Result<&Token> {
        self.initialize_peek()?;
        Ok(self.peek.as_ref().unwrap())
    }

    pub fn pop(&mut self) -> Result<Token> {
        self.initialize_peek()?;
        //  return the peek, but replace it with none
        Ok(self.peek.take().unwrap())
    }
}
