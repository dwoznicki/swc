use std::collections::HashMap;

use ahash::RandomState;
use serde::{Serialize, Deserialize};
use serde_json::Value;

// Source: https://github.com/babel/babel/blob/main/packages/babel-types/src/ast-types/generated/index.ts

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loc {
    pub start: LineCol,
    pub end: LineCol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseComment {
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Comment {
    #[serde(rename = "CommentBlock")]
    Block(BaseComment),
    #[serde(rename = "CommentLine")]
    Line(BaseComment),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseNode {
    #[serde(default)]
    pub leading_comments: Vec<Comment>,
    #[serde(default)]
    pub inner_comments: Vec<Comment>,
    #[serde(default)]
    pub trailing_comments: Vec<Comment>,

    #[serde(default)]
    pub start: Option<usize>,
    #[serde(default)]
    pub end: Option<usize>,
    #[serde(default)]
    pub loc: Option<Loc>,

    #[serde(default)]
    pub extra: Option<HashMap<String, Value, RandomState>>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum CommentTypeShorthand {
//     Leading,
//     Inner,
//     Trailing,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {

    // TODO

    // AnyTypeAnnotation(AnyTypeAnnotation),
    // ArgumentPlaceholder(ArgumentPlaceholder),
    // ArrayExpression(ArrayExpression),
    // ArrayPattern(ArrayPattern),
    // ArrayTypeAnnotation(ArrayTypeAnnotation),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // AssignmentExpression(AssignmentExpression),
    // AssignmentPattern(AssignmentPattern),
    // AwaitExpression(AwaitExpression),
    // BigIntLiteral(BigIntLiteral),
    // Binary(Binary),
    // BinaryExpression(BinaryExpression),
    // BindExpression(BindExpression),
    // Block(Block),
    // BlockParent(BlockParent),
    // BlockStatement(BlockStatement),
    // BooleanLiteral(BooleanLiteral),
    // BooleanLiteralTypeAnnotation(BooleanLiteralTypeAnnotation),
    // BooleanTypeAnnotation(BooleanTypeAnnotation),
    // BreakStatement(BreakStatement),
    // CallExpression(CallExpression),
    // CatchClause(CatchClause),
    // Class(Class),
    // ClassBody(ClassBody),
    // ClassDeclaration(ClassDeclaration),
    // ClassExpression(ClassExpression),
    // ClassImplements(ClassImplements),
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),
    // ClassPrivateProperty(ClassPrivateProperty),
    // ClassProperty(ClassProperty),
    // CompletionStatement(CompletionStatement),
    // Conditional(Conditional),
    // ConditionalExpression(ConditionalExpression),
    // ContinueStatement(ContinueStatement),
    // DebuggerStatement(DebuggerStatement),
    // DecimalLiteral(DecimalLiteral),
    // Declaration(Declaration),
    // DeclareClass(DeclareClass),
    // DeclareExportAllDeclaration(DeclareExportAllDeclaration),
    // DeclareExportDeclaration(DeclareExportDeclaration),
    // DeclareFunction(DeclareFunction),
    // DeclareInterface(DeclareInterface),
    // DeclareModule(DeclareModule),
    // DeclareModuleExports(DeclareModuleExports),
    // DeclareOpaqueType(DeclareOpaqueType),
    // DeclareTypeAlias(DeclareTypeAlias),
    // DeclareVariable(DeclareVariable),
    // DeclaredPredicate(DeclaredPredicate),
    // Decorator(Decorator),
    // Directive(Directive),
    // DirectiveLiteral(DirectiveLiteral),
    // DoExpression(DoExpression),
    // DoWhileStatement(DoWhileStatement),
    // EmptyStatement(EmptyStatement),
    // EmptyTypeAnnotation(EmptyTypeAnnotation),
    // EnumBody(EnumBody),
    // EnumBooleanBody(EnumBooleanBody),
    // EnumBooleanMember(EnumBooleanMember),
    // EnumDeclaration(EnumDeclaration),
    // EnumDefaultedMember(EnumDefaultedMember),
    // EnumMember(EnumMember),
    // EnumNumberBody(EnumNumberBody),
    // EnumNumberMember(EnumNumberMember),
    // EnumStringBody(EnumStringBody),
    // EnumStringMember(EnumStringMember),
    // EnumSymbolBody(EnumSymbolBody),
    // ExistsTypeAnnotation(ExistsTypeAnnotation),
    // ExportAllDeclaration(ExportAllDeclaration),
    // ExportDeclaration(ExportDeclaration),
    // ExportDefaultDeclaration(ExportDefaultDeclaration),
    // ExportDefaultSpecifier(ExportDefaultSpecifier),
    // ExportNamedDeclaration(ExportNamedDeclaration),
    // ExportNamespaceSpecifier(ExportNamespaceSpecifier),
    // ExportSpecifier(ExportSpecifier),
    // Expression(Expression),
    // ExpressionStatement(ExpressionStatement),
    // ExpressionWrapper(ExpressionWrapper),
    // File(File),
    // Flow(Flow),
    // FlowBaseAnnotation(FlowBaseAnnotation),
    // FlowDeclaration(FlowDeclaration),
    // FlowPredicate(FlowPredicate),
    // FlowType(FlowType),
    // For(For),
    // ForInStatement(ForInStatement),
    // ForOfStatement(ForOfStatement),
    // ForStatement(ForStatement),
    // ForXStatement(ForXStatement),
    // Function(Function),
    // FunctionDeclaration(FunctionDeclaration),
    // FunctionExpression(FunctionExpression),
    // FunctionParent(FunctionParent),
    // FunctionTypeAnnotation(FunctionTypeAnnotation),
    // FunctionTypeParam(FunctionTypeParam),
    // GenericTypeAnnotation(GenericTypeAnnotation),
    // Identifier(Identifier),
    // IfStatement(IfStatement),
    // Immutable(Immutable),
    // Import(Import),
    // ImportAttribute(ImportAttribute),
    // ImportDeclaration(ImportDeclaration),
    // ImportDefaultSpecifier(ImportDefaultSpecifier),
    // ImportNamespaceSpecifier(ImportNamespaceSpecifier),
    // ImportSpecifier(ImportSpecifier),
    // InferredPredicate(InferredPredicate),
    // InterfaceDeclaration(InterfaceDeclaration),
    // InterfaceExtends(InterfaceExtends),
    // InterfaceTypeAnnotation(InterfaceTypeAnnotation),
    // InterpreterDirective(InterpreterDirective),
    // IntersectionTypeAnnotation(IntersectionTypeAnnotation),
    // JSX(JSX),
    // JSXAttribute(JSXAttribute),
    // JSXClosingElement(JSXClosingElement),
    // JSXClosingFragment(JSXClosingFragment),
    // JSXElement(JSXElement),
    // JSXEmptyExpression(JSXEmptyExpression),
    // JSXExpressionContainer(JSXExpressionContainer),
    // JSXFragment(JSXFragment),
    // JSXIdentifier(JSXIdentifier),
    // JSXMemberExpression(JSXMemberExpression),
    // JSXNamespacedName(JSXNamespacedName),
    // JSXOpeningElement(JSXOpeningElement),
    // JSXOpeningFragment(JSXOpeningFragment),
    // JSXSpreadAttribute(JSXSpreadAttribute),
    // JSXSpreadChild(JSXSpreadChild),
    // JSXText(JSXText),
    // LVal(LVal),
    // LabeledStatement(LabeledStatement),
    // Literal(Literal),
    // LogicalExpression(LogicalExpression),
    // Loop(Loop),
    // MemberExpression(MemberExpression),
    // MetaProperty(MetaProperty),
    // Method(Method),
    // MixedTypeAnnotation(MixedTypeAnnotation),
    // ModuleDeclaration(ModuleDeclaration),
    // ModuleExpression(ModuleExpression),
    // ModuleSpecifier(ModuleSpecifier),
    // NewExpression(NewExpression),
    // Noop(Noop),
    // NullLiteral(NullLiteral),
    // NullLiteralTypeAnnotation(NullLiteralTypeAnnotation),
    // NullableTypeAnnotation(NullableTypeAnnotation),
    // NumberLiteral(NumberLiteral),
    // NumberLiteralTypeAnnotation(NumberLiteralTypeAnnotation),
    // NumberTypeAnnotation(NumberTypeAnnotation),
    // NumericLiteral(NumericLiteral),
    // ObjectExpression(ObjectExpression),
    // ObjectMember(ObjectMember),
    // ObjectMethod(ObjectMethod),
    // ObjectPattern(ObjectPattern),
    // ObjectProperty(ObjectProperty),
    // ObjectTypeAnnotation(ObjectTypeAnnotation),
    // ObjectTypeCallProperty(ObjectTypeCallProperty),
    // ObjectTypeIndexer(ObjectTypeIndexer),
    // ObjectTypeInternalSlot(ObjectTypeInternalSlot),
    // ObjectTypeProperty(ObjectTypeProperty),
    // ObjectTypeSpreadProperty(ObjectTypeSpreadProperty),
    // OpaqueType(OpaqueType),
    // OptionalCallExpression(OptionalCallExpression),
    // OptionalMemberExpression(OptionalMemberExpression),
    // ParenthesizedExpression(ParenthesizedExpression),
    // Pattern(Pattern),
    // PatternLike(PatternLike),
    // PipelineBareFunction(PipelineBareFunction),
    // PipelinePrimaryTopicReference(PipelinePrimaryTopicReference),
    // PipelineTopicExpression(PipelineTopicExpression),
    // Placeholder(Placeholder),
    // Private(Private),
    // PrivateName(PrivateName),
    // Program(Program),
    // Property(Property),
    // Pureish(Pureish),
    // QualifiedTypeIdentifier(QualifiedTypeIdentifier),
    // RecordExpression(RecordExpression),
    // RegExpLiteral(RegExpLiteral),
    // RegexLiteral(RegexLiteral),
    // RestElement(RestElement),
    // RestProperty(RestProperty),
    // ReturnStatement(ReturnStatement),
    // Scopable(Scopable),
    // SequenceExpression(SequenceExpression),
    // SpreadElement(SpreadElement),
    // SpreadProperty(SpreadProperty),
    // Statement(Statement),
    // StaticBlock(StaticBlock),
    // StringLiteral(StringLiteral),
    // StringLiteralTypeAnnotation(StringLiteralTypeAnnotation),
    // StringTypeAnnotation(StringTypeAnnotation),
    // Super(Super),
    // SwitchCase(SwitchCase),
    // SwitchStatement(SwitchStatement),
    // SymbolTypeAnnotation(SymbolTypeAnnotation),
    // TSAnyKeyword(TSAnyKeyword),
    // TSArrayType(TSArrayType),
    // TSAsExpression(TSAsExpression),
    // TSBaseType(TSBaseType),
    // TSBigIntKeyword(TSBigIntKeyword),
    // TSBooleanKeyword(TSBooleanKeyword),
    // TSCallSignatureDeclaration(TSCallSignatureDeclaration),
    // TSConditionalType(TSConditionalType),
    // TSConstructSignatureDeclaration(TSConstructSignatureDeclaration),
    // TSConstructorType(TSConstructorType),
    // TSDeclareFunction(TSDeclareFunction),
    // TSDeclareMethod(TSDeclareMethod),
    // TSEntityName(TSEntityName),
    // TSEnumDeclaration(TSEnumDeclaration),
    // TSEnumMember(TSEnumMember),
    // TSExportAssignment(TSExportAssignment),
    // TSExpressionWithTypeArguments(TSExpressionWithTypeArguments),
    // TSExternalModuleReference(TSExternalModuleReference),
    // TSFunctionType(TSFunctionType),
    // TSImportEqualsDeclaration(TSImportEqualsDeclaration),
    // TSImportType(TSImportType),
    // TSIndexSignature(TSIndexSignature),
    // TSIndexedAccessType(TSIndexedAccessType),
    // TSInferType(TSInferType),
    // TSInterfaceBody(TSInterfaceBody),
    // TSInterfaceDeclaration(TSInterfaceDeclaration),
    // TSIntersectionType(TSIntersectionType),
    // TSIntrinsicKeyword(TSIntrinsicKeyword),
    // TSLiteralType(TSLiteralType),
    // TSMappedType(TSMappedType),
    // TSMethodSignature(TSMethodSignature),
    // TSModuleBlock(TSModuleBlock),
    // TSModuleDeclaration(TSModuleDeclaration),
    // TSNamedTupleMember(TSNamedTupleMember),
    // TSNamespaceExportDeclaration(TSNamespaceExportDeclaration),
    // TSNeverKeyword(TSNeverKeyword),
    // TSNonNullExpression(TSNonNullExpression),
    // TSNullKeyword(TSNullKeyword),
    // TSNumberKeyword(TSNumberKeyword),
    // TSObjectKeyword(TSObjectKeyword),
    // TSOptionalType(TSOptionalType),
    // TSParameterProperty(TSParameterProperty),
    // TSParenthesizedType(TSParenthesizedType),
    // TSPropertySignature(TSPropertySignature),
    // TSQualifiedName(TSQualifiedName),
    // TSRestType(TSRestType),
    // TSStringKeyword(TSStringKeyword),
    // TSSymbolKeyword(TSSymbolKeyword),
    // TSThisType(TSThisType),
    // TSTupleType(TSTupleType),
    // TSType(TSType),
    // TSTypeAliasDeclaration(TSTypeAliasDeclaration),
    // TSTypeAnnotation(TSTypeAnnotation),
    // TSTypeAssertion(TSTypeAssertion),
    // TSTypeElement(TSTypeElement),
    // TSTypeLiteral(TSTypeLiteral),
    // TSTypeOperator(TSTypeOperator),
    // TSTypeParameter(TSTypeParameter),
    // TSTypeParameterDeclaration(TSTypeParameterDeclaration),
    // TSTypeParameterInstantiation(TSTypeParameterInstantiation),
    // TSTypePredicate(TSTypePredicate),
    // TSTypeQuery(TSTypeQuery),
    // TSTypeReference(TSTypeReference),
    // TSUndefinedKeyword(TSUndefinedKeyword),
    // TSUnionType(TSUnionType),
    // TSUnknownKeyword(TSUnknownKeyword),
    // TSVoidKeyword(TSVoidKeyword),
    // TaggedTemplateExpression(TaggedTemplateExpression),
    // TemplateElement(TemplateElement),
    // TemplateLiteral(TemplateLiteral),
    // Terminatorless(Terminatorless),
    // ThisExpression(ThisExpression),
    // ThisTypeAnnotation(ThisTypeAnnotation),
    // ThrowStatement(ThrowStatement),
    // TryStatement(TryStatement),
    // TupleExpression(TupleExpression),
    // TupleTypeAnnotation(TupleTypeAnnotation),
    // TypeAlias(TypeAlias),
    // TypeAnnotation(TypeAnnotation),
    // TypeCastExpression(TypeCastExpression),
    // TypeParameter(TypeParameter),
    // TypeParameterDeclaration(TypeParameterDeclaration),
    // TypeParameterInstantiation(TypeParameterInstantiation),
    // TypeofTypeAnnotation(TypeofTypeAnnotation),
    // UnaryExpression(UnaryExpression),
    // UnaryLike(UnaryLike),
    // UnionTypeAnnotation(UnionTypeAnnotation),
    // UpdateExpression(UpdateExpression),
    // UserWhitespacable(UserWhitespacable),
    // V8IntrinsicIdentifier(V8IntrinsicIdentifier),
    // VariableDeclaration(VariableDeclaration),
    // VariableDeclarator(VariableDeclarator),
    // Variance(Variance),
    // VoidTypeAnnotation(VoidTypeAnnotation),
    // While(While),
    // WhileStatement(WhileStatement),
    // WithStatement(WithStatement),
    // YieldExpression(YieldExpression),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrayEl {
    // TODO
    // Expression(Expression),
    // SpreadElement(SpreadElement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ArrayExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub elements: Vec<Option<ArrayEl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AssignmentExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub operator: String,
    // TODO
    // pub left: LVal,
    // pub right: Expression,
}

// Source:
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    #[serde(rename = "+")]
    Addition,
    #[serde(rename = "-")]
    Subtraction,
    #[serde(rename = "/")]
    Division,
    #[serde(rename = "%")]
    Remainder,
    #[serde(rename = "*")]
    Multiplication,
    #[serde(rename = "**")]
    Exponentiation,
    #[serde(rename = "&")]
    And,
    #[serde(rename = "|")]
    Or,
    #[serde(rename = ">>")]
    RightShift,
    #[serde(rename = ">>>")]
    UnsignedRightShift,
    #[serde(rename = "<<")]
    LeftShift,
    #[serde(rename = "^")]
    Xor,
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "===")]
    StrictEqual,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = "!==")]
    StrictNotEqual,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "instanceof")]
    Instanceof,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<=")]
    LessThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindaryLeft {
    Expression(Expression),
    // TODO
    // PrivateName(PrivateName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct BinaryExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub operator: BinaryOp,
    pub left: BindaryLeft,
    pub right: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct InterpreterDirective {
    #[serde(flatten)]
    pub base: BaseNode,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Directive {
    #[serde(flatten)]
    pub base: BaseNode,
    pub value: DirectiveLiteral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DirectiveLiteral {
    #[serde(flatten)]
    pub base: BaseNode,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct BlockStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    // TODO
    // pub body: Vec<Statement>,
    pub directives: Vec<Directive>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct BreakStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    // TODO
    // pub label: Option<Identifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Callee {
    Expression(Expression),
    // TODO
    // V8IntrinsicIdentifier(V8IntrinsicIdentifier),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Arg {
    Expression(Expression),
    // TODO
    // SpreadElement(SpreadElement),
    // JSXNamespacedName(JSXNamespacedName),
    // ArgumentPlaceholder(ArgumentPlaceholder),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct CallExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub callee: Callee,
    pub arguments: Vec<Arg>,
    pub optional: Option<bool>,
    // TODO
    // pub type_arguments: Option<TypeParameterInstantiation>,
    // pub type_parameters: Option<TSTypeParameterInstantiation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatchParam {
    // TODO
    // Identifier(Identifier),
    // ArrayPattern(ArrayPattern),
    // ObjectPattern(ObjectPattern),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct CatchClause {
    #[serde(flatten)]
    pub base: BaseNode,
    pub param: Option<CatchParam>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ConditionalExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub test: Expression,
    pub consequent: Expression,
    pub altername: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ContinueStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    // TODO
    // pub label: Option<Identifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub base: BaseNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct DoWhileStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub test: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub base: BaseNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ExpressionStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub expression: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct File {
    #[serde(flatten)]
    pub base: BaseNode,
    // TODO
    // pub program: Program,
    pub comments: Option<Vec<Comment>>,
    // pub tokens: Option<Vec<_>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForInLeft {
    // TODO
    // VariableDeclaration(VariableDeclaration),
    LVal(LVal),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ForInStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub left: ForInLeft,
    pub right: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForInit {
    // TODO
    // VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ForStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub init: Option<ForInit>,
    pub test: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeAnnotation {

    // TODO
    // FlowTypeAnnotation(FlowTypeAnnotation),
    // TSTypeAnnotation(TSTypeAnnotation),
    // Noop(Noop),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Param {
    Identifier(Identifier),
    Pattern(Pattern),
    RestElement(RestElement),
    // TODO
    // TSParameterTypeProperty(TSParameterTypeProperty),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeParameterDeclaration {

    // TODO
    // FlowTypeParameterDeclaration(FlowTypeParameterDeclaration),
    // TSTypeParameterDeclaration(TSTypeParameterDeclaration),
    // Noop(Noop),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct FunctionDeclaration {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Option<Identifier>,
    pub params: Vec<Param>,
    pub body: BlockStatement,
    pub generator: Option<bool>,
    #[serde(rename = "async")]
    pub is_async: Option<bool>,
    pub return_type: Option<TypeAnnotation>,
    pub type_parameters: Option<TypeParameterDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct Identifier {
    #[serde(flatten)]
    pub base: BaseNode,
    pub name: String,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
    pub optional: Option<bool>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct IfStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub test: Expression,
    pub consequent: Statement,
    pub alternate: Option<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct LabeledStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub label: Identifier,
    pub body: Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct StringLiteral {
    #[serde(flatten)]
    pub base: BaseNode,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct NumericLiteral {
    #[serde(flatten)]
    pub base: BaseNode,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct NullLiteral {
    #[serde(flatten)]
    pub base: BaseNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct BooleanLiteral {
    #[serde(flatten)]
    pub base: BaseNode,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct RegExpLiteral {
    #[serde(flatten)]
    pub base: BaseNode,
    pub pattern: String,
    pub flags: String,
}

// Source: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOp {
    #[serde(rename = "||")]
    Or,
    #[serde(rename = "&&")]
    And,
    #[serde(rename = "??")]
    Nullish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct LogicalExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub operator: LogicalOp,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberProp {
    Expression(Expression),
    Identifier(Identifier),
    // TODO
    // PrivateName(PrivateName),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct MemberExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub object: Expression,
    pub property: MemberProp,
    pub computed: bool,
    pub optional: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct NewExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub callee: Callee,
    pub arguments: Vec<Arg>,
    pub optional: Option<bool>,
    // TODO
    // pub type_arguments: Option<FlowTypeParameterInstantiation>,
    // pub type_parameters: Option<TSTypeParameterInstantiation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Script,
    Module,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct Program {
    #[serde(flatten)]
    pub base: BaseNode,
    pub body: Vec<Statement>,
    pub directives: Vec<Directive>,
    pub source_type: SourceType,
    pub interpreter: Option<InterpreterDirective>,
    pub source_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectExpressionProp {
    // TODO
    // ObjectMethod(ObjectMethod),
    // ObjectProperty(ObjectProperty),
    // SpreadElement(SpreadElement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct ObjectExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub properties: Vec<ObjectExpressionProp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectKind {
    Method,
    Get,
    Set,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectKey {
    Expression(Expression),
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumericLiteral(NumericLiteral),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct ObjectMethod {
    #[serde(flatten)]
    pub base: BaseNode,
    pub kind: ObjectKind,
    pub key: ObjectKey,
    pub params: Vec<Param>,
    pub body: BlockStatement,
    pub computed: bool,
    pub generator: Option<bool>,
    #[serde(rename = "async")]
    pub is_async: Option<bool>,
    // TODO
    // pub decorator: Option<Vec<Decorator>>,
    pub return_type: Option<TypeAnnotation>,
    pub type_parameters: Option<TypeParameterDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectPropertyVal {
    Expression(Expression),
    PatternLike(PatternLike),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ObjectProperty {
    #[serde(flatten)]
    pub base: BaseNode,
    pub key: ObjectKey,
    pub value: ObjectPropertyVal,
    pub computed: bool,
    pub shorthand: bool,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub argument: LVal,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ReturnStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub argument: Option<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SequenceExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ParenthesizedExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub expression: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SwitchCase {
    #[serde(flatten)]
    pub base: BaseNode,
    pub test: Option<Expression>,
    pub consequent: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SwitchStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub descriminant: Expression,
    pub cases: Vec<SwitchCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ThisExpression {
    #[serde(flatten)]
    pub base: BaseNode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ThrowStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub argument: Expression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct TryStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub block: BlockStatement,
    pub handler: Option<CatchClause>,
    pub finalizer: Option<BlockStatement>,
}

// Source: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UnaryOp {
    Void,
    Throw,
    Delete,
    #[serde(rename = "!")]
    LogicalNot,
    #[serde(rename = "+")]
    Plus,
    #[serde(rename = "-")]
    Negation,
    #[serde(rename = "~")]
    BitwiseNot,
    Typeof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct UnaryExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub operator: UnaryOp,
    pub argument: Expression,
    pub prefix: bool,
}

// Source: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Expressions_and_Operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateOp {
    #[serde(rename = "++")]
    Increment,
    #[serde(rename = "--")]
    Decrement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct UpdateExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub operator: UpdateOp,
    pub argument: Expression,
    pub prefix: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub base: BaseNode,
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<VariableDeclarator>,
    pub declare: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: LVal,
    pub init: Option<Expression>,
    pub definite: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct WhileStatement {
    #[serde(flatten)]
    pub base: BaseNode,
    pub test: Expression,
    pub body: Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentPatternLeft {
    Identifier(Identifier),
    // TODO
    // ObjectPattern(ObjectPattern),
    // ArrayPattern(ArrayPattern),
    MemberExpression(MemberExpression),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub base: BaseNode,
    pub left: AssignmentPatternLeft,
    pub right: Expression,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub base: BaseNode,
    pub elements: Vec<Option<PatternLike>>,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
    pub type_annotation: Option<TypeAnnotation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowFunctionBody {
    BlockStatement(BlockStatement),
    Expression(Expression),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct ArrowFunctionExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub params: Vec<Param>,
    pub body: ArrowFunctionBody,
    #[serde(rename = "async")]
    pub is_async: bool,
    pub expression: bool,
    pub generator: bool,
    pub return_type: TypeAnnotation,
    pub type_parameters: Option<TypeParameterDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassBodyEl {
    // TODO
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),
    // ClassProperty(ClassProperty),
    // ClassPrivateProperty(ClassPrivateProperty),
    // TSDeclareMethod(TSDeclareMethod),
    // TSIndexSignature(TSIndexSignature),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct ClassBody {
    #[serde(flatten)]
    pub base: BaseNode,
    pub body: Vec<ClassBodyEl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClassImpl {
    // TODO
    // TSExpressionWithTypeArguments(TSExpressionWithTypeArguments),
    // ClassImplements(ClassImplements),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeParameterInstantiation {
    // TODO
    // FlowTypeParameterInstantiation(FlowTypeParameterInstantiation),
    // TSTypeParameterInstantiation(TSTypeParameterInstantiation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct ClassExpression {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Option<Identifier>,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
    pub implements: Option<ClassImpl>,
    // pub mixins: Option<InterfaceExtends>,
    pub super_type_parameters: Option<TypeParameterInstantiation>,
    pub type_parameters: Option<TypeParameterDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub struct ClassDeclaration {
    #[serde(flatten)]
    pub base: BaseNode,
    pub id: Identifier,
    pub super_class: Option<Expression>,
    pub body: ClassBody,
    // TODO
    // pub decorators: Option<Vec<Decorator>>,
    #[serde(rename = "abstract")]
    pub is_abstract: Option<bool>,
    pub declare: Option<bool>,
    pub implements: Option<ClassImpl>,
    // pub mixins: Option<InterfaceExtends>,
    pub super_type_parameters: Option<TypeParameterInstantiation>,
    pub type_parameters: Option<TypeParameterDeclaration>,
}







#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {

    // TODO
    // ArrayExpression(ArrayExpression),
    // AssignmentExpression(AssignmentExpression),
    // BinaryExpression(BinaryExpression),
    // CallExpression(CallExpression),
    // ConditionalExpression(ConditionalExpression),
    // FunctionExpression(FunctionExpression),
    // Identifier(Identifier),
    // StringLiteral(StringLiteral),
    // NumericLiteral(NumericLiteral),
    // NullLiteral(NullLiteral),
    // BooleanLiteral(BooleanLiteral),
    // RegExpLiteral(RegExpLiteral),
    // LogicalExpression(LogicalExpression),
    // MemberExpression(MemberExpression),
    // NewExpression(NewExpression),
    // ObjectExpression(ObjectExpression),
    // SequenceExpression(SequenceExpression),
    // ParenthesizedExpression(ParenthesizedExpression),
    // ThisExpression(ThisExpression),
    // UnaryExpression(UnaryExpression),
    // UpdateExpression(UpdateExpression),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // ClassExpression(ClassExpression),
    // MetaProperty(MetaProperty),
    // Super(Super),
    // TaggedTemplateExpression(TaggedTemplateExpression),
    // TemplateLiteral(TemplateLiteral),
    // YieldExpression(YieldExpression),
    // AwaitExpression(AwaitExpression),
    // Import(Import),
    // BigIntLiteral(BigIntLiteral),
    // OptionalMemberExpression(OptionalMemberExpression),
    // OptionalCallExpression(OptionalCallExpression),
    // TypeCastExpression(TypeCastExpression),
    // JSXElement(JSXElement),
    // JSXFragment(JSXFragment),
    // BindExpression(BindExpression),
    // PipelinePrimaryTopicReference(PipelinePrimaryTopicReference),
    // DoExpression(DoExpression),
    // RecordExpression(RecordExpression),
    // TupleExpression(TupleExpression),
    // DecimalLiteral(DecimalLiteral),
    // ModuleExpression(ModuleExpression),
    // TSAsExpression(TSAsExpression),
    // TSTypeAssertion(TSTypeAssertion),
    // TSNonNullExpression(TSNonNullExpression),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Binary {

    // TODO
    // BinaryExpression(BinaryExpression),
    // LogicalExpression(LogicalExpression),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scopable {

    // TODO
    // BlockStatement(BlockStatement),
    // CatchClause(CatchClause),
    // DoWhileStatement(DoWhileStatement),
    // ForInStatement(ForInStatement),
    // ForStatement(ForStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // FunctionExpression(FunctionExpression),
    // Program(Program),
    // ObjectMethod(ObjectMethod),
    // SwitchStatement(SwitchStatement),
    // WhileStatement(WhileStatement),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // ClassExpression(ClassExpression),
    // ClassDeclaration(ClassDeclaration),
    // ForOfStatement(ForOfStatement),
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),
    // StaticBlock(StaticBlock),
    // TSModuleBlock(TSModuleBlock),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockParent  {

    // TODO
    // BlockStatement(BlockStatement),
    // CatchClause(CatchClause),
    // DoWhileStatement(DoWhileStatement),
    // ForInStatement(ForInStatement),
    // ForStatement(ForStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // FunctionExpression(FunctionExpression),
    // Program(Program),
    // ObjectMethod(ObjectMethod),
    // SwitchStatement(SwitchStatement),
    // WhileStatement(WhileStatement),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // ForOfStatement(ForOfStatement),
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),
    // StaticBlock(StaticBlock),
    // TSModuleBlock(TSModuleBlock),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Block {

    // TODO
    // BlockStatement(BlockStatement),
    // Program(Program),
    // TSModuleBlock(TSModuleBlock),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {

    // TODO
    // BlockStatement(BlockStatement),
    // BreakStatement(BreakStatement),
    // ContinueStatement(ContinueStatement),
    // DebuggerStatement(DebuggerStatement),
    // DoWhileStatement(DoWhileStatement),
    // EmptyStatement(EmptyStatement),
    // ExpressionStatement(ExpressionStatement),
    // ForInStatement(ForInStatement),
    // ForStatement(ForStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // IfStatement(IfStatement),
    // LabeledStatement(LabeledStatement),
    // ReturnStatement(ReturnStatement),
    // SwitchStatement(SwitchStatement),
    // ThrowStatement(ThrowStatement),
    // TryStatement(TryStatement),
    // VariableDeclaration(VariableDeclaration),
    // WhileStatement(WhileStatement),
    // WithStatement(WithStatement),
    // ClassDeclaration(ClassDeclaration),
    // ExportAllDeclaration(ExportAllDeclaration),
    // ExportDefaultDeclaration(ExportDefaultDeclaration),
    // ExportNamedDeclaration(ExportNamedDeclaration),
    // ForOfStatement(ForOfStatement),
    // ImportDeclaration(ImportDeclaration),
    // DeclareClass(DeclareClass),
    // DeclareFunction(DeclareFunction),
    // DeclareInterface(DeclareInterface),
    // DeclareModule(DeclareModule),
    // DeclareModuleExports(DeclareModuleExports),
    // DeclareTypeAlias(DeclareTypeAlias),
    // DeclareOpaqueType(DeclareOpaqueType),
    // DeclareVariable(DeclareVariable),
    // DeclareExportDeclaration(DeclareExportDeclaration),
    // DeclareExportAllDeclaration(DeclareExportAllDeclaration),
    // InterfaceDeclaration(InterfaceDeclaration),
    // OpaqueType(OpaqueType),
    // TypeAlias(TypeAlias),
    // EnumDeclaration(EnumDeclaration),
    // TSDeclareFunction(TSDeclareFunction),
    // TSInterfaceDeclaration(TSInterfaceDeclaration),
    // TSTypeAliasDeclaration(TSTypeAliasDeclaration),
    // TSEnumDeclaration(TSEnumDeclaration),
    // TSModuleDeclaration(TSModuleDeclaration),
    // TSImportEqualsDeclaration(TSImportEqualsDeclaration),
    // TSExportAssignment(TSExportAssignment),
    // TSNamespaceExportDeclaration(TSNamespaceExportDeclaration),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Terminatorless {

    // TODO
    // BreakStatement(BreakStatement),
    // ContinueStatement(ContinueStatement),
    // ReturnStatement(ReturnStatement),
    // ThrowStatement(ThrowStatement),
    // YieldExpression(YieldExpression),
    // AwaitExpression(AwaitExpression),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionStatement {

    // TODO
    // BreakStatement(BreakStatement),
    // ContinueStatement(ContinueStatement),
    // ReturnStatement(ReturnStatement),
    // ThrowStatement(ThrowStatement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Conditional {

    // TODO
    // ConditionalExpression(ConditionalExpression),
    // IfStatement(IfStatement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Loop {

    // TODO
    // DoWhileStatement(DoWhileStatement),
    // ForInStatement(ForInStatement),
    // ForStatement(ForStatement),
    // WhileStatement(WhileStatement),
    // ForOfStatement(ForOfStatement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum While {

    // TODO
    // DoWhileStatement(DoWhileStatement),
    // WhileStatement(WhileStatement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionWrapper {

    // TODO
    // ExpressionStatement(ExpressionStatement),
    // ParenthesizedExpression(ParenthesizedExpression),
    // TypeCastExpression(TypeCastExpression),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum For {

    // TODO
    // ForInStatement(ForInStatement),
    // ForStatement(ForStatement),
    // ForOfStatement(ForOfStatement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForXStatement {

    // TODO
    // ForInStatement(ForInStatement),
    // ForOfStatement(ForOfStatement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Function {

    // TODO
    // FunctionDeclaration(FunctionDeclaration),
    // FunctionExpression(FunctionExpression),
    // ObjectMethod(ObjectMethod),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunctionParent {

    // TODO
    // FunctionDeclaration(FunctionDeclaration),
    // FunctionExpression(FunctionExpression),
    // ObjectMethod(ObjectMethod),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pureish {

    // TODO
    // FunctionDeclaration(FunctionDeclaration),
    // FunctionExpression(FunctionExpression),
    // StringLiteral(StringLiteral),
    // NumericLiteral(NumericLiteral),
    // NullLiteral(NullLiteral),
    // BooleanLiteral(BooleanLiteral),
    // RegExpLiteral(RegExpLiteral),
    // ArrowFunctionExpression(ArrowFunctionExpression),
    // BigIntLiteral(BigIntLiteral),
    // DecimalLiteral(DecimalLiteral),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {

    // TODO
    // FunctionDeclaration(FunctionDeclaration),
    // VariableDeclaration(VariableDeclaration),
    // ClassDeclaration(ClassDeclaration),
    // ExportAllDeclaration(ExportAllDeclaration),
    // ExportDefaultDeclaration(ExportDefaultDeclaration),
    // ExportNamedDeclaration(ExportNamedDeclaration),
    // ImportDeclaration(ImportDeclaration),
    // DeclareClass(DeclareClass),
    // DeclareFunction(DeclareFunction),
    // DeclareInterface(DeclareInterface),
    // DeclareModule(DeclareModule),
    // DeclareModuleExports(DeclareModuleExports),
    // DeclareTypeAlias(DeclareTypeAlias),
    // DeclareOpaqueType(DeclareOpaqueType),
    // DeclareVariable(DeclareVariable),
    // DeclareExportDeclaration(DeclareExportDeclaration),
    // DeclareExportAllDeclaration(DeclareExportAllDeclaration),
    // InterfaceDeclaration(InterfaceDeclaration),
    // OpaqueType(OpaqueType),
    // TypeAlias(TypeAlias),
    // EnumDeclaration(EnumDeclaration),
    // TSDeclareFunction(TSDeclareFunction),
    // TSInterfaceDeclaration(TSInterfaceDeclaration),
    // TSTypeAliasDeclaration(TSTypeAliasDeclaration),
    // TSEnumDeclaration(TSEnumDeclaration),
    // TSModuleDeclaration(TSModuleDeclaration),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternLike {

    // TODO
    // Identifier(Identifier),
    // RestElement(RestElement),
    // AssignmentPattern(AssignmentPattern),
    // ArrayPattern(ArrayPattern),
    // ObjectPattern(ObjectPattern),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LVal {

    // TODO
    // Identifier(Identifier),
    // MemberExpression(MemberExpression),
    // RestElement(RestElement),
    // AssignmentPattern(AssignmentPattern),
    // ArrayPattern(ArrayPattern),
    // ObjectPattern(ObjectPattern),
    // TSParameterProperty(TSParameterProperty),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TSEntityName {

    // TODO
    // Identifier(Identifier),
    // TSQualifiedName(TSQualifiedName),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {

    // TODO
    // StringLiteral(StringLiteral),
    // NumericLiteral(NumericLiteral),
    // NullLiteral(NullLiteral),
    // BooleanLiteral(BooleanLiteral),
    // RegExpLiteral(RegExpLiteral),
    // TemplateLiteral(TemplateLiteral),
    // BigIntLiteral(BigIntLiteral),
    // DecimalLiteral(DecimalLiteral),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Immutable {

    // TODO
    // StringLiteral(StringLiteral),
    // NumericLiteral(NumericLiteral),
    // NullLiteral(NullLiteral),
    // BooleanLiteral(BooleanLiteral),
    // BigIntLiteral(BigIntLiteral),
    // JSXAttribute(JSXAttribute),
    // JSXClosingElement(JSXClosingElement),
    // JSXElement(JSXElement),
    // JSXExpressionContainer(JSXExpressionContainer),
    // JSXSpreadChild(JSXSpreadChild),
    // JSXOpeningElement(JSXOpeningElement),
    // JSXText(JSXText),
    // JSXFragment(JSXFragment),
    // JSXOpeningFragment(JSXOpeningFragment),
    // JSXClosingFragment(JSXClosingFragment),
    // DecimalLiteral(DecimalLiteral),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserWhitespacable {

    // TODO
    // ObjectMethod(ObjectMethod),
    // ObjectProperty(ObjectProperty),
    // ObjectTypeInternalSlot(ObjectTypeInternalSlot),
    // ObjectTypeCallProperty(ObjectTypeCallProperty),
    // ObjectTypeIndexer(ObjectTypeIndexer),
    // ObjectTypeProperty(ObjectTypeProperty),
    // ObjectTypeSpreadProperty(ObjectTypeSpreadProperty),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Method {

    // TODO
    // ObjectMethod(ObjectMethod),
    // ClassMethod(ClassMethod),
    // ClassPrivateMethod(ClassPrivateMethod),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectMember {

    // TODO
    // ObjectMethod(ObjectMethod),
    // ObjectProperty(ObjectProperty),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {

    // TODO
    // ObjectProperty(ObjectProperty),
    // ClassProperty(ClassProperty),
    // ClassPrivateProperty(ClassPrivateProperty),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryLike {

    // TODO
    // UnaryExpression(UnaryExpression),
    // SpreadElement(SpreadElement),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {

    // TODO
    // AssignmentPattern(AssignmentPattern),
    // ArrayPattern(ArrayPattern),
    // ObjectPattern(ObjectPattern),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Class {

    // TODO
    // ClassExpression(ClassExpression),
    // ClassDeclaration(ClassDeclaration),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleDeclaration {

    // TODO
    // ExportAllDeclaration(ExportAllDeclaration),
    // ExportDefaultDeclaration(ExportDefaultDeclaration),
    // ExportNamedDeclaration(ExportNamedDeclaration),
    // ImportDeclaration(ImportDeclaration),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportDeclaration {

    // TODO
    // ExportAllDeclaration(ExportAllDeclaration),
    // ExportDefaultDeclaration(ExportDefaultDeclaration),
    // ExportNamedDeclaration(ExportNamedDeclaration),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleSpecifier {

    // TODO
    // ExportSpecifier(ExportSpecifier),
    // ImportDefaultSpecifier(ImportDefaultSpecifier),
    // ImportNamespaceSpecifier(ImportNamespaceSpecifier),
    // ImportSpecifier(ImportSpecifier),
    // ExportNamespaceSpecifier(ExportNamespaceSpecifier),
    // ExportDefaultSpecifier(ExportDefaultSpecifier),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Flow {

    // TODO
    // AnyTypeAnnotation(AnyTypeAnnotation),
    // ArrayTypeAnnotation(ArrayTypeAnnotation),
    // BooleanTypeAnnotation(BooleanTypeAnnotation),
    // BooleanLiteralTypeAnnotation(BooleanLiteralTypeAnnotation),
    // NullLiteralTypeAnnotation(NullLiteralTypeAnnotation),
    // ClassImplements(ClassImplements),
    // DeclareClass(DeclareClass),
    // DeclareFunction(DeclareFunction),
    // DeclareInterface(DeclareInterface),
    // DeclareModule(DeclareModule),
    // DeclareModuleExports(DeclareModuleExports),
    // DeclareTypeAlias(DeclareTypeAlias),
    // DeclareOpaqueType(DeclareOpaqueType),
    // DeclareVariable(DeclareVariable),
    // DeclareExportDeclaration(DeclareExportDeclaration),
    // DeclareExportAllDeclaration(DeclareExportAllDeclaration),
    // DeclaredPredicate(DeclaredPredicate),
    // ExistsTypeAnnotation(ExistsTypeAnnotation),
    // FunctionTypeAnnotation(FunctionTypeAnnotation),
    // FunctionTypeParam(FunctionTypeParam),
    // GenericTypeAnnotation(GenericTypeAnnotation),
    // InferredPredicate(InferredPredicate),
    // InterfaceExtends(InterfaceExtends),
    // InterfaceDeclaration(InterfaceDeclaration),
    // InterfaceTypeAnnotation(InterfaceTypeAnnotation),
    // IntersectionTypeAnnotation(IntersectionTypeAnnotation),
    // MixedTypeAnnotation(MixedTypeAnnotation),
    // EmptyTypeAnnotation(EmptyTypeAnnotation),
    // NullableTypeAnnotation(NullableTypeAnnotation),
    // NumberLiteralTypeAnnotation(NumberLiteralTypeAnnotation),
    // NumberTypeAnnotation(NumberTypeAnnotation),
    // ObjectTypeAnnotation(ObjectTypeAnnotation),
    // ObjectTypeInternalSlot(ObjectTypeInternalSlot),
    // ObjectTypeCallProperty(ObjectTypeCallProperty),
    // ObjectTypeIndexer(ObjectTypeIndexer),
    // ObjectTypeProperty(ObjectTypeProperty),
    // ObjectTypeSpreadProperty(ObjectTypeSpreadProperty),
    // OpaqueType(OpaqueType),
    // QualifiedTypeIdentifier(QualifiedTypeIdentifier),
    // StringLiteralTypeAnnotation(StringLiteralTypeAnnotation),
    // StringTypeAnnotation(StringTypeAnnotation),
    // SymbolTypeAnnotation(SymbolTypeAnnotation),
    // ThisTypeAnnotation(ThisTypeAnnotation),
    // TupleTypeAnnotation(TupleTypeAnnotation),
    // TypeofTypeAnnotation(TypeofTypeAnnotation),
    // TypeAlias(TypeAlias),
    // TypeAnnotation(TypeAnnotation),
    // TypeCastExpression(TypeCastExpression),
    // TypeParameter(TypeParameter),
    // TypeParameterDeclaration(TypeParameterDeclaration),
    // TypeParameterInstantiation(TypeParameterInstantiation),
    // UnionTypeAnnotation(UnionTypeAnnotation),
    // Variance(Variance),
    // VoidTypeAnnotation(VoidTypeAnnotation),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowType {

    // TODO
    // AnyTypeAnnotation(AnyTypeAnnotation),
    // ArrayTypeAnnotation(ArrayTypeAnnotation),
    // BooleanTypeAnnotation(BooleanTypeAnnotation),
    // BooleanLiteralTypeAnnotation(BooleanLiteralTypeAnnotation),
    // NullLiteralTypeAnnotation(NullLiteralTypeAnnotation),
    // ExistsTypeAnnotation(ExistsTypeAnnotation),
    // FunctionTypeAnnotation(FunctionTypeAnnotation),
    // GenericTypeAnnotation(GenericTypeAnnotation),
    // InterfaceTypeAnnotation(InterfaceTypeAnnotation),
    // IntersectionTypeAnnotation(IntersectionTypeAnnotation),
    // MixedTypeAnnotation(MixedTypeAnnotation),
    // EmptyTypeAnnotation(EmptyTypeAnnotation),
    // NullableTypeAnnotation(NullableTypeAnnotation),
    // NumberLiteralTypeAnnotation(NumberLiteralTypeAnnotation),
    // NumberTypeAnnotation(NumberTypeAnnotation),
    // ObjectTypeAnnotation(ObjectTypeAnnotation),
    // StringLiteralTypeAnnotation(StringLiteralTypeAnnotation),
    // StringTypeAnnotation(StringTypeAnnotation),
    // SymbolTypeAnnotation(SymbolTypeAnnotation),
    // ThisTypeAnnotation(ThisTypeAnnotation),
    // TupleTypeAnnotation(TupleTypeAnnotation),
    // TypeofTypeAnnotation(TypeofTypeAnnotation),
    // UnionTypeAnnotation(UnionTypeAnnotation),
    // VoidTypeAnnotation(VoidTypeAnnotation),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowBaseAnnotation {

    // TODO
    // AnyTypeAnnotation(AnyTypeAnnotation),
    // BooleanTypeAnnotation(BooleanTypeAnnotation),
    // NullLiteralTypeAnnotation(NullLiteralTypeAnnotation),
    // MixedTypeAnnotation(MixedTypeAnnotation),
    // EmptyTypeAnnotation(EmptyTypeAnnotation),
    // NumberTypeAnnotation(NumberTypeAnnotation),
    // StringTypeAnnotation(StringTypeAnnotation),
    // SymbolTypeAnnotation(SymbolTypeAnnotation),
    // ThisTypeAnnotation(ThisTypeAnnotation),
    // VoidTypeAnnotation(VoidTypeAnnotation),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowDeclaration {

    // TODO
    // DeclareClass(DeclareClass),
    // DeclareFunction(DeclareFunction),
    // DeclareInterface(DeclareInterface),
    // DeclareModule(DeclareModule),
    // DeclareModuleExports(DeclareModuleExports),
    // DeclareTypeAlias(DeclareTypeAlias),
    // DeclareOpaqueType(DeclareOpaqueType),
    // DeclareVariable(DeclareVariable),
    // DeclareExportDeclaration(DeclareExportDeclaration),
    // DeclareExportAllDeclaration(DeclareExportAllDeclaration),
    // InterfaceDeclaration(InterfaceDeclaration),
    // OpaqueType(OpaqueType),
    // TypeAlias(TypeAlias),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowPredicate {

    // TODO
    // DeclaredPredicate(DeclaredPredicate),
    // InferredPredicate(InferredPredicate),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumBody {

    // TODO
    // EnumBooleanBody(EnumBooleanBody),
    // EnumNumberBody(EnumNumberBody),
    // EnumStringBody(EnumStringBody),
    // EnumSymbolBody(EnumSymbolBody),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnumMember {

    // TODO
    // EnumBooleanMember(EnumBooleanMember),
    // EnumNumberMember(EnumNumberMember),
    // EnumStringMember(EnumStringMember),
    // EnumDefaultedMember(EnumDefaultedMember),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JSX {

    // TODO
    // JSXAttribute(JSXAttribute),
    // JSXClosingElement(JSXClosingElement),
    // JSXElement(JSXElement),
    // JSXEmptyExpression(JSXEmptyExpression),
    // JSXExpressionContainer(JSXExpressionContainer),
    // JSXSpreadChild(JSXSpreadChild),
    // JSXIdentifier(JSXIdentifier),
    // JSXMemberExpression(JSXMemberExpression),
    // JSXNamespacedName(JSXNamespacedName),
    // JSXOpeningElement(JSXOpeningElement),
    // JSXSpreadAttribute(JSXSpreadAttribute),
    // JSXText(JSXText),
    // JSXFragment(JSXFragment),
    // JSXOpeningFragment(JSXOpeningFragment),
    // JSXClosingFragment(JSXClosingFragment),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Private {

    // TODO
    // ClassPrivateProperty(ClassPrivateProperty),
    // ClassPrivateMethod(ClassPrivateMethod),
    // PrivateName(PrivateName),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TSTypeElement {

    // TODO
    // TSCallSignatureDeclaration(TSCallSignatureDeclaration),
    // TSConstructSignatureDeclaration(TSConstructSignatureDeclaration),
    // TSPropertySignature(TSPropertySignature),
    // TSMethodSignature(TSMethodSignature),
    // TSIndexSignature(TSIndexSignature),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TSType {

    // TODO
    // TSAnyKeyword(TSAnyKeyword),
    // TSBooleanKeyword(TSBooleanKeyword),
    // TSBigIntKeyword(TSBigIntKeyword),
    // TSIntrinsicKeyword(TSIntrinsicKeyword),
    // TSNeverKeyword(TSNeverKeyword),
    // TSNullKeyword(TSNullKeyword),
    // TSNumberKeyword(TSNumberKeyword),
    // TSObjectKeyword(TSObjectKeyword),
    // TSStringKeyword(TSStringKeyword),
    // TSSymbolKeyword(TSSymbolKeyword),
    // TSUndefinedKeyword(TSUndefinedKeyword),
    // TSUnknownKeyword(TSUnknownKeyword),
    // TSVoidKeyword(TSVoidKeyword),
    // TSThisType(TSThisType),
    // TSFunctionType(TSFunctionType),
    // TSConstructorType(TSConstructorType),
    // TSTypeReference(TSTypeReference),
    // TSTypePredicate(TSTypePredicate),
    // TSTypeQuery(TSTypeQuery),
    // TSTypeLiteral(TSTypeLiteral),
    // TSArrayType(TSArrayType),
    // TSTupleType(TSTupleType),
    // TSOptionalType(TSOptionalType),
    // TSRestType(TSRestType),
    // TSUnionType(TSUnionType),
    // TSIntersectionType(TSIntersectionType),
    // TSConditionalType(TSConditionalType),
    // TSInferType(TSInferType),
    // TSParenthesizedType(TSParenthesizedType),
    // TSTypeOperator(TSTypeOperator),
    // TSIndexedAccessType(TSIndexedAccessType),
    // TSMappedType(TSMappedType),
    // TSLiteralType(TSLiteralType),
    // TSExpressionWithTypeArguments(TSExpressionWithTypeArguments),
    // TSImportType(TSImportType),;

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TSBaseType {

    // TSAnyKeyword(TSAnyKeyword),
    // TSBooleanKeyword(TSBooleanKeyword),
    // TSBigIntKeyword(TSBigIntKeyword),
    // TSIntrinsicKeyword(TSIntrinsicKeyword),
    // TSNeverKeyword(TSNeverKeyword),
    // TSNullKeyword(TSNullKeyword),
    // TSNumberKeyword(TSNumberKeyword),
    // TSObjectKeyword(TSObjectKeyword),
    // TSStringKeyword(TSStringKeyword),
    // TSSymbolKeyword(TSSymbolKeyword),
    // TSUndefinedKeyword(TSUndefinedKeyword),
    // TSUnknownKeyword(TSUnknownKeyword),
    // TSVoidKeyword(TSVoidKeyword),
    // TSThisType(TSThisType),
    // TSLiteralType(TSLiteralType),;

}

