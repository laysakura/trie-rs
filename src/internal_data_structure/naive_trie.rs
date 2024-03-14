pub mod naive_trie;
pub mod naive_trie_b_f_iter;
pub mod naive_trie_b_f_into_iter;

/// Naive trie with ordered Label sequence in edges.
///
/// The following naive trie contains these words.
/// - a
/// - app
/// - apple
/// - application
///
/// ```text
/// <Root>
///   |
///   | a: Label
/// <IntermOrLeaf (Terminate)>
///   |
///   | p
/// <IntermOrLeaf>
///   |
///   | p
/// <IntermOrLeaf (Terminate)>
///   |
///   | l
/// <IntermOrLeaf>
///   |------------------------------+
///   | e                            | i
/// <IntermOrLeaf (Terminate)>     <IntermOrLeaf>
///                                  |
///                                  | c
///                               <IntermOrLeaf>
///                                  |
///                                  | a
///                                <IntermOrLeaf>
///                                  |
///                                  | t
///                                <IntermOrLeaf>
///                                  |
///                                  | i
///                                <IntermOrLeaf>
///                                  |
///                                  | o
///                                <IntermOrLeaf>
///                                  |
///                                  | n
///                                <IntermOrLeaf (Terminate)>
/// ```
pub enum NaiveTrie<Label, Value> {
    Root(Box<NaiveTrieRoot<Label, Value>>),
    IntermOrLeaf(Box<NaiveTrieIntermOrLeaf<Label, Value>>),

    /// Used for Breadth-First iteration.
    ///
    /// ```text
    /// <Root>
    ///   |
    ///   |------------------+- - - - - - - - +
    ///   | a                | i              |
    /// <IntermOrLeaf>     <IntermOrLeaf>   <PhantomSibling>
    ///   |                  |
    ///   .                  +- - - - - - - - +
    ///   |                  |  n             |
    /// <PhantomSibling>   <IntermOrLeaf>   <PhantomSibling>
    ///                      |
    ///                      |
    ///                      |
    ///                    <PhantomSibling>
    /// ```
    ///
    /// This trie's BFIter emits:
    /// `a i <PhantomSibling> <PhantomSibling> n <PhantomSibling> <PhantomSibling>`
    PhantomSibling,
}

pub struct NaiveTrieRoot<Label, Value> {
    /// Sorted by Label's order.
    children: Vec<Box<NaiveTrie<Label, Value>>>,
}

pub struct NaiveTrieIntermOrLeaf<Label, Value> {
    /// Sorted by Label's order.
    children: Vec<Box<NaiveTrie<Label, Value>>>,
    pub(crate) label: Label,
    pub(crate) value: Option<Value>,
}
