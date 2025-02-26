use crate::{
    iter::{Keys, KeysExt, PostfixIter},
    label::LabelKind,
    map::NodeRef,
    try_collect::TryFromIterator,
};

/// A reference to a trie key.
#[derive(Debug, PartialEq, Eq)]
pub struct KeyRef<'t, Token>(pub(crate) NodeRef<'t, Token, ()>);

impl<Token> KeyRef<'_, Token> {
    /// Returns the kind of the node's label.
    #[inline]
    pub fn kind(&self) -> LabelKind {
        self.0.kind()
    }

    /// Returns `true`` if the node's label is an exact match.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.0.is_exact()
    }

    /// Returns `true`` if the node's label is a prefix match.
    #[inline]
    pub fn is_prefix(&self) -> bool {
        self.0.is_prefix()
    }

    /// Returns the token of the node.
    #[inline]
    pub fn token(&self) -> &Token {
        self.0.token()
    }

    /// Iterate over child nodes.
    pub fn children(&self) -> impl Iterator<Item = KeyRef<'_, Token>> {
        self.0.children().map(KeyRef)
    }

    /// Returns the postfixes and values of all children of this node.
    pub fn postfix_search<C, M>(&self) -> Keys<PostfixIter<'_, Token, (), C, M>>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone + Ord,
    {
        PostfixIter::new(self.0.trie, self.0.node_num).keys()
    }
}
