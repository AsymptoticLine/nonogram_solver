#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Filled,
}

impl Cell {
    pub fn to_pattern(&self) -> CellPattern {
        match self {
            Self::Empty => CellPattern::Empty,
            Self::Filled => CellPattern::Filled,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellPattern {
    Empty,
    Uncertain,
    Filled,
}

impl CellPattern {
    pub fn merge(x: Self, y: Self) -> Self {
        match (x, y) {
            (Self::Empty, Self::Empty) => Self::Empty,
            (Self::Filled, Self::Filled) => Self::Filled,
            _ => Self::Uncertain,
        }
    }

    pub fn merge_known(x: Self, y: Self) -> Result<Self, Box<dyn std::error::Error>> {
        match (x, y) {
            (Self::Empty, Self::Empty) => Ok(Self::Empty),
            (Self::Filled, Self::Filled) => Ok(Self::Filled),
            (Self::Uncertain, a) | (a, Self::Uncertain) => Ok(a),
            _ => Err(format!("Cannot merge {:?} and {:?}.", x, y).into()),
        }
    }
}
