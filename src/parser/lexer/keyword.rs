use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::parser::tokenizer::token::Token;

pub fn parse_keyword_void(input: &str) -> IResult<&str, Token> {
    map(tag("void"), |_| Token::Void)(input)
}

pub fn parse_keyword_int(input: &str) -> IResult<&str, Token> {
    map(tag("int"), |_| Token::Int)(input)
}

pub fn parse_keyword_long(input: &str) -> IResult<&str, Token> {
    map(tag("long"), |_| Token::Long)(input)
}

pub fn parse_keyword_float(input: &str) -> IResult<&str, Token> {
    map(tag("float"), |_| Token::Float)(input)
}

pub fn parse_keyword_double(input: &str) -> IResult<&str, Token> {
    map(tag("double"), |_| Token::Double)(input)
}

pub fn parse_keyword_char(input: &str) -> IResult<&str, Token> {
    map(tag("char"), |_| Token::Char)(input)
}

pub fn parse_keyword_struct(input: &str) -> IResult<&str, Token> {
    map(tag("struct"), |_| Token::Struct)(input)
}

pub fn parse_keyword_union(input: &str) -> IResult<&str, Token> {
    map(tag("union"), |_| Token::Union)(input)
}

pub fn parse_keyword_if(input: &str) -> IResult<&str, Token> {
    map(tag("if"), |_| Token::If)(input)
}

pub fn parse_keyword_else(input: &str) -> IResult<&str, Token> {
    map(tag("else"), |_| Token::Else)(input)
}

pub fn parse_keyword_while(input: &str) -> IResult<&str, Token> {
    map(tag("while"), |_| Token::While)(input)
}

pub fn parse_keyword_do(input: &str) -> IResult<&str, Token> {
    map(tag("do"), |_| Token::Do)(input)
}

pub fn parse_keyword_for(input: &str) -> IResult<&str, Token> {
    map(tag("for"), |_| Token::For)(input)
}

pub fn parse_keyword_return(input: &str) -> IResult<&str, Token> {
    map(tag("return"), |_| Token::Return)(input)
}

pub fn parse_keyword(input: &str) -> IResult<&str, Token> {
    alt((
        parse_keyword_void,
        parse_keyword_int,
        parse_keyword_long,
        parse_keyword_float,
        parse_keyword_double,
        parse_keyword_char,
        parse_keyword_struct,
        parse_keyword_union,
        parse_keyword_if,
        parse_keyword_else,
        parse_keyword_while,
        parse_keyword_for,
        parse_keyword_return,
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::keyword::*;

    #[test]
    fn test_parse_keyword_void() {
        let result = parse_keyword_void("void");
        assert_eq!(result, Ok(("", Token::Void)));
    }

    #[test]
    fn test_parse_keyword_int() {
        let result = parse_keyword_int("int");
        assert_eq!(result, Ok(("", Token::Int)));
    }

    #[test]
    fn test_parse_keyword_long() {
        let result = parse_keyword_long("long");
        assert_eq!(result, Ok(("", Token::Long)));
    }

    #[test]
    fn test_parse_keyword_float() {
        let result = parse_keyword_float("float");
        assert_eq!(result, Ok(("", Token::Float)));
    }

    #[test]
    fn test_parse_keyword_double() {
        let result = parse_keyword_double("double");
        assert_eq!(result, Ok(("", Token::Double)));
    }

    #[test]
    fn test_parse_keyword_char() {
        let result = parse_keyword_char("char");
        assert_eq!(result, Ok(("", Token::Char)));
    }

    #[test]
    fn test_parse_keyword_struct() {
        let result = parse_keyword_struct("struct");
        assert_eq!(result, Ok(("", Token::Struct)));
    }

    #[test]
    fn test_parse_keyword_union() {
        let result = parse_keyword_union("union");
        assert_eq!(result, Ok(("", Token::Union)));
    }

    #[test]
    fn test_parse_keyword_if() {
        let result = parse_keyword_if("if");
        assert_eq!(result, Ok(("", Token::If)));
    }

    #[test]
    fn test_parse_keyword_else() {
        let result = parse_keyword_else("else");
        assert_eq!(result, Ok(("", Token::Else)));
    }

    #[test]
    fn test_parse_keyword_while() {
        let result = parse_keyword_while("while");
        assert_eq!(result, Ok(("", Token::While)));
    }

    #[test]
    fn test_parse_keyword_do() {
        let result = parse_keyword_do("do");
        assert_eq!(result, Ok(("", Token::Do)));
    }

    #[test]
    fn test_parse_keyword_for() {
        let result = parse_keyword_for("for");
        assert_eq!(result, Ok(("", Token::For)));
    }

    #[test]
    fn test_parse_keyword_return() {
        let result = parse_keyword_return("return");
        assert_eq!(result, Ok(("", Token::Return)));
    }
}
