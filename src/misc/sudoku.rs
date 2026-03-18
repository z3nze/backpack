pub struct SudokuGame {
    layout: [[u8; 9]; 9]
}

impl SudokuGame {
    pub fn from_string_vec(sv: Vec<String>) -> Self {
        SudokuGame { layout: [[0; 9]; 9] }
    }
}
