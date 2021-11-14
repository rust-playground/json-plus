use json_plus::{strip, Strip};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = json!({"key":"old value", "null":null, "empty":[]});

    let stripped = strip(Strip::Nulls | Strip::Empties, base).unwrap();
    println!("{}", stripped.to_string());
    Ok(())
}
