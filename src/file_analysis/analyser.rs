use crate::lexer::{self, token::Token};
use crate::file_analysis::stats_file::StatsFile;

#[derive(Copy, Clone)]
pub enum State {
    COMMENT(u64),
    CODE,
    PROOF,
}

pub struct Analyser {
    state: State,
    previous_state: State,
}

impl Analyser{

    pub fn new() -> Self {
        Self {
            state: State::CODE,
            previous_state: State::CODE,
        }
    }

    // Increment the inner comment
    pub fn incr_comment(&mut self) {
        match self.state {
            State::COMMENT(i) => {
                self.state = State::COMMENT(i + 1);
            },
            state => {
                self.previous_state = state;
                self.state = State::COMMENT(1);
            },
        }
    }

    // Decrement the inner comment
    pub fn decr_comment(&mut self) {
        match self.state {
            State::COMMENT(i) => {
                if i <= 1 {
                    self.state = self.previous_state;
                }
                else {
                    self.state = State::COMMENT(i - 1);
                }
            },
            state => {
                self.previous_state = state;
                self.state = State::COMMENT(1);
            },
        }
    }

    // Return true if the end of the line or the file is reached
    fn analyse_token(&mut self, token: Token) -> bool {
        let mut res = false;

        match token {
            Token::LCOMM => {
                self.incr_comment();
            },
            Token::EOF | Token::EOL => {
                res = true;
            },
            _ => {
                // Implement the other cases
            },
        }
        res
    }

    pub fn analyse_line(&mut self,
                             lexer: &mut lexer::Lexer,
                             stats: &mut StatsFile,
                             fst_token: Token) {
        // Analysis of the first token
        self.analyse_token(fst_token);

        match self.state {
            State::COMMENT(_) => {
                stats.comments += 1;
            }
            State::CODE => {
                stats.code += 1;
            }
            State::PROOF => {

            }
        }

        while self.analyse_token(lexer.next_token()){

        }

    }
}