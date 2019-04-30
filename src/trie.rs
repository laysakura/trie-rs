pub mod trie;
pub mod trie_builder;

pub struct Trie<Label> {
    container: Vec<Label>, // TODO
}

pub struct TrieBuilder<Label> {
    words: Vec<Label>, // TODO should be tree
}
