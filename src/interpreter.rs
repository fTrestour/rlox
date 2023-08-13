use crate::{grammar::Expression, value::LoxValue};

pub fn interpret(expression: Expression) -> LoxValue {
    match expression {
        Expression::Number(n) => LoxValue::LoxNumber(n),
        Expression::String(s) => LoxValue::LoxString(s),
        Expression::True => LoxValue::LoxBoolean(true),
        Expression::False => LoxValue::LoxBoolean(false),
        Expression::Nil => LoxValue::LoxNil,
        Expression::Paren(expression) => interpret(*expression),
        Expression::Not(expression) => {
            let value = interpret(*expression);
            LoxValue::LoxBoolean(!value.is_truthy())
        }
        Expression::Minus(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxNumber(left.as_number() - right.as_number())
        }
        Expression::Multiply(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxNumber(left.as_number() * right.as_number())
        }
        Expression::Divide(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxNumber(left.as_number() / right.as_number())
        }
        Expression::Plus(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            match (left, right) {
                (LoxValue::LoxNumber(n1), LoxValue::LoxNumber(n2)) => LoxValue::LoxNumber(n1 + n2),
                (LoxValue::LoxString(s1), LoxValue::LoxString(s2)) => LoxValue::LoxString(s1 + &s2),
                _ => todo!(),
            }
        }
        Expression::Greater(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxBoolean(left.as_number() > right.as_number())
        }
        Expression::GreaterEqual(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxBoolean(left.as_number() >= right.as_number())
        }
        Expression::Less(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxBoolean(left.as_number() < right.as_number())
        }
        Expression::LessEqual(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxBoolean(left.as_number() <= right.as_number())
        }
        Expression::Equal(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxBoolean(left.is_equal(&right))
        }
        Expression::NotEqual(left, right) => {
            let left = interpret(*left);
            let right = interpret(*right);
            LoxValue::LoxBoolean(!left.is_equal(&right))
        }
    }
}
