use std::collections::VecDeque;

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
/// <Node>
///   |
///   | a: Label
/// <Node (Terminate)>
///   |
///   | p
/// <Node>
///   |
///   | p
/// <Node (Terminate)>
///   |
///   | l
/// <Node>
///   |------------------+
///   | e                | i
/// <Node (Terminate)>     <Node>
///                      |
///                      | c
///                   <Node>
///                      |
///                      | a
///                    <Node>
///                      |
///                      | t
///                    <Node>
///                      |
///                      | i
///                    <Node>
///                      |
///                      | o
///                    <Node>
///                      |
///                      | n
///                    <Node (Terminate)>
/// ```
pub struct NaiveTrie<Label> {
    /// Sorted by Label's order.
    children: Vec<Box<NaiveTrie<Label>>>,

    /// Only root node is None.
    label: Option<Label>,

    is_terminal: bool,
}

/// Iterates over NaiveTrie in Breadth-First manner.
pub struct NaiveTrieBFIter<'trie, Label> {
    unvisited: VecDeque<&'trie NaiveTrie<Label>>,
}

enum LabelOrNull<L> {
    Null,
}
