use super::Trie;

impl<Label: Ord + Clone> Trie<Label> {
    pub fn exact_match<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        true
    }

    pub fn predictive_search<Arr: AsRef<[Label]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }

    pub fn common_prefix_search<Arr: AsRef<[Label]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }
}
