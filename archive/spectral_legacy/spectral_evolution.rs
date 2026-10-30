use nalgebra::Vector3;

/// A6.1 Temporal relaxation + eigen‑space attenuation
pub struct SpectralEvolution {
    pub alpha: f64,
    pub history: Vec<Vector3<f64>>,
    pub total_energy: f64,
}

impl SpectralEvolution {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            history: Vec::with_capacity(6),
            total_energy: 0.0,
        }
    }

    pub fn evolve(&mut self, psi_t: Vector3<f64>) -> Vector3<f64> {
        let prev = self.history.last().cloned().unwrap_or_else(Vector3::zeros);
        let evolved = (self.alpha * psi_t) + ((1.0 - self.alpha) * prev);

        self.history.push(evolved);

        if self.history.len() > 5 {
            self.attenuate();
        }

        evolved
    }

    fn attenuate(&mut self) {
        let evacuated = self.history.remove(0);
        self.total_energy += evacuated.norm() * self.alpha;
    }
}
