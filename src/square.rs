pub type Square = u8;

#[allow(dead_code)]
pub enum SquareIndex {
    A1 = 0,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

pub fn rank_file_to_index(rank: u8, file: u8) -> Square {
    8 * rank + file
}

pub fn square_to_file(s: Square) -> u8 {
    s % 8
}

#[allow(dead_code)]
pub fn square_to_rank(s: Square) -> u8 {
    s % 8
}

pub fn algebraic_to_square(alg: &str) -> Square {
    let mut s = alg.chars();
    let file = s.next().unwrap();
    let rank = s.next().unwrap();
    let file = match file as char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 0,
    };
    let rank = char::to_digit(rank, 10).unwrap() as u8;
    rank_file_to_index(rank - 1, file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_rank_file_to_index() {
        let a1 = rank_file_to_index(0, 0);
        let b1 = rank_file_to_index(0, 1);
        let h1 = rank_file_to_index(0, 7);
        let a3 = rank_file_to_index(2, 0);
        let h8 = rank_file_to_index(7, 7);
        assert_eq!(a1, 0);
        assert_eq!(b1, 1);
        assert_eq!(h1, 7);
        assert_eq!(a3, 16);
        assert_eq!(h8, 63);
    }

    #[test]
    fn converts_e4_to_square() {
        let index = algebraic_to_square("e4");
        assert_eq!(index, 28);
    }

    #[test]
    fn converts_a8_to_square() {
        let index = algebraic_to_square("a8");
        assert_eq!(index, 56);
    }

    #[test]
    fn converts_a4_to_file() {
        let square = algebraic_to_square("a4");
        println!("{}", square);
        let file = square_to_file(square);
        assert_eq!(file, 0);
    }

    #[test]
    fn converts_b4_to_file() {
        let square = algebraic_to_square("b4");
        println!("{}", square);
        let file = square_to_file(square);
        assert_eq!(file, 1);
    }
}
