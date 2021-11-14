#[macro_use]
extern crate criterion;

use criterion::{Criterion, Throughput};
use json_plus::{diff, merge, strip, Strip};
use serde_json::{json, Value};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("json");

    let old_input = r#"{"M":{"a":1,"b":"foo"},"A":["foo"],"B":true}"#;
    let new_input = r#"{"M":{"a":1,"b":"bar"},"A":["foo","bar"],"B":false}"#;
    let old: Value = serde_json::from_str(old_input).unwrap();
    let new: Value = serde_json::from_str(new_input).unwrap();

    group.throughput(Throughput::Bytes(old_input.len() as u64));
    group.bench_function("diff", |b| {
        b.iter(|| {
            let _res = diff(&old, &new);
        });
    });

    let old: Value = serde_json::from_str(old_input).unwrap();
    let new: Value = serde_json::from_str(new_input).unwrap();

    group.throughput(Throughput::Bytes(old_input.len() as u64));
    group.bench_function("merge", |b| {
        let mut v = old.clone();
        b.iter(|| {
            let _res = merge(&mut v, &new);
        });
    });

    group.throughput(Throughput::Bytes(old_input.len() as u64));
    group.bench_function("strip", |b| {
        b.iter(|| {
            let to_strip: Value = json!({"key": "value", "obj": {"key": null}, "arr":[]});
            let _res = strip(Strip::Nulls | Strip::Empties, to_strip);
        });
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
