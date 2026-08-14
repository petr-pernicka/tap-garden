#[derive(Clone, Copy)]
pub struct BitSet32 {
    data: u32,
    size: usize,
}

impl BitSet32 {
    pub fn new(size: usize) -> Self {
        debug_assert!(size <= 32);

        Self { data: 0, size }
    }

    pub fn set(&mut self, index: usize) {
        debug_assert!(index < self.size);

        let mask = 1 << index;
        self.data |= mask;
    }

    pub fn clear(&mut self, index: usize) {
        debug_assert!(index < self.size);

        let mask = 1 << index;
        self.data &= !mask;
    }

    pub fn get(&self, index: usize) -> bool {
        debug_assert!(index < self.size);

        let mask = 1 << index;
        self.data & mask != 0
    }

    pub fn is_empty(&self) -> bool {
        self.data == 0
    }

    pub fn clear_all(&mut self, set: &BitSet32) {
        self.data &= !set.data;
    }

    pub fn iter_ones(&self) -> OneIter {
        OneIter(self.data)
    }
}

pub struct OneIter(u32);

impl Iterator for OneIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            _ => {
                let index = self.0.trailing_zeros() as usize;
                self.0 ^= 1 << index;
                Some(index)
            }
        }
    }
}
