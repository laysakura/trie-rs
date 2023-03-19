pub mod naive_trie;
pub mod naive_trie_b_f_iter;

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
#[derive(Debug, Clone)]
pub enum NaiveTrie<K, V> {
    Root(NaiveTrieRoot<K, V>),
    IntermOrLeaf(Box<NaiveTrieIntermOrLeaf<K, V>>),

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

#[derive(Debug, Clone)]
pub struct NaiveTrieRoot<K, V> {
    /// Sorted by Label's order.
    children: Vec<NaiveTrie<K, V>>,
}

#[derive(Debug, Clone)]
pub struct NaiveTrieIntermOrLeaf<K, V> {
    /// Sorted by Label's order.
    children: Vec<NaiveTrie<K, V>>,
    key: K,
    value: V,
    is_terminal: bool,
}
