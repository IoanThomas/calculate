use crate::{
    expression_parse_error::ExpressionParseError,
    parsable_expression::{InfixItem, NonConstant, ParsableExpression, RpnItem},
};

#[derive(Debug, Clone)]
pub enum ParenthesisVariant {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Parenthesis {
    pub variant: ParenthesisVariant,
}

impl Parenthesis {
    fn parse_right_parenthesis(
        self: Box<Self>,
        rpn_stack: &mut Vec<RpnItem>,
        non_constant_stack: &mut Vec<NonConstant>,
    ) -> Result<(), ExpressionParseError> {
        while let Some(non_constant) = non_constant_stack.last() {
            if Self::is_left_parenthesis(non_constant) {
                break;
            }

            match non_constant_stack.pop() {
                Some(non_constant) => match non_constant {
                    NonConstant::Operator(operator) => rpn_stack.push(RpnItem::Operator(operator)),
                    NonConstant::Parenthesis(_) => todo!(),
                },
                None => {}
            }
        }

        non_constant_stack.pop();

        Ok(())
    }

    fn is_left_parenthesis(non_constant: &NonConstant) -> bool {
        match non_constant {
            NonConstant::Operator(_) => false,
            NonConstant::Parenthesis(parenthesis) => match parenthesis.variant {
                ParenthesisVariant::Right => false,
                ParenthesisVariant::Left => true,
            },
        }
    }
}

impl ParsableExpression for Parenthesis {
    fn parse_to_rpn(
        self: Box<Self>,
        _tokens: &Vec<InfixItem>,
        _index: usize,
        rpn_stack: &mut Vec<RpnItem>,
        non_constant_stack: &mut Vec<NonConstant>,
    ) -> Result<(), crate::expression_parse_error::ExpressionParseError> {
        match self.variant {
            ParenthesisVariant::Left => non_constant_stack.push(NonConstant::Parenthesis(*self)),
            ParenthesisVariant::Right => {
                self.parse_right_parenthesis(rpn_stack, non_constant_stack)?
            }
        }

        Ok(())
    }
}
