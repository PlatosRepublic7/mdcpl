#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file_name: String,
    pub line_num: usize,
    pub column_num: usize
}

impl SourceLocation {
    pub fn new(fname: &String, line: usize, col: usize) -> Self {
        SourceLocation {
            file_name: fname.clone(),
            line_num: line,
            column_num: col
        }
    }
}
