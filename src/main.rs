use crate::expression_parser::ExpressionParser;

mod constant;
mod expression_parse_error;
mod expression_parser;
mod operator;
mod parenthesis;
mod parsable_expression;

fn main() {
    let mut expression_parser = ExpressionParser::new();
    expression_parser.parse_to_rpn("4*(3+5)/2");
}
