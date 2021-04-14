#[derive(Copy, Clone)]
pub struct Castle {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

impl Castle {
    pub fn default() -> Castle {
        Castle {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }
}
