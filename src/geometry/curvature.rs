pub trait CurvatureInvariant {
    type Point;

    fn curvature(&self, p: &Self::Point) -> f64;
}
