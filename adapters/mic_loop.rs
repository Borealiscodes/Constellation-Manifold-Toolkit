// Altitude: A3 Membrane Boundary Interface -> A4.2 Proto-Spectral Runtime
// Hardware audio capture + routing into A6–A7 spectral modules.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use nalgebra::Vector3;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// --- Imported Sovereign-Library spectral modules ---
use crate::spectral::solfeggio::SolfeggioField;
use crate::spectral::spectral_evolution::SpectralEvolution;
use crate::spectral::membrane::Membrane;
use crate::spectral::collapse_surface::CollapseSurface;

/// Unified spectral adapter wrapper (A3–A4 boundary)
pub struct ConstellationAdapter {
    pub solfeggio: SolfeggioField,
    pub evolution: SpectralEvolution,
    pub membrane: Membrane,
    pub collapse: CollapseSurface,
}

impl ConstellationAdapter {
    pub fn new(alpha: f64, tear_threshold: f64) -> Self {
        Self {
            solfeggio: SolfeggioField {
                frequencies: Vector3::new(432.0, 528.0, 639.0),
                amplitudes: Vector3::zeros(),
                phases: Vector3::zeros(),
            },
            evolution: SpectralEvolution::new(alpha),
            membrane: Membrane::new(tear_threshold),
            collapse: CollapseSurface,
        }
    }

    /// A3–A4 boundary: route PCM → SolfeggioField → A6–A7 spectral modules
    pub fn process_tick(&mut self, amplitudes: Vector3<f64>, phases: Vector3<f64>, dt: f64) -> String {
        // Update Solfeggio field
        self.solfeggio.amplitudes = amplitudes;
        self.solfeggio.phases = phases;

        // Compute psi_t (A6.0)
        let psi_t = self.solfeggio.psi_t(dt);

        // Previous spectral state
        let prev = self.evolution.history.last().cloned().unwrap_or_else(Vector3::zeros);

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

/// Hardware Initialization Hook
pub fn start_hardware_mic_loop(adapter: Arc<Mutex<ConstellationAdapter>>) -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| "Failed to find default audio input device".to_string())?;

    let config = device.default_input_config()
        .map_err(|e| format!("Failed to get default input config: {}", e))?;

    let start_time = Instant::now();
    let adapter_clone = Arc::clone(&adapter);

    let stream = device.build_input_stream(
        &config.config(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let dt = start_time.elapsed().as_secs_f64();

            // A3: PCM → amplitude + phase heuristics
            let rms = (data.iter().map(|x| x * x).sum::<f32>() / data.len() as f32).sqrt() as f64;

            let amplitudes = Vector3::new(rms, rms * 0.8, rms * 0.6);
            let phases = Vector3::new(dt * 0.1, dt * 0.15, dt * 0.2);

            if let Ok(mut lock) = adapter_clone.lock() {
                let status = lock.process_tick(amplitudes, phases, dt);

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
