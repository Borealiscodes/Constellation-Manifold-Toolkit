pub trait FsfeManifold {
    type Point;

    fn metric(&self, p: &Self::Point) -> f64;
    fn slack(&self, p: &Self::Point) -> Vec<f64>;
    fn curvature(&self, p: &Self::Point) -> f64;
    fn spectral(&self, p: &Self::Point) -> f64;
}
