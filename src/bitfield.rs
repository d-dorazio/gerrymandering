use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitfield {
    data: u64,
}

impl Bitfield {
    pub const fn max_size() -> i64 {
        64
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

    pub fn data(&self) -> u64 {
        self.data
    }

    pub fn ones(&self) -> Ones {
        Ones {
            data: self.data,
            ix: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ones {
    data: u64,
    ix: i64,
}

impl std::iter::Iterator for Ones {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let tz = self.data.trailing_zeros();

        if tz == 64 {
            return None;
        }

        self.data = self.data >> (tz + 1);
        self.ix += i64::from(tz) + 1;
        Some(self.ix - 1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Ok(o) = usize::try_from(self.data.count_ones()) {
            (o, Some(o))
        } else {
            (0, Some(64))
        }
    }
}
