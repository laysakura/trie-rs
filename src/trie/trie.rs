use super::Trie;
use crate::traits::trie_methods::TrieMethods;

impl<Label: Ord + Clone> TrieMethods<Label> for Trie<Label> {
    fn children(&self) -> &Vec<Box<Self>> {
        &self
            .louds
            .parent_to_children(&self.current_node_num)
            .iter()
            .map(|child_index| {
                let child_node_num = self.louds.index_to_node_num(child_index);
                self.tries[child_node_num.value() as usize].unwrap()
            })
            .collect()
    }

    fn is_terminal(&self) -> bool {
        self.trie_nodes[self.current_node_num.value() as usize].map_or(false, |t| t.is_terminal)
    }

    /// # Panics
    /// If self.current_node_num points to 0, 1, or out-of-bound.
    fn label(&self) -> Label {
        self.trie_nodes[self.current_node_num.value() as usize].unwrap().label
    }
}
