

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    ILLEGAL(char),
    EOF,
    END,
    EOL,
    DOT,
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LCOMM,
    RCOMM,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LEMMA,
    THEOREM,
    PROOF,
    ADMITTED,
    QED,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "Lemma" => Ok(Token::LEMMA),
        "Theorem" => Ok(Token::THEOREM),
        "Proof." => Ok(Token::PROOF),
        "Admitted." => Ok(Token::ADMITTED),
        "Qed." => Ok(Token::QED),
        _ => Err(String::from("Not a keyword"))
    }
}
