pub trait CollapseSurface {
    type Point;

    fn is_on_surface(&self, p: &Self::Point, k: f64) -> bool;
    fn transport_on_surface(&self, p: &Self::Point) -> Self::Point;
}
