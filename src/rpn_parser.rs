use bigdecimal::BigDecimal;

use crate::operator::Operator;
use crate::{parsable_expression::RpnItem, rpn_parse_error::RpnParseError};

pub struct RpnParser {
    value_stack: Vec<BigDecimal>,
}

impl RpnParser {
    pub fn new() -> RpnParser {
        RpnParser {
            value_stack: vec![],
        }
    }

    pub fn parse_rpn(&mut self, rpn_stack: &mut Vec<RpnItem>) -> Result<BigDecimal, RpnParseError> {
        for rpn_item in rpn_stack.drain(0..) {
            match rpn_item {
                RpnItem::Constant(constant) => self.value_stack.push(constant.value),
                RpnItem::Operator(operator) => self.parse_operator(operator)?,
                RpnItem::Parenthesis(_) => return Err(RpnParseError {}),
            }
        }

        match self.value_stack.len() {
            0 => Err(RpnParseError {}),
            _ => Ok(self.value_stack.remove(0)),
        }
    }

    fn parse_operator(&mut self, operator: Operator) -> Result<(), RpnParseError> {
        match operator.symbol {
            "+" => {
                let top_value = self.pop_top_value()?;
                *self.get_top_value()? += top_value;
            }
            "-" => {
                let top_value = self.pop_top_value()?;
                *self.get_top_value()? -= top_value;
            }
            "*" => {
                let top_value = self.pop_top_value()?;
                *self.get_top_value()? *= top_value;
            }
            "/" => {
                let top_value = self.pop_top_value()?;
                let itr = self.pop_top_value()?;

                // /= Doesn't seem to work with BigDecimal
                self.value_stack.push(itr / top_value);
            }
            "^" => {
                let _top_value = self.pop_top_value()?;
                // TODO: BigDecimal doesn't have power operations
                todo!();
            }
            _ => {}
        }

        Ok(())
    }

    fn pop_top_value(&mut self) -> Result<BigDecimal, RpnParseError> {
        match self.value_stack.pop() {
            Some(top_value) => Ok(top_value),
            None => Err(RpnParseError {}),
        }
    }

    fn get_top_value(&mut self) -> Result<&mut BigDecimal, RpnParseError> {
        match self.value_stack.last_mut() {
            Some(top_value) => Ok(top_value),
            None => Err(RpnParseError {}),
        }
    }
}
