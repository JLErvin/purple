use crate::magic::util::MagicPiece;
use rand::prelude::ThreadRng;
use rand::RngCore;

pub struct MagicRandomizer {
    random: ThreadRng,
}

impl MagicRandomizer {
    pub fn new<'a>() -> MagicRandomizer {
        MagicRandomizer {
            random: ThreadRng::default(),
        }
    }

    pub fn gen_random_number(&mut self) -> u64 {
        let n1: u64 = self.gen_u64();
        let n2: u64 = self.gen_u64();
        let n3: u64 = self.gen_u64();
        n1 & n2 & n3
    }

    fn gen_u64(&mut self) -> u64 {
        let u1: u64 = self.random.next_u64() & 0xFFFF;
        let u2: u64 = self.random.next_u64() & 0xFFFF;
        let u3: u64 = self.random.next_u64() & 0xFFFF;
        let u4: u64 = self.random.next_u64() & 0xFFFF;
        u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
    }
}
