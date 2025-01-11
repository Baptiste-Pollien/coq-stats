#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal(char),
    Eof,
    End,
    Eol,
    Dot,
    Ident(Vec<char>),
    Int(Vec<char>),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lcomm,
    Rcomm,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lemma,
    Theorem,
    Proof,
    Admitted,
    Qed,
    Next,
    Obligation,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
}

pub fn get_keyword_token(ident: &[char]) -> Result<Token, String> {
    let identifier: String = ident.iter().collect();
    match &identifier[..] {
        "Lemma" => Ok(Token::Lemma),
        "Theorem" => Ok(Token::Theorem),
        "Proof." => Ok(Token::Proof),
        "Admitted." => Ok(Token::Admitted),
        "Qed." => Ok(Token::Qed),
        "Next" => Ok(Token::Next),
        "Obligation." => Ok(Token::Obligation),
        _ => Err(String::from("Not a keyword")),
    }
}
