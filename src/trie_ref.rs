//! Common behavior between trie reference types.

use crate::{label::LabelKind, try_from::TryFromTokens};

/// Common behavior between [`NodeRef`](crate::map::NodeRef), [`NodeMut`](crate::map::NodeMut), and [`KeyRef`](crate::set::KeyRef).
pub trait TrieRef<'t, Token: 't> {
    /// A reference to a node.
    type Ref;

    /// An iterator over a range of nodes.
    type Range: Iterator<Item = Self::Ref>;

    /// Return type of prefix iterators.
    type Prefixes
    where
        Token: Clone;

    /// Return type of suffix iterators.
    type Suffixes;

    /// Returns the kind of this node's label.
    fn kind(&self) -> LabelKind;

    /// Returns `true` if this node's label is an exact match.
    fn is_exact(&self) -> bool;

    /// Returns `true` if this node's label is a prefix match.
    fn is_prefix(&self) -> bool;

    /// Iterate over this node's child nodes.
    fn children(&'t self) -> impl Iterator<Item = Self::Ref>;

    /// Returns the token of this node.
    fn token(&self) -> &Token;

    /// Returns the range of this node.
    fn range(&'t self) -> Self::Range;

    /// Returns the label of this node.
    fn label<L: TryFromTokens<Token>>(&self) -> L::Result
    where
        Token: Clone;

    /// Returns the exact matches that come before this node.
    ///
    /// e.g. "apple" → "app"
    fn prefixes_of(&'t self) -> Self::Prefixes
    where
        Token: Clone + Ord;

    /// Returns the exact matches as suffixes that follow after this node.
    ///
    /// e.g. "app" → "le" (as in "apple")
    ///
    /// Strips this node from the results; to include this node as a prefix, see [`TrieRef::starts_with`].
    fn suffixes_of(&'t self) -> Self::Suffixes
    where
        Token: Clone + Ord;

    /// Returns the exact matches that follow after this node.
    ///
    /// e.g. "app" → "apple"
    fn starts_with(&'t self) -> Self::Suffixes
    where
        Token: Clone + Ord;
}
