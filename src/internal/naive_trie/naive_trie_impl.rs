use super::naive_trie_b_f_iter::NaiveTrieBFIter;
use super::{NaiveTrie, NaiveTrieIntermOrLeaf, NaiveTrieRoot};
use std::vec::Drain;

impl<Token: Ord, Value> NaiveTrie<Token, Value> {
    pub fn make_root() -> Self {
        NaiveTrie::Root(NaiveTrieRoot { children: vec![] })
    }

    pub fn make_interm_or_leaf(token: Token, terminal: Option<Value>) -> Self {
        NaiveTrie::IntermOrLeaf(NaiveTrieIntermOrLeaf {
            children: vec![],
            token,
            value: terminal,
        })
    }

    pub fn insert<Arr: Iterator<Item = Token>>(&mut self, word: Arr, value: Value) {
        let mut trie = self;
        for chr in word {
            let res = trie
                .children()
                .binary_search_by(|child| child.token().cmp(&chr));
            match res {
                Ok(j) => {
                    trie = match trie {
                        NaiveTrie::Root(node) => &mut node.children[j],
                        NaiveTrie::IntermOrLeaf(node) => &mut node.children[j],
                        _ => panic!("Unexpected type"),
                    };
                }
                Err(j) => {
                    let child_trie = Self::make_interm_or_leaf(chr, None);
                    trie = match trie {
                        NaiveTrie::Root(node) => {
                            node.children.insert(j, child_trie);
                            &mut node.children[j]
                        }
                        NaiveTrie::IntermOrLeaf(node) => {
                            node.children.insert(j, child_trie);
                            &mut node.children[j]
                        }
                        _ => panic!("Unexpected type"),
                    };
                }
            };
        }
        match trie {
            NaiveTrie::IntermOrLeaf(node) => node.value = Some(value),
            _ => panic!("Unexpected type"),
        }
    }

    pub fn children(&self) -> &[Self] {
        match self {
            NaiveTrie::Root(node) => &node.children,
            NaiveTrie::IntermOrLeaf(node) => &node.children,
            _ => panic!("Unexpected type"),
        }
    }

    pub fn drain_children(&mut self) -> Drain<'_, Self> {
        match self {
            NaiveTrie::Root(node) => node.children.drain(0..),
            NaiveTrie::IntermOrLeaf(node) => node.children.drain(0..),
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    #[allow(dead_code)]
    pub fn value(&self) -> Option<&Value> {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.value.as_ref(),
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn token(&self) -> &Token {
        match self {
            NaiveTrie::IntermOrLeaf(node) => &node.token,
            _ => panic!("Unexpected type"),
        }
    }
}

impl<Token: Ord, Value> IntoIterator for NaiveTrie<Token, Value> {
    type Item = NaiveTrie<Token, Value>;
    type IntoIter = NaiveTrieBFIter<Token, Value>;
    fn into_iter(self) -> NaiveTrieBFIter<Token, Value> {
        NaiveTrieBFIter::new(self)
    }
}
