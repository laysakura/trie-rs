pub mod trie;
pub mod trie_builder;

pub struct Trie<T> {
    container: Vec<T>, // TODO
}

pub struct TrieBuilder<T> {
    words: Vec<T>, // TODO should be tree
}
