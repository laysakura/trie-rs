/// Strips off `Value`s from [crate::map::Trie].
pub struct Entries<I>(I);

impl<I> Entries<I> {
    ///
    pub fn new(iter: I) -> Self {
        Self(iter)
    }
}

// TODO: This is generic for V, which is a stand-in for the Value, but in a
// `map::Trie<K,V>`, its iterators will actually reurn `(C, &V)`. Hopefully that
// won't matter.
impl<I, C, V> Iterator for Entries<I>
where
    I: Iterator<Item = (C, V)>,
{
    type Item = C;
    fn next(&mut self) -> Option<C> {
        self.0.next().map(|x| x.0)
    }
}
