use super::Trie;
use louds_rs::LoudsNodeNum;

impl<Label: Ord + Clone> Trie<Label> {
    /// Return true if [query] is an exact match.
    pub fn exact_match<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        self.exact_match_node(query).is_some()
    }

    pub(crate) fn exact_match_node<Arr: AsRef<[Label]>>(&self, query: Arr) -> Option<LoudsNodeNum> {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 && self.is_terminal(child_node_num) {
                        return Some(child_node_num);
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => return None,
            }
        }
        None
    }

    /// Return true if [query] is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        let mut cur_node_num = LoudsNodeNum(1);

        for chr in query.as_ref().iter() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(j) => cur_node_num = children_node_nums[j],
                Err(_) => return false,
            }
        }
        // Are there more nodes after our query?
        self.has_children_node_nums(cur_node_num)
    }

    /// Return all entries that match [query].
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<Arr: AsRef<[Label]>>(&self, query: Arr) -> Vec<Vec<Label>> {
        self.rec_predictive_search(query, LoudsNodeNum(1))
    }

    fn rec_predictive_search<Arr: AsRef<[Label]>>(
        &self,
        query: Arr,
        node_num: LoudsNodeNum,
    ) -> Vec<Vec<Label>> {
        assert!(!query.as_ref().is_empty());
        let mut cur_node_num = node_num;

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => return vec![],
            }
        }

        let mut results = if self.is_terminal(cur_node_num) {
            let mut v = query.as_ref().to_vec();
            // Copy the last label from the tree for dict::trie's sake. This
            // will copy its value.
            *v.last_mut().unwrap() = self.label(cur_node_num);
            vec![v]
        } else {
            vec![]
        };
        let all_words_under_cur: Vec<Vec<Label>> = self
            .children_node_nums(cur_node_num)
            .iter()
            .flat_map(|child_node_num| {
                self.rec_predictive_search(vec![self.label(*child_node_num)], cur_node_num)
            })
            .collect();

        for word in all_words_under_cur {
            let mut result: Vec<Label> = query.as_ref().to_vec();
            result.extend(word);
            results.push(result);
        }
        results
    }

    /// Return the common prefixes.
    pub fn common_prefix_search<Arr: AsRef<[Label]>>(&self, query: Arr) -> Vec<Vec<Label>> {
        let mut results: Vec<Vec<Label>> = Vec::new();
        let mut labels_in_path: Vec<Label> = Vec::new();

        let mut cur_node_num = LoudsNodeNum(1);

        for chr in query.as_ref() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    labels_in_path.push(self.label(child_node_num));
                    if self.is_terminal(child_node_num) {
                        results.push(labels_in_path.clone());
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => break,
            }
        }
        results
    }

    fn has_children_node_nums(&self, node_num: LoudsNodeNum) -> bool {
        !self.louds
            .parent_to_children(node_num)
            .is_empty()
    }

    fn children_node_nums(&self, node_num: LoudsNodeNum) -> Vec<LoudsNodeNum> {
        self.louds
            .parent_to_children(node_num)
            .iter()
            .map(|child_idx| self.louds.index_to_node_num(*child_idx))
            .collect()
    }

    fn bin_search_by_children_labels(
        &self,
        query: &Label,
        children_node_nums: &[LoudsNodeNum],
    ) -> Result<usize, usize> {
        children_node_nums.binary_search_by_key(query, |child_node_num| self.label(*child_node_num))
    }

    pub(crate) fn label(&self, node_num: LoudsNodeNum) -> Label {
        self.trie_labels[(node_num.0 - 2) as usize].label.clone()
    }

    fn is_terminal(&self, node_num: LoudsNodeNum) -> bool {
        self.trie_labels[(node_num.0 - 2) as usize].is_terminal
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

    mod is_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.is_prefix(query);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", false),
            t4: ("application", false),
            t5: ("better", false),
            t6: ("アップル🍎", false),
            t7: ("appl", true),
            t8: ("appler", false),
            t9: ("アップル", true),
            t10: ("ed", false),
            t11: ("e", false),
            t12: ("", true),
        }
    }

    mod predictive_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results = trie.predictive_search(query);
                    let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a", "app", "apple", "application"]),
            t2: ("app", vec!["app", "apple", "application"]),
            t3: ("appl", vec!["apple", "application"]),
            t4: ("apple", vec!["apple"]),
            t5: ("b", vec!["better"]),
            t6: ("c", Vec::<&str>::new()),
            t7: ("アップ", vec!["アップル🍎"]),
        }
    }

    mod common_prefix_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results = trie.common_prefix_search(query);
                    let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a"]),
            t2: ("ap", vec!["a"]),
            t3: ("appl", vec!["a", "app"]),
            t4: ("appler", vec!["a", "app", "apple"]),
            t5: ("bette", Vec::<&str>::new()),
            t6: ("betterment", vec!["better"]),
            t7: ("c", Vec::<&str>::new()),
            t8: ("アップル🍎🍏", vec!["アップル🍎"]),
        }
    }
}
