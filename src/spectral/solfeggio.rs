use nalgebra::Vector3;

/// A6.0 Solfeggio triad field representation
/// Frequencies, amplitudes, and phases define the proto‑spectral input manifold.
#[derive(Clone)]
pub struct SolfeggioField {
    pub frequencies: Vector3<f64>,
    pub amplitudes: Vector3<f64>,
    pub phases: Vector3<f64>,
}

impl SolfeggioField {
    pub fn psi_t(&self, dt: f64) -> Vector3<f64> {
        let mut psi = Vector3::zeros();

        for i in 0..3 {
            let omega = 2.0 * std::f64::consts::PI * self.frequencies[i];
            let phase_angle = (omega * dt) + self.phases[i];
            psi[i] = self.amplitudes[i] * phase_angle.cos();
        }

        psi
    }
}
