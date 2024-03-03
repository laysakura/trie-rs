use super::Trie;
use louds_rs::{self, LoudsNodeNum, ChildNodeIter};
use crate::map::postfix_iter::PostfixIter;
use crate::map::prefix_iter::PrefixIter;
use crate::map::search_iter::SearchIter;
use frayed::{Chunk, fraught::Prefix};

impl<Label: Ord, Value> Trie<Label, Value> {
    /// Return true if [query] is an exact match.
    pub fn exact_match<L>(&self, query: impl AsRef<[L]>) -> Option<&Value>
    where Label: PartialOrd<L> {
        self.exact_match_node(query).and_then(|x| self.value(x))
    }

    pub(crate) fn exact_match_node<L>(&self, query: impl AsRef<[L]>) -> Option<LoudsNodeNum>
        where Label: PartialOrd<L> {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums: Vec<LoudsNodeNum> = self.children_node_nums(cur_node_num)
                                                            .collect();
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
    pub fn is_prefix<L>(&self, query: impl AsRef<[L]>) -> bool
    where Label: PartialOrd<L> {
        let mut cur_node_num = LoudsNodeNum(1);

        for chr in query.as_ref().iter() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num)
                                                 .collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(j) => cur_node_num = children_node_nums[j],
                Err(_) => return false,
            }
        }
        // Are there more nodes after our query?
        self.has_children_node_nums(cur_node_num)
    }

    pub fn predictive_search<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Vec<(Vec<Label>, Value)>
    where Label: PartialOrd<L> + Clone,
    Value: Clone {
        let chunk = self.predictive_search_ref(query);
        chunk
            .map(|v| (v.cloned().collect(),
                      chunk.iter_ref().value().cloned().unwrap()))
            .into_iter()
            .collect()
    }

    /// Return all entries that match [query].
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search_ref<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        // Chunk<Prefix<std::vec::IntoIter<&Label>, PostfixIter<'a, Label, Value>>>
        Chunk<SearchIter<'a, Label, Value>>
    where Label: PartialOrd<L>,
    {
        assert!(!query.as_ref().is_empty());
        let mut cur_node_num = LoudsNodeNum(1);
        let mut prefix = Vec::new();

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num)
                .collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => {
                    return Chunk::new(SearchIter::empty(self))
                }
            }
            // prefix.push(self.label(cur_node_num));
            prefix.push(cur_node_num);
        }
        let _ = prefix.pop();
        Chunk::new(SearchIter::new(self, prefix, cur_node_num))
        // Chunk::new(Prefix::new(prefix.into_iter(), self.postfix_search_unfused(cur_node_num)).prefix_empty(true))
        // Chunk::new(Prefix::new(prefix.into_iter(), self.postfix_search_unfused(cur_node_num)).prefix_empty(true))
    }

    pub fn postfix_search<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Chunk<PostfixIter<'a, Label, Value>>
    where Label: PartialOrd<L>,
    {
        assert!(!query.as_ref().is_empty());
        let mut cur_node_num = LoudsNodeNum(1);

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num)
                .collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => {
                    return Chunk::new(PostfixIter::empty(self));
                }
            }
        }
        Chunk::new(self.postfix_search_unfused(cur_node_num))
    }

    fn postfix_search_unfused<'a>(
        &'a self,
        node_num: LoudsNodeNum
    ) -> PostfixIter<'a, Label, Value>
    {
        PostfixIter::new(self, node_num)
    }

    /// Return the common prefixes.
    pub fn common_prefix_search<L>(&self, query: impl AsRef<[L]>) -> Vec<(Vec<Label>, Value)>
    where Label: PartialOrd<L> + Clone, L: Clone, Value: Clone {
        let chunk = self.common_prefix_search_ref(query.as_ref().to_vec());
        chunk
            .map(|v| (v.cloned().collect(),
                      chunk.iter_ref().value().cloned().unwrap()))
            .into_iter()
            .collect()
            // .into_iter()
            // .map(|v| (v.into_iter().cloned().collect(),
            //           // NOTE: This will not always work if this wasn't being
            //           // collected and processed in order.
            //           chunk.iter_ref().value().cloned().unwrap()))
            // .collect()
    }

    /// Return the common prefixes.
    pub fn common_prefix_search_ref<L>(&self, query: Vec<L>)
                                       -> Chunk<PrefixIter<'_, L, Label, Value>>
        where Label: PartialOrd<L> + Clone {
        Chunk::new(PrefixIter::new(&self, query))
    }

    fn has_children_node_nums(&self, node_num: LoudsNodeNum) -> bool {
        self.louds
            .parent_to_children_indices(node_num)
            .next()
            .is_some()
    }

    pub(crate) fn children_node_nums(&self, node_num: LoudsNodeNum) -> ChildNodeIter {
        self.louds
            .parent_to_children_nodes(node_num)
    }

    pub(crate) fn bin_search_by_children_labels<L>(
        &self,
        query: &L,
        children_node_nums: &[LoudsNodeNum],
    ) -> Result<usize, usize>
        where Label: PartialOrd<L> {
        children_node_nums.binary_search_by(|child_node_num| self.label(*child_node_num).partial_cmp(query).unwrap())
    }

    pub(crate) fn label(&self, node_num: LoudsNodeNum) -> &Label {
        &self.trie_labels[(node_num.0 - 2) as usize].label
    }

    pub(crate) fn label_mut(&mut self, node_num: LoudsNodeNum) -> &mut Label {
        &mut self.trie_labels[(node_num.0 - 2) as usize].label
    }

    pub(crate) fn is_terminal(&self, node_num: LoudsNodeNum) -> bool {
        self.trie_labels[(node_num.0 - 2) as usize].is_terminal.is_some()
    }

    pub(crate) fn value(&self, node_num: LoudsNodeNum) -> Option<&Value> {
        self.trie_labels[(node_num.0 - 2) as usize].is_terminal.as_ref()
    }

    pub(crate) fn value_mut(&mut self, node_num: LoudsNodeNum) -> Option<&mut Value> {
        self.trie_labels[(node_num.0 - 2) as usize].is_terminal.as_mut()
    }
}

