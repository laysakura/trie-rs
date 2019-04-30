/// Provides trie's search methods:
///
/// - exact_match()
/// - predictive_search()
/// - common_prefix_search()
pub trait TrieSearchMethods<Elm: Ord> {
    fn exact_match<Arr: AsRef<[Elm]>>(&self, query: Arr) -> bool {
        let mut trie = self;
        for (i, chr) in query.as_ref().iter().enumerate() {
            let children = trie.children();
            let res = children.binary_search_by_key(&Some(chr), |child| child.label());
            match res {
                Ok(j) => {
                    let child = &children[j];
                    if i == query.as_ref().len() - 1 && child.is_terminal() {
                        return true;
                    };
                    trie = child;
                }
                Err(_) => return false,
            }
        }
        false
    }

    fn predictive_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }

    fn common_prefix_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Arr> {
        vec![]
    }

    /// Sorted by Elm's order.
    fn children(&self) -> &Vec<Box<Self>>;

    /// Returns label of node. None for root node.
    fn label(&self) -> Option<&Elm>;

    /// Returns whether this node has label of last element.
    fn is_terminal(&self) -> bool;
}
