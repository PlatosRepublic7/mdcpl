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
}

pub struct DiagnosticLocation {
    filename: String,
    line_number: u64,
    column_number: u64,
    span: (u16, u16),
    source_line: String
}

pub struct Diagnostic {
    severity: Severity,
    kind: DiagnosticKind,
    message : String,
    location: DiagnosticLocation,
    children: Vec<Diagnostic>
}

impl Diagnostic {
    pub fn new(sev: Severity, diag_kind: DiagnosticKind, message_str: String, loc: DiagnosticLocation) -> Diagnostic {
        let children_vec: Vec<Diagnostic> = Vec::new();
        Diagnostic {
            severity: sev,
            kind: diag_kind,
            message: message_str,
            location: loc,
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
    pub fn new() -> DiagnosticEngine {
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

    pub fn emit(&mut self, severity: Severity, kind: DiagnosticKind, message: String, location: DiagnosticLocation) -> bool {
        let diagnostic: Diagnostic = Diagnostic::new(severity, kind, message, location);
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
