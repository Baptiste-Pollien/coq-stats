use crate::file_analysis::stats_file::StatsFile;
use crate::lexer::{self, token::Token};

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    COMMENT(u64),
    CODE,
    PROPOSITION,
    PROOF,
    NEXTOBLIGATION,
}

pub struct Analyser {
    state: State,
    previous_state: State,
}

impl Default for Analyser {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyser {
    pub fn new() -> Self {
        Self {
            state: State::CODE,
            previous_state: State::CODE,
        }
    }

    /// Return true is the analyser is in a COMMENT state
    fn is_comment_state(&self) -> bool {
        matches!(self.state, State::COMMENT(_))
    }

    /// Return true is the analyser is in a NEXT_OBLIGATION state
    fn is_next_state(&self) -> bool {
        matches!(self.state, State::NEXTOBLIGATION)
    }

    // Increment the inner comment
    pub fn incr_comment(&mut self) {
        match self.state {
            State::COMMENT(i) => {
                self.state = State::COMMENT(i + 1);
            }
            state => {
                self.previous_state = state;
                self.state = State::COMMENT(1);
            }
        }
    }

    // Decrement the inner comment
    pub fn decr_comment(&mut self) {
        match self.state {
            State::COMMENT(i) => {
                if i <= 1 {
                    self.state = self.previous_state;
                } else {
                    self.state = State::COMMENT(i - 1);
                }
            }
            state => {
                self.previous_state = state;
                self.state = State::COMMENT(1);
            }
        }
    }

    /// Function that takes the first token of the line
    /// and return true if the analyse must be before the
    /// analysis of the token
    fn test_fst_token(token: &Token) -> bool {
        matches!(token, Token::Rcomm | Token::Qed | Token::Admitted)
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
            State::PROPOSITION => {
                stats.coq_stats.line_proposition += 1;
            }
            State::PROOF | State::NEXTOBLIGATION => {
                stats.coq_stats.line_proof += 1;
            }
        }
    }

    /// Analyse the token and update the stats. This function must not be
    /// called in the COMMENT state.
    fn analyse_token_coq(&mut self, token: Token, stats: &mut StatsFile) {
        match token {
            Token::Lemma => {
                stats.coq_stats.nb_lemma += 1;
                self.state = State::PROPOSITION;
            }
            Token::Theorem => {
                stats.coq_stats.nb_theorem += 1;
                self.state = State::PROPOSITION;
            }
            Token::Proof => self.state = State::PROOF,
            Token::Qed => {
                stats.coq_stats.nb_proof += 1;
                self.state = State::CODE;
            }
            Token::Admitted => {
                stats.coq_stats.nb_admitted += 1;
                self.state = State::CODE;
            }
            Token::Next => {
                self.previous_state = self.state;
                self.state = State::NEXTOBLIGATION;
            }
            _ => {}
        }
    }

    /// Analysis of the token is the NEXT_OBLIGATION stae
    fn analyse_token_next(&mut self, token: Token) {
        match token {
            Token::Obligation => {
                self.state = State::PROOF;
            }
            _ => {
                self.state = self.previous_state;
            }
        }
    }

    /// Return true if the end of the line or the file is reached
    fn analyse_token(&mut self, token: Token, stats: &mut StatsFile) -> bool {
        let mut res = false;

        match token {
            Token::Lcomm => {
                self.incr_comment();
            }
            Token::Rcomm => {
                self.decr_comment();
            }
            Token::Eof | Token::Eol | Token::End => {
                res = true;
            }
            _ => {
                if self.is_next_state() {
                    self.analyse_token_next(token);
                } else if !self.is_comment_state() {
                    self.analyse_token_coq(token, stats);
                }
            }
        }
        res
    }

    /// Analyse a line
    pub fn analyse_line(
        &mut self,
        lexer: &mut lexer::Lexer,
        stats: &mut StatsFile,
        fst_token: Token,
    ) {
        if Analyser::test_fst_token(&fst_token) {
            self.analyse_state(stats);
            self.analyse_token(fst_token, stats);
        } else {
            self.analyse_token(fst_token, stats);
            self.analyse_state(stats);
        }

        while !self.analyse_token(lexer.next_token(), stats) {}
    }
}
