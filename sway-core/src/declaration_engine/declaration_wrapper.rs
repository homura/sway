use crate::{
    semantic_analysis::{
        TypedImplTrait, TypedStructDeclaration, TypedTraitDeclaration, TypedTraitFn,
    },
    TypedFunctionDeclaration,
};

/// The [DeclarationWrapper] type is used in the [DeclarationEngine]
/// as a means of placing all declaration types into the same type.
#[derive(Clone, Debug)]
pub(crate) enum DeclarationWrapper<'de> {
    // no-op variant to fulfill the default trait
    Default,
    Function(TypedFunctionDeclaration<'de>),
    Trait(TypedTraitDeclaration),
    TraitFn(TypedTraitFn),
    TraitImpl(TypedImplTrait<'de>),
    Struct(TypedStructDeclaration),
}

impl Default for DeclarationWrapper<'_> {
    fn default() -> Self {
        DeclarationWrapper::Default
    }
}

impl DeclarationWrapper<'_> {
    pub(super) fn expect_function(
        self,
    ) -> Result<TypedFunctionDeclaration<'static>, DeclarationWrapper<'static>> {
        match self {
            DeclarationWrapper::Function(decl) => Ok(decl),
            actually => Err(actually),
        }
    }

    pub(super) fn expect_trait(self) -> Result<TypedTraitDeclaration, DeclarationWrapper<'static>> {
        match self {
            DeclarationWrapper::Trait(decl) => Ok(decl),
            actually => Err(actually),
        }
    }

    pub(super) fn expect_trait_fn(self) -> Result<TypedTraitFn, DeclarationWrapper<'static>> {
        match self {
            DeclarationWrapper::TraitFn(decl) => Ok(decl),
            actually => Err(actually),
        }
    }

    pub(super) fn expect_trait_impl(
        self,
    ) -> Result<TypedImplTrait<'static>, DeclarationWrapper<'static>> {
        match self {
            DeclarationWrapper::TraitImpl(decl) => Ok(decl),
            actually => Err(actually),
        }
    }

    pub(super) fn expect_struct(
        self,
    ) -> Result<TypedStructDeclaration, DeclarationWrapper<'static>> {
        match self {
            DeclarationWrapper::Struct(decl) => Ok(decl),
            actually => Err(actually),
        }
    }
}
