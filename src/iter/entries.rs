pub struct Entries<I>(I);

impl<I> Entries<I> {
    pub fn new(iter: I) -> Self {
        Self(iter)
    }
}

impl<I,C,V> Iterator for Entries<I> where
    I: Iterator<Item = (C, V)>
    {
    type Item = C;
    fn next(&mut self) -> Option<C> {
        self.0.next().map(|x| x.0)
    }
}
