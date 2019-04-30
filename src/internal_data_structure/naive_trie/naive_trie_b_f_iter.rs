use super::NaiveTrie;
use std::collections::VecDeque;

/// Iterates over NaiveTrie in Breadth-First manner.
struct NaiveTrieBFIter<'trie, Label> {
    unvisited: VecDeque<&'trie NodeType<Label>>,
}

/// Used for Breadth-First iteration.
///
/// ```text
/// <Root>
///   |
///   |------------------+- - - - - - - - +
///   |                  |                |
/// <IntermOrLeaf>     <IntermOrLeaf>   <PhantomSibling>
///   |                  |
///   .                  +- - - - - - - - +
///   |                  |                |
/// <PhantomSibling>   <IntermOrLeaf>   <PhantomSibling>
/// ```
enum NodeType<Label> {
    Root(NaiveTrie<Label>),
    IntermOrLeaf(NaiveTrie<Label>),
    PhantomSibling,
}

impl<'trie, Label> NaiveTrieBFIter<'trie, Label> {
    pub fn new(iter_start: &'trie NodeType<Label>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Label: Ord + Clone> Iterator for NaiveTrieBFIter<'trie, Label> {
    type Item = NodeType<Label>;

    fn next(&mut self) -> Option<Self::Item> {
        // -> None: All nodes are visited.
        // -> Some(NodeType::Root): Root node.
        // -> Some(NodeType::IntermOrLeaf): Intermediate or leaf node.
        // -> Some(NodeType::Null): Marker to represent "all siblings are iterated".
        let mut next1 = || {
            self.unvisited.pop_front().map(|trie| {
                for child in &trie.children {
                    self.unvisited.push_back(child);
                }
                self.unvisited.push_back(Some(Some(LabelOrNull::Null)));
                trie.label.clone()
            })
        };

        next1().map(
            // skip root node since it does not have label
            |opt_elm| opt_elm.or_else(|| next1()?),
        )?
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
                let (words, expected_chars) = $value;
                let mut trie = NaiveTrie::make_root();
                for word in words {
                    trie.push(word);
                }
                let chars: Vec<u8> = trie.bf_iter().collect();
                assert_eq!(chars, expected_chars);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: (Vec::<&str>::new(), "".as_bytes()),
        t2: (vec!["a"], "a".as_bytes()),
        t3: (vec!["a", "a"], "a".as_bytes()),
        t4: (vec!["a", "an", "bad"], "abnad".as_bytes()),
        t5: (vec!["a", "bad", "an"], "abnad".as_bytes()),
        t6: (
            // 'り' => 227, 130, 138
            // 'ん' => 227, 130, 147
            // 'ご' => 227, 129, 148
            vec!["a", "an", "りんご", "りんりん"],
            vec!['a' as u8, 227, 'n' as u8, 130, 138, 227, 130, 147, 227, 129, 130, 148, 138, 227, 130, 147],
        ),
        t7: (
            // '🍎' => 240, 159, 141, 142
            vec!["🍎", "りんご"],
            vec![227, 240, 130, 159, 138, 141, 227, 142, 130, 147, 227, 129, 148],
        ),
    }
}
