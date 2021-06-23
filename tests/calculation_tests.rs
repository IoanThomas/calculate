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
fn addition_decimal() {
    calculate_assert_eq("4.4+2.3", "6.7");
    calculate_assert_eq("0.8+0.1", "0.9");
}

#[test]
fn subtraction() {
    calculate_assert_eq("4-3", "1");
    calculate_assert_eq("15-23", "-8");
}

#[test]
fn subtraction_decimal() {
    calculate_assert_eq("4.5-3.3", "1.2");
    calculate_assert_eq("15.5-23.8", "-8.3");
}

#[test]
fn multiplication() {
    calculate_assert_eq("4*3", "12");
    calculate_assert_eq("50*50", "2500");
}

#[test]
fn multiplication_decimal() {
    calculate_assert_eq("4.5*3.2", "14.4");
    calculate_assert_eq("50*50.2", "2510");
}

#[test]
fn division() {
    calculate_assert_eq("26/2", "13");
    calculate_assert_eq("5/2", "2.5");
}

#[test]
fn division_decimal() {
    calculate_assert_eq("26.5/2", "13.25");
    calculate_assert_eq("5/8", "0.625");
}

#[test]
fn combined() {
    calculate_assert_eq("1+1/4", "1.25");
    calculate_assert_eq("4*(3+5)/2", "16");
    calculate_assert_eq("4.6*(1.8+3.2*9)/18", "7.82");
}

#[test]
fn unary_plus() {
    calculate_assert_eq("+5", "5");
    calculate_assert_eq("+-2.2", "2.2");
    calculate_assert_eq("+(5-8)", "3");
}

#[test]
fn unary_minus() {
    calculate_assert_eq("-5", "-5");
    calculate_assert_eq("-+2.2", "-2.2");
    calculate_assert_eq("-(5-8)", "3");
}
