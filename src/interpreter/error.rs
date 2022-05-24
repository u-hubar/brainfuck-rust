use std::{error::Error, fmt::{Display, Formatter, Debug}, fmt::Result as FmtResult};

pub enum ParseError {
    InvalidOpenBracket,
    InvalidCloseBracket,
}

impl ParseError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidOpenBracket => "Misplaced open bracket '['",
            Self::InvalidCloseBracket => "Misplaced close bracket ']'",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
