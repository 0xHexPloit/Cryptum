
pub trait Compress {
    fn compress(self, d_value: u32) -> Self;
}

pub trait Decompress {
    fn decompress(self, d_value: u32) -> Self;
}