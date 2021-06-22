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
    let mut expression_parser = ExpressionParser::new();

    let mut rpn_stack = match expression_parser.parse_to_rpn("4*(3+5)/2") {
        Ok(rpn_stack) => rpn_stack,
        Err(_) => {
            println!("Error whilst parsing expression");
            return;
        }
    };

    let mut rpn_parser = RpnParser::new();

    let result = match rpn_parser.parse_rpn(&mut rpn_stack) {
        Ok(result) => result,
        Err(_) => {
            println!("Error whilst parsing RPN");
            return;
        }
    };

    println!("{}", result);
}
