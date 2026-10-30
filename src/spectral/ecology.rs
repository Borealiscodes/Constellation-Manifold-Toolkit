//! Phase 5: Spectral Ecology Engine
//! Constellation-Manifold-Toolkit · Research Fork
//! This file introduces spectral ecology: the interaction, stabilization, and
//! feedback behavior between spectral energy, curvature, slack, and NDH
//! displacement across runtime steps.

use crate::invariant::Invariant;

/// Spectral ecology engine.
/// Applies spectral feedback, stabilization, and cross-invariant coupling.
pub struct SpectralEcology;

impl SpectralEcology {
    /// Apply spectral ecology to an invariant.
    /// Phase 5 introduces the first spectral feedback loop.
    pub fn apply(&self, inv: &mut Invariant) {
        // Spectral damping (Phase 5: simple exponential decay)
        inv.spectral_energy *= 0.97;

        // Curvature–spectral coupling
        let coupling = inv.curvature * inv.spectral_energy * 0.02;
        inv.curvature += coupling;

        // Slack spectral modulation
        let slack_modulated = inv
            .slack
            .iter()
            .map(|s| s + inv.spectral_energy * 0.001)
            .collect::<Vec<f64>>();

        inv.slack = slack_modulated;

        // NDH spectral drift correction
        let ndh_corrected = inv
            .ndh_displacement
            .iter()
            .map(|d| d - inv.spectral_energy * 0.0005)
            .collect::<Vec<f64>>();

        inv.ndh_displacement = ndh_corrected;
    }
}
