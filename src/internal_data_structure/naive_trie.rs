use std::collections::VecDeque;

/// Naive trie with ordered Elm sequence in edges.
///
/// The following naive trie contains these words.
/// - a
/// - app
/// - apple
/// - applicatio
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
enum NaiveTrie<Elm> {
    Root(Node<Elm>),
    NoneRoot(Elm, Node<Elm>),
}

struct Node<Elm> {
    children: Vec<NaiveTrie<Elm>>,
    is_stop: bool,
}

impl<Elm> NaiveTrie<Elm> {
    pub fn new() -> Self {
        let root = Node {
            children: vec![],
            is_stop: false,
        };
        NaiveTrie::Root(root)
    }

    pub fn push<Arr: AsRef<[Elm]>>(&mut self, word: Arr) {}

    pub fn bf_iter(&self) -> NaiveTrieBFIter<Elm> {
        let mut iter = NaiveTrieBFIter::new(self);
        iter
    }
}

/// Iterates over NaiveTrie in Breadth-First manner.
struct NaiveTrieBFIter<'trie, Elm> {
    unvisited: VecDeque<&'trie NaiveTrie<Elm>>,
}

impl<'trie, Elm> NaiveTrieBFIter<'trie, Elm> {
    pub fn new(iter_start: &'trie NaiveTrie<Elm>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Elm> Iterator for NaiveTrieBFIter<'trie, Elm> {
    type Item = &'trie Elm;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::NaiveTrie;

    // TODO parameterized tests
    #[test]
    fn todo() {
        let mut trie = NaiveTrie::new();

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
