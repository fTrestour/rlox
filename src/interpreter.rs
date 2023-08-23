use crate::{
    environment::Environment,
    error::LoxRuntimeError,
    grammar::{Declaration, Expression},
    value::Value,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new_global(),
        }
    }

    pub fn with(environment: Environment) -> Interpreter {
        Interpreter {
            environment: environment,
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
            Declaration::Block(declarations) => {
                let mut interpreter = Interpreter::with(self.environment.new_local());
                for declaration in declarations {
                    interpreter.interpret(declaration)?;
                }

                Ok(())
            }
            Declaration::If(condition, if_statement, else_statement) => {
                let condition = self.evaluate(condition)?;

                if condition.is_truthy() {
                    self.interpret(*if_statement)?;
                } else if let Some(else_statement) = else_statement {
                    self.interpret(*else_statement)?;
                }

                Ok(())
            }
        }
    }

    pub fn evaluate(&mut self, expression: Expression) -> Result<Value, LoxRuntimeError> {
        match expression {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::String(s) => Ok(Value::String(s)),
            Expression::True => Ok(Value::Boolean(true)),
            Expression::False => Ok(Value::Boolean(false)),
            Expression::Nil => Ok(Value::Nil),
            Expression::Paren(expression) => self.evaluate(*expression),
            Expression::Not(expression) => {
                let value = self.evaluate(*expression)?;
                Ok(Value::Boolean(!value.is_truthy()))
            }
            Expression::Minus(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(Value::Number(left - right))
            }
            Expression::Multiply(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(Value::Number(left * right))
            }
            Expression::Divide(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(Value::Number(left / right))
            }
            Expression::Plus(left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                match (left, right) {
                    (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                    (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
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
                Ok(Value::Boolean(left > right))
            }
            Expression::GreaterEqual(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(Value::Boolean(left >= right))
            }
            Expression::Less(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(Value::Boolean(left < right))
            }
            Expression::LessEqual(left, right) => {
                let left = self.evaluate(*left)?;
                let left = left.as_number()?;
                let right = self.evaluate(*right)?;
                let right = right.as_number()?;
                Ok(Value::Boolean(left <= right))
            }
            Expression::Equal(left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                Ok(Value::Boolean(left.is_equal(&right)))
            }
            Expression::NotEqual(left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                Ok(Value::Boolean(!left.is_equal(&right)))
            }
            Expression::Variable(name) => self.environment.get(&name),
            Expression::Assignment(name, value) => {
                let value = self.evaluate(*value)?;
                self.environment.assign(name, value.clone())?;

                Ok(value)
            }
        }
    }
}
