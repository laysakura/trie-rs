use trie_rs::{Trie, TrieBuilder};

fn build_trie() -> Trie {
    let mut builder = TrieBuilder::new();
    builder.push("a");
    builder.push("app");
    builder.push("apple");
    builder.push("better");
    builder.push("application");
    builder.build()
}

#[test]
fn exact_match() {
    let trie = build_trie();
    assert_eq!(trie.exact_match("a"), true);
    assert_eq!(trie.exact_match("app"), true);
    assert_eq!(trie.exact_match("apple"), true);
    assert_eq!(trie.exact_match("application"), true);
    assert_eq!(trie.exact_match("better"), true);
    assert_eq!(trie.exact_match("appler"), false);
}

#[test]
fn predictive_search() {
    let empty: Vec<&str> = vec![];
    let trie = build_trie();
    assert_eq!(
        trie.predictive_search("a"),
        vec!["a", "app", "apple", "application"]
    );
    assert_eq!(
        trie.predictive_search("app"),
        vec!["app", "apple", "application"]
    );
    assert_eq!(trie.predictive_search("appl"), vec!["apple", "application"]);
    assert_eq!(trie.predictive_search("apple"), vec!["apple"]);
    assert_eq!(trie.predictive_search("appler"), empty);
    assert_eq!(trie.predictive_search("b"), vec!["better"]);
    assert_eq!(trie.predictive_search("c"), empty);
}

#[test]
fn common_prefix_search() {
    let empty: Vec<&str> = vec![];
    let trie = build_trie();
    assert_eq!(trie.predictive_search("a"), vec!["a"]);
    assert_eq!(trie.predictive_search("ap"), vec!["a"]);
    assert_eq!(trie.predictive_search("appl"), vec!["a", "app"]);
    assert_eq!(trie.predictive_search("appler"), vec!["apple"]);
    assert_eq!(trie.predictive_search("bette"), empty);
    assert_eq!(trie.predictive_search("betterment"), vec!["better"]);
    assert_eq!(trie.predictive_search("c"), empty);
}
