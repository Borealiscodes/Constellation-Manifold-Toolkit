pub trait TernaryKernelAdapter {
    type Point;

    fn from_seed(&self, seed: u64) -> Self::Point;
    fn to_trit(&self, p: &Self::Point) -> i64;
}
