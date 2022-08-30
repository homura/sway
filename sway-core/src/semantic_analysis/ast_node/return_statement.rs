use crate::{
    declaration_engine::declaration_engine::DeclarationEngine,
    types::{CompileWrapper, ToCompileWrapper},
};

use super::{CopyTypes, TypeMapping, TypedExpression};

#[derive(Clone, Debug)]
pub struct TypedReturnStatement {
    pub expr: TypedExpression,
}

impl PartialEq for CompileWrapper<'_, TypedReturnStatement> {
    fn eq(&self, other: &Self) -> bool {
        let CompileWrapper {
            inner: me,
            declaration_engine: de,
        } = self;
        let CompileWrapper {
            inner: them,
            declaration_engine: _,
        } = other;
        me.expr.wrap_ref(de) == them.expr.wrap_ref(de)
    }
}

impl CopyTypes for TypedReturnStatement {
    /// Makes a fresh copy of all types contained in this statement.
    fn copy_types(&mut self, type_mapping: &TypeMapping, de: &DeclarationEngine) {
        self.expr.copy_types(type_mapping, de);
    }
}
