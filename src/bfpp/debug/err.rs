pub struct BfppError {
    pub message: String,
    pub errcode: u8,
    pub file: String,
    pub line: usize
}

pub struct BfppWarning {
    pub message: String,
    pub file: String,
    pub line: usize
}