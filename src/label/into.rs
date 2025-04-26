use super::Label;

/// Used to convert a token iterator into a `Label<Token>` type.
pub trait IntoLabel<Token>: Iterator<Item = Token> {
    /// Wraps the iterator so it can be used as a label.
    fn into_label(self) -> LabelIter<Token, Self>
    where
        Self: Sized;
}

impl<Token, T: Iterator<Item = Token> + ?Sized> IntoLabel<Token> for T {
    fn into_label(self) -> LabelIter<Token, Self>
    where
        Self: Sized,
    {
        LabelIter(self)
    }
}

/// Newtype for any token iterators.
pub struct LabelIter<Token, Iter: Iterator<Item = Token>>(Iter);

impl<Token, Iter: Iterator<Item = Token>> Label<Token> for LabelIter<Token, Iter> {
    type IntoTokens = Iter;

    fn into_tokens(self) -> Self::IntoTokens {
        self.0
    }
}

#[cfg(test)]
mod label_tests {
    use super::{IntoLabel, Label};

    #[test]
    fn generic_iter() {
        // let's turn a regular char iterator into a label
        let chars = "hello".chars();
        let label = chars.into_label();
        let _tokens = label.into_tokens();
    }
}
