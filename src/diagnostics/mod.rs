use crate::source::SourceLocation;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Severity {
    Fatal,
    Error,
    Warning
}

pub enum DiagnosticKind {
    Lexer(LexerDiagnosticKind)
}

pub enum LexerDiagnosticKind {
    UnterminatedStringLiteral,
    InvalidIdentifier,
}

#[derive(Clone)]
pub struct DiagnosticLocation {
    source_location: SourceLocation,
    span_end: usize,
    source_line: String
}

impl DiagnosticLocation {
    pub fn new(source_loc: &SourceLocation, sp: usize, sline: &str) -> Self {
        DiagnosticLocation {
            source_location: source_loc.clone(),
            span_end: sp,
            source_line: sline.to_owned()
        }
    }
}

pub struct Diagnostic {
    severity: Severity,
    kind: DiagnosticKind,
    message : String,
    location: DiagnosticLocation,
    children: Vec<Diagnostic>
}

impl Diagnostic {
    pub fn new(sev: Severity, diag_kind: DiagnosticKind, message_str: &str, loc: SourceLocation, span_end: usize, source_line: &str) -> Self {
        let children_vec: Vec<Diagnostic> = Vec::new();
        let diag_loc: DiagnosticLocation = DiagnosticLocation::new(&loc, span_end, source_line);
        Diagnostic {
            severity: sev,
            kind: diag_kind,
            message: message_str.to_owned(),
            location: diag_loc,
            children: children_vec
        }
    }
}

pub struct DiagnosticEngine {
    diagnostics_vec: Vec<Diagnostic>,
    error_count: u64,
    warning_count: u64,
    has_fatal: bool
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        let diag_vec: Vec<Diagnostic> = Vec::new();
        let err_count: u64 = 0;
        let war_count: u64 = 0;
        let fatal = false;

        DiagnosticEngine {
            diagnostics_vec: diag_vec,
            error_count: err_count,
            warning_count: war_count,
            has_fatal: fatal
        }
    }

    pub fn emit(&mut self, severity: Severity, kind: DiagnosticKind, message: &str, location: SourceLocation, span_end: usize, source_line: &str) -> bool {
        let diagnostic: Diagnostic = Diagnostic::new(severity, kind, message, location, span_end, source_line);
        let cur_severity = diagnostic.severity.clone();
        self.diagnostics_vec.push(diagnostic);

        match cur_severity {
            Severity::Error => {
                self.error_count += 1;
            },
            Severity::Warning => {
                self.warning_count += 1;
            },
            Severity::Fatal => {
                self.has_fatal = true;
            }
        }

        if self.has_fatal {
            return false
        }

        true
    }
}
