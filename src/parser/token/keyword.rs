use nom::{bytes::complete::tag, combinator::map, IResult};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Long,
    Float,
    Double,
    Char,
    Struct,
    Union,
    If,
    Else,
    While,
    For,
    Return,
}

fn parse_keyword_int(input: &str) -> IResult<&str, Token> {
    map(tag("int"), |_| Token::Keyword(Keyword::Int))(input)
}

fn parse_keyword_long(input: &str) -> IResult<&str, Token> {
    map(tag("long"), |_| Token::Keyword(Keyword::Long))(input)
}

fn parse_keyword_float(input: &str) -> IResult<&str, Token> {
    map(tag("float"), |_| Token::Keyword(Keyword::Float))(input)
}

fn parse_keyword_double(input: &str) -> IResult<&str, Token> {
    map(tag("double"), |_| Token::Keyword(Keyword::Double))(input)
}

fn parse_keyword_char(input: &str) -> IResult<&str, Token> {
    map(tag("char"), |_| Token::Keyword(Keyword::Char))(input)
}

fn parse_keyword_struct(input: &str) -> IResult<&str, Token> {
    map(tag("struct"), |_| Token::Keyword(Keyword::Struct))(input)
}

fn parse_keyword_union(input: &str) -> IResult<&str, Token> {
    map(tag("union"), |_| Token::Keyword(Keyword::Union))(input)
}

fn parse_keyword_if(input: &str) -> IResult<&str, Token> {
    map(tag("if"), |_| Token::Keyword(Keyword::If))(input)
}

fn parse_keyword_else(input: &str) -> IResult<&str, Token> {
    map(tag("else"), |_| Token::Keyword(Keyword::Else))(input)
}
fn parse_keyword_while(input: &str) -> IResult<&str, Token> {
    map(tag("while"), |_| Token::Keyword(Keyword::While))(input)
}

fn parse_keyword_for(input: &str) -> IResult<&str, Token> {
    map(tag("for"), |_| Token::Keyword(Keyword::For))(input)
}

fn parse_keyword_return(input: &str) -> IResult<&str, Token> {
    map(tag("return"), |_| Token::Keyword(Keyword::Return))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword_int() {
        let result = parse_keyword_int("int");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Int))));
    }

    #[test]
    fn test_parse_keyword_long() {
        let result = parse_keyword_long("long");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Long))));
    }

    #[test]
    fn test_parse_keyword_float() {
        let result = parse_keyword_float("float");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Float))));
    }

    #[test]
    fn test_parse_keyword_double() {
        let result = parse_keyword_double("double");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Double))));
    }

    #[test]
    fn test_parse_keyword_char() {
        let result = parse_keyword_char("char");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Char))));
    }

    #[test]
    fn test_parse_keyword_struct() {
        let result = parse_keyword_struct("struct");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Struct))));
    }

    #[test]
    fn test_parse_keyword_union() {
        let result = parse_keyword_union("union");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Union))));
    }

    #[test]
    fn test_parse_keyword_if() {
        let result = parse_keyword_if("if");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::If))));
    }

    #[test]
    fn test_parse_keyword_else() {
        let result = parse_keyword_else("else");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Else))));
    }

    #[test]
    fn test_parse_keyword_while() {
        let result = parse_keyword_while("while");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::While))));
    }

    #[test]
    fn test_parse_keyword_for() {
        let result = parse_keyword_for("for");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::For))));
    }

    #[test]
    fn test_parse_keyword_return() {
        let result = parse_keyword_return("return");
        assert_eq!(result, Ok(("", Token::Keyword(Keyword::Return))));
    }
}
