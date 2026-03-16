use crate::lexer::token::{Token, TokenKind};
use crate::diagnostics::{DiagnosticEngine, ParserDiagnosticKind, DiagnosticKind, DiagnosticLocation, Severity};
use crate::source::SourceLocation;

pub mod ast;
use ast::{Program, Declaration, FunctionDeclaration, Type, Parameter, CompoundStatement, Statement, ReturnStatement, Expression};

pub struct Parser {
    token_vec: Vec<Token>,
    current: usize,
    filename: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, fname: &str) -> Self {
        Parser {
            token_vec: tokens,
            current: 0,
            filename: fname.to_string()
        } 
    }

    pub fn parse(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Program {
        let mut declaration_vec: Vec<Declaration> = Vec::new();
        while self.peek_kind() != TokenKind::Eof {
            match self.parse_declaration(diagnostic_engine) {
                Some(declaration) => declaration_vec.push(declaration),
                None => {}
            };
        }

        Program { declarations: declaration_vec }
    }
    
    fn parse_declaration(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<Declaration> {
        let function_declaration = self.parse_function_declaration(diagnostic_engine)?;
        Some(Declaration::FunctionDeclaration(function_declaration))
    }

    fn parse_function_declaration(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<FunctionDeclaration> {
        let f_location: SourceLocation = self.current_token().location.clone();
        let r_type: Type  = self.parse_type(diagnostic_engine)?;
        let f_name: String = self.expect(TokenKind::Identifier, diagnostic_engine)?.lexeme;
        self.expect(TokenKind::LeftParen, diagnostic_engine)?;
        
        let mut f_params: Vec<Parameter> = Vec::new();
        while self.peek_kind() != TokenKind::RightParen {
            let parameter: Parameter = self.parse_parameter(diagnostic_engine)?;
            f_params.push(parameter);
        }

        self.consume();

        // Here we will parse the body of the function
        self.expect(TokenKind::LeftBrace, diagnostic_engine)?;

        // Parse compound statement
        let compound_statement: CompoundStatement = self.parse_compound_statement(diagnostic_engine)?;
        self.consume();
        let function_declaration: FunctionDeclaration = FunctionDeclaration {
            name: f_name,
            return_type: r_type,
            parameters: f_params,
            body: compound_statement,
            location: f_location
        };
        Some(function_declaration)
    }

    fn parse_compound_statement(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<CompoundStatement> {
        let current_token: Token = self.current_token().clone();
        let mut statement_vec: Vec<Statement> = Vec::new();
        while self.peek_kind() != TokenKind::RightBrace {
            let statement = self.parse_statement(diagnostic_engine)?;
            statement_vec.push(statement);
            //self.consume();
        }

        let compound_statement: CompoundStatement = CompoundStatement {
            statements: statement_vec,
            location: current_token.location
        };
        Some(compound_statement)
    }

    fn parse_statement(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<Statement> {
        // For now, we will only handle return statements
        match self.peek_kind() {
            TokenKind::Return => {
                // return statement
                let return_statement: ReturnStatement = self.parse_return_statement(diagnostic_engine)?;
                let statement = Statement::Return(return_statement);
                return Some(statement);
            }
            _ => {
                // Everything else gets an error right now
                return None;
            }
        }
    }

    fn parse_return_statement(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<ReturnStatement> {
        self.expect(TokenKind::Return, diagnostic_engine)?;
        let current_token: Token = self.current_token().clone();
        let return_expression: Expression = self.parse_expression(diagnostic_engine)?;
        self.expect(TokenKind::Semicolon, diagnostic_engine)?;
        let return_statement: ReturnStatement = ReturnStatement {
            expression: Some(return_expression),
            location: current_token.location.clone()
        };

        Some(return_statement)
    }

    fn parse_expression(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<Expression> {
        let expression: Expression;
        let current_token: &Token = self.current_token();
        match self.peek_kind() {
            TokenKind::IntegerLiteral => {
                let int_literal: i64 = current_token.lexeme.parse::<i64>().unwrap();
                expression = Expression::IntegerLiteral(int_literal, current_token.location.clone());
                self.consume();
                return Some(expression);
            }
            _ => {
                return None;
            }
        }
    }

    fn parse_parameter(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<Parameter> {
        let p_location: SourceLocation = self.current_token().location.clone();
        let parameter_type: Type = self.parse_type(diagnostic_engine)?;
        if parameter_type == Type::Void {
            let p_name: String = String::from("");
            let parameter: Parameter = Parameter {
                name: p_name,
                param_type: parameter_type,
                location: p_location
            };
            return Some(parameter);
        }
        let identifier_token: Token = self.expect(TokenKind::Identifier, diagnostic_engine)?;
        let p_name: String = identifier_token.lexeme;
        let parameter: Parameter = Parameter {
            name: p_name,
            param_type: parameter_type,
            location: p_location
        };
        return Some(parameter);
    }

    fn parse_type(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Option<Type> {
        let ast_type = match self.peek_kind() {
            TokenKind::Int => Type::Int,
            TokenKind::Short => Type::Short,
            TokenKind::Long => Type::Long,
            TokenKind::Float => Type::Float,
            TokenKind::Double => Type::Double,
            TokenKind::Char => Type::Char,
            TokenKind::Void => Type::Void,
            _ => {
                let cur_token = self.current_token();
                self.create_parser_diagnostic(diagnostic_engine, ParserDiagnosticKind::InvalidTypeKeyword(format!("Invalid type keyword: {}", cur_token.lexeme)), Severity::Error, cur_token.location.line_num, cur_token.location.column_num);
                return None;
            }
        };
        self.consume();
        Some(ast_type)
    }

    fn peek_kind(&self) -> TokenKind {
        return self.token_vec[self.current].kind
    }

    fn current_token(&self) -> &Token {
        return &self.token_vec[self.current]
    }

    fn consume(&mut self) -> Token {
        let cur_token = self.token_vec[self.current].clone();
        if self.token_vec.len() - 1 > self.current && self.peek_kind() != TokenKind::Eof {
            self.current += 1;
        }
        return cur_token;
    }

    fn expect(&mut self, expected_kind: TokenKind, diagnostic_engine: &mut DiagnosticEngine) -> Option<Token> {
        if self.peek_kind() == expected_kind {
            return Some(self.consume());
        }

        let cur_token = self.current_token();
        self.create_parser_diagnostic(diagnostic_engine, ParserDiagnosticKind::UnexpectedToken, Severity::Error, cur_token.location.line_num, cur_token.location.column_num);
        
        None
    }

    fn create_parser_diagnostic(&self, diag_engine: &mut DiagnosticEngine, parser_diag: ParserDiagnosticKind, severity: Severity, start_line: usize, start_col: usize) {
        let diag_kind: DiagnosticKind = DiagnosticKind::Parser(parser_diag);
        let source_location: SourceLocation = SourceLocation::new(&self.filename, start_line, start_col);
        diag_engine.emit(severity, diag_kind, source_location, 0, None);
    }
}


