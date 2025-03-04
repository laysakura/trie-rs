/// Returns labels from `(label, value)` iterators.
pub struct Labels<I>(pub(crate) I);

impl<I, L, Value> Iterator for Labels<I>
where
    I: Iterator<Item = (L, Value)>,
{
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(l, _)| l)
    }
}
