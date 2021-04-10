use crate::magic::constants::MAGIC_NUMBERS;
use crate::magic::util::MagicPiece;
use rand::prelude::ThreadRng;
use rand::RngCore;
use std::iter;
use std::slice::Iter;

#[derive(PartialEq)]
pub enum GenerationScheme {
    PseudoRandom,
    PreComputed,
}

pub struct MagicRandomizer {
    random: ThreadRng,
    scheme: GenerationScheme,
    itr: Box<Iter<'static, u64>>,
}

pub trait Random {
    fn gen_random_number(&mut self) -> u64;
}

impl Random for MagicRandomizer {
    fn gen_random_number(&mut self) -> u64 {
        if self.scheme == GenerationScheme::PreComputed {
            return *self.itr.next().unwrap();
        }
        let n1: u64 = self.gen_u64();
        let n2: u64 = self.gen_u64();
        let n3: u64 = self.gen_u64();
        n1 & n2 & n3
    }
}

impl MagicRandomizer {
    pub fn new(scheme: GenerationScheme) -> MagicRandomizer {
        let itr = match scheme {
            GenerationScheme::PseudoRandom => Box::new([0; 0].iter()),
            GenerationScheme::PreComputed => Box::new(MAGIC_NUMBERS.iter()),
        };
        MagicRandomizer {
            random: ThreadRng::default(),
            scheme,
            itr,
        }
    }

    fn gen_u64(&mut self) -> u64 {
        let u1: u64 = self.random.next_u64() & 0xFFFF;
        let u2: u64 = self.random.next_u64() & 0xFFFF;
        let u3: u64 = self.random.next_u64() & 0xFFFF;
        let u4: u64 = self.random.next_u64() & 0xFFFF;
        u1 | (u2 << 16) | (u3 << 32) | (u4 << 48)
    }
}
