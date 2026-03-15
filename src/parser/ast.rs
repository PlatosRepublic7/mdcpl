use crate::source::SourceLocation;

#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Declaration>
}

#[derive(Debug)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    Error
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
    pub body: CompoundStatement,
    pub location: SourceLocation
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    Char,
    Float,
    Double,
    Long,
    Short,
    Void,
    Unsigned(Box<Type>),
    Pointer(Box<Type>)
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub location: SourceLocation
}

// Statements
#[derive(Debug)]
pub enum Statement {
    Return(ReturnStatement),
    Compound(CompoundStatement),
    Expression(ExpressionStatement),
    If(Box<IfStatement>),
    While(WhileStatement),
    Null
}

#[derive(Debug)]
pub struct CompoundStatement {
    pub statements: Vec<Statement>,
    pub location: SourceLocation
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
    pub location: SourceLocation
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Expression,
    location: SourceLocation
}

#[derive(Debug)]
pub struct IfStatement {
    condition: Expression,
    then_branch: Box<Statement>,
    else_branch: Option<Box<Statement>>,
    location: SourceLocation
}

#[derive(Debug)]
pub struct WhileStatement {
    condition: Expression,
    body: Box<Statement>,
    location: SourceLocation
}

// Expressions
#[derive(Debug)]
pub enum Expression {
    IntegerLiteral(i64, SourceLocation),
    FloatLiteral(f64, SourceLocation),
    StringLiteral(String, SourceLocation),
    CharLiteral(char, SourceLocation),
    Identifier(String, SourceLocation),
    BinaryOp(Box<BinaryOpExpression>),
    UnaryOp(Box<UnaryOpExpression>),
    FunctionCall(Box<FunctionCallExpression>),
    Assignment(Box<AssignmentExpression>)
}

#[derive(Debug)]
pub struct BinaryOpExpression {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
    location: SourceLocation
}

#[derive(Debug)]
pub struct UnaryOpExpression {
    operator: UnaryOperator,
    operand: Expression,
    location: SourceLocation
}

#[derive(Debug)]
pub struct FunctionCallExpression {
    name: String,
    arguments: Vec<Expression>,
    location: SourceLocation
}

#[derive(Debug)]
pub struct AssignmentExpression {
    target: String,
    value: Box<Expression>,
    location: SourceLocation
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    LogicalNot,
    BitwiseNot,
    Dereference,
    AddressOf,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add, Subtract, Multiply, Divide, Modulus,
    Equal, NotEqual, LessThan, GreaterThan,
    LessThanOrEqual, GreaterThanOrEqual,
    LogicalAnd, LogicalOr,
    BitwiseAnd, BitwiseOr, BitwiseXor,
    ShiftLeft, ShiftRight
}


