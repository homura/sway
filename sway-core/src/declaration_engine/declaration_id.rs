use std::fmt;

use super::declaration_engine::DeclarationEngine;

/// An ID used to refer to an item in the [DeclarationEngine]
#[derive(Clone, Copy, Debug)]
pub struct DeclarationId<'de>(usize, &'de DeclarationEngine<'de>);

impl fmt::Display for DeclarationId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for DeclarationId<'_> {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<usize> for DeclarationId<'_> {
    fn into(self) -> usize {
        self.0
    }
}
