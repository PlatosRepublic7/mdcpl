#[derive(Clone)]
pub struct SourceLocation {
    file_name: String,
    line_num: usize,
    column_num: usize
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
