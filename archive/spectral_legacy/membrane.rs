use nalgebra::Vector3;

/// A6.2 Membrane stress detection + gradient norm evaluation
pub struct Membrane {
    pub tear_threshold: f64,
}

impl Membrane {
    pub fn new(tear_threshold: f64) -> Self {
        Self { tear_threshold }
    }

    pub fn stress_metric(&self, psi_t: Vector3<f64>, prev: Vector3<f64>) -> f64 {
        (psi_t - prev).norm()
    }

    pub fn is_tear(&self, stress: f64) -> bool {
        stress > self.tear_threshold
    }
}
