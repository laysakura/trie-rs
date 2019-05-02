use super::Trie;
use crate::traits::trie_methods::TrieMethods;
use louds_rs::LoudsNodeNum;

impl<Label: Ord + Clone> Trie<Label> {
    pub fn exact_match<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        let mut cur_node_num = LoudsNodeNum(1);
        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums: Vec<LoudsNodeNum> = self
                .louds
                .parent_to_children(&cur_node_num)
                .iter()
                .map(|child_idx| self.louds.index_to_node_num(child_idx))
                .collect();

            let res = children_node_nums.binary_search_by_key(chr, |child_node_num| {
                let child_trie_label = self.trie_labels[child_node_num.0 as usize]
                    .as_ref()
                    .unwrap();
                child_trie_label.label.clone()
            });

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    let child_trie_label = self.trie_labels[child_node_num.0 as usize]
                        .as_ref()
                        .unwrap();
                    if i == query.as_ref().len() - 1 && child_trie_label.is_terminal {
                        return true;
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => return false,
            }
        }
        false
    }
}

#[cfg(test)]
mod search_tests {
    use crate::{Trie, TrieBuilder};

    fn build_trie() -> Trie<u8> {
        let mut builder = TrieBuilder::new();
        builder.push("a");
        builder.push("app");
        builder.push("apple");
        builder.push("better");
        builder.push("application");
        builder.push("アップル🍎");
        builder.build()
    }

    mod exact_match_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.exact_match(query);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", true),
            t4: ("application", true),
            t5: ("better", true),
            t6: ("アップル🍎", true),
            t7: ("appl", false),
            t8: ("appler", false),
        }
    }

    // mod predictive_search_tests {
    //     macro_rules! parameterized_tests {
    //         ($($name:ident: $value:expr,)*) => {
    //         $(
    //             #[test]
    //             fn $name() {
    //                 let (query, expected_results) = $value;
    //                 let trie = super::build_trie();
    //                 let results = trie.predictive_search(query);
    //                 let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
    //                 assert_eq!(results, expected_results);
    //             }
    //         )*
    //         }
    //     }

    //     parameterized_tests! {
    //         t1: ("a", vec!["a", "app", "apple", "application"]),
    //         t2: ("app", vec!["app", "apple", "application"]),
    //         t3: ("appl", vec!["apple", "application"]),
    //         t4: ("apple", vec!["apple"]),
    //         t5: ("b", vec!["better"]),
    //         t6: ("c", Vec::<&str>::new()),
    //         t7: ("アップ", vec!["アップル🍎"]),
    //     }
    // }

    // mod common_prefix_search_tests {
    //     macro_rules! parameterized_tests {
    //         ($($name:ident: $value:expr,)*) => {
    //         $(
    //             #[test]
    //             fn $name() {
    //                 let (query, expected_results) = $value;
    //                 let trie = super::build_trie();
    //                 let results = trie.common_prefix_search(query);
    //                 let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
    //                 assert_eq!(results, expected_results);
    //             }
    //         )*
    //         }
    //     }

    //     parameterized_tests! {
    //         t1: ("a", vec!["a"]),
    //         t2: ("ap", vec!["a"]),
    //         t3: ("appl", vec!["a", "app"]),
    //         t4: ("appler", vec!["a", "app", "apple"]),
    //         t5: ("bette", Vec::<&str>::new()),
    //         t6: ("betterment", vec!["better"]),
    //         t7: ("c", Vec::<&str>::new()),
    //         t8: ("アップル🍎🍏", vec!["アップル🍎"]),
    //     }
    // }
}
