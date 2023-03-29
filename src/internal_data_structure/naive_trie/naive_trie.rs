use super::naive_trie_b_f_iter::NaiveTrieBFIter;
use super::{NaiveTrie, NaiveTrieIntermOrLeaf, NaiveTrieRoot};

impl<'trie, K: Ord + Clone, V: Clone> NaiveTrie<K, V> {
    pub fn make_root() -> Self {
        NaiveTrie::Root(NaiveTrieRoot { children: vec![] })
    }

    pub fn make_interm_or_leaf(key: &K, value: V, is_terminal: bool) -> Self {
        let value = if is_terminal { Some(value) } else { None };
        NaiveTrie::IntermOrLeaf(Box::new(NaiveTrieIntermOrLeaf {
            children: vec![],
            key: key.clone(),
            value,
            is_terminal,
        }))
    }

    pub fn push<Key: AsRef<[K]>>(&'trie mut self, key: Key, value: V) {
        let mut trie = self;
        for (i, chr) in key.as_ref().iter().enumerate() {
            let res = {
                trie.children()
                    .binary_search_by_key(chr, |child| child.label())
            };

            let is_terminal = i == key.as_ref().len() - 1;
            match res {
                Ok(j) => {
                    let new_trie = match trie {
                        NaiveTrie::Root(node) => &mut node.children[j],
                        NaiveTrie::IntermOrLeaf(node) => &mut node.children[j],
                        _ => panic!("Unexpected type"),
                    };
                    if is_terminal {
                        match new_trie {
                            NaiveTrie::IntermOrLeaf(node) => {
                                node.value = Some(value.clone());
                                node.is_terminal = true;
                            }
                            _ => panic!("Unexpected type"),
                        }
                    }
                    trie = new_trie;
                }
                Err(j) => {
                    let child_trie = Self::make_interm_or_leaf(chr, value.clone(), is_terminal);
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

    pub fn bf_iter(&'trie self) -> NaiveTrieBFIter<K, V> {
        NaiveTrieBFIter::new(self)
    }

    pub fn children(&self) -> &[Self] {
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
    pub fn label(&self) -> K {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.key.clone(),
            _ => panic!("Unexpected type"),
        }
    }

    pub fn value(&self) -> Option<V> {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.value.clone(),
            _ => panic!("Unexpected type"),
        }
    }
}
