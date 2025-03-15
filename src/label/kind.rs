/// The role that a label holds in a trie.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LabelKind {
    /// There is a prefix here.
    Prefix,
    /// There is an exact match here.
    Exact,
    /// There is a prefix and an exact match here.
    PrefixAndExact,
}

impl LabelKind {
    /// Is this label a prefix?
    pub fn is_prefix(&self) -> bool {
        matches!(self, LabelKind::Prefix | LabelKind::PrefixAndExact)
    }

    /// Is this label an exact match?
    pub fn is_exact(&self) -> bool {
        matches!(self, LabelKind::Exact | LabelKind::PrefixAndExact)
    }
}
