use std::{iter::Peekable, slice::Iter};

use crate::parser::tokenizer::token::Token;

fn parse_identifier(tokens_iter: &mut Peekable<Iter<Token>>) -> Result<String, String> {
    if let Some(Token::Identifier(name)) = tokens_iter.next() {
        Ok(name.to_owned())
    } else {
        Err("Expected an identifier".to_string())
    }
}
