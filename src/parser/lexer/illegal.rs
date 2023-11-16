use nom::{
    branch::alt,
    character::complete::anychar,
    error::{Error, ErrorKind},
    multi::many_till,
    IResult,
};

use crate::parser::tokenizer::token::Token;

use super::{
    comment::parse_comment, identifier::parse_identifier, keyword::*, literal::parse_literal,
    operator::*, punctuation::parse_punctuation,
};

pub fn parse_illegal(input: &str) -> IResult<&str, Token> {
    // 使用`many_till`来收集所有字符，直到遇到已知的Token
    let (remaining, (illegal_chars, _)) = many_till(anychar, parse_known_token)(input)?;

    // 如果`illegal_chars`为空，意味着输入以已知Token开始
    if illegal_chars.is_empty() {
        // 创建并返回一个nom的Error
        Err(nom::Err::Error(Error::new(input, ErrorKind::Fail)))
    } else {
        // 否则，返回收集到的非法字符作为`Token::Illegal`
        Ok((
            remaining,
            Token::Illegal(illegal_chars.into_iter().collect()),
        ))
    }
}

pub fn parse_known_token(input: &str) -> IResult<&str, bool> {
    let (remaining, _) = alt((
        parse_comment,
        parse_identifier,
        parse_keyword,
        parse_literal,
        parse_operator,
        parse_punctuation,
    ))(input)?;

    Ok((remaining, true))
}

#[cfg(test)]
mod tests {}
