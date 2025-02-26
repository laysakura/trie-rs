use crate::{label::LabelKind, map::NodeRef};

/// A reference to a trie key.
pub struct KeyRef<'t, Token>(pub(crate) NodeRef<'t, Token, ()>);

impl<'t, Token> KeyRef<'t, Token> {
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
}
