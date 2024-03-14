use super::naive_trie_b_f_iter::NaiveTrieBFIter;
use super::naive_trie_b_f_into_iter::NaiveTrieBFIntoIter;
use super::{NaiveTrie, NaiveTrieIntermOrLeaf, NaiveTrieRoot};
use std::vec::Drain;

impl<'trie, Label: Ord, Value> NaiveTrie<Label, Value> {
    pub fn make_root() -> Self {
        NaiveTrie::Root(Box::new(NaiveTrieRoot { children: vec![] }))
    }

    pub fn make_interm_or_leaf(label: Label, terminal: Option<Value>) -> Self {
        NaiveTrie::IntermOrLeaf(Box::new(NaiveTrieIntermOrLeaf {
            children: vec![],
            label,
            value: terminal,
        }))
    }

    pub fn push<Arr: AsRef<[Label]>>(&'trie mut self, word: Arr, value: Value) where Label: Clone {
        let mut trie = self;
        let mut value = Some(value);
        for (i, chr) in word.as_ref().iter().enumerate() {
            let res = {
                trie.children()
                    .binary_search_by_key(chr, |child| child.label().clone())
            };
            match res {
                Ok(j) => {
                    trie = match trie {
                        NaiveTrie::Root(node) => &mut node.children[j],
                        NaiveTrie::IntermOrLeaf(node) => &mut node.children[j],
                        _ => panic!("Unexpected type"),
                    };
                }
                Err(j) => {
                    let is_terminal = i == word.as_ref().len() - 1;
                    let child_trie = Box::new(Self::make_interm_or_leaf(
                        chr.clone(),
                        is_terminal.then(|| value.take().unwrap()),
                    ));
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
    }

    pub fn bf_iter(&'trie self) -> NaiveTrieBFIter<Label, Value> {
        NaiveTrieBFIter::new(self)
    }

    pub fn into_iter(self) -> NaiveTrieBFIntoIter<Label, Value> {
        NaiveTrieBFIntoIter::new(self)
    }

    pub fn children(&self) -> &[Box<Self>] {
        match self {
            NaiveTrie::Root(node) => &node.children,
            NaiveTrie::IntermOrLeaf(node) => &node.children,
            _ => panic!("Unexpected type"),
        }
    }

    pub fn drain_children(&mut self) ->  Drain<'_, Box<Self>> {
        match self {
            NaiveTrie::Root(node) => node.children.drain(0..),
            NaiveTrie::IntermOrLeaf(node) => node.children.drain(0..),
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn value(&self) -> Option<&Value> {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.value.as_ref(),
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn label(&self) -> &Label {
        match self {
            NaiveTrie::IntermOrLeaf(node) => &node.label,
            _ => panic!("Unexpected type"),
        }
    }
}
