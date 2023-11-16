use nom::{branch::alt, bytes::complete::tag, character::complete::char, combinator::map, IResult};

use crate::parser::tokenizer::token::Token;

pub fn parse_operator_plus(input: &str) -> IResult<&str, Token> {
    map(char('+'), |_| Token::Plus)(input)
}

pub fn parse_operator_minus(input: &str) -> IResult<&str, Token> {
    map(char('-'), |_| Token::Minus)(input)
}

pub fn parse_operator_multiply(input: &str) -> IResult<&str, Token> {
    map(char('*'), |_| Token::Multiply)(input)
}

pub fn parse_operator_divide(input: &str) -> IResult<&str, Token> {
    map(char('/'), |_| Token::Divide)(input)
}

pub fn parse_operator_modulo(input: &str) -> IResult<&str, Token> {
    map(char('%'), |_| Token::Modulo)(input)
}

pub fn parse_operator_increment(input: &str) -> IResult<&str, Token> {
    map(tag("++"), |_| Token::Increment)(input)
}

pub fn parse_operator_decrement(input: &str) -> IResult<&str, Token> {
    map(tag("--"), |_| Token::Decrement)(input)
}

pub fn parse_operator_equal(input: &str) -> IResult<&str, Token> {
    map(tag("=="), |_| Token::Equal)(input)
}

pub fn parse_operator_not_equal(input: &str) -> IResult<&str, Token> {
    map(tag("!="), |_| Token::NotEqual)(input)
}

pub fn parse_operator_less_than(input: &str) -> IResult<&str, Token> {
    map(char('<'), |_| Token::LessThan)(input)
}

pub fn parse_operator_greater_than(input: &str) -> IResult<&str, Token> {
    map(char('>'), |_| Token::GreaterThan)(input)
}

pub fn parse_operator_less_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tag("<="), |_| Token::LessThanOrEqual)(input)
}

pub fn parse_operator_greater_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tag(">="), |_| Token::GreaterThanOrEqual)(input)
}

pub fn parse_operator_logical_and(input: &str) -> IResult<&str, Token> {
    map(tag("&&"), |_| Token::LogicalAnd)(input)
}

pub fn parse_operator_logical_or(input: &str) -> IResult<&str, Token> {
    map(tag("||"), |_| Token::LogicalOr)(input)
}

pub fn parse_operator_logical_not(input: &str) -> IResult<&str, Token> {
    map(char('!'), |_| Token::LogicalNot)(input)
}

pub fn parse_operator_bitwise_and(input: &str) -> IResult<&str, Token> {
    map(char('&'), |_| Token::BitwiseAnd)(input)
}

pub fn parse_operator_bitwise_or(input: &str) -> IResult<&str, Token> {
    map(char('|'), |_| Token::BitwiseOr)(input)
}

pub fn parse_operator_bitwise_not(input: &str) -> IResult<&str, Token> {
    map(char('~'), |_| Token::BitwiseNot)(input)
}

pub fn parse_operator_bitwise_xor(input: &str) -> IResult<&str, Token> {
    map(char('^'), |_| Token::BitwiseXor)(input)
}

pub fn parse_operator_left_shift(input: &str) -> IResult<&str, Token> {
    map(tag("<<"), |_| Token::LeftShift)(input)
}

pub fn parse_operator_right_shift(input: &str) -> IResult<&str, Token> {
    map(tag(">>"), |_| Token::RightShift)(input)
}

pub fn parse_operator_assignment(input: &str) -> IResult<&str, Token> {
    map(char('='), |_| Token::Assignment)(input)
}

pub fn parse_operator_add_assign(input: &str) -> IResult<&str, Token> {
    map(tag("+="), |_| Token::AddAssign)(input)
}

pub fn parse_operator_subtract_assign(input: &str) -> IResult<&str, Token> {
    map(tag("-="), |_| Token::SubtractAssign)(input)
}

pub fn parse_operator_multiply_assign(input: &str) -> IResult<&str, Token> {
    map(tag("*="), |_| Token::MultiplyAssign)(input)
}

pub fn parse_operator_divide_assign(input: &str) -> IResult<&str, Token> {
    map(tag("/="), |_| Token::DivideAssign)(input)
}

pub fn parse_operator_modulo_assign(input: &str) -> IResult<&str, Token> {
    map(tag("%="), |_| Token::ModuloAssign)(input)
}

