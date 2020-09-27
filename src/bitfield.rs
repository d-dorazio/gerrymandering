#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitfield {
    data: u128,
}

impl Bitfield {
    pub const fn max_size() -> i64 {
        128
    }

    pub fn new() -> Self {
        Self { data: 0 }
    }

    pub fn set(&mut self, i: i64) {
        self.data |= 1 << i;
    }

    pub fn clear(&mut self, i: i64) {
        self.data &= !(1 << i);
    }

    pub fn toggle(&mut self, i: i64) {
        self.data ^= 1 << i;
    }

    pub fn get(&self, i: i64) -> bool {
        (self.data & (1 << i)) != 0
    }

    pub fn data(&self) -> u128 {
        self.data
    }
}
