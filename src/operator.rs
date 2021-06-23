use std::borrow::Borrow;

use crate::{
    expression_parse_error::ExpressionParseError,
    parenthesis::ParenthesisVariant,
    parsable_expression::{InfixItem, NonConstant, ParsableExpression, RpnItem},
};

#[derive(Debug, Clone)]
pub struct Operator {
    pub symbol: &'static str,
    pub precedence: u8,
    pub is_left_associative: bool,
}

impl Operator {
    fn make_unary(&mut self) {
        match self.symbol {
            "+" => {
                self.symbol = "p";
                self.precedence = 5;
                self.is_left_associative = false;
            }
            "-" => {
                self.symbol = "n";
                self.precedence = 5;
                self.is_left_associative = false;
            }
            _ => {}
        }
    }

    fn get_top_stack_precedence(non_constant_stack: &mut Vec<NonConstant>) -> u8 {
        match non_constant_stack.last() {
            Some(top_stack) => match top_stack {
                NonConstant::Operator(operator) => operator.precedence,
                NonConstant::Parenthesis(_) => 0,
            },
            None => 0,
        }
    }
}

impl ParsableExpression for Operator {
    fn parse_to_rpn(
        mut self: Box<Self>,
        tokens: &Vec<InfixItem>,
        index: usize,
        rpn_stack: &mut Vec<RpnItem>,
        non_constant_stack: &mut Vec<NonConstant>,
    ) -> Result<(), ExpressionParseError> {
        if index > 0 {
            match tokens[index - 1].borrow() {
                InfixItem::Operator(_) => self.make_unary(),
                InfixItem::Parenthesis(parenthesis) => {
                    if let ParenthesisVariant::Left = parenthesis.variant {
                        self.make_unary();
                    }
                }
                _ => {}
            }
        } else {
            self.make_unary();
        }

        if !non_constant_stack.is_empty() {
            let mut top_stack_precedence = Operator::get_top_stack_precedence(non_constant_stack);

            while (self.is_left_associative && self.precedence <= top_stack_precedence)
                || (self.precedence < top_stack_precedence)
            {
                if let Some(top_stack) = non_constant_stack.pop() {
                    if let NonConstant::Operator(operator) = top_stack {
                        rpn_stack.push(RpnItem::Operator(operator))
                    }
                }

                if non_constant_stack.is_empty() {
                    break;
                }

                top_stack_precedence = Operator::get_top_stack_precedence(non_constant_stack);
            }
        }

        non_constant_stack.push(NonConstant::Operator(*self));

        println!();

        Ok(())
    }
}
