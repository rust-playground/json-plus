use json_plus::diff;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let old = json!({"key":"old value", "arr":[]});
    let new = json!({"key":"new value", "arr":[]});

    let diffed = diff(&old, &new).unwrap();
    println!("{}", diffed.to_string());
    Ok(())
}
