use std::str::FromStr;

use bigdecimal::BigDecimal;
use regex::Regex;

use crate::{
    constant::Constant,
    expression_parse_error::ExpressionParseError,
    operator::Operator,
    parenthesis::{Parenthesis, ParenthesisVariant},
    parsable_expression::{InfixItem, NonConstant, ParsableExpression, RpnItem},
};

pub struct ExpressionParser {
    tokens: Vec<InfixItem>,
    parsable_expressions: Vec<Box<dyn ParsableExpression>>,
}

impl ExpressionParser {
    pub fn new() -> ExpressionParser {
        ExpressionParser {
            tokens: vec![],
            parsable_expressions: vec![],
        }
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

        // Parse to list of expressions
        for field in regex.find_iter(expression) {
            self.parse_token(field.as_str())?;
        }

        let mut rpn_stack: Vec<RpnItem> = vec![];
        let mut non_constant_stack: Vec<NonConstant> = vec![];

        // Parse to RPN
        for (i, expression) in self.parsable_expressions.drain(0..).enumerate() {
            expression.parse_to_rpn(&self.tokens, i, &mut rpn_stack, &mut non_constant_stack)?;
        }

        for non_constant in non_constant_stack.drain(0..).rev() {
            match non_constant {
                NonConstant::Operator(operator) => rpn_stack.push(RpnItem::Operator(operator)),
                _ => {}
            }
        }

        Ok(rpn_stack)
    }

    fn parse_token(&mut self, token: &str) -> Result<(), ExpressionParseError> {
        if let Ok(number) = BigDecimal::from_str(token) {
            return self.parse_number(number);
        };

        match token {
            "+" => {
                let operator = Operator {
                    symbol: "+",
                    precedence: 2,
                    is_left_associative: true,
                };
                self.parsable_expressions.push(Box::new(operator.clone()));
                self.tokens.push(InfixItem::Operator(operator));
            }
            "-" => {
                let operator = Operator {
                    symbol: "-",
                    precedence: 2,
                    is_left_associative: true,
                };
                self.parsable_expressions.push(Box::new(operator.clone()));
                self.tokens.push(InfixItem::Operator(operator));
            }
            "*" => {
                let operator = Operator {
                    symbol: "*",
                    precedence: 3,
                    is_left_associative: true,
                };
                self.parsable_expressions.push(Box::new(operator.clone()));
                self.tokens.push(InfixItem::Operator(operator));
            }
            "/" => {
                let operator = Operator {
                    symbol: "/",
                    precedence: 3,
                    is_left_associative: true,
                };
                self.parsable_expressions.push(Box::new(operator.clone()));
                self.tokens.push(InfixItem::Operator(operator));
            }
            "^" => {
                let operator = Operator {
                    symbol: "^",
                    precedence: 5,
                    is_left_associative: false,
                };
                self.parsable_expressions.push(Box::new(operator.clone()));
                self.tokens.push(InfixItem::Operator(operator));
            }
            "(" => {
                let parenthesis = Parenthesis {
                    variant: ParenthesisVariant::Left,
                };
                self.parsable_expressions
                    .push(Box::new(parenthesis.clone()));
                self.tokens.push(InfixItem::Parenthesis(parenthesis));
            }
            ")" => {
                let parenthesis = Parenthesis {
                    variant: ParenthesisVariant::Right,
                };
                self.parsable_expressions
                    .push(Box::new(parenthesis.clone()));
                self.tokens.push(InfixItem::Parenthesis(parenthesis));
            }
            _ => {}
        }

        Ok(())
    }

    fn parse_number(&mut self, number: BigDecimal) -> Result<(), ExpressionParseError> {
        let constant = Constant { value: number };

        self.parsable_expressions.push(Box::new(constant.clone()));
        self.tokens.push(InfixItem::Constant(constant));

        Ok(())
    }
}
