use nom::{bytes::complete::tag, character::complete::char, combinator::map, IResult};

use super::Token;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Increment,
    Decrement,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    LeftShift,
    RightShift,
    Assignment,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    LeftShiftAssign,
    RightShiftAssign,
}

fn parse_operator_plus(input: &str) -> IResult<&str, Token> {
    map(char('+'), |_| Token::Operator(Operator::Plus))(input)
}

fn parse_operator_minus(input: &str) -> IResult<&str, Token> {
    map(char('-'), |_| Token::Operator(Operator::Minus))(input)
}

fn parse_operator_multiply(input: &str) -> IResult<&str, Token> {
    map(char('*'), |_| Token::Operator(Operator::Multiply))(input)
}

fn parse_operator_divide(input: &str) -> IResult<&str, Token> {
    map(char('/'), |_| Token::Operator(Operator::Divide))(input)
}

fn parse_operator_modulo(input: &str) -> IResult<&str, Token> {
    map(char('%'), |_| Token::Operator(Operator::Modulo))(input)
}
fn parse_operator_increment(input: &str) -> IResult<&str, Token> {
    map(tag("++"), |_| Token::Operator(Operator::Increment))(input)
}

fn parse_operator_decrement(input: &str) -> IResult<&str, Token> {
    map(tag("--"), |_| Token::Operator(Operator::Decrement))(input)
}

fn parse_operator_equal(input: &str) -> IResult<&str, Token> {
    map(tag("=="), |_| Token::Operator(Operator::Equal))(input)
}

fn parse_operator_not_equal(input: &str) -> IResult<&str, Token> {
    map(tag("!="), |_| Token::Operator(Operator::NotEqual))(input)
}

fn parse_operator_less_than(input: &str) -> IResult<&str, Token> {
    map(char('<'), |_| Token::Operator(Operator::LessThan))(input)
}

fn parse_operator_greater_than(input: &str) -> IResult<&str, Token> {
    map(char('>'), |_| Token::Operator(Operator::GreaterThan))(input)
}

fn parse_operator_less_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tag("<="), |_| Token::Operator(Operator::LessThanOrEqual))(input)
}

fn parse_operator_greater_than_or_equal(input: &str) -> IResult<&str, Token> {
    map(tag(">="), |_| Token::Operator(Operator::GreaterThanOrEqual))(input)
}

fn parse_operator_logical_and(input: &str) -> IResult<&str, Token> {
    map(tag("&&"), |_| Token::Operator(Operator::LogicalAnd))(input)
}

fn parse_operator_logical_or(input: &str) -> IResult<&str, Token> {
    map(tag("||"), |_| Token::Operator(Operator::LogicalOr))(input)
}

fn parse_operator_logical_not(input: &str) -> IResult<&str, Token> {
    map(char('!'), |_| Token::Operator(Operator::LogicalNot))(input)
}

fn parse_operator_bitwise_and(input: &str) -> IResult<&str, Token> {
    map(char('&'), |_| Token::Operator(Operator::BitwiseAnd))(input)
}

fn parse_operator_bitwise_or(input: &str) -> IResult<&str, Token> {
    map(char('|'), |_| Token::Operator(Operator::BitwiseOr))(input)
}

fn parse_operator_bitwise_not(input: &str) -> IResult<&str, Token> {
    map(char('~'), |_| Token::Operator(Operator::BitwiseNot))(input)
}

fn parse_operator_bitwise_xor(input: &str) -> IResult<&str, Token> {
    map(char('^'), |_| Token::Operator(Operator::BitwiseXor))(input)
}

fn parse_operator_left_shift(input: &str) -> IResult<&str, Token> {
    map(tag("<<"), |_| Token::Operator(Operator::LeftShift))(input)
}

fn parse_operator_right_shift(input: &str) -> IResult<&str, Token> {
    map(tag(">>"), |_| Token::Operator(Operator::RightShift))(input)
}

fn parse_operator_assignment(input: &str) -> IResult<&str, Token> {
    map(char('='), |_| Token::Operator(Operator::Assignment))(input)
}

fn parse_operator_add_assign(input: &str) -> IResult<&str, Token> {
    map(tag("+="), |_| Token::Operator(Operator::AddAssign))(input)
}

fn parse_operator_subtract_assign(input: &str) -> IResult<&str, Token> {
    map(tag("-="), |_| Token::Operator(Operator::SubtractAssign))(input)
}

fn parse_operator_multiply_assign(input: &str) -> IResult<&str, Token> {
    map(tag("*="), |_| Token::Operator(Operator::MultiplyAssign))(input)
}

fn parse_operator_divide_assign(input: &str) -> IResult<&str, Token> {
    map(tag("/="), |_| Token::Operator(Operator::DivideAssign))(input)
}

fn parse_operator_modulo_assign(input: &str) -> IResult<&str, Token> {
    map(tag("%="), |_| Token::Operator(Operator::ModuloAssign))(input)
}

fn parse_operator_and_assign(input: &str) -> IResult<&str, Token> {
    map(tag("&="), |_| Token::Operator(Operator::AndAssign))(input)
}

fn parse_operator_or_assign(input: &str) -> IResult<&str, Token> {
    map(tag("~="), |_| Token::Operator(Operator::OrAssign))(input)
}

fn parse_operator_xor_assign(input: &str) -> IResult<&str, Token> {
    map(tag("^="), |_| Token::Operator(Operator::XorAssign))(input)
}

