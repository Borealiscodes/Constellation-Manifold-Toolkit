// Altitude: A3 Membrane Boundary Interface -> A4.2 Proto-Spectral Runtime
// Hardware audio capture + coupled 3x3 modal routing into A6–A7 spectral modules.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use nalgebra::{Matrix3, Vector3};
use std::sync::{Arc, Mutex};

// --- Imported Sovereign-Library spectral modules ---
use crate::spectral::collapse_surface::CollapseSurface;
use crate::spectral::membrane::Membrane;
use crate::spectral::spectral_evolution::SpectralEvolution;

/// Coupled 3-mode spectral operator M(alpha, beta) = I_3 - alpha * L(beta).
/// Replaces hardcoded Solfeggio constants with emergent standing-wave dynamics.
pub struct ModalCouplingField {
    pub alpha: f64,
    pub beta: f64,
    pub tau_safe: f64,
    pub state: Vector3<f64>,
}

impl ModalCouplingField {
    pub fn new(alpha: f64, beta: f64, tau_safe: f64) -> Self {
        Self {
            alpha,
            beta,
            tau_safe,
            state: Vector3::zeros(),
        }
    }

    /// Construct M(alpha, beta) from symmetric ring consensus + skew-symmetric chiral rotation.
    pub fn coupling_matrix(&self) -> Matrix3<f64> {
        let w_sym = Matrix3::new(
            0.0, 0.5, 0.5,
            0.5, 0.0, 0.5,
            0.5, 0.5, 0.0,
        );
        let w_skew = Matrix3::new(
             0.0,  1.0, -1.0,
            -1.0,  0.0,  1.0,
             1.0, -1.0,  0.0,
        );
        let w = w_sym + self.beta * w_skew;
        let laplacian = Matrix3::identity() - w;
        Matrix3::identity() - self.alpha * laplacian
    }

    /// Advance modal state x_{n+1} = M_eff * x_n + u_n while clamping spectral radius <= tau_safe.
    pub fn step(&mut self, stimulus: Vector3<f64>) -> (Vector3<f64>, f64, bool) {
        let m = self.coupling_matrix();

        // 8-step power iteration to bound spectral radius rho(M)
        let mut v = Vector3::new(1.0, 1.0, 1.0).normalize();
        for _ in 0..8 {
            let mv = m * v;
            let nrm = mv.norm();
            if nrm > 1e-15 {
                v = mv / nrm;
            }
        }
        let rho_est = (m * v).norm();

        let (m_eff, damped) = if rho_est > self.tau_safe {
            let scale = self.tau_safe / rho_est;
            (Matrix3::identity() + scale * (m - Matrix3::identity()), true)
        } else {
            (m, false)
        };

        self.state = m_eff * self.state + stimulus;
        (self.state, rho_est, damped)
    }
}

/// Unified spectral adapter wrapper (A3–A4 boundary)
pub struct ConstellationAdapter {
    pub modal_field: ModalCouplingField,
    pub evolution: SpectralEvolution,
    pub membrane: Membrane,
    pub collapse: CollapseSurface,
}

impl ConstellationAdapter {
    pub fn new(alpha: f64, beta: f64, tear_threshold: f64) -> Self {
        Self {
            modal_field: ModalCouplingField::new(alpha, beta, 0.96),
            evolution: SpectralEvolution::new(alpha),
            membrane: Membrane::new(tear_threshold),
            collapse: CollapseSurface,
        }
    }

    /// A3–A4 boundary: route 3-band PCM stimulus -> ModalCouplingField -> A6–A7 spectral modules
    pub fn process_tick(&mut self, stimulus: Vector3<f64>) -> String {
        // Compute emergent modal standing-wave state psi_t (A6.0) with Dream-Lane damping
        let (psi_t, _rho, _damped) = self.modal_field.step(stimulus);

        // Previous spectral state
        let prev = self
            .evolution
            .history
            .last()
            .cloned()
            .unwrap_or_else(Vector3::zeros);

        // Membrane stress metric (A6.2)
        let stress = self.membrane.stress_metric(psi_t, prev);

        // Collapse surface routing (A7.3)
        if self.membrane.is_tear(stress) {
            return self.collapse.route(stress, psi_t);
        }

        // Temporal relaxation + eigen-space attenuation (A6.1)
        self.evolution.evolve(psi_t);

        "STATUS::A6.0_STABLE_FLOW".to_string()
    }
}

/// Partition incoming PCM buffer into 3 sub-frame RMS energy components (replaces fake 0.8/0.6 multipliers).
fn extract_three_band_rms(data: &[f32]) -> Vector3<f64> {
    if data.is_empty() {
        return Vector3::zeros();
    }
    let chunk_len = (data.len() / 3).max(1);
    let mut bands = [0.0_f64; 3];
    for (i, band) in bands.iter_mut().enumerate() {
        let start = i * chunk_len;
        if start >= data.len() {
            break;
        }
        let end = if i == 2 {
            data.len()
        } else {
            ((i + 1) * chunk_len).min(data.len())
        };
        let slice = &data[start..end];
        let sum_sq: f32 = slice.iter().map(|x| x * x).sum();
        *band = (sum_sq / slice.len() as f32).sqrt() as f64;
    }
    Vector3::new(bands[0], bands[1], bands[2])
}

/// Hardware Initialization Hook
pub fn start_hardware_mic_loop(
    adapter: Arc<Mutex<ConstellationAdapter>>,
) -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| "Failed to find default audio input device".to_string())?;

    let config = device
        .default_input_config()
        .map_err(|e| format!("Failed to get default input config: {}", e))?;

    let adapter_clone = Arc::clone(&adapter);

    let stream = device
        .build_input_stream(
            &config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let stimulus = extract_three_band_rms(data);

                if let Ok(mut lock) = adapter_clone.lock() {
                    let status = lock.process_tick(stimulus);

                    if status.contains("A7.3_COLLAPSE") {
                        eprintln!("[RUNTIME ALARM] Structural Boundary Breached: {}", status);
                    }
                }
            },
            move |err| {
                eprintln!("[HARDWARE ERROR] cpal stream failure: {}", err);
            },
            None,
        )
        .map_err(|e| format!("Failed to build input stream: {}", e))?;

    stream
        .play()
        .map_err(|e| format!("Failed to activate stream: {}", e))?;
    Ok(stream)
}
