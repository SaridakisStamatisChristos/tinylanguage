#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Int(i64),
    Bool(bool),
    Ident(String),
    Let,
    In,
    If,
    Then,
    Else,
    Lambda,
    Colon,
    Arrow,
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    EqEq,
    Lt,
    Assign,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '<' => {
                chars.next();
                tokens.push(Token::Lt);
            }
            '=' => {
                chars.next();
                if matches!(chars.peek(), Some('=')) {
                    chars.next();
                    tokens.push(Token::EqEq);
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '-' => {
                chars.next();
                if matches!(chars.peek(), Some('>')) {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '\\' => {
                chars.next();
                tokens.push(Token::Lambda);
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(c @ '0'..='9') = chars.peek().copied() {
                    num.push(c);
                    chars.next();
                }
                let value = num.parse::<i64>().map_err(|err| LexError {
                    message: format!("invalid integer literal: {err}"),
                })?;
                tokens.push(Token::Int(value));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(c @ ('a'..='z' | 'A'..='Z' | '_' | '0'..='9')) =
                    chars.peek().copied()
                {
                    ident.push(c);
                    chars.next();
                }
                match ident.as_str() {
                    "let" => tokens.push(Token::Let),
                    "in" => tokens.push(Token::In),
                    "if" => tokens.push(Token::If),
                    "then" => tokens.push(Token::Then),
                    "else" => tokens.push(Token::Else),
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            _ => {
                return Err(LexError {
                    message: format!("unexpected character '{ch}'"),
                });
            }
        }
    }

    Ok(tokens)
}
