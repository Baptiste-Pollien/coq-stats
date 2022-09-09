use crate::lexer::{self, token::Token};
use crate::file_analysis::stats_file::StatsFile;

#[derive(Copy, Clone, PartialEq)]
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

    /// Return true is the analyser is in a COMMENT state
    fn is_comment_state (&self) -> bool {
        match self.state {
            State::COMMENT(_) => {
                false
            }
            _ => {
                true
            }
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

    /// Function that takes the first token of the line
    /// and return true if the analyse must be before the 
    /// analysis of the token
    fn test_fst_token(token: &Token) -> bool {
        match token {
            Token::RCOMM 
            | Token::QED 
            | Token::ADMITTED=> {
                true
            }
            _ => {
                false
            }
        }
    }

    // Analyse the current state and update the stats
    fn analyse_state(&mut self, stats: &mut StatsFile) {
        match self.state {
            State::COMMENT(_) => {
                stats.comments += 1;
            }
            State::CODE => {
                stats.coq_stats.line_code += 1;
            }
            State::PROOF => {
                stats.coq_stats.line_proof += 1;
            }
        }
    }

    /// Analyse the token and update the stats. This function must not be
    /// called in the COMMENT state.
    fn analyse_token_coq(&mut self, token: Token, stats: &mut StatsFile) {
        match token {
            Token::LEMMA => {
                stats.coq_stats.nb_lemma += 1;
            },
            Token::THEOREM => {
                stats.coq_stats.nb_theorem += 1;
            },
            Token::PROOF => {
                self.state = State::PROOF
            },
            Token::QED => {
                stats.coq_stats.nb_proof += 1;
                self.state = State::CODE;
            },
            Token::ADMITTED => {
                stats.coq_stats.nb_admitted += 1;
                self.state = State::CODE;
            },
            _ => {

            }
        }
    }

    /// Return true if the end of the line or the file is reached
    fn analyse_token(&mut self, token: Token, stats: &mut StatsFile)
                                                                -> bool {
        let mut res = false;

        match token {
            Token::LCOMM => {
                self.incr_comment();
            },
            Token::RCOMM => {
                self.decr_comment();
            },
            Token::EOF | Token::EOL | Token::END => {
                res = true;
            },
            _ => {
                if self.is_comment_state() {
                    self.analyse_token_coq(token, stats);
                }
            },
        }
        res
    }

    /// Analyse a line
    pub fn analyse_line(&mut self,
                             lexer: &mut lexer::Lexer,
                             stats: &mut StatsFile,
                             fst_token: Token) {

        if Analyser::test_fst_token(&fst_token) {
            self.analyse_state(stats);
            self.analyse_token(fst_token, stats);
        } else {
            self.analyse_token(fst_token, stats);
            self.analyse_state(stats);
        }

        while !self.analyse_token(lexer.next_token(), stats){
        }
    }

}