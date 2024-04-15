pub mod naive_trie_impl;
pub mod naive_trie_b_f_iter;

#[cfg(feature = "mem_dbg")]
use mem_dbg::MemDbg;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    Root(NaiveTrieRoot<Label, Value>),
    IntermOrLeaf(NaiveTrieIntermOrLeaf<Label, Value>),

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
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NaiveTrieRoot<Label, Value> {
    /// Sorted by Label's order.
    children: Vec<NaiveTrie<Label, Value>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NaiveTrieIntermOrLeaf<Label, Value> {
    /// Sorted by Label's order.
    children: Vec<NaiveTrie<Label, Value>>,
    pub(crate) label: Label,
    pub(crate) value: Option<Value>,
}
