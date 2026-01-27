use crate::ast::{BinOp, Expr, Type};
use crate::lexer::{lex, LexError, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
    let tokens = lex(input).map_err(|err| ParseError {
        message: err.message,
    })?;
    let mut parser = Parser { tokens: &tokens, pos: 0 };
    let expr = parser.parse_expression()?;
    if parser.peek().is_some() {
        return Err(ParseError {
            message: "unexpected tokens after expression".to_string(),
        });
    }
    Ok(expr)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        if self.consume(&Token::Let) {
            let name = self.expect_ident()?;
            self.expect(&Token::Assign)?;
            let value = self.parse_expression()?;
            self.expect(&Token::In)?;
            let body = self.parse_expression()?;
            return Ok(Expr::Let(name, Box::new(value), Box::new(body)));
        }
        if self.consume(&Token::If) {
            let cond = self.parse_expression()?;
            self.expect(&Token::Then)?;
            let then_branch = self.parse_expression()?;
            self.expect(&Token::Else)?;
            let else_branch = self.parse_expression()?;
            return Ok(Expr::If(
                Box::new(cond),
                Box::new(then_branch),
                Box::new(else_branch),
            ));
        }
        if self.consume(&Token::Lambda) {
            let param = self.expect_ident()?;
            self.expect(&Token::Colon)?;
            let param_type = self.parse_type()?;
            self.expect(&Token::Arrow)?;
            let body = self.parse_expression()?;
            return Ok(Expr::Lambda {
                param,
                param_type,
                body: Box::new(body),
            });
        }
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        while self.consume(&Token::EqEq) {
            let rhs = self.parse_comparison()?;
            expr = Expr::BinOp(BinOp::Eq, Box::new(expr), Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        while self.consume(&Token::Lt) {
            let rhs = self.parse_term()?;
            expr = Expr::BinOp(BinOp::Lt, Box::new(expr), Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        loop {
            if self.consume(&Token::Plus) {
                let rhs = self.parse_factor()?;
                expr = Expr::BinOp(BinOp::Add, Box::new(expr), Box::new(rhs));
            } else if self.consume(&Token::Minus) {
                let rhs = self.parse_factor()?;
                expr = Expr::BinOp(BinOp::Sub, Box::new(expr), Box::new(rhs));
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_application()?;
        while self.consume(&Token::Star) {
            let rhs = self.parse_application()?;
            expr = Expr::BinOp(BinOp::Mul, Box::new(expr), Box::new(rhs));
        }
        Ok(expr)
    }

    fn parse_application(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_atom()?;
        loop {
            let backup = self.pos;
            if let Ok(arg) = self.parse_atom() {
                expr = Expr::App(Box::new(expr), Box::new(arg));
            } else {
                self.pos = backup;
                break;
            }
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::Int(value)) => {
                let value = *value;
                self.pos += 1;
                Ok(Expr::Int(value))
            }
            Some(Token::Bool(value)) => {
                let value = *value;
                self.pos += 1;
                Ok(Expr::Bool(value))
            }
            Some(Token::Ident(_)) => {
                let name = self.expect_ident()?;
                Ok(Expr::Var(name))
            }
            Some(Token::LParen) => {
                self.pos += 1;
                let expr = self.parse_expression()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            _ => Err(ParseError {
                message: "expected expression".to_string(),
            }),
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let mut ty = match self.peek() {
            Some(Token::Ident(name)) if name == "Int" => {
                self.pos += 1;
                Type::Int
            }
            Some(Token::Ident(name)) if name == "Bool" => {
                self.pos += 1;
                Type::Bool
            }
            Some(Token::LParen) => {
                self.pos += 1;
                let inner = self.parse_type()?;
                self.expect(&Token::RParen)?;
                inner
            }
            _ => {
                return Err(ParseError {
                    message: "expected type".to_string(),
                })
            }
        };

        if self.consume(&Token::Arrow) {
            let ret = self.parse_type()?;
            ty = Type::Func(Box::new(ty), Box::new(ret));
        }

        Ok(ty)
    }

    fn expect(&mut self, token: &Token) -> Result<(), ParseError> {
        match self.peek() {
            Some(next) if next == token => {
                self.pos += 1;
                Ok(())
            }
            _ => Err(ParseError {
                message: format!("expected token {token:?}"),
            }),
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.pos += 1;
                Ok(name)
            }
            _ => Err(ParseError {
                message: "expected identifier".to_string(),
            }),
        }
    }

    fn consume(&mut self, token: &Token) -> bool {
        if matches!(self.peek(), Some(next) if next == token) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        ParseError { message: err.message }
    }
}
