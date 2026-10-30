pub trait SpectralAntiDrift {
    type Point;

    fn spectral_integral(&self, p: &Self::Point) -> f64;
    fn antidrift(&self, p: &Self::Point) -> Self::Point;
}
