/// The role that a label holds in a trie.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LabelKind {
    /// There is a prefix here.
    Prefix,
    /// There is an exact match here.
    Match,
    /// There is a prefix and an exact match here.
    PrefixAndMatch,
}

impl LabelKind {
    /// Is this label a prefix?
    pub fn is_prefix(&self) -> bool {
        matches!(self, LabelKind::Prefix | LabelKind::PrefixAndMatch)
    }

    /// Is this label an exact match?
    pub fn is_match(&self) -> bool {
        matches!(self, LabelKind::Match | LabelKind::PrefixAndMatch)
    }

    pub(crate) fn new(is_prefix: bool, is_match: bool) -> Option<Self> {
        match (is_prefix, is_match) {
            (true, false) => Some(LabelKind::Prefix),
            (false, true) => Some(LabelKind::Match),
            (true, true) => Some(LabelKind::PrefixAndMatch),
            (false, false) => None,
        }
    }
}
