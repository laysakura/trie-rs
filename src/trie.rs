pub mod trie;
pub mod trie_builder;

pub struct Trie<Elm> {
    container: Vec<Elm>, // TODO
}

pub struct TrieBuilder<Elm> {
    words: Vec<Elm>, // TODO should be tree
}
