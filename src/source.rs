#[derive(Clone)]
pub struct SourceLocation {
    file_name: String,
    line_num: u64,
    column_num: u64
}

impl SourceLocation {
    pub fn new(fname: &String, line: u64, col: u64) -> Self {
        SourceLocation {
            file_name: fname.clone(),
            line_num: line,
            column_num: col
        }
    }
}
