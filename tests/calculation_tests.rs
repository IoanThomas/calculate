use std::str::FromStr;

use bigdecimal::BigDecimal;
use calculate::{expression_parser::ExpressionParser, rpn_parser::RpnParser};

fn calculate(expression: &str) -> Result<BigDecimal, ()> {
    let mut expression_parser = ExpressionParser::new();

    let mut rpn_stack = match expression_parser.parse_to_rpn(expression) {
        Ok(rpn_stack) => rpn_stack,
        Err(_) => return Err(()),
    };

    let mut rpn_parser = RpnParser::new();

    match rpn_parser.parse_rpn(&mut rpn_stack) {
        Ok(result) => Ok(result),
        Err(_) => Err(()),
    }
}

fn make_bigdecimal(number: &str) -> Result<BigDecimal, ()> {
    match BigDecimal::from_str(number) {
        Ok(number) => Ok(number),
        Err(_) => Err(()),
    }
}

fn calculate_assert_eq(expression: &str, expected: &str) {
    let result = match calculate(expression) {
        Ok(result) => result,
        Err(_) => return,
    };

    let expected = match make_bigdecimal(expected) {
        Ok(expected) => expected,
        Err(_) => return,
    };

    assert_eq!(result, expected);
}

#[test]
fn addition() {
    calculate_assert_eq("4+2", "6");
    calculate_assert_eq("0+0", "0");
}

#[test]
fn subtraction() {
    calculate_assert_eq("4-3", "1");
    calculate_assert_eq("15-23", "-8");
}

#[test]
fn multiplication() {
    calculate_assert_eq("4*3", "12");
    calculate_assert_eq("50*50", "2500");
}

#[test]
fn division() {
    calculate_assert_eq("26/2", "13");
    calculate_assert_eq("5/2", "2.5");
}
