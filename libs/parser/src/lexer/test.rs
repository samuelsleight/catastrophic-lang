#![cfg(test)]

use super::*;

use catastrophic_core::span::Location;

fn span<D>(data: D, from_line: usize, from_col: usize, to_line: usize, to_col: usize) -> Span<D> {
    Span::new(Location::new(from_line, from_col), Location::new(to_line, to_col), data)
}

fn lexer_test(input: &str, expected: &[Span<Token>]) {
    let lexer = Lexer::from_str(input);
    let result = lexer.collect().unwrap();

    assert_eq!(result, expected);
}

fn combined_test(input1: &str, expected1: &[Span<Token>], input2: &str, expected2: &[Span<Token>]) {
    if input1.is_empty() {
        return lexer_test(input2, expected2);
    } else if input2.is_empty() {
        return lexer_test(input1, expected1);
    }

    let input = format!("{input1} {input2}");

    let mut expected = expected1.to_owned();
    let new_line = input1.ends_with(|c| c == '\n' || c == '\r');

    let (line, col) = if let Some(last) = expected.last() {
        if new_line {
            (last.end.line + 1, 1)
        } else {
            (last.end.line, last.end.col + 1)
        }
    } else if new_line {
        (1, 1)
    } else {
        (0, 1)
    };

    for item in expected2 {
        if item.start.line == 0 {
            expected.push(span(
                item.data.clone(),
                item.start.line + line,
                item.start.col + col,
                item.end.line + line,
                item.end.col + col,
            ));
        } else {
            expected.push(span(
                item.data.clone(),
                item.start.line + line,
                item.start.col,
                item.end.line + line,
                item.end.col,
            ));
        }
    }

    lexer_test(&input, &expected);
}

macro_rules! test_cases {
        ($($name:ident($input:expr, $expected:expr))+) => {
            test_cases!(expand_single $(($name, $input, $expected))+);
            test_cases!(expand_multi $(($name, $input, $expected))+);
        };

        (expand_single $(($name:ident, $input:expr, $expected:expr))+) => {
            $(
                paste::paste! {
                    #[test]
                    fn [<lex_ $name>]() {
                        lexer_test($input, $expected);
                    }
                }
            )+
        };

        (expand_multi ($name1:ident, $input1:expr, $expected1:expr)($name2:ident, $input2:expr, $expected2:expr)) => {
            test_cases!(expand_multi_cases ($name1, $input1, $expected1)($name2, $input2, $expected2));
        };

        (expand_multi ($name1:ident, $input1:expr, $expected1:expr)($name2:ident, $input2:expr, $expected2:expr)$(($names:ident, $inputs:expr, $expecteds:expr))+) => {
            test_cases!(expand_multi_cases ($name1, $input1, $expected1)($name2, $input2, $expected2)$(($names, $inputs, $expecteds))+);
            test_cases!(expand_multi ($name2, $input2, $expected2)$(($names, $inputs, $expecteds))+);
        };

        (expand_multi_cases ($name1:ident, $input1:expr, $expected1:expr)($name2:ident, $input2:expr, $expected2:expr)) => {
            test_cases!(expand_multi_case ($name1, $input1, $expected1)($name2, $input2, $expected2));
        };

        (expand_multi_cases ($name1:ident, $input1:expr, $expected1:expr)($name2:ident, $input2:expr, $expected2:expr)$(($names:ident, $inputs:expr, $expecteds:expr))+) => {
            test_cases!(expand_multi_case ($name1, $input1, $expected1)($name2, $input2, $expected2));
            test_cases!(expand_multi_cases ($name1, $input1, $expected1)$(($names, $inputs, $expecteds))+);
        };

        (expand_multi_case ($name1:ident, $input1:expr, $expected1:expr)($name2:ident, $input2:expr, $expected2:expr)) => {
            paste::paste! {
                #[test]
                fn [<lex_ $name1 _then_ $name2>]() {
                    combined_test($input1, $expected1, $input2, $expected2)
                }

                #[test]
                fn [<lex_ $name2 _then_ $name1>]() {
                    combined_test($input2, $expected2, $input1, $expected1)
                }
            }
        };
    }

test_cases! {
    empty_input("", &[])
    empty_string("\"\"", &[span(Token::String("".to_owned()), 0, 0, 0, 2)])
    simple_ident("hello", &[span(Token::Ident("hello".to_owned()), 0, 0, 0, 5)])
    simple_string("\"hello\"", &[span(Token::String("hello".to_owned()), 0, 0, 0, 7)])
    simple_integer("10293", &[span(Token::Integer(10293), 0, 0, 0, 5)])
    numeric_ident("a1b2c3", &[span(Token::Ident("a1b2c3".to_owned()), 0, 0, 0, 6)])
    emoji_ident("🐉", &[span(Token::Ident("🐉".to_owned()), 0, 0, 0, 1)])
    arrow("->", &[span(Token::Arrow, 0, 0, 0, 2)])
    parens("()", &[span(Token::Parens, 0, 0, 0, 2)])
    plus("+", &[span(Token::Plus, 0, 0, 0, 1)])
    minus("-", &[span(Token::Minus, 0, 0, 0, 1)])
    equals("=", &[span(Token::Equals, 0, 0, 0, 1)])
    less_than("<", &[span(Token::LessThan, 0, 0, 0, 1)])
    greater_than(">", &[span(Token::GreaterThan, 0, 0, 0, 1)])
    period(".", &[span(Token::Period, 0, 0, 0, 1)])
    comma(",", &[span(Token::Comma, 0, 0, 0, 1)])
    ampersand("&", &[span(Token::Ampersand, 0, 0, 0, 1)])
    tilde("~", &[span(Token::Tilde, 0, 0, 0, 1)])
    colon(":", &[span(Token::Colon, 0, 0, 0, 1)])
    question("?", &[span(Token::Question, 0, 0, 0, 1)])
    l_curly("{", &[span(Token::LCurly, 0, 0, 0, 1)])
    r_curly("}", &[span(Token::RCurly, 0, 0, 0, 1)])
    l_paren("(", &[span(Token::Unexpected('('), 0, 0, 0, 1)])
    r_paren(")", &[span(Token::Unexpected(')'), 0, 0, 0, 1)])
    comment("# comment\n", &[])
}
