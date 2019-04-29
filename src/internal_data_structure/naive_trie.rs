use std::collections::VecDeque;

/// Naive trie with ordered Elm sequence in edges.
///
/// The following naive trie contains these words.
/// - a
/// - app
/// - apple
/// - application
///
/// ```text
/// <Node>
///   |
///   | a: Elm
/// <Node (Stop)>
///   |
///   | p
/// <Node>
///   |
///   | p
/// <Node (Stop)>
///   |
///   | l
/// <Node>
///   |------------------+
///   | e                | i
/// <Node (Stop)>     <Node>
///                      |
///                      | c
///                   <Node>
///                      |
///                      | a
///                    <Node>
///                      |
///                      | t
///                    <Node>
///                      |
///                      | i
///                    <Node>
///                      |
///                      | o
///                    <Node>
///                      |
///                      | n
///                    <Node (Stop)>
/// ```
pub struct NaiveTrie<Elm> {
    /// Sorted by Elm's order.
    children: Vec<Box<NaiveTrie<Elm>>>,

    /// Only root node is None.
    label: Option<Elm>,

    is_terminal: bool,
}

impl<Elm: Eq + Ord + Clone> NaiveTrie<Elm> {
    pub fn make_root() -> Self {
        Self {
            children: vec![],
            label: None,
            is_terminal: false,
        }
    }

    pub fn push<Arr: AsRef<[Elm]>>(&mut self, word: Arr) {
        let mut trie = self;
        for chr in word.as_ref() {
            let children = &mut trie.children;
            let res = children.binary_search_by_key(&Some(chr), |child| child.label.as_ref());
            match res {
                Ok(i) => {
                    trie = &mut children[i];
                }
                Err(i) => {
                    let is_terminal = false; // TODO
                    let child_trie = Box::new(Self::make_non_root(chr, is_terminal));
                    children.insert(i, child_trie);
                    trie = &mut children[i];
                }
            }
        }
    }

    pub fn bf_iter(&self) -> NaiveTrieBFIter<Elm> {
        NaiveTrieBFIter::new(self)
    }

    fn make_non_root(label: &Elm, is_terminal: bool) -> Self {
        Self {
            children: vec![],
            label: Some(label.clone()),
            is_terminal,
        }
    }
}

/// Iterates over NaiveTrie in Breadth-First manner.
pub struct NaiveTrieBFIter<'trie, Elm> {
    unvisited: VecDeque<&'trie NaiveTrie<Elm>>,
}

impl<'trie, Elm> NaiveTrieBFIter<'trie, Elm> {
    pub fn new(iter_start: &'trie NaiveTrie<Elm>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Elm: Eq + Ord + Clone> Iterator for NaiveTrieBFIter<'trie, Elm> {
    type Item = &'trie Elm;
    fn next(&mut self) -> Option<Self::Item> {
        // -> None: All nodes are visited.
        // -> Some(None): Root node.
        // -> Some(Some(Elm)): Intermediate or leaf node.
        let mut next1 = || {
            self.unvisited.pop_front().map(|trie| {
                for child in &trie.children {
                    self.unvisited.push_back(child);
                }
                trie.label.as_ref()
            })
        };

        next1().map(
            // skip root node since it does not have label
            |opt_elm| opt_elm.or_else(|| next1()?),
        )?
    }
}

#[cfg(test)]
mod tests {
    use super::NaiveTrie;

    // TODO parameterized tests
    #[test]
    fn todo() {
        let mut trie = NaiveTrie::make_root();

        trie.push("a");
        trie.push("an");
        trie.push("bad");

        let mut iter = trie.bf_iter();
        assert_eq!(iter.next(), Some(&('a' as u8)));
        assert_eq!(iter.next(), Some(&('b' as u8)));
        assert_eq!(iter.next(), Some(&('n' as u8)));
        assert_eq!(iter.next(), Some(&('a' as u8)));
        assert_eq!(iter.next(), Some(&('d' as u8)));
        assert_eq!(iter.next(), None);
    }
}
