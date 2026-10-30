//! Phase 4: Invariant Population Logic
//! Constellation-Manifold-Toolkit · Research Fork
//! This file introduces the invariant population layer. It enriches the
//! invariant container with derived quantities and stabilizes values across
//! runtime steps.

use crate::invariant::Invariant;

/// Invariant populator.
/// Computes derived invariant quantities and stabilizes values.
pub struct InvariantPopulator;

impl InvariantPopulator {
    /// Populate and refine invariants.
    /// Phase 4 introduces the first derived invariant fields.
    pub fn populate(&self, inv: &mut Invariant) {
        // Derived curvature energy
        let curvature_energy = inv.curvature.abs() * 0.5;

        // Derived spectral curvature coupling
        let spectral_coupling = inv.spectral_energy * inv.curvature * 0.01;

        // Slack stabilization (Phase 4: simple normalization)
        let slack_norm: f64 = inv.slack.iter().map(|x| x.abs()).sum();
        let slack_stable = if slack_norm > 0.0 {
            inv.slack.iter().map(|x| x / slack_norm).collect()
        } else {
            inv.slack.clone()
        };

        // NDH displacement smoothing
        let ndh_smooth = inv
            .ndh_displacement
            .iter()
            .map(|d| d * 0.9)
            .collect::<Vec<f64>>();

        // Update invariant
        inv.slack = slack_stable;
        inv.ndh_displacement = ndh_smooth;

        // Phase 4 derived fields (added to struct in Phase 5)
        // For now, they are logged or used internally.
        let _derived_curvature_energy = curvature_energy;
        let _derived_spectral_coupling = spectral_coupling;
    }
}
