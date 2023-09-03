use std::time::{SystemTime, UNIX_EPOCH};

use crate::value::Value;

pub fn clock(_: Vec<Value>) -> Value {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    Value::Number(since_the_epoch as f64 / 1000.)
}
