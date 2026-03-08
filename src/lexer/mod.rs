pub mod token;
use token::{Token, TokenKind};
use crate::source::SourceLocation;
use crate::diagnostics::{DiagnosticEngine, DiagnosticKind, LexerDiagnosticKind, Severity};
use std::collections::HashMap;

pub struct Lexer {
    source_text: String,
    filename: String,
    cur_pos: usize,
    cur_line: usize,
    cur_column: usize,
    token_vec: Vec<Token>,
    keyword_table: HashMap<String, TokenKind>,
    punctuation_table: HashMap<String, TokenKind>
}

impl Lexer {
    pub fn new(text: String, fname: &str) -> Self {
        let t_vec: Vec<Token> = Vec::new();

        Lexer {
            source_text: text,
            filename: fname.to_owned(),
            cur_pos: 0,
            cur_line: 1,
            cur_column: 0,
            token_vec: t_vec,
            keyword_table: Self::build_keyword_table(),
            punctuation_table: Self::build_punctuation_table()
        }
    }

    pub fn tokenize(mut self, diag_engine: &mut DiagnosticEngine) -> Vec<Token> {
        // Lexer for a preprocessed .i file
        let mut current_line: usize = 1;
        let mut current_col: usize = 0;

        // Iterate through self.source_text a character at a time
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c.is_alphabetic() || c == '_' {
                self.id_or_kw_scanner(current_line, current_col, diag_engine);
            } else if c.is_ascii_punctuation() {
                self.punctuation_scanner(current_line, current_col, diag_engine);
            } else if c.is_numeric() {
                self.numeric_scanner(current_line, current_col, diag_engine);
            }
            current_line = self.cur_line;
            current_col = self.cur_column;
        }
        
