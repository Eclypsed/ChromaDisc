// Realistically this file is temporary and everything in here should be moved elsewhere

pub trait Bit {
    fn bit(self, n: u32) -> bool;
}

impl Bit for u8 {
    #[inline]
    fn bit(self, n: u32) -> bool {
        (self & (1 << n)) != 0
    }
}
