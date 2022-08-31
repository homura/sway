use super::{CopyTypes, TypeMapping, TypedExpression};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedReturnStatement<'de> {
    pub expr: TypedExpression<'de>,
}

impl CopyTypes for TypedReturnStatement<'_> {
    /// Makes a fresh copy of all types contained in this statement.
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.expr.copy_types(type_mapping);
    }
}
