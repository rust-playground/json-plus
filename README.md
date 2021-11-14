# json_plus &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/json_plus.svg
[crates.io]: https://crates.io/crates/json_plus

This crate provides JSON helper functions beyond Serialization & Deserialization such as `diff`, `merge`, `...`

---

```toml
[dependencies]
json_plus = "0.1"
```

## Example
```rust
use json_plus::diff;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let old = json!({"key":"old value", "arr":[]});
    let new = json!({"key":"new value", "arr":[]});

    let diffed = diff(&old, &new).unwrap();
    println!("{}", diffed.to_string());
    Ok(())
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Proteus by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>