#[cfg(test)]
mod search_tests {
    use crate::{Trie, TrieBuilder};
    // type Trie<T> = crate::Trie<T, ()>;
    // type TrieBuilder<T> = crate::TrieBuilder<T, ()>;

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
                    let results = trie.predictive_search(query).into_iter().map(|g| String::from_utf8(g).unwrap()).collect::<Vec<_>>();
                    // results.sort_by(|a, b| a.len().cmp(&b.len()));
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

    mod posfix_search_tests {

        #[test]
        fn postfix_unfused() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("app");
            let mut iter = postfixes.into_inner().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert!(iter.next().is_none());
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), Some('l'));
            assert_eq!(iter.next(), Some('e'));
        }

        #[test]
        fn postfix_baseline() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("app");
            let mut chunks = postfixes.into_iter();
            // assert_eq!(chunks.by_ref().count(), 3);
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), None);
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), Some('l'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), None);
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), Some('l'));
            assert_eq!(iter.next(), Some('i'));
            assert_eq!(iter.next(), Some('c'));
            assert_eq!(iter.next(), Some('a'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('i'));
            assert_eq!(iter.next(), Some('o'));
            assert_eq!(iter.next(), Some('n'));
            assert_eq!(iter.next(), None);


            assert!(chunks.next().is_none());
        }

        #[test]
        fn postfix_2() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("b");
            let mut chunks = postfixes.into_iter();
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('b'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), Some('r'));
            assert_eq!(iter.next(), None);
            assert!(chunks.next().is_none());
        }

        #[test]
        fn postfix_3() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("bet");
            let mut chunks = postfixes.into_iter();
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), Some('r'));
            assert_eq!(iter.next(), None);
            assert!(chunks.next().is_none());
        }

        #[test]
        fn postfix_no_match() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("NOT-THERE");
            let chunks = postfixes.into_iter();
            assert_eq!(chunks.count(), 0);
        }

        #[test]
        fn vec_into_iter_clone() {
            let v = vec![1,2,3];
            let i = v.into_iter();
            let c = i.clone();
            assert_eq!(c.count(), 3);
            assert_eq!(i.count(), 3);
        }

    }
}
