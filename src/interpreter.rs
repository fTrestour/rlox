use crate::{
    error::LoxRuntimeError,
    grammar::{Declaration, Expression},
    value::LoxValue,
};

pub fn interpret(declaration: Declaration) -> Result<(), LoxRuntimeError> {
    match declaration {
        Declaration::Print(expression) => {
            let value = evaluate(expression);
            value.map(|value| println!("{}", value))?;
            Ok(())
        }
        Declaration::Expression(expression) => {
            evaluate(expression)?;
            Ok(())
        }
        Declaration::Var(_, _) => todo!(),
    }
}

pub fn evaluate(expression: Expression) -> Result<LoxValue, LoxRuntimeError> {
    match expression {
        Expression::Number(n) => Ok(LoxValue::LoxNumber(n)),
        Expression::String(s) => Ok(LoxValue::LoxString(s)),
        Expression::True => Ok(LoxValue::LoxBoolean(true)),
        Expression::False => Ok(LoxValue::LoxBoolean(false)),
        Expression::Nil => Ok(LoxValue::LoxNil),
        Expression::Paren(expression) => evaluate(*expression),
        Expression::Not(expression) => {
            let value = evaluate(*expression)?;
            Ok(LoxValue::LoxBoolean(!value.is_truthy()))
        }
        Expression::Minus(left, right) => {
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxNumber(left - right))
        }
        Expression::Multiply(left, right) => {
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxNumber(left * right))
        }
        Expression::Divide(left, right) => {
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxNumber(left / right))
        }
        Expression::Plus(left, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
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
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left > right))
        }
        Expression::GreaterEqual(left, right) => {
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left >= right))
        }
        Expression::Less(left, right) => {
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left < right))
        }
        Expression::LessEqual(left, right) => {
            let left = evaluate(*left)?;
            let left = left.as_number()?;
            let right = evaluate(*right)?;
            let right = right.as_number()?;
            Ok(LoxValue::LoxBoolean(left <= right))
        }
        Expression::Equal(left, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
            Ok(LoxValue::LoxBoolean(left.is_equal(&right)))
        }
        Expression::NotEqual(left, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;
            Ok(LoxValue::LoxBoolean(!left.is_equal(&right)))
        }
        Expression::Variable(_) => todo!(),
    }
}
