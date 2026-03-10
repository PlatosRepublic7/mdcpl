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
    MultipleDecimalPointsInFloat,
    TrailingDecimalPointInFloat,
    InvalidCharacter,
    Null
}

#[derive(Debug, Clone)]
pub struct DiagnosticLocation {
    source_location: SourceLocation,
    span_end: usize,
    source_line: Option<String>
}

impl DiagnosticLocation {
    pub fn new(source_loc: &SourceLocation, sp: usize, sline: Option<&str>) -> Self {
        DiagnosticLocation {
            source_location: source_loc.clone(),
            span_end: sp,
            source_line: sline.map(|s| s.to_owned())
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
    pub fn new(sev: Severity, diag_kind: DiagnosticKind, message_str: &str, loc: SourceLocation, span_end: usize, source_line: Option<&str>) -> Self {
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

struct DiagnosticRenderer;

impl DiagnosticRenderer {
    fn render(diagnostics_vec: &[Diagnostic]) {
        for diagnostic in diagnostics_vec {
            let source_location_str: String = format!("{}:{}:{}:", diagnostic.location.source_location.file_name, diagnostic.location.source_location.line_num, diagnostic.location.source_location.column_num);
            let mut severity_str: String = String::from("");
            match diagnostic.severity {
                Severity::Error => {
                    severity_str = "error".to_string();
                }
                Severity::Warning => {
                    severity_str = "warning".to_string();
                }
                Severity::Fatal => {
                    severity_str = "fatal".to_string();
                }
            }
            
            let information_str: String = format!("{} {}: {}", source_location_str, severity_str, diagnostic.message);
            eprintln!("{}", information_str);

            if let Some(ref source_line) = diagnostic.location.source_line {
                eprintln!("\t{}", source_line);
                let total_indicator_chars = &(diagnostic.location.source_location.column_num + diagnostic.location.span_end);
                let mut indicator_str: String = String::with_capacity(*total_indicator_chars);
                for i in 0..*total_indicator_chars {
                    if i < diagnostic.location.source_location.column_num {
                        indicator_str.insert(i, ' ');
                    } else if i == diagnostic.location.source_location.column_num {
                        indicator_str.insert(i, '^');
                    } else {
                        indicator_str.insert(i, '~');
                    }
                }
                eprintln!("\t{}", indicator_str);
            }
        }
    }
}

pub struct DiagnosticEngine {
    pub diagnostics_vec: Vec<Diagnostic>,
    pub error_count: u64,
    pub warning_count: u64,
    pub has_fatal: bool,
    pub exit_code: i32
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
            has_fatal: fatal,
            exit_code: 0
        }
    }

    pub fn emit(&mut self, severity: Severity, kind: DiagnosticKind, message: &str, location: SourceLocation, span_end: usize, source_line: Option<&str>) -> bool {
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

    pub fn has_errors(&self) -> bool {
        if self.error_count > 0 {
            return true
        }
        false
    }

    pub fn render(&self) {
        DiagnosticRenderer::render(&self.diagnostics_vec);
    }
}
