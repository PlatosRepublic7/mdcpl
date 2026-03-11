use crate::lexer::token::{Token, TokenKind};
use crate::diagnostics::{DiagnosticEngine, ParserDiagnosticKind, DiagnosticKind, DiagnosticLocation, Severity};
use crate::source::SourceLocation;

mod ast;
use ast::{Program, Declaration};

pub struct Parser {
    token_vec: Vec<Token>,
    current: usize,
    filename: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, fname: String) -> Self {
        Parser {
            token_vec: tokens,
            current: 0,
            filename: fname
        } 
    }

    pub fn parse(&mut self, diagnostic_engine: &mut DiagnosticEngine) -> Program {
        // Iterate through all the tokens in token_vec
        let mut declaration_vec: Vec<Declaration> = Vec::new();
        while let Some(token) = self.peek() {
            self.advance();
        }
        
        Program {
            declarations: declaration_vec
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.token_vec.get(self.current)
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|token| token.kind)
    }

    fn advance(&mut self) {
        if self.current < self.token_vec.len() {
            self.current += 1;
        }
    }

    fn expect(&mut self, expected_kind: TokenKind, diagnostic_engine: &mut DiagnosticEngine) -> bool {
        let (current_kind, current_line, current_col) = match self.peek() {
            Some(token) => (token.kind, token.location.line_num, token.location.column_num),
            None => {
                let last = self.token_vec.len() - 1;
                let line = self.token_vec[last].location.line_num;
                let col = self.token_vec[last].location.column_num;
                self.create_parser_diagnostic(diagnostic_engine, ParserDiagnosticKind::UnexpectedEndOfInput, Severity::Error, line, col);
                return false;
            }
        };
        
        if current_kind == expected_kind {
            self.advance();
            true
        } else {
            self.create_parser_diagnostic(diagnostic_engine, ParserDiagnosticKind::UnexpectedToken, Severity::Error,current_line, current_col);
            false
        }
    }

    fn create_parser_diagnostic(&self, diag_engine: &mut DiagnosticEngine, parser_diag: ParserDiagnosticKind, severity: Severity, start_line: usize, start_col: usize) {
        let diag_kind: DiagnosticKind = DiagnosticKind::Parser(parser_diag);
        let source_location: SourceLocation = SourceLocation::new(&self.filename, start_line, start_col);
        diag_engine.emit(severity, diag_kind, source_location, 0, None);
    }
}


