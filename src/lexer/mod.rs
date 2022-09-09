pub mod token;
/// Module based on the code found in 
/// https://github.com/mohitk05/monkey-rust

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' 
        || ch == '_' || ch == '.'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0'
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position == self.input.len() {
            self.ch = '0';
        } else if self.read_position > self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
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
            '\n' => {
                tok = token::Token::EOL
            }
            '.' => {
                tok = token::Token::DOT
            }
            '=' => {
                tok = token::Token::ASSIGN;
            },
            '+' => {
                tok = token::Token::PLUS;
            },
            '-' => {
                tok = token::Token::MINUS;
            },
            '!' => {
                tok = token::Token::BANG;
            },
            '/' => {
                tok = token::Token::SLASH;
            },
            '*' => {
                match self.look_ahead() {
                    ')' => {
                        self.read_char();
                        tok = token::Token::RCOMM;
                    },
                    _ => {
                        tok = token::Token::ASTERISK;
                    }
                }
            },
            '<' => {
                tok = token::Token::LT;
            },
            '>' => {
                tok = token::Token::GT;
            },
            ';' => {
                tok = token::Token::SEMICOLON;
            },
            '(' => {
                match self.look_ahead() {
                    '*' => {
                        self.read_char();
                        tok = token::Token::LCOMM;
                    },
                    _ => {
                        tok = token::Token::LPAREN;
                    },
                }
            },
            ')' => {
               tok = token::Token::RPAREN;
            },
            ',' => {
                tok = token::Token::COMMA;
            },
            '{' => {
                tok = token::Token::LBRACE;
            },
            '}' => {
                tok = token::Token::RBRACE;
            },
            '0' => {
                tok = token::Token::EOF;
            },
            '\0' => {
                tok = token::Token::END;
            }
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keywork_token) => {
                            return keywork_token;
                        },
                        Err(_err) => {
                            return token::Token::IDENT(ident);
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return token::Token::INT(ident);
                } 
                else {
                    tok = token::Token::ILLEGAL(self.ch);
                }
            }
        }
        self.read_char();
        tok
    }
}
