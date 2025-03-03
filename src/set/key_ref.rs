use crate::{
    iter::{KeyIter, KeyRefExt, Keys, KeysExt, TokenIter},
    label::LabelKind,
    map::NodeRef,
    search::{PostfixIter, PrefixIter},
    trie_ref::TrieRef,
    try_from::TryFromTokens,
};

/// A reference to a trie key.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct KeyRef<'t, Token>(pub(crate) NodeRef<'t, Token, ()>);

impl<'t, Token> KeyRef<'t, Token> {
    /// Returns the kind of this node's label.
    pub fn kind(&self) -> LabelKind {
        self.0.kind()
    }

    /// Returns `true` if this node's label is an exact match.
    pub fn is_exact(&self) -> bool {
        self.0.is_exact()
    }

    /// Returns `true` if this node's label is a prefix match.
    pub fn is_prefix(&self) -> bool {
        self.0.is_prefix()
    }

    /// Iterate over this node's child nodes.
    pub fn children(&'t self) -> impl Iterator<Item = KeyRef<'t, Token>> {
        self.0.children().map(KeyRef)
    }

    /// Returns the token of this node.
    pub fn token(&self) -> &Token {
        self.0.token()
    }

    /// Returns the range of this node.
    pub fn range(&'t self) -> KeyIter<'t, Token> {
        KeyIter(self.0.range())
    }

    /// Returns the label of this node.
    pub fn label<L: TryFromTokens<Token>>(&self) -> Result<L, L::Error>
    where
        Token: Clone,
    {
        let tokens = self.range().map(|k| k.token().clone());
        L::try_from_tokens(tokens, true)
    }

    /// Returns the exact matches that come before this node.
    ///
    /// e.g. "apple" → "app"
    pub fn prefixes_of(
        &'t self,
    ) -> Keys<PrefixIter<'t, Token, (), TokenIter<Token, KeyIter<'t, Token>>>>
    where
        Token: Clone + Ord,
    {
        PrefixIter::from_tokens(self.0.trie, self.range().tokens()).keys()
    }

    /// Returns the exact matches as suffixes that follow after this node.
    ///
    /// e.g. "app" → "le" (as in "apple")
    ///
    /// Strips this node from the results; to include this node as a prefix, see [`Self::starts_with`].
    pub fn suffixes_of(&'t self) -> Keys<PostfixIter<'t, Token, ()>>
    where
        Token: Clone + Ord,
    {
        PostfixIter::suffixes_of(self.0.trie, self.0.node_num).keys()
    }

    /// Returns the exact matches that follow after this node.
    ///
    /// e.g. "app" → "apple"
    pub fn starts_with(&'t self) -> Keys<PostfixIter<'t, Token, ()>>
    where
        Token: Clone + Ord,
    {
        PostfixIter::starts_with(self.0.trie, self.0.node_num).keys()
    }
}

impl<'t, Token> TrieRef<'t, Token> for KeyRef<'t, Token> {
    type Ref = KeyRef<'t, Token>;

    type Range = KeyIter<'t, Token>;

    type Prefixes
        = Keys<PrefixIter<'t, Token, (), TokenIter<Token, Self::Range>>>
    where
        Token: Clone;

    type Suffixes = Keys<PostfixIter<'t, Token, ()>>;

    fn kind(&self) -> LabelKind {
        self.kind()
    }

    fn is_exact(&self) -> bool {
        self.is_exact()
    }

    fn is_prefix(&self) -> bool {
        self.is_prefix()
    }

    fn children(&'t self) -> impl Iterator<Item = Self::Ref> {
        self.children()
    }

    fn token(&self) -> &Token {
        self.token()
    }

    fn range(&'t self) -> Self::Range {
        self.range()
    }

    fn label<L: TryFromTokens<Token>>(&self) -> Result<L, L::Error>
    where
        Token: Clone,
    {
        self.label()
    }

    fn prefixes_of(&'t self) -> Self::Prefixes
    where
        Token: Clone + Ord,
    {
        self.prefixes_of()
    }

    fn suffixes_of(&'t self) -> Self::Suffixes
    where
        Token: Clone + Ord,
    {
        self.suffixes_of()
    }

    fn starts_with(&'t self) -> Self::Suffixes
    where
        Token: Clone + Ord,
    {
        self.starts_with()
    }
}
