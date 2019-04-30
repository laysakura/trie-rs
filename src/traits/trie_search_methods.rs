use super::naive_trie::NaiveTrie;

/// Provides trie's search methods:
///
/// - exact_match()
/// - predictive_search()
/// - common_prefix_search()
pub trait TrieSearchMethods<Label: Ord + Clone> {
    fn exact_match<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        let mut trie = self;
        for (i, chr) in query.as_ref().iter().enumerate() {
            let children = trie.children();
            let res = children.binary_search_by_key(chr, |child| child.label());
            match res {
                Ok(j) => {
                    let mut child = &children[j];
                    if i == query.as_ref().len() - 1 && child.is_terminal() {
                        return true;
                    };
                    trie = &mut child;
                }
                Err(_) => return false,
            }
        }
        false
    }

    /// # Panics
    /// If `query` is empty.
    fn predictive_search<Arr: AsRef<[Label]>>(&self, query: Arr) -> Vec<Vec<Label>> {
        assert!(!query.as_ref().is_empty());

        let mut trie = self;

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children = trie.children();
            let res = children.binary_search_by_key(chr, |child| child.label());
            match res {
                Ok(j) => trie = &children[j],
                Err(_) => return vec![],
            }
        }

        let mut results: Vec<Vec<Label>> = if trie.is_terminal() {
            vec![query.as_ref().to_vec()]
        } else {
            vec![]
        };
        let all_words_under_node: Vec<Vec<Label>> = trie
            .children()
            .iter()
            .flat_map(|child| trie.predictive_search(vec![child.label()]))
            .collect();

        for word in all_words_under_node {
            let mut result: Vec<Label> = query.as_ref().to_vec();
            result.extend(word);
            results.push(result);
        }
        results
    }

    fn common_prefix_search<Arr: AsRef<[Label]>>(&self, query: Arr) -> Vec<Vec<Label>> {
        let mut results: Vec<Vec<Label>> = Vec::new();
        let mut labels_in_path: Vec<Label> = Vec::new();

        let mut trie = self;
        for chr in query.as_ref() {
            let children = trie.children();
            let res = children.binary_search_by_key(chr, |child| child.label());
            match res {
                Ok(j) => {
                    let child = &children[j];
                    labels_in_path.push(child.label());
                    if child.is_terminal() {
                        results.push(labels_in_path.clone());
                    };
                    trie = child;
                }
                Err(_) => break,
            }
        }
        results
    }

    /// Sorted by Label's order.
    fn children(&self) -> &Vec<Box<Self>>;

    /// Returns whether this node has label of last element.
    fn is_terminal(&self) -> bool;

    /// Returns label of self's node.
    fn label(&self) -> Label;
}
