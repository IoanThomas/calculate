use crate::expression_parser::ExpressionParser;

mod constant;
mod expression_parse_error;
mod expression_parser;
mod operator;
mod parenthesis;
mod parsable_expression;

fn main() {
    let mut expression_parser = ExpressionParser::new();
    if let Ok(rpn_stack) = expression_parser.parse_to_rpn("4*(3+5)/2") {
        todo!();
    } else {
        println!("Error whilst parsing expression");
    }
}
