#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Duration;

fn c() -> Criterion {
    Criterion::default()
        .sample_size(10) // must be >= 10 for Criterion v0.3
        .warm_up_time(Duration::from_secs(1))
        .with_plots()
}

fn git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

mod trie {
    use criterion::{BatchSize, Criterion};

    const NS: [u64; 5] = [1 << 11, 1 << 12, 1 << 13, 1 << 14, 1 << 15];

    pub fn to_be_removed_benchmark(_: &mut Criterion) {
        let times = 10_000;

        super::c().bench_function_over_inputs(
            &format!("[{}] 1+1 {} times", super::git_hash(), times,),
            move |b, &&_n| b.iter_batched(|| (), |_| 1 + 1, BatchSize::SmallInput),
            &NS,
        );
    }
}

criterion_group!(benches, trie::to_be_removed_benchmark);
criterion_main!(benches);
