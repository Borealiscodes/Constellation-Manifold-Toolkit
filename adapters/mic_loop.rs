// adapters/mic_loop.rs
// Altitude: A4.2 Proto-Spectral Runtime -> A3 Membrane Boundary Interface
// Functional integration script for real-time hardware audio capture.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use nalgebra::Vector3;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Unified Solfeggio resonant frequency triad mapping
pub struct SolfeggioField {
    pub frequencies: Vector3<f64>, // [432.0, 528.0, 639.0] Hz
    pub amplitudes: Vector3<f64>,
    pub phases: Vector3<f64>,
}

/// A6.0 Dual-Algebra aligned memory buffer
pub struct ConstellationAdapter {
    pub alpha: f64,              // Smooth temporal relaxation constant (Fix 3)
    pub tear_threshold: f64,     // Critical Epsilon limit for membrane stress (Fix 9)
    pub spectral_history: Vec<Vector3<f64>>,
    pub total_manifold_energy: f64,
}

impl ConstellationAdapter {
    pub fn new(alpha: f64, tear_threshold: f64) -> Self {
        Self {
            alpha,
            tear_threshold,
            spectral_history: Vec::with_capacity(6),
            total_manifold_energy: 0.0,
        }
    }

    /// Processes high-frequency scalar signal metrics into the geometric substrate
    pub fn process_signal_tick(&mut self, input: &SolfeggioField, dt: f64) -> String {
        // 1. Map current state to complex line bundle cross-section (Fix 1 & 2)
        let mut psi_t = Vector3::zeros();
        for i in 0..3 {
            let omega = 2.0 * std::f64::consts::PI * input.frequencies[i];
            let phase_angle = (omega * dt) + input.phases[i];
            psi_t[i] = input.amplitudes[i] * phase_angle.cos();
        }

        let prev_psi = self.spectral_history.last().cloned().unwrap_or_else(Vector3::zeros);

        // 2. Continuous tear detection via space-time gradient norm (Fix 4)
        let delta_spec = (psi_t - prev_psi).norm();

        // 3. Evaluate the Membrane Envelope Condition (Fix 9 & 10)
        if delta_spec > self.tear_threshold {
            return self.route_to_collapse_surface(psi_t, delta_spec);
        }

        // 4. Apply smooth temporal evolution (Fix 3)
        let evolved_state = (self.alpha * psi_t) + ((1.0 - self.alpha) * prev_psi);
        self.spectral_history.push(evolved_state);

        // 5. Quantize the configuration space via 5-eigenspace tracking (Fix 7 & 8)
        if self.spectral_history.len() > 5 {
            self.attenuate_to_harmonic_sand();
        }

        "STATUS::A6.0_STABLE_FLOW".to_string()
    }

    fn route_to_collapse_surface(&self, _state: Vector3<f64>, stress: f64) -> String {
        // Enforces isolation and routes to A7 non-dual collapse surface boundary
        format!("STATUS::A7.3_COLLAPSE_PROJECTION -> L^2(Σ) | Stress Metric: {:.4}", stress)
    }

    fn attenuate_to_harmonic_sand(&mut self) {
        // Compresses out-of-bounds metrics smoothly into continuous global background history
        let evacuated_wave = self.spectral_history.remove(0);
        self.total_manifold_energy += evacuated_wave.norm() * self.alpha;
    }
}

/// Hardware Initialization Hook: Spawns the physical background thread listener
pub fn start_hardware_mic_loop(adapter: Arc<Mutex<ConstellationAdapter>>) -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| "Failed to find default audio input device".to_string())?;
    
    let config = device.default_input_config()
        .map_err(|e| format!("Failed to get default input config: {}", e))?;

    let start_time = Instant::now();
    let adapter_clone = Arc::clone(&adapter);

    // High-frequency stream callback execution
    let stream = device.build_input_stream(
        &config.config(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let dt = start_time.elapsed().as_secs_f64();
            
            // Simple heuristic mapping raw PCM wave frames to Solfeggio amplitudes
            let rms_amplitude = (data.iter().map(|&x| x * x).sum::<f32>() / data.len() as f32).sqrt() as f64;
            
            let current_field = SolfeggioField {
                frequencies: Vector3::new(432.0, 528.0, 639.0),
                amplitudes: Vector3::new(rms_amplitude, rms_amplitude * 0.8, rms_amplitude * 0.6),
                phases: Vector3::new(dt * 0.1, dt * 0.15, dt * 0.2),
            };

            if let Ok(mut lock) = adapter_clone.lock() {
                let status = lock.process_signal_tick(&current_field, dt);
                if status.contains("A7.3_COLLAPSE") {
                    eprintln!("[RUNTIME ALARM] Structural Boundary Breached: {}", status);
                }
            }
        },
        move |err| {
            eprintln!("[HARDWARE ERROR] cpal stream failure: {}", err);
        },
        None
    ).map_err(|e| format!("Failed to build input stream: {}", e))?;

    stream.play().map_err(|e| format!("Failed to activate stream: {}", e))?;
    Ok(stream)
}
