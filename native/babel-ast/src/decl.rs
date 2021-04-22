use crate::util::SerializeUnion;
use serde::{Serialize, Deserialize};
use swc_common::ast_serde;

use crate::{
    class::ClassDeclaration,
    common::{BaseNode, LVal, Identifier, Param, TypeAnnotOrNoop, TypeParamDeclOrNoop},
    expr::{Expression, FunctionExpression},
    flow::{
        OpaqueType, DeclareOpaqueType, TypeAlias, DeclareClass, DeclareFunction, DeclareInterface,
        DeclareModule, DeclareModuleExports, DeclareTypeAlias, DeclareVariable,
        DeclareExportDeclaration, DeclareExportAllDeclaration, InterfaceDeclaration
    },
    lit::{BooleanLiteral, NumericLiteral, StringLiteral},
    module::{
        ExportAllDeclaration, ExportDefaultDeclaration, ExportNamedDeclaration, ImportDeclaration,
    },
    stmt::BlockStatement,
    typescript::{
        TSDeclareFunction, TSInterfaceDeclaration, TSTypeAliasDeclaration, TSEnumDeclaration,
        TSModuleDeclaration
    },
};

#[derive(Debug, Clone, SerializeUnion, Deserialize, PartialEq)]
#[serde(tag = "type")]
// #[serde(untagged)]
pub enum Declaration {
    #[serde(rename = "FunctionDeclaration")]
    FuncDecl(FunctionDeclaration),
    #[serde(rename = "VariableDeclaration")]
    VarDecl(VariableDeclaration),
    #[serde(rename = "ClassDeclaration")]
    ClassDecl(ClassDeclaration),
    #[serde(rename = "ExportAllDeclaration")]
    ExportAllDecl(ExportAllDeclaration),
    #[serde(rename = "ExportDefaultDeclaration")]
    ExportDefaultDecl(ExportDefaultDeclaration),
    #[serde(rename = "ExportNamedDeclaration")]
    ExportNamedDecl(ExportNamedDeclaration),
    #[serde(rename = "ImportDeclaration")]
    ImportDecl(ImportDeclaration),
    #[serde(rename = "DeclareClass")]
    DeclClass(DeclareClass),
    #[serde(rename = "DeclareFuncction")]
    DeclFunc(DeclareFunction),
    #[serde(rename = "DeclareInterface")]
    DeclInterface(DeclareInterface),
    #[serde(rename = "DeclareModule")]
    DeclModule(DeclareModule),
    #[serde(rename = "DeclareModuleExports")]
    DeclModuleExports(DeclareModuleExports),
    #[serde(rename = "DeclareTypeAlias")]
    DeclTypeAlias(DeclareTypeAlias),
    #[serde(rename = "DeclareOpaqueType")]
    DeclOpaqueType(DeclareOpaqueType),
    #[serde(rename = "DeclareVariable")]
    DeclVar(DeclareVariable),
    #[serde(rename = "DeclareExportDeclaration")]
    DeclExportDecl(DeclareExportDeclaration),
    #[serde(rename = "DeclareExportAllDeclaration")]
    DeclExportAllDecl(DeclareExportAllDeclaration),
    #[serde(rename = "InterfaceDeclaration")]
    InterfaceDecl(InterfaceDeclaration),
    OpaqueType(OpaqueType),
    TypeAlias(TypeAlias),
    #[serde(rename = "EnumDeclaration")]
    EnumDecl(EnumDeclaration),
    #[serde(rename = "TSDeclareFunction")]
    TSDeclFunc(TSDeclareFunction),
    #[serde(rename = "TSIterfaceDeclaration")]
    TSInterfaceDecl(TSInterfaceDeclaration),
    #[serde(rename = "TSTypeAliasDeclaration")]
    TSTypeAliasDecl(TSTypeAliasDeclaration),
    #[serde(rename = "TSEnumDeclaration")]
    TSEnumDecl(TSEnumDeclaration),
    #[serde(rename = "TSModuleDeclaration")]
    TSModuleDecl(TSModuleDeclaration),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: LVal,
    #[serde(default)]
    pub init: Option<Box<Expression>>,
    #[serde(default)]
    pub definite: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub base: BaseNode,
    pub kind: VariableDeclarationKind,
    #[serde(default)]
    pub declarations: Vec<VariableDeclarator>,
    #[serde(default)]
    pub declare: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub base: BaseNode,
    #[serde(default)]
    pub id: Option<Identifier>,
    #[serde(default)]
    pub params: Vec<Param>,
    pub body: BlockStatement,
    #[serde(default)]
    pub generator: Option<bool>,
    #[serde(default, rename = "async")]
    pub is_async: Option<bool>,
    #[serde(default)]
    pub return_type: Option<Box<TypeAnnotOrNoop>>,
    #[serde(default)]
    pub type_parameters: Option<TypeParamDeclOrNoop>,
}

