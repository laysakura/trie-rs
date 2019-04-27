use super::Trie;

impl<Elm: Eq + Ord + Clone> Trie<Elm> {
    // TODO generics
    pub fn exact_match<Arr: AsRef<[Elm]>>(&self, query: Arr) -> bool {
        true
    }

    pub fn predictive_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }

    pub fn common_prefix_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }
}
