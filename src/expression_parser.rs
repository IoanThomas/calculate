use std::str::FromStr;

use bigdecimal::BigDecimal;
use regex::Regex;

use crate::{
    constant::Constant,
    expression_parse_error::ExpressionParseError,
    operator::Operator,
    parenthesis::{Parenthesis, ParenthesisVariant},
    parsable_expression::{self, NonConstant, ParsableExpression, RpnItem},
};

pub struct ExpressionParser {
    tokens: Vec<Box<dyn ParsableExpression>>,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser { tokens: vec![] }
    }

    pub fn parse_to_rpn(&mut self, expression: &str) -> Result<Vec<RpnItem>, ExpressionParseError> {
        // Split expression
        let regex = match Regex::new(r"([+\-*/\^\(\)]|\d*\.?\d*)") {
            Ok(regex) => regex,
            Err(e) => {
                println!("{:?}", e);
                return Err(ExpressionParseError {});
            }
        };

        // Parse to RPN
        for field in regex.find_iter(expression) {
            self.parse_token(field.as_str())?;
        }

        let mut rpn_stack: Vec<RpnItem> = vec![];
        let mut non_constant_stack: Vec<NonConstant> = vec![];

        for expression in self.tokens.drain(0..) {
            expression.parse_to_rpn(&mut rpn_stack, &mut non_constant_stack)?;
        }

        for non_constant in non_constant_stack.drain(0..).rev() {
            match non_constant {
                parsable_expression::NonConstant::Operator(operator) => {
                    rpn_stack.push(RpnItem::Operator(operator))
                }
                parsable_expression::NonConstant::Parenthesis(parenthesis) => {
                    rpn_stack.push(RpnItem::Parenthesis(parenthesis))
                }
            }
        }

        // TODO: Remove
        //for r in rpn_stack.iter() {
        //    println!("{:?}", r);
        //}

        Ok(rpn_stack)
    }

    fn parse_token(&mut self, token: &str) -> Result<(), ExpressionParseError> {
        if let Ok(number) = BigDecimal::from_str(token) {
            return self.parse_number(number);
        };

        match token {
            "+" => self.tokens.push(Box::new(Operator {
                symbol: "+",
                precedence: 2,
                is_left_associative: true,
            })),
            "-" => self.tokens.push(Box::new(Operator {
                symbol: "-",
                precedence: 2,
                is_left_associative: true,
            })),
            "*" => self.tokens.push(Box::new(Operator {
                symbol: "*",
                precedence: 3,
                is_left_associative: true,
            })),
            "/" => self.tokens.push(Box::new(Operator {
                symbol: "/",
                precedence: 3,
                is_left_associative: true,
            })),
            "^" => self.tokens.push(Box::new(Operator {
                symbol: "^",
                precedence: 5,
                is_left_associative: true,
            })),
            "(" => self.tokens.push(Box::new(Parenthesis {
                variant: ParenthesisVariant::Left,
            })),
            ")" => self.tokens.push(Box::new(Parenthesis {
                variant: ParenthesisVariant::Right,
            })),
            _ => {}
        }

        Ok(())
    }

    fn parse_number(&mut self, number: BigDecimal) -> Result<(), ExpressionParseError> {
        self.tokens.push(Box::new(Constant { value: number }));

        Ok(())
    }
}