impl From<FunctionExpression> for FunctionDeclaration {
    fn from(expr: FunctionExpression) -> Self {
        FunctionDeclaration {
            base: expr.base,
            id: expr.id,
            params: expr.params,
            body: expr.body,
            generator: expr.generator,
            is_async: expr.is_async,
            return_type: expr.return_type,
            type_parameters: expr.type_parameters,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct EnumBooleanMember {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Identifier,
    pub init: BooleanLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct EnumNumberMember {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Identifier,
    pub init: NumericLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct EnumStringMember {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Identifier,
    pub init: StringLiteral,
}

#[derive(Debug, Clone, SerializeUnion, Deserialize, PartialEq)]
#[serde(tag = "type")]
// #[serde(untagged)]
pub enum EnumStringBodyMember {
    #[serde(rename = "EnumStringBodyMember")]
    String(EnumStringMember),
    #[serde(rename = "EnumDefaultedMember")]
    Defaulted(EnumDefaultedMember),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct EnumDefaultedMember {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Identifier,
}

#[derive(Debug, Clone, PartialEq)]
#[ast_serde]
pub enum EnumMember {
    #[tag("EnumBooleanMember")]
    Boolean(EnumBooleanMember),
    #[tag("EnumNumberMember")]
    Number(EnumNumberMember),
    #[tag("EnumStringMember")]
    String(EnumStringMember),
    #[tag("EnumDefaultedMember")]
    Defaulted(EnumDefaultedMember),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct EnumBooleanBody {
    #[serde(flatten)]
    pub base: BaseNode,
    #[serde(default)]
    pub members: Vec<EnumBooleanMember>,
    #[serde(default)]
    pub explicit_type: bool,
    #[serde(default)]
    pub has_unknown_members: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct EnumNumberBody {
    #[serde(flatten)]
    pub base: BaseNode,
    #[serde(default)]
    pub members: Vec<EnumNumberMember>,
    #[serde(default)]
    pub explicit_type: bool,
    #[serde(default)]
    pub has_unknown_members: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct EnumStringBody {
    #[serde(flatten)]
    pub base: BaseNode,
    #[serde(default)]
    pub members: Vec<EnumStringBodyMember>,
    #[serde(default)]
    pub explicit_type: bool,
    #[serde(default)]
    pub has_unknown_members: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct EnumSymbolBody {
    #[serde(flatten)]
    pub base: BaseNode,
    #[serde(default)]
    pub members: Vec<EnumDefaultedMember>,
    #[serde(default)]
    pub has_unknown_members: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[ast_serde]
pub enum EnumBody {
    #[tag("EnumBooleanBody")]
    Boolean(EnumBooleanBody),
    #[tag("EnumNumberBody")]
    Number(EnumNumberBody),
    #[tag("EnumStringBody")]
    String(EnumStringBody),
    #[tag("EnumSymbolBody")]
    Symbol(EnumSymbolBody),
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub struct EnumDeclaration {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Identifier,
    pub body: EnumBody,
}

