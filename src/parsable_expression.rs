use crate::{
    constant::Constant, expression_parse_error::ExpressionParseError, operator::Operator,
    parenthesis::Parenthesis,
};

#[derive(Debug)]
pub enum InfixItem {
    Constant(Constant),
    Operator(Operator),
    Parenthesis(Parenthesis),
}

#[derive(Debug)]
pub enum NonConstant {
    Operator(Operator),
    Parenthesis(Parenthesis),
}

#[derive(Debug)]
pub enum RpnItem {
    Constant(Constant),
    Operator(Operator),
}

pub trait ParsableExpression {
    fn parse_to_rpn(
        self: Box<Self>,
        tokens: &Vec<InfixItem>,
        index: usize,
        rpn_stack: &mut Vec<RpnItem>,
        non_constant_stack: &mut Vec<NonConstant>,
    ) -> Result<(), ExpressionParseError>;
}
