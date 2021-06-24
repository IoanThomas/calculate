use std::env;

use expression_parser::ExpressionParser;
use rpn_parser::RpnParser;

mod constant;
mod expression_parse_error;
mod expression_parser;
mod operator;
mod parenthesis;
mod parsable_expression;
mod rpn_parse_error;
mod rpn_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let expression = args.join("");

    let mut expression_parser = ExpressionParser::new();

    let rpn_stack = match expression_parser.parse_to_rpn(&expression) {
        Ok(rpn_stack) => rpn_stack,
        Err(_) => {
            println!("Error whilst parsing expression");
            return;
        }
    };

    let mut rpn_parser = RpnParser::new();

    let result = match rpn_parser.parse_rpn(&rpn_stack) {
        Ok(result) => result,
        Err(error) => {
            println!("Error whilst parsing RPN: {}", error.message);
            return;
        }
    };

    println!("{}", result);
}
