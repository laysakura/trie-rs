use crate::Trie;
use louds_rs::LoudsNodeNum;

pub struct PrefixIter<'a, L, Label, Value>
{
    trie: &'a Trie<Label, Value>,
    query: Vec<L>,
    node: LoudsNodeNum,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
}

impl<'a, L, Label, Value> PrefixIter<'a, L, Label, Value>
{
    #[inline]
    pub fn new(trie: &'a Trie<Label, Value>, mut query: Vec<L>) -> Self {
        query.reverse();
        Self {
            trie,
            node: LoudsNodeNum(1),
            query,
            buffer: Vec::new(),
            consume: None,
        }
    }

    #[inline]
    pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1),
            query: Vec::new(),
            buffer: Vec::new(),
            consume: None,
        }
    }
}

impl<'a, L, Label: Ord + Clone, Value> Iterator for PrefixIter<'a, L, Label, Value>
    where Label: PartialOrd<L>
{
    type Item = &'a Label;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some(chr) = self.query.pop() {
            // for chr in query.as_ref() {
                let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node)
                    .collect();
                let res = self.trie.bin_search_by_children_labels::<L>(&chr, &children_node_nums[..]);
                match res {
                    Ok(j) => {
                        let child_node_num = children_node_nums[j];
                        self.buffer.push(self.trie.label(child_node_num));
                        if self.trie.is_terminal(child_node_num) {
                            self.consume = Some(0);
                        };
                        self.node = child_node_num;
                    }
                    Err(_) => break,
                }
            } else {
                return None;
            }
        }
        if let Some(i) = self.consume.take() {
            if i >= self.buffer.len() {
                None
            } else {
                self.consume = Some(i + 1);
                Some(self.buffer[i])
            }
        } else {
            None
        }
    }
}
