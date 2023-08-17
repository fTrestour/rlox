use crate::{error::LoxRuntimeError, grammar::Expression, value::LoxValue};

pub fn interpret(expression: Expression) -> Result<LoxValue, LoxRuntimeError> {
    match expression {
        Expression::Number(n) => Ok(LoxValue::LoxNumber(n)),
        Expression::String(s) => Ok(LoxValue::LoxString(s)),
        Expression::True => Ok(LoxValue::LoxBoolean(true)),
        Expression::False => Ok(LoxValue::LoxBoolean(false)),
        Expression::Nil => Ok(LoxValue::LoxNil),
        Expression::Paren(expression) => interpret(*expression),
        Expression::Not(expression) => {
            let value = interpret(*expression)?;
            Ok(LoxValue::LoxBoolean(!value.is_truthy()))
        }
        Expression::Minus(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxNumber(left - right))
        }
        Expression::Multiply(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxNumber(left * right))
        }
        Expression::Divide(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxNumber(left / right))
        }
        Expression::Plus(left, right) => {
            let left = interpret(*left)?;
            let right = interpret(*right)?;
            match (left, right) {
                (LoxValue::LoxNumber(n1), LoxValue::LoxNumber(n2)) => {
                    Ok(LoxValue::LoxNumber(n1 + n2))
                }
                (LoxValue::LoxString(s1), LoxValue::LoxString(s2)) => {
                    Ok(LoxValue::LoxString(s1 + &s2))
                }
                _ => Err(LoxRuntimeError {
                    message: "Operands must be two numbers or two strings.".to_owned(),
                }),
            }
        }
        Expression::Greater(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left > right))
        }
        Expression::GreaterEqual(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left >= right))
        }
        Expression::Less(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left < right))
        }
        Expression::LessEqual(left, right) => {
            let left = interpret(*left)?;
            let left = left.as_number()?;
            let right = interpret(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left <= right))
        }
        Expression::Equal(left, right) => {
            let left = interpret(*left)?;
            let right = interpret(*right)?;
            Ok(LoxValue::LoxBoolean(left.is_equal(&right)))
        }
        Expression::NotEqual(left, right) => {
            let left = interpret(*left)?;
            let right = interpret(*right)?;
            Ok(LoxValue::LoxBoolean(!left.is_equal(&right)))
        }
    }
}
