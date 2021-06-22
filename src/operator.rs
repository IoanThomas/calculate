use crate::{
    expression_parse_error::ExpressionParseError,
    parsable_expression::{NonConstant, ParsableExpression, RpnItem},
};

#[derive(Debug)]
pub struct Operator {
    pub symbol: &'static str,
    pub precedence: u8,
    pub is_left_associative: bool,
}

impl ParsableExpression for Operator {
    fn parse_to_rpn(
        self: Box<Self>,
        rpn_stack: &mut Vec<RpnItem>,
        non_constant_stack: &mut Vec<NonConstant>,
    ) -> Result<(), ExpressionParseError> {
        if !non_constant_stack.is_empty() {
            let top_stack_precedence = match non_constant_stack.last() {
                Some(top_stack) => match top_stack {
                    NonConstant::Operator(operator) => operator.precedence,
                    NonConstant::Parenthesis(_) => 0,
                },
                None => 0,
            };

            if (self.is_left_associative && self.precedence <= top_stack_precedence)
                || (self.precedence < top_stack_precedence)
            {
                match non_constant_stack.pop() {
                    Some(top_stack) => match top_stack {
                        NonConstant::Operator(operator) => {
                            rpn_stack.push(RpnItem::Operator(operator))
                        }
                        NonConstant::Parenthesis(_) => todo!(),
                    },
                    None => {}
                }
            }
        }

        non_constant_stack.push(NonConstant::Operator(*self));

        Ok(())
    }
}
