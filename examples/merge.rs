use json_plus::merge;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut base = json!({"key":"old value", "arr":[]});
    let patch = json!({"key2":"value 2"});

    merge(&mut base, &patch);
    println!("{}", base);
    Ok(())
}
