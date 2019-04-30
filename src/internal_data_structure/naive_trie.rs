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
pub struct NaiveTrie<'trie, Label> {
    /// Sorted by Label's order.
    children: Vec<NodeType<'trie, Label>>,
    is_terminal: bool,
}

pub enum NodeType<'trie, Label> {
    Root(&'trie NaiveTrie<'trie, Label>),
    IntermOrLeaf(&'trie NaiveTrie<'trie, Label>, Label),

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

/// Iterates over NaiveTrie in Breadth-First manner.
pub struct NaiveTrieBFIter<'trie, Label> {
    unvisited: VecDeque<&'trie NodeType<'trie, Label>>,
}

impl<'trie, Label: Ord + Clone> NaiveTrie<'trie, Label> {
    pub fn make_root() -> NodeType<'trie, Label> {
        NodeType::Root(&NaiveTrie {
            children: vec![],
            is_terminal: false,
        })
    }

    pub fn push<Arr: AsRef<[Label]>>(&'trie mut self, word: Arr) {
        let mut trie = self;
        for (i, chr) in word.as_ref().iter().enumerate() {
            let children = &mut trie.children;
            let res = children.binary_search_by_key(&chr, |child| child.interm_or_leaf().1);
            let mut node_type = match res {
                Ok(j) => &mut children[j],
                Err(j) => {
                    let is_terminal = i == word.as_ref().len() - 1;
                    let child_trie = Self::make_non_root(chr, is_terminal);
                    children.insert(j, child_trie);
                    &mut children[j]
                }
            };
            let (child_trie, _) = node_type.interm_or_leaf();
            trie = &mut child_trie;
        }
    }

    pub fn bf_iter(&'trie self) -> NaiveTrieBFIter<Label> {
        NaiveTrieBFIter::new(&NodeType::Root(self))
    }

    fn make_non_root(label: &Label, is_terminal: bool) -> NodeType<'trie, Label> {
        NodeType::IntermOrLeaf(
            &Self {
                children: vec![],
                is_terminal,
            },
            label.clone(),
        )
    }
}

impl<'trie, Label: Ord + Clone> TrieSearchMethods<Label> for NaiveTrie<'trie, Label> {
    fn children(&self) -> &Vec<NodeType<Label>> {
        &self.children
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    fn label(&self) -> Label {}
}

impl<'trie, Label> NaiveTrieBFIter<'trie, Label> {
    pub fn new(iter_start: &'trie NodeType<Label>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Label: Ord + Clone> Iterator for NaiveTrieBFIter<'trie, Label> {
    type Item = &'trie NodeType<'trie, Label>;

    /// Returns:
    ///
    /// - None: All nodes are visited.
    /// - Some(NodeType::Root): Root node.
    /// - Some(NodeType::IntermOrLeaf): Intermediate or leaf node.
    /// - Some(NodeType::PhantomSibling): Marker to represent "all siblings are iterated".
    fn next(&mut self) -> Option<Self::Item> {
        self.unvisited.pop_front().map(|node_type| {
            match node_type {
                NodeType::Root(trie) | NodeType::IntermOrLeaf(trie, _) => {
                    for child in &trie.children {
                        self.unvisited.push_back(child);
                    }
                    self.unvisited.push_back(&NodeType::PhantomSibling);
                }
                NodeType::PhantomSibling => {}
            };
            node_type
        })
    }
}

impl<'trie, Label> NodeType<'trie, Label> {
    /// # Panics
    /// If self is not Root.
    pub fn root(&'trie self) -> &NaiveTrie<Label> {
        match self {
            NodeType::Root(trie) => trie,
            _ => panic!("Unexpected NodeType"),
        }
    }

    /// # Panics
    /// If self is not IntermOrLeaf.
    pub fn interm_or_leaf(&'trie self) -> (&NaiveTrie<Label>, &Label) {
        match self {
            NodeType::IntermOrLeaf(trie, label) => (trie, label),
            _ => panic!("Unexpected NodeType"),
        }
    }
}

#[cfg(test)]
mod bf_iter_tests {
    use super::{NaiveTrie, NodeType};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (words, expected_node_types) = $value;
                let mut trie = NaiveTrie::make_root().root();
                for word in words {
                    trie.push(word);
                }
                let node_types: Vec<&NodeType<u8>> = trie.bf_iter().collect();
                assert_eq!(node_types.len(), expected_node_types.len());
                for i in 0..node_types.len() {
                    let node_type = node_types[i];
                    let expected_node_type = expected_node_types[i];

                    assert!(std::mem::discriminant(node_type) == std::mem::discriminant(&expected_node_type));

                    if let NodeType::IntermOrLeaf(_, label) = node_type {
                        let (_, expected_label) = expected_node_type.interm_or_leaf();
                        assert_eq!(label, expected_label);
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
            ]
        ),
        t2: (
            vec!["a"],
            vec![
                NaiveTrie::make_root(),
                NaiveTrie::make_non_root(&('a' as u8), false),
                NodeType::PhantomSibling,
                NodeType::PhantomSibling,
            ]
        ),
        // t3: (vec!["a", "a"], "a".as_bytes()),
        // t4: (vec!["a", "an", "bad"], "abnad".as_bytes()),
        // t5: (vec!["a", "bad", "an"], "abnad".as_bytes()),
        // t6: (
        //     // 'り' => 227, 130, 138
        //     // 'ん' => 227, 130, 147
        //     // 'ご' => 227, 129, 148
        //     vec!["a", "an", "りんご", "りんりん"],
        //     vec!['a' as u8, 227, 'n' as u8, 130, 138, 227, 130, 147, 227, 129, 130, 148, 138, 227, 130, 147],
        // ),
        // t7: (
        //     // '🍎' => 240, 159, 141, 142
        //     vec!["🍎", "りんご"],
        //     vec![227, 240, 130, 159, 138, 141, 227, 142, 130, 147, 227, 129, 148],
        // ),
    }
}

// #[cfg(test)]
// mod search_tests {
//     use super::NaiveTrie;

//     fn build_trie<'trie>() -> NaiveTrie<'trie, u8> {
//         let mut trie = NaiveTrie::make_root();
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
