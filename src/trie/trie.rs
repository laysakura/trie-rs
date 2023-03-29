use super::Trie;
use louds_rs::LoudsNodeNum;

impl<K: Ord + Clone, V: Clone> Trie<K, V> {
    pub fn exact_match<Key: AsRef<[K]>>(&self, query: Key) -> bool {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 && self.is_terminal(child_node_num) {
                        return true;
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => return false,
            }
        }
        false
    }

    pub fn get<Key: AsRef<[K]>>(&self, query: Key) -> Option<&V> {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 && self.is_terminal(child_node_num) {
                        let value_opts = &self.trie_labels[child_node_num.0 as usize - 2].value;
                        return match value_opts {
                            Some(value) => Some(value),
                            None => None,
                        };
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => return None,
            }
        }
        None
    }

    pub fn get_mut<Key: AsRef<[K]>>(&mut self, query: Key) -> Option<&mut V> {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 && self.is_terminal(child_node_num) {
                        let value_opts = &mut self.trie_labels[child_node_num.0 as usize - 2].value;
                        return match value_opts {
                            Some(ref mut value) => Some(value),
                            None => None,
                        };
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => {
                    return None;
                }
            }
        }
        None
    }

    pub fn set<Key: AsRef<[K]>>(&mut self, query: Key, value: V) {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);

            if let Ok(j) = res {
                let child_node_num = children_node_nums[j];
                if i == query.as_ref().len() - 1 {
                    self.trie_labels[child_node_num.0 as usize - 2].value = Some(value);
                    return;
                };
                cur_node_num = child_node_num;
            }
        }
    }

    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<Arr: AsRef<[K]>>(&self, query: Arr) -> Vec<Vec<K>> {
        self.rec_predictive_search(query, LoudsNodeNum(1))
    }
    fn rec_predictive_search<Arr: AsRef<[K]>>(
        &self,
        query: Arr,
        node_num: LoudsNodeNum,
    ) -> Vec<Vec<K>> {
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
            vec![query.as_ref().to_vec()]
        } else {
            vec![]
        };
        let all_words_under_cur: Vec<Vec<K>> = self
            .children_node_nums(cur_node_num)
            .iter()
            .flat_map(|child_node_num| {
                self.rec_predictive_search(vec![self.label(*child_node_num)], cur_node_num)
            })
            .collect();

        for word in all_words_under_cur {
            let mut result: Vec<K> = query.as_ref().to_vec();
            result.extend(word);
            results.push(result);
        }
        results
    }

    pub fn common_prefix_search<Key: AsRef<[K]>>(&self, query: Key) -> Vec<Vec<K>> {
        let mut results: Vec<Vec<K>> = Vec::new();
        let mut labels_in_path: Vec<K> = Vec::new();

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

    pub fn common_prefix_search_with_values<Key: AsRef<[K]>>(
        &self,
        query: Key,
    ) -> Vec<(Vec<K>, V)> {
        let mut results: Vec<(Vec<K>, V)> = Vec::new();
        let mut labels_in_path: Vec<K> = Vec::new();

        let mut cur_node_num = LoudsNodeNum(1);

        for chr in query.as_ref() {
            let children_node_nums = self.children_node_nums(cur_node_num);
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    labels_in_path.push(self.label(child_node_num));
                    if self.is_terminal(child_node_num) {
                        match self.value(child_node_num) {
                            Some(value) => results.push((labels_in_path.clone(), value)),
                            None => panic!("Trie is inconsistent"),
                        }
                    };
                    cur_node_num = child_node_num;
                }
                Err(_) => break,
            }
        }
        results
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
        query: &K,
        children_node_nums: &[LoudsNodeNum],
    ) -> Result<usize, usize> {
        children_node_nums.binary_search_by_key(query, |child_node_num| self.label(*child_node_num))
    }

    fn label(&self, node_num: LoudsNodeNum) -> K {
        self.trie_labels[(node_num.0 - 2) as usize].key.clone()
    }

    fn value(&self, node_num: LoudsNodeNum) -> Option<V> {
        self.trie_labels[(node_num.0 - 2) as usize].value.clone()
    }

    fn is_terminal(&self, node_num: LoudsNodeNum) -> bool {
        self.trie_labels[(node_num.0 - 2) as usize].is_terminal
    }
}

#[cfg(test)]
mod search_tests {
    use crate::{Trie, TrieBuilder};

    fn build_trie() -> Trie<u8, String> {
        let mut builder = TrieBuilder::new();
        builder.push("a", "random_value_1".to_string());
        builder.push("app", "random_value_2".to_string());
        builder.push("apple", "random_value_3".to_string());
        builder.push("better", "random_value_4".to_string());
        builder.push("application", "random_value_5".to_string());
        builder.push("アップル🍎", "random_value_6".to_string());
        builder.build()
    }

    fn build_trie_mut() -> Trie<u8, String> {
        let mut builder = TrieBuilder::new();
        builder.push("a", "random_value_1".to_string());
        builder.push("a", "".to_string());
        builder.push("app", "random_value_2".to_string());
        builder.push("app", "random_value_3".to_string());
        builder.push("apple", "random_value_4".to_string());
        builder.build()
    }

    #[test]
    fn test_common_prefix_with_values_search() {
        let trie = build_trie();
        let result = trie.common_prefix_search_with_values("apple");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, b"a".to_vec());
        assert_eq!(result[0].1, "random_value_1".to_string());
        assert_eq!(result[1].0, b"app".to_vec());
        assert_eq!(result[1].1, "random_value_2".to_string());
        assert_eq!(result[2].0, b"apple".to_vec());
        assert_eq!(result[2].1, "random_value_3".to_string());
    }

    #[test]
    fn test_get_mut() {
        let mut trie = build_trie_mut();
        let result = trie.get_mut("a");
        assert_eq!(result.unwrap(), &mut "".to_string());
        let result = trie.get_mut("apple");
        assert_eq!(result.unwrap(), &mut "random_value_4".to_string());
        let result = trie.get_mut("app");
        assert_eq!(result.unwrap(), &mut "random_value_3".to_string());
    }

    #[test]
    fn test_get() {
        let trie = build_trie();
        let result = trie.get("better");
        assert_eq!(result.unwrap(), &"random_value_4".to_string());
    }

    #[test]
    fn test_set_multiple() {
        let mut builder = TrieBuilder::new();
        let mut contents = vec!["1", "2", "3", "4"];
        builder.push("a", vec!["x", "y"]);
        builder.push("axe", contents.clone());
        contents.push("5");
        let mut trie = builder.build();
        trie.set("axe", contents);
        assert_eq!(trie.get("a").unwrap(), &vec!["x", "y"]);
        assert_eq!(trie.get("ax"), None);
        assert_eq!(trie.get("axe").unwrap(), &vec!["1", "2", "3", "4", "5"]);
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
