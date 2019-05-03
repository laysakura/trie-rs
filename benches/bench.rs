#[macro_use]
extern crate criterion;

#[macro_use]
extern crate lazy_static;

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
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str;
    use trie_rs::{Trie, TrieBuilder};

    lazy_static! {
        // Construct Japanese dictionary using EDICT (http://www.edrdg.org/jmdict/edict.html).
        static ref TRIE_EDICT: Trie<u8> = {
            let mut builder = TrieBuilder::new();

            let repo_root = env::var("REPO_ROOT").expect("REPO_ROOT environment variable must be set.");
            let edict2_path = format!("{}/benches/edict.furigana", repo_root);
            println!("Reading dictionary file from: {}", edict2_path);
            for result in BufReader::new(File::open(edict2_path).unwrap()).lines() {
                let l = result.unwrap();
                builder.push(l);
            }
            builder.build()
            // TODO print heap usage
        };
    }

    pub fn predictive_search(_: &mut Criterion) {
        let times = 100;

        super::c().bench_function(
            &format!(
                "[{}] Trie::predictive_match() {} times",
                super::git_hash(),
                times
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        let results_in_u8s = trie.predictive_search("すし");
                        for _ in 0..(times - 1) {
                            trie.predictive_search("すし");
                        }

                        let results_in_str: Vec<&str> = results_in_u8s
                            .iter()
                            .map(|u8s| str::from_utf8(u8s).unwrap())
                            .collect();
                        assert_eq!(
                            results_in_str,
                            vec![
                                "すし",
                                "すしだね",
                                "すしづめ",
                                "すしのぐ",
                                "すしめし",
                                "すしや",
                                "すしをにぎる"
                            ]
                        );
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(benches, trie::predictive_search);
criterion_main!(benches);
