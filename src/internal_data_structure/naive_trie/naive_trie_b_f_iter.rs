use super::{NaiveTrie, NaiveTrieBFIter};
use std::collections::VecDeque;

impl<'trie, Elm> NaiveTrieBFIter<'trie, Elm> {
    pub fn new(iter_start: &'trie NaiveTrie<Elm>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Elm: Eq + Ord + Clone> Iterator for NaiveTrieBFIter<'trie, Elm> {
    type Item = Elm;
    fn next(&mut self) -> Option<Self::Item> {
        // -> None: All nodes are visited.
        // -> Some(None): Root node.
        // -> Some(Some(Elm)): Intermediate or leaf node.
        let mut next1 = || {
            self.unvisited.pop_front().map(|trie| {
                for child in &trie.children {
                    self.unvisited.push_back(child);
                }
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
