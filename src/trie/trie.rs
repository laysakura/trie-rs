use super::Trie;

impl<T: Eq + Ord> Trie<T> {
    // TODO generics
    pub fn exact_match<U: Into<T>>(&self, query: U) -> bool {
        true
    }

    pub fn predictive_search<U: Into<T>>(&self, query: U) -> Vec<U> {
        vec![]
    }

    pub fn common_prefix_search<U: Into<T>>(&self, query: U) -> Vec<U> {
        vec![]
    }
}
