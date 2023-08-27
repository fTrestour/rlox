use crate::{
    environment::Environment,
    error::LoxRuntimeError,
    grammar::{Declaration, Expression},
    value::Value,
};
pub fn interpret(
    declaration: Declaration,
    environment: &mut Environment,
) -> Result<(), LoxRuntimeError> {
    match declaration {
        Declaration::Print(expression) => {
            let value = evaluate(expression, environment);
            value.map(|value| println!("{}", value))?;
            Ok(())
        }
        Declaration::Expression(expression) => {
            evaluate(expression, environment)?;
            Ok(())
        }
        Declaration::Var(name, value) => {
            let value = value
                .map(|value| evaluate(value, environment))
                .transpose()?;
            environment.define(name, value);
            Ok(())
        }
        Declaration::Block(declarations) => {
            let mut local_environment = environment.new_local();
            for declaration in declarations {
                interpret(declaration, &mut local_environment)?;
            }

            Ok(())
        }
        Declaration::If(condition, if_statement, else_statement) => {
            let condition = evaluate(condition, environment)?;

            if condition.is_truthy() {
                interpret(*if_statement, environment)?;
            } else if let Some(else_statement) = else_statement {
                interpret(*else_statement, environment)?;
            }

            Ok(())
        }
        Declaration::While(condition, while_statement) => {
            while evaluate(condition.clone(), environment)?.is_truthy() {
                interpret(*while_statement.clone(), environment)?;
            }

            Ok(())
        }
    }
}

pub fn evaluate(
    expression: Expression,
    environment: &mut Environment,
) -> Result<Value, LoxRuntimeError> {
    match expression {
        Expression::Number(n) => Ok(Value::Number(n)),
        Expression::String(s) => Ok(Value::String(s)),
        Expression::True => Ok(Value::Boolean(true)),
        Expression::False => Ok(Value::Boolean(false)),
        Expression::Nil => Ok(Value::Nil),
        Expression::Paren(expression) => evaluate(*expression, environment),
        Expression::Not(expression) => {
            let value = evaluate(*expression, environment)?;
            Ok(Value::Boolean(!value.is_truthy()))
        }
        Expression::Minus(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Number(left - right))
        }
        Expression::Multiply(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Number(left * right))
        }
        Expression::Divide(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Number(left / right))
        }
        Expression::Plus(left, right) => {
            let left = evaluate(*left, environment)?;
            let right = evaluate(*right, environment)?;
            match (left, right) {
                (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
                _ => Err(LoxRuntimeError {
                    message: "Operands must be two numbers or two strings.".to_owned(),
                }),
            }
        }
        Expression::Greater(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Boolean(left > right))
        }
        Expression::GreaterEqual(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Boolean(left >= right))
        }
        Expression::Less(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Boolean(left < right))
        }
        Expression::LessEqual(left, right) => {
            let left = evaluate(*left, environment)?;
            let left = left.as_number()?;
            let right = evaluate(*right, environment)?;
            let right = right.as_number()?;
            Ok(Value::Boolean(left <= right))
        }
        Expression::Equal(left, right) => {
            let left = evaluate(*left, environment)?;
            let right = evaluate(*right, environment)?;
            Ok(Value::Boolean(left.is_equal(&right)))
        }
        Expression::NotEqual(left, right) => {
            let left = evaluate(*left, environment)?;
            let right = evaluate(*right, environment)?;
            Ok(Value::Boolean(!left.is_equal(&right)))
        }
        Expression::Variable(name) => environment.get(&name),
        Expression::Assignment(name, value) => {
            let value = evaluate(*value, environment)?;
            environment.assign(name, value.clone())?;

            Ok(value)
        }
        Expression::And(left, right) => {
            let left = evaluate(*left, environment)?;
            if left.is_truthy() {
                let right = evaluate(*right, environment)?;

                Ok(right)
            } else {
                Ok(left)
            }
        }
        Expression::Or(left, right) => {
            let left = evaluate(*left, environment)?;
            if !left.is_truthy() {
                let right = evaluate(*right, environment)?;

                Ok(right)
            } else {
                Ok(left)
            }
        }
    }
}
