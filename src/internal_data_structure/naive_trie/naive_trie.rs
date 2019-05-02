use super::naive_trie_b_f_iter::NaiveTrieBFIter;
use super::{NaiveTrie, NaiveTrieIntermOrLeaf, NaiveTrieRoot};

impl<'trie, Label: Ord + Clone> NaiveTrie<Label> {
    pub fn make_root() -> Self {
        NaiveTrie::Root(Box::new(NaiveTrieRoot { children: vec![] }))
    }

    pub fn make_interm_or_leaf(label: &Label, is_terminal: bool) -> Self {
        NaiveTrie::IntermOrLeaf(Box::new(NaiveTrieIntermOrLeaf {
            children: vec![],
            label: label.clone(),
            is_terminal,
        }))
    }

    pub fn push<Arr: AsRef<[Label]>>(&'trie mut self, word: Arr) {
        let mut trie = self;
        for (i, chr) in word.as_ref().iter().enumerate() {
            let res = {
                trie.children()
                    .binary_search_by_key(chr, |child| child.label())
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
                    let child_trie = Box::new(Self::make_interm_or_leaf(chr, is_terminal));
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

    pub fn bf_iter(&'trie self) -> NaiveTrieBFIter<Label> {
        NaiveTrieBFIter::new(self)
    }

    pub fn children(&self) -> &[Box<Self>] {
        match self {
            NaiveTrie::Root(node) => &node.children,
            NaiveTrie::IntermOrLeaf(node) => &node.children,
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn is_terminal(&self) -> bool {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.is_terminal,
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn label(&self) -> Label {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.label.clone(),
            _ => panic!("Unexpected type"),
        }
    }
}