pub fn parse_operator_and_assign(input: &str) -> IResult<&str, Token> {
    map(tag("&="), |_| Token::AndAssign)(input)
}

pub fn parse_operator_or_assign(input: &str) -> IResult<&str, Token> {
    map(tag("~="), |_| Token::OrAssign)(input)
}

pub fn parse_operator_xor_assign(input: &str) -> IResult<&str, Token> {
    map(tag("^="), |_| Token::XorAssign)(input)
}

pub fn parse_operator_left_shift_assign(input: &str) -> IResult<&str, Token> {
    map(tag("<<="), |_| Token::LeftShiftAssign)(input)
}

pub fn parse_operator_right_shift_assign(input: &str) -> IResult<&str, Token> {
    map(tag(">>="), |_| Token::RightShiftAssign)(input)
}

pub fn parse_operator_basic(input: &str) -> IResult<&str, Token> {
    alt((
        parse_operator_plus,
        parse_operator_minus,
        parse_operator_multiply,
        parse_operator_divide,
        parse_operator_modulo,
        parse_operator_increment,
        parse_operator_decrement,
    ))(input)
}

pub fn parse_operator_logical(input: &str) -> IResult<&str, Token> {
    alt((
        parse_operator_equal,
        parse_operator_not_equal,
        parse_operator_less_than,
        parse_operator_greater_than,
        parse_operator_less_than_or_equal,
        parse_operator_greater_than_or_equal,
        parse_operator_logical_and,
        parse_operator_logical_or,
        parse_operator_logical_not,
    ))(input)
}

pub fn parse_operator_bitwise(input: &str) -> IResult<&str, Token> {
    alt((
        parse_operator_bitwise_and,
        parse_operator_bitwise_or,
        parse_operator_bitwise_not,
        parse_operator_bitwise_xor,
        parse_operator_left_shift,
        parse_operator_right_shift,
    ))(input)
}

pub fn parse_operator_assign(input: &str) -> IResult<&str, Token> {
    alt((
        parse_operator_assignment,
        parse_operator_add_assign,
        parse_operator_subtract_assign,
        parse_operator_multiply_assign,
        parse_operator_divide_assign,
        parse_operator_modulo_assign,
        parse_operator_and_assign,
        parse_operator_or_assign,
        parse_operator_xor_assign,
        parse_operator_left_shift_assign,
        parse_operator_right_shift_assign,
    ))(input)
}

