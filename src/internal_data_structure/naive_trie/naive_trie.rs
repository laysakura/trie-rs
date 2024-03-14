use super::naive_trie_b_f_iter::NaiveTrieBFIter;
use super::{NaiveTrie, NaiveTrieIntermOrLeaf, NaiveTrieRoot};
use std::vec::Drain;

impl<'trie, Label: Ord, Value> NaiveTrie<Label, Value> {
    pub fn make_root() -> Self {
        NaiveTrie::Root(NaiveTrieRoot { children: vec![] })
    }

    pub fn make_interm_or_leaf(label: Label, terminal: Option<Value>) -> Self {
        NaiveTrie::IntermOrLeaf(NaiveTrieIntermOrLeaf {
            children: vec![],
            label,
            value: terminal,
        })
    }

    pub fn push<Arr: Iterator<Item = Label>>(&'trie mut self, word: Arr, value: Value) {
        let mut trie = self;
        let mut value = Some(value);
        let mut word = word.peekable();
        while let Some(chr) = word.next() {
            let res = trie
                .children()
                .binary_search_by(|child| child.label().cmp(&chr));
            match res {
                Ok(j) => {
                    trie = match trie {
                        NaiveTrie::Root(node) => &mut node.children[j],
                        NaiveTrie::IntermOrLeaf(node) => &mut node.children[j],
                        _ => panic!("Unexpected type"),
                    };
                }
                Err(j) => {
                    let is_terminal = word.peek().is_none();
                    let child_trie =
                        Self::make_interm_or_leaf(chr, is_terminal.then(|| value.take().unwrap()));
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
    pub fn label(&self) -> &Label {
        match self {
            NaiveTrie::IntermOrLeaf(node) => &node.label,
            _ => panic!("Unexpected type"),
        }
    }
}

impl<Label: Ord, Value> IntoIterator for NaiveTrie<Label, Value> {
    type Item = NaiveTrie<Label, Value>;
    type IntoIter = NaiveTrieBFIter<Label, Value>;
    fn into_iter(self) -> NaiveTrieBFIter<Label, Value> {
        NaiveTrieBFIter::new(self)
    }
}
