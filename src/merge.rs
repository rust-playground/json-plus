use serde_json::map::Entry;
use serde_json::{Map, Value};

/// merges the `patch` Value on top of the `base`.
/// ``` rust
///use json_plus::merge;
///use serde_json::json;
///
///fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let mut base = json!({"key":"old value", "arr":[]});
///    let patch = json!({"key2":"value 2"});
///
///    merge(&mut base, &patch);
///    println!("{}", base);
///    Ok(())
///}
/// ```
#[inline]
pub fn merge(base: &mut Value, patch: &Value) {
    match patch {
        Value::Object(p) => {
            match base {
                Value::Object(b) => {
                    merge_object(b, p);
                }
                _ => {
                    *base = patch.clone();
                }
            };
        }
        _ => {
            *base = patch.clone();
        }
    }
}

fn merge_object(base: &mut Map<String, Value>, patch: &Map<String, Value>) {
    for (k, v) in patch {
        match base.entry(k) {
            Entry::Vacant(e) => {
                e.insert(v.clone());
            }
            Entry::Occupied(mut e) => {
                merge(e.get_mut(), v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn different_values() {
        assert_eq!(merge_helper(Value::Bool(true), Value::Null), Value::Null);
        assert_eq!(
            merge_helper(Value::Bool(true), Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(
            merge_helper(Value::String("blah".into()), Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(
            merge_helper(json!({"a":"a"}), Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(
            merge_helper(Value::Bool(false), json!({"a":"a"})),
            json!({"a":"a"})
        );
        assert_eq!(
            merge_helper(Value::Null, json!({"a":"a", "b": null})),
            json!({"a":"a", "b": null})
        );
    }

    #[test]
    fn objects() {
        assert_eq!(
            merge_helper(json!({"a":"a"}), json!({"b":"b","c":"c"})),
            json!({"a":"a","b":"b","c":"c"})
        );
        assert_eq!(
            merge_helper(json!({"a":"a"}), json!({"b":"b","a":"c"})),
            json!({"a":"c","b":"b"})
        );
        assert_eq!(
            merge_helper(json!({"a":"a","arr":[1]}), json!({"b":"b","arr":[1,2,3]})),
            json!({"a":"a","b":"b","arr":[1,2,3]})
        );
        assert_eq!(
            merge_helper(
                json!({"a":{"b":{"c":"c"}}}),
                json!({"a":{"b":{"c":"q"},"d":1}})
            ),
            json!({"a":{"b":{"c":"q"},"d":1}})
        );
        assert_eq!(
            merge_helper(
                json!({"M":{"a":1,"b":"foo"},"A":["foo"],"B":true}),
                json!({"M":{"a":1,"b":"bar"},"A":["foo","bar"],"B":false})
            ),
            json!({"M":{"a":1,"b":"bar"},"A":["foo","bar"],"B":false})
        );
    }

    fn merge_helper(mut base: Value, patch: Value) -> Value {
        merge(&mut base, &patch);
        base
    }
}
