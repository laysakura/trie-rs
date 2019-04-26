pub struct Trie {}

impl Trie {
    // TODO generics
    pub fn exact_match(&self, query: &str) -> bool {
        true
    }

    pub fn predictive_search(&self, query: &str) -> Vec<&str> {
        vec![]
    }

    pub fn common_prefix_search(&self, query: &str) -> Vec<&str> {
        vec![]
    }
}