        self.token_vec
    }

    fn numeric_scanner(&mut self, start_line: usize, start_col: usize, diag_engine: &mut DiagnosticEngine) {
        let mut lexeme = String::new();

        // We can keep track of whether of not a decimal point appears in the lexeme
        // If it does, we can make sure that it isn't the last character in the lexeme,
        // and that there is only one present within the lexeme, otherwise we throw an error
        let mut point_count: u8 = 0;
        let mut found_error: bool = false;
        let mut error_message = "";
        let mut found_warning: bool = false;
        let mut warning_message = "";
        let token_kind: TokenKind;
        let mut lexer_diag: LexerDiagnosticKind = LexerDiagnosticKind::Null;

        while let Some(c) = self.peek() {
            if c.is_numeric() {
                lexeme += &c.to_string();
                self.advance();
            } else if c == '.' {
                point_count += 1;
                if point_count > 1 {
                    found_error = true;
                    error_message = "multiple decimal points in floating point literal";
                    lexer_diag = LexerDiagnosticKind::MultipleDecimalPointsInFloat;
                    break;
                }
                lexeme += &c.to_string();
                self.advance();
            } else {
                break;
            }
        }

        if let Some(c) = lexeme.chars().last() && c == '.' {
            found_warning = true;
            warning_message = "trailing decimal point found in float literal, consider adding a trailing '0'";
            lexer_diag = LexerDiagnosticKind::TrailingDecimalPointInFloat;
        }

        if point_count == 1 {
            token_kind = TokenKind::FloatingPoint;
        } else {
            token_kind = TokenKind::Integer;
        }

        let mut severity: Severity = Severity::Fatal;
        let mut diag_message: String = String::new();
        if found_error {
            severity = Severity::Error;
            diag_message = String::from(error_message);
        }
        if found_warning {
            severity = Severity::Warning;
            diag_message = String::from(warning_message);
        }

        if found_error || found_warning {
            self.create_lexer_diagnostic(diag_engine, lexer_diag, severity, diag_message, start_line, start_col);
        }

        self.create_token(token_kind, lexeme, start_line, start_col);
    }

    fn punctuation_scanner(&mut self, start_line: usize, start_col: usize, diag_engine: &mut DiagnosticEngine) {
        let mut lexeme = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_punctuation() {
                lexeme += &c.to_string();
                self.advance();
            } else {
                break;
            }
        }

        let token_kind: TokenKind;
        match self.punctuation_table.get(lexeme.as_str()).cloned() {
            Some(kind) => {
                token_kind = kind;
            }
            None => {
                token_kind = TokenKind::Unknown;
            }
        }

        self.create_token(token_kind, lexeme, start_line, start_col);
    }

    fn id_or_kw_scanner(&mut self, start_line: usize, start_col: usize, diag_engine: &mut DiagnosticEngine) {
        // Scans for identifiers or keywords
        let mut lexeme = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                lexeme += &c.to_string();
                self.advance();
            } else if c == '\n' || c.is_whitespace() || c.is_ascii_punctuation() {
                break;
            } else {
                let lexer_error: LexerDiagnosticKind = LexerDiagnosticKind::InvalidIdentifier;
                let severity: Severity = Severity::Error;
                let err_message = String::from("invalid identifier");
                self.create_lexer_diagnostic(diag_engine, lexer_error, severity, err_message, start_line, start_col);
            }
        }

        let token_kind: TokenKind;

        // Determine the TokenKind (Keyword or Identifier)
        match self.keyword_table.get(lexeme.as_str()).cloned() {
            Some(kind) => {
                token_kind = kind;
            },
            None => {
                token_kind = TokenKind::Identifier;
            }
        }

        // Create the token 
        self.create_token(token_kind, lexeme, start_line, start_col);
    }

    fn peek(&self) -> Option<char> {
        self.source_text[self.cur_pos..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.source_text[self.cur_pos+1..].chars().next()
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

    fn create_token(&mut self, token_kind: TokenKind, lexeme: String, start_line: usize, start_col: usize) {
        // Create SourceLocation object
        let source_location: SourceLocation = SourceLocation::new(&self.filename, start_line, start_col);
        let token: Token = Token::new(token_kind, lexeme, source_location);
        self.token_vec.push(token);
    }

    fn create_lexer_diagnostic(&self, diag_engine: &mut DiagnosticEngine, lexer_diag: LexerDiagnosticKind, severity: Severity, message: String, start_line: usize, start_col: usize) -> bool {
        let diag_kind: DiagnosticKind = DiagnosticKind::Lexer(lexer_diag);
        let source_location: SourceLocation = SourceLocation::new(&self.filename, start_line, start_col);
        let span_end = self.cur_column - start_col;
        let source_line = self.current_line_text();
        diag_engine.emit(severity, diag_kind, &message, source_location, span_end, Some(source_line.as_str()))
    }

    fn current_line_text(&self) -> String {
        // Produces the current line of text from the source code for Diagnostic's
        // Find the beginning index of the line
        let mut rev_offset: usize = 0;
        
        for c in self.source_text[..self.cur_pos].chars().rev() {
            if c == '\n' {
                break;
            }
            rev_offset += c.len_utf8();
        }

        // Find the end of the line
        let mut forward_offset: usize = 0;

        for c in self.source_text[self.cur_pos..].chars() {
            if c == '\n' {
                break;
            }
            forward_offset += c.len_utf8();
        }

        let beginning_offset = self.cur_pos - rev_offset;
        let ending_offset = self.cur_pos + forward_offset;

        self.source_text[beginning_offset..ending_offset].to_owned()
    }

    fn build_keyword_table() -> HashMap<String, TokenKind> {
        let mut keyword_table: HashMap<String, TokenKind> = HashMap::new();
        keyword_table.insert("char".to_owned(), TokenKind::Char);
        keyword_table.insert("double".to_owned(), TokenKind::Double);
        keyword_table.insert("float".to_owned(), TokenKind::Float);
        keyword_table.insert("int".to_owned(), TokenKind::Int);
        keyword_table.insert("long".to_owned(), TokenKind::Long);
        keyword_table.insert("short".to_owned(), TokenKind::Short);
        keyword_table.insert("signed".to_owned(), TokenKind::Signed);
        keyword_table.insert("unsigned".to_owned(), TokenKind::Unsigned);
        keyword_table.insert("void".to_owned(), TokenKind::Void);
        keyword_table.insert("auto".to_owned(), TokenKind::Auto);
        keyword_table.insert("const".to_owned(), TokenKind::Const);
        keyword_table.insert("extern".to_owned(), TokenKind::Extern);
        keyword_table.insert("register".to_owned(), TokenKind::Register);
        keyword_table.insert("static".to_owned(), TokenKind::Static);
        keyword_table.insert("volatile".to_owned(), TokenKind::Volatile);
        keyword_table.insert("break".to_owned(), TokenKind::Break);
        keyword_table.insert("case".to_owned(), TokenKind::Case);
        keyword_table.insert("continue".to_owned(), TokenKind::Continue);
        keyword_table.insert("default".to_owned(), TokenKind::Default);
        keyword_table.insert("do".to_owned(), TokenKind::Do);
        keyword_table.insert("else".to_owned(), TokenKind::Else);
        keyword_table.insert("for".to_owned(), TokenKind::For);
        keyword_table.insert("goto".to_owned(), TokenKind::Goto);
        keyword_table.insert("if".to_owned(), TokenKind::If);
        keyword_table.insert("return".to_owned(), TokenKind::Return);
        keyword_table.insert("switch".to_owned(), TokenKind::Switch);
        keyword_table.insert("while".to_owned(), TokenKind::While);
        keyword_table.insert("enum".to_owned(), TokenKind::Enum);
        keyword_table.insert("sizeof".to_owned(), TokenKind::Sizeof);
        keyword_table.insert("struct".to_owned(), TokenKind::Struct);
        keyword_table.insert("typedef".to_owned(), TokenKind::Typedef);
        keyword_table.insert("union".to_owned(), TokenKind::Union);

        keyword_table
    }

    fn build_punctuation_table() -> HashMap<String, TokenKind> {
        let mut punc_table: HashMap<String, TokenKind> = HashMap::new();
        punc_table.insert("(".to_owned(), TokenKind::LeftParen);
        punc_table.insert(")".to_owned(), TokenKind::RightParen);
        punc_table.insert("[".to_owned(), TokenKind::LeftBracket);
        punc_table.insert("]".to_owned(), TokenKind::RightBracket);
        punc_table.insert("{".to_owned(), TokenKind::LeftBrace);
        punc_table.insert("}".to_owned(), TokenKind::RightBrace);
        punc_table.insert("\"".to_owned(), TokenKind::DoubleQuote);
        punc_table.insert("\'".to_owned(), TokenKind::SingleQuote);
        punc_table.insert(";".to_owned(), TokenKind::Semicolon);
        punc_table.insert(":".to_owned(), TokenKind::Colon);
        punc_table.insert(",".to_owned(), TokenKind::Comma);
        punc_table.insert(".".to_owned(), TokenKind::Point);
        
        punc_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod keyword_tests {
        use super::*;
        #[test]
        fn test_int_keyword() {
            let mut diag_engine = DiagnosticEngine::new();
            let lexer = Lexer::new(String::from("int"), "test.c");
            let tokens = lexer.tokenize(&mut diag_engine);

            assert_eq!(tokens.len(), 1);
            assert_eq!(tokens[0].kind, TokenKind::Int);
            assert_eq!(tokens[0].lexeme, "int");
        }
    }

    mod numeric_tests {
        use super::*;
        #[test]
        fn test_multiple_decimal_points() {
            let mut diag_engine = DiagnosticEngine::new();
            let lexer = Lexer::new(String::from("1.2.3"), "test.c");
            let tokens = lexer.tokenize(&mut diag_engine);

            assert_eq!(diag_engine.error_count, 1);

            for token in tokens {
                println!("{:#?}", token);
            }
        }
    }

}
