pub mod token;
use token::{Token, TokenKind};
use crate::diagnostics::DiagnosticEngine;

pub struct Lexer {
    source_text: String,
    filename: String,
    cur_pos: usize,
    cur_line: u64,
    cur_column: u64,
    token_vec: Vec<Token>
}

impl Lexer {
    pub fn new(text: String, fname: &str) -> Self {
        let t_vec: Vec<Token> = Vec::new();

        Lexer {
            source_text: text,
            filename: fname.to_owned(),
            cur_pos: 0,
            cur_line: 0,
            cur_column: 0,
            token_vec: t_vec
        }
    }

    pub fn tokenize(mut self, diag_engine: &mut DiagnosticEngine) -> Vec<Token> {
        // Lexer for a preprocessed .i file
        
        // Iterate through self.source_text a character at a time
        while let Some(c) = self.peek() {
            println!("Current char: {}", c);
            self.advance();
        }
        
        self.token_vec
    }

    fn peek(&self) -> Option<char> {
        self.source_text[self.cur_pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.cur_pos += c.len_utf8();
        self.cur_column += 1;
        if c == '\n' {
            self.cur_line += 1;
            self.cur_column = 0;
        }

        Some(c)
    }
}
