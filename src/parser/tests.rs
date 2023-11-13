use crate::{
    ast::node::*,
    parser::{
        lexer::{comment::*, identifier::*, keyword::*, operator::*, punctuation::*},
        syntax::{declaration::*, literal::*},
        tokenizer::token::Token,
    },
};

// lexer

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
fn test_parse_keyword_for() {
    let result = parse_keyword_for("for");
    assert_eq!(result, Ok(("", Token::For)));
}

#[test]
fn test_parse_keyword_return() {
    let result = parse_keyword_return("return");
    assert_eq!(result, Ok(("", Token::Return)));
}

#[test]
fn test_parse_integer_literal() {
    let result = parse_integer_literal("12345");
    assert_eq!(result, Ok(("", Expression::IntegerLiteral(12345))));
}

#[test]
fn test_parse_floating_literal() {
    let result = parse_floating_literal("123.45");
    assert_eq!(result, Ok(("", Expression::FloatingLiteral(123.45))));
}

#[test]
fn test_parse_char_literal() {
    let result = parse_char_literal("'a'");
    assert_eq!(result, Ok(("", Expression::CharLiteral('a'))));
}

#[test]
fn test_parse_string_literal() {
    let result = parse_string_literal("\"Hello, World!\"");
    assert_eq!(
        result,
        Ok(("", Expression::StringLiteral("Hello, World!".to_string())))
    );
}

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

#[test]
fn test_parse_comma() {
    let result = parse_comma(",");
    assert_eq!(result, Ok(("", Token::Comma)));
}

#[test]
fn test_parse_semicolon() {
    let result = parse_semicolon(";");
    assert_eq!(result, Ok(("", Token::SemiColon)));
}

#[test]
fn test_parse_open_paren() {
    let result = parse_open_paren("(");
    assert_eq!(result, Ok(("", Token::OpenParen)));
}

#[test]
fn test_parse_close_paren() {
    let result = parse_close_paren(")");
    assert_eq!(result, Ok(("", Token::CloseParen)));
}

#[test]
fn test_parse_open_brace() {
    let result = parse_open_brace("{");
    assert_eq!(result, Ok(("", Token::OpenBrace)));
}

#[test]
fn test_parse_close_brace() {
    let result = parse_close_brace("}");
    assert_eq!(result, Ok(("", Token::CloseBrace)));
}

#[test]
fn test_parse_open_bracket() {
    let result = parse_open_bracket("[");
    assert_eq!(result, Ok(("", Token::OpenBracket)));
}

#[test]
fn test_parse_close_bracket() {
    let result = parse_close_bracket("]");
    assert_eq!(result, Ok(("", Token::CloseBracket)));
}

#[test]
fn test_parse_dot() {
    let result = parse_dot(".");
    assert_eq!(result, Ok(("", Token::Dot)));
}

#[test]
fn test_parse_arrow() {
    let result = parse_arrow("->");
    assert_eq!(result, Ok(("", Token::Arrow)));
}

#[test]
fn test_parse_ellipsis() {
    let result = parse_ellipsis("...");
    assert_eq!(result, Ok(("", Token::Ellipsis)));
}

#[test]
fn test_parse_identifier() {
    let result = parse_identifier("myVariable123");
    assert_eq!(
        result,
        Ok(("", Token::Identifier("myVariable123".to_string())))
    );
}

#[test]
fn test_parse_identifier_with_underscore() {
    let result = parse_identifier("_myVariable_123");
    assert_eq!(
        result,
        Ok(("", Token::Identifier("_myVariable_123".to_string())))
    );
}

#[test]
fn test_parse_identifier_all_underscore() {
    let result = parse_identifier("___");
    assert_eq!(result, Ok(("", Token::Identifier("___".to_string()))));
}

#[test]
fn test_parse_identifier_starts_with_underscore() {
    let result = parse_identifier("_var");
    assert_eq!(result, Ok(("", Token::Identifier("_var".to_string()))));
}

#[test]
fn test_parse_identifier_starts_with_letter() {
    let result = parse_identifier("var123");
    assert_eq!(result, Ok(("", Token::Identifier("var123".to_string()))));
}

#[test]
fn test_parse_comment_line() {
    let input = "// This is a single line comment\nNext line";
    let result = parse_comment_line(input);
    assert_eq!(
        result,
        Ok((
            "Next line",
            Token::CommentLine("// This is a single line comment".to_string())
        ))
    );
}

#[test]
fn test_parse_comment_block() {
    let input = "/* This is a\nmulti-line comment */Next part";
    let result = parse_comment_block(input);
    assert_eq!(
        result,
        Ok((
            "Next part",
            Token::CommentBlock(" This is a\nmulti-line comment ".to_string())
        ))
    );
}

// syntax
#[test]
fn test_parse_type_specifier() {
    let result = parse_type_specifier("int").unwrap().1;
    assert_eq!(result, TypeSpecifier::Int);

    let result = parse_type_specifier("float").unwrap().1;
    assert_eq!(result, TypeSpecifier::Float);
}

#[test]
fn test_parse_variable_declaration() {
    let input = "int x;";
    let expected = VariableDeclaration {
        type_specifier: TypeSpecifier::Int,
        identifier: "x".to_string(),
    };

    let (_, result) = parse_variable_declaration(input).unwrap();
    assert_eq!(result, expected);
}
