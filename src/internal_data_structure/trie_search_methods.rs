/// Provides trie's search methods:
///
/// - exact_match()
/// - predictive_search()
/// - common_prefix_search()
pub trait TrieSearchMethods<Elm: Ord + Clone> {
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

    /// # Panics
    /// If `query` is empty.
    fn predictive_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Vec<Elm>> {
        assert!(!query.as_ref().is_empty());

        let mut trie = self;

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children = trie.children();
            let res = children.binary_search_by_key(&Some(chr), |child| child.label());
            match res {
                Ok(j) => trie = &children[j],
                Err(_) => return vec![],
            }
        }

        let mut results: Vec<Vec<Elm>> = if trie.is_terminal() {
            vec![query.as_ref().to_vec()]
        } else {
            vec![]
        };
        let all_words_under_node: Vec<Vec<Elm>> = trie
            .children()
            .iter()
            .flat_map(|child| trie.predictive_search(vec![child.label().unwrap().clone()]))
            .collect();

        for word in all_words_under_node {
            let mut result: Vec<Elm> = query.as_ref().to_vec();
            result.extend(word);
            results.push(result);
        }
        results
    }

    fn common_prefix_search<Arr: AsRef<[Elm]>>(&self, query: Arr) -> Vec<Vec<Elm>> {
        let mut results: Vec<Vec<Elm>> = Vec::new();
        let mut elms_in_path: Vec<Elm> = Vec::new();

        let mut trie = self;
        for chr in query.as_ref() {
            let children = trie.children();
            let res = children.binary_search_by_key(&Some(chr), |child| child.label());
            match res {
                Ok(j) => {
                    let child = &children[j];
                    elms_in_path.push(child.label().unwrap().clone());
                    if child.is_terminal() {
                        results.push(elms_in_path.clone());
                    };
                    trie = child;
                }
                Err(_) => break,
            }
        }
        results
    }

    /// Sorted by Elm's order.
    fn children(&self) -> &Vec<Box<Self>>;

    /// Returns label of node. None for root node.
    fn label(&self) -> Option<&Elm>;

    /// Returns whether this node has label of last element.
    fn is_terminal(&self) -> bool;
}
