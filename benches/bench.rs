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
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}

mod trie {
    use criterion::{BatchSize, Criterion, black_box};
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use trie_rs::{Trie, TrieBuilder};

    lazy_static! {
        // Construct Japanese dictionary using EDICT (http://www.edrdg.org/jmdict/edict.html).
        static ref TRIE_EDICT: Trie<u8> = {
            let mut builder = TrieBuilder::new();

            let repo_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable must be set.");
            let edict2_path = format!("{}/benches/edict.furigana", repo_root);
            println!("Reading dictionary file from: {}", edict2_path);

            let mut n_words = 0;
            for result in BufReader::new(File::open(edict2_path).unwrap()).lines() {
                let l = result.unwrap();
                builder.insert(&*l);
                n_words += 1;
            }
            println!("Read {} words.", n_words);

            builder.build()
            // TODO print memory footprint compared to original `edict.furigana` file
        };
    }

    pub fn build(_: &mut Criterion) {
        let items = 10_000;

        super::c().bench_function(
            &format!("[{}] Trie::build() {} items", super::git_hash(), items),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |_trie| {
                        let mut builder: TrieBuilder<u8> = TrieBuilder::new();

                        let repo_root = env::var("CARGO_MANIFEST_DIR")
                            .expect("CARGO_MANIFEST_DIR environment variable must be set.");
                        let edict2_path = format!("{}/benches/edict.furigana", repo_root);

                        let mut n_words = 0;
                        for result in BufReader::new(File::open(edict2_path).unwrap()).lines() {
                            let l = result.unwrap();
                            builder.insert(&*l);
                            n_words += 1;
                            if n_words >= items {
                                break;
                            }
                        }
                        black_box(builder.build())
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    pub fn exact_match(_: &mut Criterion) {
        let times = 100;

        super::c().bench_function(
            &format!(
                "[{}] Trie::exact_match() {} times",
                super::git_hash(),
                times
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        // iter_batched() does not properly time `routine` time
                        // when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build().
                        // So loop many times.
                        let result = trie.exact_match("すしをにぎる");
                        for _ in 0..(times - 1) {
                            assert!(trie.exact_match("すしをにぎる"));
                        }
                        assert!(result);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    pub fn predictive_search(_: &mut Criterion) {
        let times = 100;

        super::c().bench_function(
            &format!(
                "[{}] Trie::predictive_search() {} times",
                super::git_hash(),
                times
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        // iter_batched() does not properly time `routine` time
                        // when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build().
                        // So loop many times.
                        let results_in_u8s: Vec<Vec<u8>> = trie.predictive_search("すし").collect();
                        for _ in 0..(times - 1) {
                            for entry in trie.predictive_search::<Vec<u8>, _>("すし") {
                                black_box(entry);
                            }
                        }

                        let results_in_str: Vec<String> = results_in_u8s
                            .into_iter()
                            .map(|u8s| String::from_utf8(u8s).unwrap())
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

    pub fn predictive_search_big_output(_: &mut Criterion) {
        super::c().bench_function(
            &format!(
                "[{}] Trie::predictive_search_big_output()",
                super::git_hash(),
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        let results: Vec<Vec<u8>> = trie.predictive_search("す").collect();
                        assert_eq!(results.len(), 4220);
                        let results_in_u8s = results.into_iter().take(100);
                        assert_eq!(results_in_u8s.len(), 100);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    pub fn predictive_search_limited_big_output(_: &mut Criterion) {
        super::c().bench_function(
            &format!(
                "[{}] Trie::predictive_search_limited_big_output()",
                super::git_hash(),
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        let results_in_u8s: Vec<Vec<u8>> =
                            trie.predictive_search("す").take(100).collect();
                        assert_eq!(results_in_u8s.len(), 100);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    pub fn common_prefix_search(_: &mut Criterion) {
        let times = 100;

        super::c().bench_function(
            &format!(
                "[{}] Trie::common_prefix_search() {} times",
                super::git_hash(),
                times
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        // iter_batched() does not properly time `routine` time
                        // when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build().
                        // So loop many times.
                        let results_in_str: Vec<String> =
                            trie.common_prefix_search("すしをにぎる").collect();
                        for _ in 0..(times - 1) {
                            for entry in trie.common_prefix_search("すしをにぎる") {
                                black_box::<Vec<u8>>(entry);
                            }
                        }
                        assert_eq!(results_in_str, vec!["す", "すし", "すしをにぎる"]);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    pub fn common_prefix_match(_: &mut Criterion) {
        let times = 100;

        super::c().bench_function(
            &format!(
                "[{}] Trie::common_prefix_match() {} times",
                super::git_hash(),
                times
            ),
            move |b| {
                b.iter_batched(
                    || &TRIE_EDICT,
                    |trie| {
                        // iter_batched() does not properly time `routine` time when `setup` time is far longer than `routine` time.
                        // Tested function takes too short compared to build(). So loop many times.
                        let result = trie
                            .common_prefix_search::<Vec<u8>, _>("すしをにぎる")
                            .next()
                            .is_some();
                        for _ in 0..(times - 1) {
                            let _ = trie
                                .common_prefix_search::<Vec<u8>, _>("すしをにぎる")
                                .next()
                                .is_some();
                        }

                        assert!(result);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

criterion_group!(
    benches,
    trie::build,
    trie::exact_match,
    trie::predictive_search,
    trie::predictive_search_big_output,
    trie::predictive_search_limited_big_output,
    trie::common_prefix_search,
    trie::common_prefix_match,
);
criterion_main!(benches);
