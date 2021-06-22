use std::str::FromStr;

use bigdecimal::BigDecimal;
use regex::Regex;

use crate::{
    constant::Constant,
    expression_parse_error::ExpressionParseError,
    operator::Operator,
    parenthesis::{Parenthesis, ParenthesisVariant},
    parsable_expression::{self, ParsableExpression, RpnItem},
};

pub struct ExpressionParser {
    rpn_stack: Vec<Box<dyn ParsableExpression>>,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {
            rpn_stack: vec![]
        }
    }
    
    pub fn parse_to_rpn(&mut self, expression: &str) -> Result<(), ExpressionParseError> {
        // Split expression
        let regex = match Regex::new(r"([+\-*/\^\(\)]|\d+)") {
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

        // Return RPN

        let mut rpn_stack = vec![];
        let mut non_constant_stack = vec![];
        //self.rpn_stack.iter().for_each(|r: Box<dyn ParsableExpression>| {r.parse_to_rpn(&mut rpn_stack, &mut non_constant_stack);});

        for r in self.rpn_stack.drain(0..) {
            r.parse_to_rpn(&mut rpn_stack, &mut non_constant_stack)?;
        }

        for non in non_constant_stack {
            match non {
                parsable_expression::NonConstant::Operator(operator) => {
                    rpn_stack.push(RpnItem::Operator(operator))
                }
                parsable_expression::NonConstant::Parenthesis(parenthesis) => {
                    rpn_stack.push(RpnItem::Parenthesis(parenthesis))
                }
            }
        }

        for r in rpn_stack {
            println!("{:?}", r);
        }

        Ok(())
    }

    fn parse_token(&mut self, token: &str) -> Result<(), ExpressionParseError> {
        if let Ok(number) = BigDecimal::from_str(token) {
            return self.parse_number(number);
        };

        match token {
            "+" => self.rpn_stack.push(Box::new(Operator {
                symbol: "+",
                precedence: 2,
                is_left_associative: true,
            })),
            "-" => self.rpn_stack.push(Box::new(Operator {
                symbol: "-",
                precedence: 2,
                is_left_associative: true,
            })),
            "*" => self.rpn_stack.push(Box::new(Operator {
                symbol: "*",
                precedence: 3,
                is_left_associative: true,
            })),
            "/" => self.rpn_stack.push(Box::new(Operator {
                symbol: "/",
                precedence: 3,
                is_left_associative: true,
            })),
            "^" => self.rpn_stack.push(Box::new(Operator {
                symbol: "^",
                precedence: 5,
                is_left_associative: true,
            })),
            "(" => self.rpn_stack.push(Box::new(Parenthesis {
                variant: ParenthesisVariant::Left,
            })),
            ")" => self.rpn_stack.push(Box::new(Parenthesis {
                variant: ParenthesisVariant::Right,
            })),
            _ => {}
        }

        Ok(())
    }

    fn parse_number(&mut self, number: BigDecimal) -> Result<(), ExpressionParseError> {
        self.rpn_stack.push(Box::new(Constant { value: number }));

        Ok(())
    }
}
