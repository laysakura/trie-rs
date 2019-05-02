use super::NaiveTrie;
use std::collections::VecDeque;

/// Iterates over NaiveTrie in Breadth-First manner.
pub struct NaiveTrieBFIter<'trie, Label> {
    unvisited: VecDeque<&'trie NaiveTrie<Label>>,
}

impl<'trie, Label> NaiveTrieBFIter<'trie, Label> {
    pub fn new(iter_start: &'trie NaiveTrie<Label>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Label: Ord + Clone> Iterator for NaiveTrieBFIter<'trie, Label> {
    type Item = &'trie NaiveTrie<Label>;

    /// Returns:
    ///
    /// - None: All nodes are visited.
    /// - Some(NaiveTrie::Root): Root node.
    /// - Some(NaiveTrie::IntermOrLeaf): Intermediate or leaf node.
    /// - Some(NaiveTrie::PhantomSibling): Marker to represent "all siblings are iterated".
    fn next(&mut self) -> Option<Self::Item> {
        self.unvisited.pop_front().map(|trie| {
            match trie {
                NaiveTrie::Root(_) | NaiveTrie::IntermOrLeaf(_) => {
                    for child in trie.children() {
                        self.unvisited.push_back(child);
                    }
                    self.unvisited.push_back(&NaiveTrie::PhantomSibling);
                }
                NaiveTrie::PhantomSibling => {}
            };
            trie
        })
    }
}

#[cfg(test)]
mod bf_iter_tests {
    use super::NaiveTrie;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (words, expected_nodes) = $value;
                let mut trie = NaiveTrie::make_root();
                for word in words {
                    trie.push(word);
                }
                let nodes: Vec<&NaiveTrie<u8>> = trie.bf_iter().collect();
                assert_eq!(nodes.len(), expected_nodes.len());
                for i in 0..nodes.len() {
                    let node = nodes[i];
                    let expected_node = &expected_nodes[i];

                    assert!(std::mem::discriminant(node) == std::mem::discriminant(expected_node));

                    if let NaiveTrie::IntermOrLeaf(n) = node {
                        assert_eq!(n.label, expected_node.label());
                        assert_eq!(n.is_terminal, expected_node.is_terminal());
                    }
                }
            }
        )*
        }
    }

    parameterized_tests! {
        t1: (
            Vec::<&str>::new(),
            vec![
                NaiveTrie::make_root(),
                // parent = root
                NaiveTrie::PhantomSibling,
            ]
        ),
        t2: (
            vec!["a"],
            vec![
                NaiveTrie::make_root(),
                // parent = root
                NaiveTrie::make_interm_or_leaf(&('a' as u8), true),
                NaiveTrie::PhantomSibling,
                // parent = a
                NaiveTrie::PhantomSibling,
            ]
        ),
        t3: (
            vec!["a", "a"],
            vec![
                NaiveTrie::make_root(),
                // parent = root
                NaiveTrie::make_interm_or_leaf(&('a' as u8), true),
                NaiveTrie::PhantomSibling,
                // parent = a
                NaiveTrie::PhantomSibling,
            ]
        ),
        t4: (
            // root
            //  |-----------------------+-----------------------+
            //  |                       |                       |
            //  a (term)                b                       Ph
            //  |---------+             |-----------------+
            //  |         |             |                 |
            //  n (term)  Ph            a                 Ph
            //  |                       |--------+
            //  |                       |        |
            //  Ph                      d (term) Ph
            //                          |
            //                          |
            //                          Ph
            vec!["a", "bad", "an"],
            vec![
                NaiveTrie::make_root(),
                // parent = root
                NaiveTrie::make_interm_or_leaf(&('a' as u8), true),
                NaiveTrie::make_interm_or_leaf(&('b' as u8), false),
                NaiveTrie::PhantomSibling,
                // parent = [a]
                NaiveTrie::make_interm_or_leaf(&('n' as u8), true),
                NaiveTrie::PhantomSibling,
                // parent = b
                NaiveTrie::make_interm_or_leaf(&('a' as u8), false),
                NaiveTrie::PhantomSibling,
                // parent = n
                NaiveTrie::PhantomSibling,
                // parent = b[a]d
                NaiveTrie::make_interm_or_leaf(&('d' as u8), true),
                NaiveTrie::PhantomSibling,
                // parent = d
                NaiveTrie::PhantomSibling,
            ]
        ),
        t5: (
            // 'り' => 227, 130, 138
            // 'ん' => 227, 130, 147
            // 'ご' => 227, 129, 148
            vec!["a", "an", "りんご", "りんりん"],
            vec![
                NaiveTrie::make_root(),
                // parent = root
                NaiveTrie::make_interm_or_leaf(&('a' as u8), true),
                NaiveTrie::make_interm_or_leaf(&227, false),
                NaiveTrie::PhantomSibling,
                // parent = a
                NaiveTrie::make_interm_or_leaf(&('n' as u8), true),
                NaiveTrie::PhantomSibling,
                // parent = [227] 130 138 (り)
                NaiveTrie::make_interm_or_leaf(&130, false),
                NaiveTrie::PhantomSibling,
                // parent = n
                NaiveTrie::PhantomSibling,
                // parent = 227 [130] 138 (り)
                NaiveTrie::make_interm_or_leaf(&138, false),
                NaiveTrie::PhantomSibling,
                // parent = 227 130 [138] (り)
                NaiveTrie::make_interm_or_leaf(&227, false),
                NaiveTrie::PhantomSibling,
                // parent = [227] 130 147 (ん)
                NaiveTrie::make_interm_or_leaf(&130, false),
                NaiveTrie::PhantomSibling,
                // parent = 227 [130] 147 (ん)
                NaiveTrie::make_interm_or_leaf(&147, false),
                NaiveTrie::PhantomSibling,
                // parent = 227 130 [147] (ん)
                NaiveTrie::make_interm_or_leaf(&227, false),
                NaiveTrie::PhantomSibling,
                // parent = [227] _ _ (ご or り)
                NaiveTrie::make_interm_or_leaf(&129, false),
                NaiveTrie::make_interm_or_leaf(&130, false),
                NaiveTrie::PhantomSibling,
                // parent = 227 [129] 148 (ご)
                NaiveTrie::make_interm_or_leaf(&148, true),
                NaiveTrie::PhantomSibling,
                // parent = 227 [130] 138 (り)
                NaiveTrie::make_interm_or_leaf(&138, false),
                NaiveTrie::PhantomSibling,
                // parent = 227 129 [148] (ご)
                NaiveTrie::PhantomSibling,
                // parent = 227 130 [138] (り)
                NaiveTrie::make_interm_or_leaf(&227, false),
                NaiveTrie::PhantomSibling,
                // parent = [227] 130 147 (ん)
                NaiveTrie::make_interm_or_leaf(&130, false),
                NaiveTrie::PhantomSibling,
                // parent = 227 [130] 147 (ん)
                NaiveTrie::make_interm_or_leaf(&147, true),
                NaiveTrie::PhantomSibling,
                // parent = 227 130 [147] (ん)
                NaiveTrie::PhantomSibling,
            ]
        ),
    }
}
