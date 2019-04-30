/// Provides trie's search methods:
///
/// - exact_match()
/// - predictive_search()
/// - common_prefix_search()
pub trait TrieSearchMethods<Elm> {
    fn exact_match<Arr: AsRef<[Elm]>>(&self, query: Arr) -> bool {
        true
    }

    fn predictive_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }

    fn common_prefix_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }
}
