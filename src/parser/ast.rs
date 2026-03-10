use crate::source::SourceLocation;

pub struct Program {
    pub declarations: Vec<Declaration>
}

pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration)
}

pub struct FunctionDeclaration {
    name: String,
    return_type: Type,
    parameters: Vec<Parameter>,
    body: CompoundStatement,
    location: SourceLocation
}

enum Type {
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

pub struct Parameter {
    name: String,
    param_type: Type,
    location: SourceLocation
}

// Statements
pub enum Statement {
    Return(ReturnStatement),
    Compund(CompoundStatement),
    Expression(ExpressionStatement),
    If(Box<IfStatement>),
    While(WhileStatement),
    Null
}

pub struct CompoundStatement {
    statements: Vec<Statement>,
    location: SourceLocation
}

pub struct ReturnStatement {
    expression: Option<Expression>,
    location: SourceLocation
}

pub struct ExpressionStatement {
    expression: Expression,
    location: SourceLocation
}

pub struct IfStatement {
    condition: Expression,
    then_branch: Box<Statement>,
    else_branch: Option<Box<Statement>>,
    location: SourceLocation
}

pub struct WhileStatement {
    condition: Expression,
    body: Box<Statement>,
    location: SourceLocation
}

// Expressions
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

pub struct BinaryOpExpression {
    operator: BinaryOperator,
    left: Box<Expression>,
    right: Box<Expression>,
    location: SourceLocation
}

pub struct UnaryOpExpression {
    operator: UnaryOperator,
    operand: Expression,
    location: SourceLocation
}


pub struct FunctionCallExpression {
    name: String,
    arguments: Vec<Expression>,
    location: SourceLocation
}

pub struct AssignmentExpression {
    target: String,
    value: Box<Expression>,
    location: SourceLocation
}

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

pub enum BinaryOperator {
    Add, Subtract, Multiply, Divide, Modulus,
    Equal, NotEqual, LessThan, GreaterThan,
    LessThanOrEqual, GreaterThanOrEqual,
    LogicalAnd, LogicalOr,
    BitwiseAnd, BitwiseOr, BitwiseXor,
    ShiftLeft, ShiftRight
}


