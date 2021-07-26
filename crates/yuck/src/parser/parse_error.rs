use eww_shared_util::{AttrName, Span, VarName};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("{0}")]
    SimplExpr(simplexpr::error::Error),
    #[error("Unknown token")]
    LexicalError(Span),
}

impl ParseError {
    pub fn span(&self) -> Span {
        match self {
            ParseError::SimplExpr(err) => err.span(),
            ParseError::LexicalError(span) => *span,
        }
    }
}
