use super::trie_search_methods::TrieSearchMethods;
use std::collections::VecDeque;

/// Naive trie with ordered Label sequence in edges.
///
/// The following naive trie contains these words.
/// - a
/// - app
/// - apple
/// - application
///
/// ```text
/// <Root>
///   |
///   | a: Label
/// <IntermOrLeaf (Terminate)>
///   |
///   | p
/// <IntermOrLeaf>
///   |
///   | p
/// <IntermOrLeaf (Terminate)>
///   |
///   | l
/// <IntermOrLeaf>
///   |------------------------------+
///   | e                            | i
/// <IntermOrLeaf (Terminate)>     <IntermOrLeaf>
///                                  |
///                                  | c
///                               <IntermOrLeaf>
///                                  |
///                                  | a
///                                <IntermOrLeaf>
///                                  |
///                                  | t
///                                <IntermOrLeaf>
///                                  |
///                                  | i
///                                <IntermOrLeaf>
///                                  |
///                                  | o
///                                <IntermOrLeaf>
///                                  |
///                                  | n
///                                <IntermOrLeaf (Terminate)>
/// ```
pub enum NaiveTrie<Label> {
    Root(Box<NaiveTrieRoot<Label>>),
    IntermOrLeaf(Box<NaiveTrieIntermOrLeaf<Label>>),

    /// Used for Breadth-First iteration.
    ///
    /// ```text
    /// <Root>
    ///   |
    ///   |------------------+- - - - - - - - +
    ///   | a                | i              |
    /// <IntermOrLeaf>     <IntermOrLeaf>   <PhantomSibling>
    ///   |                  |
    ///   .                  +- - - - - - - - +
    ///   |                  |  n             |
    /// <PhantomSibling>   <IntermOrLeaf>   <PhantomSibling>
    /// ```
    ///
    /// This trie's BFIter emits:
    /// `a i <PhantomSibling> <PhantomSibling> n <PhantomSibling>`
    PhantomSibling,
}

struct NaiveTrieRoot<Label> {
    /// Sorted by Label's order.
    children: Vec<Box<NaiveTrie<Label>>>,
}

struct NaiveTrieIntermOrLeaf<Label> {
    /// Sorted by Label's order.
    children: Vec<Box<NaiveTrie<Label>>>,
    label: Label,
    is_terminal: bool,
}

/// Iterates over NaiveTrieNode in Breadth-First manner.
pub struct NaiveTrieNodeBFIter<'trie, Label> {
    unvisited: VecDeque<&'trie NaiveTrie<Label>>,
}

impl<'trie, Label: Ord + Clone> NaiveTrie<Label> {
    pub fn make_root() -> Self {
        NaiveTrie::Root(Box::new(NaiveTrieRoot { children: vec![] }))
    }

    fn make_interm_or_leaf(label: &Label, is_terminal: bool) -> Self {
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

    pub fn bf_iter(&'trie self) -> NaiveTrieNodeBFIter<Label> {
        NaiveTrieNodeBFIter::new(self)
    }
}

impl<Label: Ord + Clone> TrieSearchMethods<Label> for NaiveTrie<Label> {
    /// # Panics
    /// When self is not a Root or IntermOrLeaf
    fn children(&self) -> &Vec<Box<Self>> {
        match self {
            NaiveTrie::Root(node) => &node.children,
            NaiveTrie::IntermOrLeaf(node) => &node.children,
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    fn is_terminal(&self) -> bool {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.is_terminal,
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    fn label(&self) -> Label {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node.label.clone(),
            _ => panic!("Unexpected type"),
        }
    }
}

impl<'trie, Label> NaiveTrieNodeBFIter<'trie, Label> {
    pub fn new(iter_start: &'trie NaiveTrie<Label>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Label: Ord + Clone> Iterator for NaiveTrieNodeBFIter<'trie, Label> {
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

impl<'trie, Label> NaiveTrie<Label> {
    /// # Panics
    /// If self is not Root.
    pub fn root(&'trie self) -> &NaiveTrieRoot<Label> {
        match self {
            NaiveTrie::Root(node) => node,
            _ => panic!("Unexpected type"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn interm_or_leaf(&'trie self) -> &NaiveTrieIntermOrLeaf<Label> {
        match self {
            NaiveTrie::IntermOrLeaf(node) => node,
            _ => panic!("Unexpected type"),
        }
    }
}

#[cfg(test)]
mod bf_iter_tests {
    use super::NaiveTrie;
    use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

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

// #[cfg(test)]
// mod search_tests {
//     use super::NaiveTrieNode;

//     fn build_trie<'trie>() -> NaiveTrieNode<'trie, u8> {
//         let mut trie = NaiveTrieNode::make_root();
//         trie.push("a");
//         trie.push("app");
//         trie.push("apple");
//         trie.push("better");
//         trie.push("application");
//         trie.push("アップル🍎");
//         trie
//     }

//     mod exact_match_tests {
//         use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

//         macro_rules! parameterized_tests {
//             ($($name:ident: $value:expr,)*) => {
//             $(
//                 #[test]
//                 fn $name() {
//                     let (query, expected_match) = $value;
//                     let trie = super::build_trie();
//                     let result = trie.exact_match(query);
//                     assert_eq!(result, expected_match);
//                 }
//             )*
//             }
//         }

//         parameterized_tests! {
//             t1: ("a", true),
//             t2: ("app", true),
//             t3: ("apple", true),
//             t4: ("application", true),
//             t5: ("better", true),
//             t6: ("アップル🍎", true),
//             t7: ("appl", false),
//             t8: ("appler", false),
//         }
//     }

//     mod predictive_search_tests {
//         use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

//         macro_rules! parameterized_tests {
//             ($($name:ident: $value:expr,)*) => {
//             $(
//                 #[test]
//                 fn $name() {
//                     let (query, expected_results) = $value;
//                     let trie = super::build_trie();
//                     let results = trie.predictive_search(query);
//                     let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
//                     assert_eq!(results, expected_results);
//                 }
//             )*
//             }
//         }

//         parameterized_tests! {
//             t1: ("a", vec!["a", "app", "apple", "application"]),
//             t2: ("app", vec!["app", "apple", "application"]),
//             t3: ("appl", vec!["apple", "application"]),
//             t4: ("apple", vec!["apple"]),
//             t5: ("b", vec!["better"]),
//             t6: ("c", Vec::<&str>::new()),
//             t7: ("アップ", vec!["アップル🍎"]),
//         }
//     }

//     mod common_prefix_search_tests {
//         use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

//         macro_rules! parameterized_tests {
//             ($($name:ident: $value:expr,)*) => {
//             $(
//                 #[test]
//                 fn $name() {
//                     let (query, expected_results) = $value;
//                     let trie = super::build_trie();
//                     let results = trie.common_prefix_search(query);
//                     let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
//                     assert_eq!(results, expected_results);
//                 }
//             )*
//             }
//         }

//         parameterized_tests! {
//             t1: ("a", vec!["a"]),
//             t2: ("ap", vec!["a"]),
//             t3: ("appl", vec!["a", "app"]),
//             t4: ("appler", vec!["a", "app", "apple"]),
//             t5: ("bette", Vec::<&str>::new()),
//             t6: ("betterment", vec!["better"]),
//             t7: ("c", Vec::<&str>::new()),
//             t8: ("アップル🍎🍏", vec!["アップル🍎"]),
//         }
//     }
// }
