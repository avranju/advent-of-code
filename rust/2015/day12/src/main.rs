use std::fs;
use std::io::Result;

use serde_json::Value;

fn main() -> Result<()> {
    let json = fs::read_to_string("inp.json")?;
    let json: Value = serde_json::from_str(&json)?;
    let mut acc = 0;

    sum(&json, &mut acc);
    println!("acc = {}", acc);

    acc = 0;
    sumnr(&json, &mut acc);
    println!("acc sans red = {}", acc);

    Ok(())
}

fn sumnr(val: &Value, acc: &mut i64) {
    match val {
        Value::Null => {}
        Value::Bool(_) => {}
        Value::Number(n) => *acc += n.as_i64().unwrap(),
        Value::String(_) => {}
        Value::Array(vals) => {
            for v in vals {
                sumnr(v, acc);
            }
        }
        Value::Object(o) => {
            if !o
                .values()
                .any(|v| v.is_string() && v.as_str().unwrap() == "red")
            {
                for v in o.values() {
                    sumnr(v, acc);
                }
            }
        }
    }
}

fn sum(val: &Value, acc: &mut i64) {
    match val {
        Value::Null => {}
        Value::Bool(_) => {}
        Value::Number(n) => *acc += n.as_i64().unwrap(),
        Value::String(_) => {}
        Value::Array(vals) => {
            for v in vals {
                sum(v, acc);
            }
        }
        Value::Object(o) => {
            for v in o.values() {
                sum(v, acc);
            }
        }
    }
}
