pub struct BitReader(pub u8);

impl BitReader {
    #[inline]
    pub fn bit(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }
}
