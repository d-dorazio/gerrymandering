use crate::bitfield::Bitfield;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub(crate) data: Bitfield,
    width: i64,
    height: i64,
}

impl Board {
    pub fn new(width: i64, height: i64) -> Self {
        assert!(width >= 0);
        assert!(height >= 0);

        // TODO: lift this restriction with variable width bitfields
        assert!(width * height <= Bitfield::max_size());

        Self {
            data: Bitfield::new(),
            width,
            height,
        }
    }

    pub fn area(&self) -> i64 {
        self.width * self.height
    }
    pub fn width(&self) -> i64 {
        self.width
    }
    pub fn height(&self) -> i64 {
        self.height
    }

    pub fn toggle(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.data.toggle(y * self.width + x);
            }
        }
    }

    pub fn set(&mut self, x: i64, y: i64) {
        self.data.set(y * self.width + x)
    }

    pub fn clear(&mut self, x: i64, y: i64) {
        self.data.clear(y * self.width + x)
    }

    pub fn get(&self, x: i64, y: i64) -> bool {
        self.data.get(y * self.width + x)
    }
}
