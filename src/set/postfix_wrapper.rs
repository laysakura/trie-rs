use crate::search::PostfixIter;


struct SetPostfixIter<'t, Token>(pub(crate) PostfixIter<'t, Token, ()>);


