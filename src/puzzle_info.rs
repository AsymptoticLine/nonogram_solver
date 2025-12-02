#[derive(Debug)]
pub struct PuzzleInfo {
    width: usize,
    height: usize,
    row_blocks: Vec<Vec<usize>>,
    col_blocks: Vec<Vec<usize>>,
}

impl PuzzleInfo {
    pub fn new(
        width: usize,
        height: usize,
        row_blocks: Vec<Vec<usize>>,
        col_blocks: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            width,
            height,
            row_blocks,
            col_blocks,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn row_blocks(&self) -> &Vec<Vec<usize>> {
        &self.row_blocks
    }

    pub fn col_blocks(&self) -> &Vec<Vec<usize>> {
        &self.col_blocks
    }
}
