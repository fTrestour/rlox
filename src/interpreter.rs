use crate::{
    environment::Environment,
    error::LoxRuntimeError,
    grammar::{Declaration, Expression},
    value::LoxValue,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, declaration: Declaration) -> Result<(), LoxRuntimeError> {
        match declaration {
            Declaration::Print(expression) => {
                let value = self.evaluate(expression);
                value.map(|value| println!("{}", value))?;
                Ok(())
            }
            Declaration::Expression(expression) => {
                self.evaluate(expression)?;
                Ok(())
            }
            Declaration::Var(name, value) => {
                let value = value.map(|value| self.evaluate(value)).transpose()?;
                self.environment.define(name, value);
                Ok(())
            }
        }
    }

    pub fn evaluate(&self, expression: Expression) -> Result<LoxValue, LoxRuntimeError> {
        match expression {
            Expression::Number(n) => Ok(LoxValue::LoxNumber(n)),
            Expression::String(s) => Ok(LoxValue::LoxString(s)),
            Expression::True => Ok(LoxValue::LoxBoolean(true)),
            Expression::False => Ok(LoxValue::LoxBoolean(false)),
            Expression::Nil => Ok(LoxValue::LoxNil),
            Expression::Paren(expression) => self.evaluate(*expression),
            Expression::Not(expression) => {
                let value = self.evaluate(*expression)?;
                Ok(LoxValue::LoxBoolean(!value.is_truthy()))
            }
            Expression::Minus(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxNumber(left - right))
            }
            Expression::Multiply(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxNumber(left * right))
            }
            Expression::Divide(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxNumber(left / right))
            }
            Expression::Plus(left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
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
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxBoolean(left > right))
            }
            Expression::GreaterEqual(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxBoolean(left >= right))
            }
            Expression::Less(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxBoolean(left < right))
            }
            Expression::LessEqual(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(LoxValue::LoxBoolean(left <= right))
            }
            Expression::Equal(left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                Ok(LoxValue::LoxBoolean(left.is_equal(&right)))
            }
            Expression::NotEqual(left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                Ok(LoxValue::LoxBoolean(!left.is_equal(&right)))
            }
            Expression::Variable(name) => self.environment.get(&name),
        }
    }
}
