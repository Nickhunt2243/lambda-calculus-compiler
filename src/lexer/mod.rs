pub mod types;

use types::{Token, Keyword, BooleanOps};
use crate::lexer::types::{AdditiveOps, MultiplicativeOps};

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    char_stream: &'a [u8],
    max_idx: usize,
    curr_idx: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(raw_string: &'a str) -> Self {
        Lexer {
            char_stream: raw_string.as_bytes(),
            curr_idx: 0,
            tokens: Vec::new(),
            max_idx: raw_string.len() - 1,
        }
    }
    fn append_token(&mut self, new_token: Token, next_idx: usize) {
        self.tokens.push(new_token);
        self.curr_idx = next_idx;
    }
    fn skip_whitespace(&mut self) {
        while self.curr_idx <= self.max_idx && is_white_space(self.char_stream[self.curr_idx]) {
            self.curr_idx += 1;
        }
    }
    fn try_keyword(&mut self, keyword: &str, token: Token) -> bool {
        let keyword_length = keyword.len();
        let char_length = self.char_stream.len();
        let end_idx = self.curr_idx + keyword_length;

        if end_idx > char_length {
            return false;
        }
        let matches = &self.char_stream[self.curr_idx..end_idx] == keyword.as_bytes();

        // at end of stream or followed by whitespace
        if end_idx == char_length || is_white_space(self.char_stream[end_idx]) {
            if matches {
                self.append_token(token, end_idx);
            }
            return matches;
        }

        false
    }

    fn try_symbol(&mut self, symbol: &str, token: Token) -> bool {
        let keyword_length = symbol.len();
        let char_length = self.char_stream.len();
        let end_idx = self.curr_idx + keyword_length;

        if end_idx > char_length {
            return false;
        }
        let matches = &self.char_stream[self.curr_idx..end_idx] == symbol.as_bytes();
        if matches {
            self.append_token(token, end_idx);
        }
        matches
    }

    fn try_identifier(&mut self) -> bool {

        let mut ident: String = (self.char_stream[self.curr_idx] as char).to_string();
        self.curr_idx += 1;
        while self.curr_idx <= self.max_idx &&
            (
                self.char_stream[self.curr_idx].is_ascii_alphanumeric() ||
                    matches!(self.char_stream[self.curr_idx], b'_')
            ) {
            ident.push(self.char_stream[self.curr_idx] as char);
            self.curr_idx += 1;
        }

        if ident == "_" {
            return false;
        }
        self.tokens.push(Token::Identifier(ident));

        true
    }

    fn try_number(&mut self) -> bool {
        let initial_idx = self.curr_idx;
        let mut total_number: String = (self.char_stream[self.curr_idx] as char).to_string();
        self.curr_idx += 1;
        while self.curr_idx <= self.max_idx &&
            self.char_stream[self.curr_idx].is_ascii_digit() {
            total_number.push(self.char_stream[self.curr_idx] as char);
            self.curr_idx += 1;
        }
        let parsed_number: i64 = total_number.parse::<i64>()
            .expect(&format!("Failed to parse number at position: {initial_idx}."));

        self.tokens.push(Token::IntegerLiteral(parsed_number));

        true
    }
    
    pub fn tokenize(&mut self) -> Result<&Vec<Token>, String> {
        while self.curr_idx <= self.max_idx {
            self.skip_whitespace();
            if self.curr_idx > self.max_idx {
                break;
            }

            match self.char_stream[self.curr_idx] {
                b'(' => self.append_token(Token::LParen, self.curr_idx + 1),
                b')' => self.append_token(Token::RParen, self.curr_idx + 1),
                b'+' => self.append_token(Token::AdditiveOps(AdditiveOps::Add), self.curr_idx + 1),
                b'*' => self.append_token(Token::MultiplicativeOps(MultiplicativeOps::Mul), self.curr_idx + 1),
                b'/' => self.append_token(Token::MultiplicativeOps(MultiplicativeOps::Div), self.curr_idx + 1),
                b'-' => {
                    let last_token = self.tokens.last();
                    let is_negative_number = match last_token {
                        Some(token) => expecting_numeric(token),
                        None => true
                    };
                    if is_negative_number {
                        self.try_number();
                    } else {
                        self.append_token(Token::AdditiveOps(AdditiveOps::Sub), self.curr_idx + 1);
                    }
                },
                b'=' => {
                    if self.try_symbol("=>", Token::Arrow) { continue; }
                    else if self.try_symbol("==", Token::BooleanOps(BooleanOps::Equality)) { continue; }
                    else { self.append_token(Token::EqualSign, self.curr_idx + 1); }
                },
                b'<' => {
                    if self.try_symbol("<=", Token::BooleanOps(BooleanOps::LessThanEqualTo)) { continue; }
                    self.append_token(Token::BooleanOps(BooleanOps::LessThan), self.curr_idx + 1);
                },
                b'>' => {
                    if self.try_symbol(">=", Token::BooleanOps(BooleanOps::GreaterThanEqualTo)) { continue; }
                    self.append_token(Token::BooleanOps(BooleanOps::GreaterThan), self.curr_idx + 1);
                },
                b'f' => {
                    if self.try_keyword("fn", Token::Keyword(Keyword::Fn)) { continue; }
                    else if self.try_keyword("false", Token::BooleanLiteral(false)) { continue; }
                    else { self.try_identifier(); }
                },
                b'l' => {
                    if self.try_keyword("letrec", Token::Keyword(Keyword::LetRec)) { continue; }
                    else if self.try_keyword("let", Token::Keyword(Keyword::Let)) { continue; }
                    else { self.try_identifier(); }
                },
                b'e' => {
                    if self.try_keyword("else", Token::Keyword(Keyword::Else)) { continue; }
                    else { self.try_identifier(); }
                },
                b't' => {
                    if self.try_keyword("then", Token::Keyword(Keyword::Then)) { continue; }
                    else if self.try_keyword("true", Token::BooleanLiteral(true)) { continue; }
                    else { self.try_identifier(); }
                },
                b'i' => {
                    if self.try_keyword("if", Token::Keyword(Keyword::If)) { continue; }
                    else if self.try_keyword("in", Token::Keyword(Keyword::In)) { continue; }
                    else { self.try_identifier(); }
                },
                b'0'..=b'9' => {
                    self.try_number();
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.try_identifier();
                }
                _ => {
                    let error_msg = format!("Lex Error: Unexpected character {} @ {}.", self.char_stream[self.curr_idx], self.curr_idx).to_string();
                    return Err(error_msg);
                }
            }

        }
        Ok(&self.tokens)
    }
}

fn is_white_space(c: u8) -> bool {
    c == b' ' || c == b'\n' || c == b'\r' || c == b'\t'
}

fn expecting_numeric(last_token: &Token) -> bool {
    matches!(last_token,
        Token::Keyword(_)
        | Token::EqualSign
        | Token::AdditiveOps(_)
        | Token::MultiplicativeOps(_)
        | Token::BooleanOps(_)
        | Token::Arrow
        | Token::LParen
    )
}

