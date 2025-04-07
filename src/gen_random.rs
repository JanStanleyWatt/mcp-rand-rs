//! 実際に乱数を生成する

use rand::Rng;
use rand::rngs::ThreadRng;

pub struct GenRandom {
    rng: ThreadRng,
}
impl GenRandom {
    pub fn new() -> Self {
        GenRandom { rng: rand::rng() }
    }

    /// 0以上max未満の整数を生成する
    pub fn gen_int(&mut self, max: usize) -> usize {
        self.rng.random_range(0..max)
    }

    /// min以上max未満の整数を生成する
    pub fn gen_int_range(&mut self, min: usize, max: usize) -> usize {
        self.rng.random_range(min..max)
    }
}
impl Default for GenRandom {
    fn default() -> Self {
        Self::new()
    }
}
