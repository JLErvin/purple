pub type Square = u8;

pub fn rank_file_to_index(rank: u8, file: u8) -> Square {
    (rank - 1) * 8 + (8 - file)
}

pub fn algebraic_to_square(alg: &str) -> Square {
    let mut s = alg.chars();
    let file = s.next().unwrap();
    let rank = s.next().unwrap();
    let file = match file as char {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        _ => 0,
    };
    let rank = char::to_digit(rank, 10).unwrap() as u8;
    rank_file_to_index(rank, file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_e4_to_square() {
        let index = algebraic_to_square("e4");
        assert_eq!(index, 27);
    }

    #[test]
    fn converts_a8_to_square() {
        let index = algebraic_to_square("a8");
        assert_eq!(index, 63);
    }
}