fn parse_operator_left_shift_assign(input: &str) -> IResult<&str, Token> {
    map(tag("<<="), |_| Token::Operator(Operator::LeftShiftAssign))(input)
}

fn parse_operator_right_shift_assign(input: &str) -> IResult<&str, Token> {
    map(tag(">>="), |_| Token::Operator(Operator::RightShiftAssign))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operator_plus() {
        let result = parse_operator_plus("+");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Plus))));
    }

    #[test]
    fn test_parse_operator_minus() {
        let result = parse_operator_minus("-");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Minus))));
    }

    #[test]
    fn test_parse_operator_multiply() {
        let result = parse_operator_multiply("*");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Multiply))));
    }

    #[test]
    fn test_parse_operator_divide() {
        let result = parse_operator_divide("/");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Divide))));
    }

    #[test]
    fn test_parse_operator_modulo() {
        let result = parse_operator_modulo("%");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Modulo))));
    }

    #[test]
    fn test_parse_operator_increment() {
        let result = parse_operator_increment("++");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Increment))));
    }

    #[test]
    fn test_parse_operator_decrement() {
        let result = parse_operator_decrement("--");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Decrement))));
    }

    #[test]
    fn test_parse_operator_equal() {
        let result = parse_operator_equal("==");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Equal))));
    }

    #[test]
    fn test_parse_operator_not_equal() {
        let result = parse_operator_not_equal("!=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::NotEqual))));
    }

    #[test]
    fn test_parse_operator_less_than() {
        let result = parse_operator_less_than("<");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LessThan))));
    }

    #[test]
    fn test_parse_operator_greater_than() {
        let result = parse_operator_greater_than(">");
        assert_eq!(result, Ok(("", Token::Operator(Operator::GreaterThan))));
    }

    #[test]
    fn test_parse_operator_less_than_or_equal() {
        let result = parse_operator_less_than_or_equal("<=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LessThanOrEqual))));
    }

    #[test]
    fn test_parse_operator_greater_than_or_equal() {
        let result = parse_operator_greater_than_or_equal(">=");
        assert_eq!(
            result,
            Ok(("", Token::Operator(Operator::GreaterThanOrEqual)))
        );
    }

    #[test]
    fn test_parse_operator_logical_and() {
        let result = parse_operator_logical_and("&&");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LogicalAnd))));
    }

    #[test]
    fn test_parse_operator_logical_or() {
        let result = parse_operator_logical_or("||");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LogicalOr))));
    }

    #[test]
    fn test_parse_operator_logical_not() {
        let result = parse_operator_logical_not("!");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LogicalNot))));
    }

    #[test]
    fn test_parse_operator_bitwise_and() {
        let result = parse_operator_bitwise_and("&");
        assert_eq!(result, Ok(("", Token::Operator(Operator::BitwiseAnd))));
    }

    #[test]
    fn test_parse_operator_bitwise_or() {
        let result = parse_operator_bitwise_or("|");
        assert_eq!(result, Ok(("", Token::Operator(Operator::BitwiseOr))));
    }

    #[test]
    fn test_parse_operator_bitwise_not() {
        let result = parse_operator_bitwise_not("~");
        assert_eq!(result, Ok(("", Token::Operator(Operator::BitwiseNot))));
    }

    #[test]
    fn test_parse_operator_bitwise_xor() {
        let result = parse_operator_bitwise_xor("^");
        assert_eq!(result, Ok(("", Token::Operator(Operator::BitwiseXor))));
    }

    #[test]
    fn test_parse_operator_left_shift() {
        let result = parse_operator_left_shift("<<");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LeftShift))));
    }

    #[test]
    fn test_parse_operator_right_shift() {
        let result = parse_operator_right_shift(">>");
        assert_eq!(result, Ok(("", Token::Operator(Operator::RightShift))));
    }

    #[test]
    fn test_parse_operator_assignment() {
        let result = parse_operator_assignment("=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::Assignment))));
    }

    #[test]
    fn test_parse_operator_add_assign() {
        let result = parse_operator_add_assign("+=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::AddAssign))));
    }

    #[test]
    fn test_parse_operator_subtract_assign() {
        let result = parse_operator_subtract_assign("-=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::SubtractAssign))));
    }

    #[test]
    fn test_parse_operator_multiply_assign() {
        let result = parse_operator_multiply_assign("*=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::MultiplyAssign))));
    }

    #[test]
    fn test_parse_operator_divide_assign() {
        let result = parse_operator_divide_assign("/=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::DivideAssign))));
    }

    #[test]
    fn test_parse_operator_modulo_assign() {
        let result = parse_operator_modulo_assign("%=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::ModuloAssign))));
    }

    #[test]
    fn test_parse_operator_and_assign() {
        let result = parse_operator_and_assign("&=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::AndAssign))));
    }

    #[test]
    fn test_parse_operator_or_assign() {
        let result = parse_operator_or_assign("~=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::OrAssign))));
    }

    #[test]
    fn test_parse_operator_xor_assign() {
        let result = parse_operator_xor_assign("^=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::XorAssign))));
    }

    #[test]
    fn test_parse_operator_left_shift_assign() {
        let result = parse_operator_left_shift_assign("<<=");
        assert_eq!(result, Ok(("", Token::Operator(Operator::LeftShiftAssign))));
    }

    #[test]
    fn test_parse_operator_right_shift_assign() {
        let result = parse_operator_right_shift_assign(">>=");
        assert_eq!(
            result,
            Ok(("", Token::Operator(Operator::RightShiftAssign)))
        );
    }
}
