use super::Trie;
use crate::traits::trie_methods::TrieMethods;

impl<Label: Ord + Clone> TrieMethods<Label> for Trie<Label> {
    fn children(&self) -> &Vec<Box<Self>> {
        &self
            .louds
            .parent_to_children(&self.current_node_num)
            .iter()
            .map(|child_index| {
                Box::new(Self {
                    current_node_num: self.louds.index_to_node_num(child_index),
                    louds: self.louds.clone(),
                    label_terminal_vec: self.label_terminal_vec.clone(),
                })
            })
            .collect()
    }

    fn is_terminal(&self) -> bool {
        self.label_terminal_vec[self.current_node_num.value() as usize].map_or(false, |lt| lt.1)
    }

    /// # Panics
    /// If self.current_node_num points to 0, 1, or out-of-bound.
    fn label(&self) -> Label {
        self.label_terminal_vec[self.current_node_num.value() as usize].unwrap().0
    }
}
