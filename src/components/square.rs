pub type Square = u8;

pub fn rank_file_to_index(rank: u8, file: u8) -> Square {
    8 * rank + file
}

/*pub fn index_to_bit_index(index: u8) -> u8 {
    let file_index = index % 8;
    let rank_index = index / 8;
    (7 - file_index) + 8 * rank_index
}
*/
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

    /*    #[test]
        fn converts_h1_to_bit_index() {
            let index = algebraic_to_square("h1");
            let bit_index = index_to_bit_index(index);
            assert_eq!(bit_index, 0);
        }
    */
    /*    #[test]
        fn converts_a1_to_bit_index() {
            let index = algebraic_to_square("a1");
            let bit_index = index_to_bit_index(index);
            assert_eq!(bit_index, 7);
        }

        #[test]
        fn converts_e4_to_bit_index() {
            let index = algebraic_to_square("e4");
            let bit_index = index_to_bit_index(index);
            assert_eq!(bit_index, 27);
        }
    */
}
