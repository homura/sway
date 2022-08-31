use crate::Ident;
use crate::{semantic_analysis::*, type_system::*};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedStructExpressionField<'de> {
    pub name: Ident,
    pub value: TypedExpression<'de>,
}

impl CopyTypes for TypedStructExpressionField<'_> {
    fn copy_types(&mut self, type_mapping: &TypeMapping) {
        self.value.copy_types(type_mapping);
    }
}
