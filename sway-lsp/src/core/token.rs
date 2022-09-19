use dashmap::DashMap;
use sway_core::{
    semantic_analysis::ast_node::{
        expression::typed_expression::TypedExpression, TypeCheckedStorageReassignDescriptor,
        TypedAbiDeclaration, TypedConstantDeclaration, TypedEnumDeclaration, TypedEnumVariant,
        TypedFunctionDeclaration, TypedFunctionParameter, TypedImplTrait, TypedReassignment,
        TypedStorageField, TypedStructDeclaration, TypedStructField, TypedTraitDeclaration,
        TypedTraitFn, TypedVariableDeclaration,
    },
    type_system::TypeId,
    Declaration, EnumVariant, Expression, FunctionDeclaration, FunctionParameter,
    ReassignmentExpression, Scrutinee, StorageField, StructExpressionField, StructField, TraitFn,
};
use sway_types::{Ident, Span};

pub type TokenMap = DashMap<(Ident, Span), Token>;

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    TypeId(TypeId),
    Ident(Ident),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub parsed: AstToken,
    pub typed: Option<TypedAstToken>,
    pub type_def: Option<TypeDefinition>,
    pub kind: SymbolKind,
}

impl Token {
    pub fn from_parsed(token: AstToken, kind: SymbolKind) -> Self {
        Self {
            parsed: token,
            typed: None,
            type_def: None,
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstToken {
    Declaration(Declaration),
    Expression(Expression),
    StructExpressionField(StructExpressionField),
    FunctionDeclaration(FunctionDeclaration),
    FunctionParameter(FunctionParameter),
    StructField(StructField),
    EnumVariant(EnumVariant),
    TraitFn(TraitFn),
    Reassignment(ReassignmentExpression),
    StorageField(StorageField),
    Scrutinee(Scrutinee),
}

#[derive(Debug, Clone)]
pub enum TypedAstToken {
    TypedVariableDeclaration(TypedVariableDeclaration),
    TypedConstantDeclaration(TypedConstantDeclaration),
    TypedTraitDeclaration(TypedTraitDeclaration),
    TypedStructDeclaration(TypedStructDeclaration),
    TypedEnumDeclaration(TypedEnumDeclaration),
    TypedImplTrait(TypedImplTrait),
    TypedAbiDeclaration(TypedAbiDeclaration),
    TypedExpression(TypedExpression),
    TypedFunctionDeclaration(TypedFunctionDeclaration),
    TypedFunctionParameter(TypedFunctionParameter),
    TypedStructField(TypedStructField),
    TypedEnumVariant(TypedEnumVariant),
    TypedTraitFn(TypedTraitFn),
    TypedStorageField(TypedStorageField),
    TypeCheckedStorageReassignDescriptor(TypeCheckedStorageReassignDescriptor),
    TypedReassignment(TypedReassignment),
    GenericTypeForFunctionScope { name: Ident, type_id: TypeId },
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Field,
    ValueParam,
    Function,
    Const,
    Struct,
    Trait,
    Enum,
    Variant,
    BoolLiteral,
    ByteLiteral,
    StringLiteral,
    NumericLiteral,
    Variable,
    BuiltinType,
    Module,
    TypeParameter,
    Unknown,
}

// 1. LSP did_change event, receive [Range] of edits

// 2. Loop through our [TokenMap] and get mutable access to any tokens
// that have a [Span] starting *after* the range.start of the edit

// 3. update the Span's of each of the tokens to reflect the new edits

