//! Module based on the code found in
//! https://github.com/mohitk05/monkey-rust

use std::cmp::Ordering;

pub mod token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_' || ch == '.'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        match self.read_position.cmp(&self.input.len()) {
            Ordering::Equal => {
                self.ch = '0';
            }
            Ordering::Greater => {
                self.ch = '\0';
            }
            _ => {
                self.ch = self.input[self.read_position];
            }
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn look_ahead(&mut self) -> char {
        if self.read_position >= self.input.len() {
            '0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn end(&self) -> bool {
        self.ch == '\0'
    }

    pub fn skip_whitespace(&mut self) {
        let mut ch = self.ch;
        while ch == ' ' || ch == '\t' || ch == '\r' {
            self.read_char();
            ch = self.ch;
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '\n' => tok = token::Token::Eol,
            '.' => tok = token::Token::Dot,
            '=' => {
                tok = token::Token::Assign;
            }
            '+' => {
                tok = token::Token::Plus;
            }
            '-' => {
                tok = token::Token::Minus;
            }
            '!' => {
                tok = token::Token::Bang;
            }
            '/' => {
                tok = token::Token::Slash;
            }
            '*' => match self.look_ahead() {
                ')' => {
                    self.read_char();
                    tok = token::Token::Rcomm;
                }
                _ => {
                    tok = token::Token::Asterisk;
                }
            },
            '<' => {
                tok = token::Token::Lt;
            }
            '>' => {
                tok = token::Token::Gt;
            }
            ';' => {
                tok = token::Token::Semicolon;
            }
            '(' => match self.look_ahead() {
                '*' => {
                    self.read_char();
                    tok = token::Token::Lcomm;
                }
                _ => {
                    tok = token::Token::Lparen;
                }
            },
            ')' => {
                tok = token::Token::Rparen;
            }
            ',' => {
                tok = token::Token::Comma;
            }
            '{' => {
                tok = token::Token::Lbrace;
            }
            '}' => {
                tok = token::Token::Rbrace;
            }
            '0' => {
                tok = token::Token::Eof;
            }
            '\0' => {
                tok = token::Token::End;
            }
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        }
                        Err(_err) => {
                            return token::Token::Ident(ident);
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return token::Token::Int(ident);
                } else {
                    tok = token::Token::Illegal(self.ch);
                }
            }
        }
        self.read_char();
        tok
    }
}
