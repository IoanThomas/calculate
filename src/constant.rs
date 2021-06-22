use bigdecimal::BigDecimal;

use crate::{
    expression_parse_error::ExpressionParseError,
    parsable_expression::{NonConstant, ParsableExpression, RpnItem},
};

#[derive(Debug)]
pub struct Constant {
    pub value: BigDecimal,
}

impl ParsableExpression for Constant {
    fn parse_to_rpn(
        self: Box<Self>,
        rpn_stack: &mut Vec<RpnItem>,
        _non_constant_stack: &mut Vec<NonConstant>,
    ) -> Result<(), ExpressionParseError> {
        rpn_stack.push(RpnItem::Constant(*self));

        Ok(())
    }
}