pub fn parse_operator(input: &str) -> IResult<&str, Token> {
    alt((
        parse_operator_assign,
        parse_operator_logical,
        parse_operator_basic,
        parse_operator_bitwise,
    ))(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::lexer::operator::*;

    #[test]
    fn test_parse_operator_plus() {
        let result = parse_operator_plus("+");
        assert_eq!(result, Ok(("", Token::Plus)));
    }

    #[test]
    fn test_parse_operator_minus() {
        let result = parse_operator_minus("-");
        assert_eq!(result, Ok(("", Token::Minus)));
    }

    #[test]
    fn test_parse_operator_multiply() {
        let result = parse_operator_multiply("*");
        assert_eq!(result, Ok(("", Token::Multiply)));
    }

    #[test]
    fn test_parse_operator_divide() {
        let result = parse_operator_divide("/");
        assert_eq!(result, Ok(("", Token::Divide)));
    }

    #[test]
    fn test_parse_operator_modulo() {
        let result = parse_operator_modulo("%");
        assert_eq!(result, Ok(("", Token::Modulo)));
    }

    #[test]
    fn test_parse_operator_increment() {
        let result = parse_operator_increment("++");
        assert_eq!(result, Ok(("", Token::Increment)));
    }

    #[test]
    fn test_parse_operator_decrement() {
        let result = parse_operator_decrement("--");
        assert_eq!(result, Ok(("", Token::Decrement)));
    }

    #[test]
    fn test_parse_operator_equal() {
        let result = parse_operator_equal("==");
        assert_eq!(result, Ok(("", Token::Equal)));
    }

    #[test]
    fn test_parse_operator_not_equal() {
        let result = parse_operator_not_equal("!=");
        assert_eq!(result, Ok(("", Token::NotEqual)));
    }

    #[test]
    fn test_parse_operator_less_than() {
        let result = parse_operator_less_than("<");
        assert_eq!(result, Ok(("", Token::LessThan)));
    }

    #[test]
    fn test_parse_operator_greater_than() {
        let result = parse_operator_greater_than(">");
        assert_eq!(result, Ok(("", Token::GreaterThan)));
    }

    #[test]
    fn test_parse_operator_less_than_or_equal() {
        let result = parse_operator_less_than_or_equal("<=");
        assert_eq!(result, Ok(("", Token::LessThanOrEqual)));
    }

    #[test]
    fn test_parse_operator_greater_than_or_equal() {
        let result = parse_operator_greater_than_or_equal(">=");
        assert_eq!(result, Ok(("", Token::GreaterThanOrEqual)));
    }

    #[test]
    fn test_parse_operator_logical_and() {
        let result = parse_operator_logical_and("&&");
        assert_eq!(result, Ok(("", Token::LogicalAnd)));
    }

    #[test]
    fn test_parse_operator_logical_or() {
        let result = parse_operator_logical_or("||");
        assert_eq!(result, Ok(("", Token::LogicalOr)));
    }

    #[test]
    fn test_parse_operator_logical_not() {
        let result = parse_operator_logical_not("!");
        assert_eq!(result, Ok(("", Token::LogicalNot)));
    }

    #[test]
    fn test_parse_operator_bitwise_and() {
        let result = parse_operator_bitwise_and("&");
        assert_eq!(result, Ok(("", Token::BitwiseAnd)));
    }

    #[test]
    fn test_parse_operator_bitwise_or() {
        let result = parse_operator_bitwise_or("|");
        assert_eq!(result, Ok(("", Token::BitwiseOr)));
    }

    #[test]
    fn test_parse_operator_bitwise_not() {
        let result = parse_operator_bitwise_not("~");
        assert_eq!(result, Ok(("", Token::BitwiseNot)));
    }

    #[test]
    fn test_parse_operator_bitwise_xor() {
        let result = parse_operator_bitwise_xor("^");
        assert_eq!(result, Ok(("", Token::BitwiseXor)));
    }

    #[test]
    fn test_parse_operator_left_shift() {
        let result = parse_operator_left_shift("<<");
        assert_eq!(result, Ok(("", Token::LeftShift)));
    }

    #[test]
    fn test_parse_operator_right_shift() {
        let result = parse_operator_right_shift(">>");
        assert_eq!(result, Ok(("", Token::RightShift)));
    }

    #[test]
    fn test_parse_operator_assignment() {
        let result = parse_operator_assignment("=");
        assert_eq!(result, Ok(("", Token::Assignment)));
    }

    #[test]
    fn test_parse_operator_add_assign() {
        let result = parse_operator_add_assign("+=");
        assert_eq!(result, Ok(("", Token::AddAssign)));
    }

    #[test]
    fn test_parse_operator_subtract_assign() {
        let result = parse_operator_subtract_assign("-=");
        assert_eq!(result, Ok(("", Token::SubtractAssign)));
    }

    #[test]
    fn test_parse_operator_multiply_assign() {
        let result = parse_operator_multiply_assign("*=");
        assert_eq!(result, Ok(("", Token::MultiplyAssign)));
    }

    #[test]
    fn test_parse_operator_divide_assign() {
        let result = parse_operator_divide_assign("/=");
        assert_eq!(result, Ok(("", Token::DivideAssign)));
    }

    #[test]
    fn test_parse_operator_modulo_assign() {
        let result = parse_operator_modulo_assign("%=");
        assert_eq!(result, Ok(("", Token::ModuloAssign)));
    }

    #[test]
    fn test_parse_operator_and_assign() {
        let result = parse_operator_and_assign("&=");
        assert_eq!(result, Ok(("", Token::AndAssign)));
    }

    #[test]
    fn test_parse_operator_or_assign() {
        let result = parse_operator_or_assign("~=");
        assert_eq!(result, Ok(("", Token::OrAssign)));
    }

    #[test]
    fn test_parse_operator_xor_assign() {
        let result = parse_operator_xor_assign("^=");
        assert_eq!(result, Ok(("", Token::XorAssign)));
    }

    #[test]
    fn test_parse_operator_left_shift_assign() {
        let result = parse_operator_left_shift_assign("<<=");
        assert_eq!(result, Ok(("", Token::LeftShiftAssign)));
    }

    #[test]
    fn test_parse_operator_right_shift_assign() {
        let result = parse_operator_right_shift_assign(">>=");
        assert_eq!(result, Ok(("", Token::RightShiftAssign)));
    }
}
