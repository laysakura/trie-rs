use crate::map::Trie;
use louds_rs::LoudsNodeNum;
use crate::map::postfix_iter::PostfixIter;

pub struct SearchIter<'a, Label, Value>
{
    trie: &'a Trie<Label, Value>,
    prefix: Vec<LoudsNodeNum>,
    index: usize,
    first: Option<&'a Label>,
    postfix_iter: PostfixIter<'a, Label, Value>,
    value: Option<&'a Value>,
}

impl <'a, Label, Value> SearchIter<'a, Label, Value> {

    pub fn new(trie: &'a Trie<Label, Value>,
           prefix: Vec<LoudsNodeNum>,
           postfix_start: LoudsNodeNum) -> Self {
        SearchIter {
            trie,
            prefix,
            index: 0,
            first: None,
            value: None,
            postfix_iter: PostfixIter::new(trie, postfix_start)
        }
    }

    pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
        SearchIter {
            trie,
            prefix: vec![],
            index: 0,
            first: None,
            value: None,
            postfix_iter: PostfixIter::empty(trie)
        }
    }

    pub fn value(&self) -> Option<&Value> {
        self.value
    }
}

impl<'a, Label: Ord, Value> Iterator for SearchIter<'a, Label, Value>
{
    type Item = &'a Label;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.prefix.len() {
            let i = self.index;
            self.index += 1;
            Some(self.trie.label(self.prefix[i]))
        } else if self.first.is_some() {
            self.first.take()
        } else {
            match self.postfix_iter.next() {
                None => {
                    self.first = self.postfix_iter.next();
                    if self.first.is_some() {
                        self.index = 0;
                    }
                    None
                },
                x => {
                    self.value = self.postfix_iter.value();
                    x
                }
            }
        }
    }
}
