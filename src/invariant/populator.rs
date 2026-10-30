//! Invariant Population Logic
//! Constellation-Manifold-Toolkit · Runtime Layer
//!
//! Phase 4: Introduces initial derived invariant quantities and stabilizes
//! values across runtime steps. These values are computed internally but
//! were not yet formalized as first-class struct fields.
//!
//! Phase 6: Expands the invariant struct and formalizes derived quantities
//! as canonical runtime fields. Phase 6 does not replace Phase 4; it builds
//! on top of it. Both phases coexist to preserve altitude-layer lineage.

use crate::invariant::Invariant;

/// Invariant populator.
/// Computes derived invariant quantities and stabilizes values.
pub struct InvariantPopulator;

impl InvariantPopulator {
    // ---------------------------------------------------------------------
    // Phase 4: Initial Population Logic (Historical — preserved exactly)
    // ---------------------------------------------------------------------
    /// Populate and refine invariants.
    /// Phase 4 introduces the first derived invariant fields internally.
    pub fn populate_phase4(&self, inv: &mut Invariant) {
        // Derived curvature energy (Phase 4 internal)
        let curvature_energy = inv.curvature.abs() * 0.5;

        // Derived spectral curvature coupling (Phase 4 internal)
        let spectral_coupling = inv.spectral_energy * inv.curvature * 0.01;

        // Slack stabilization (Phase 4: simple normalization)
        let slack_norm: f64 = inv.slack.iter().map(|x| x.abs()).sum();
        let slack_stable = if slack_norm > 0.0 {
            inv.slack.iter().map(|x| x / slack_norm).collect()
        } else {
            inv.slack.clone()
        };

        // NDH displacement smoothing (Phase 4 internal)
        let ndh_smooth = inv
            .ndh_displacement
            .iter()
            .map(|d| d * 0.9)
            .collect::<Vec<f64>>();

        // Update invariant (Phase 4 stabilization)
        inv.slack = slack_stable;
        inv.ndh_displacement = ndh_smooth;

        // Phase 4 derived fields (formalized as struct fields in Phase 6)
        let _derived_curvature_energy = curvature_energy;
        let _derived_spectral_coupling = spectral_coupling;
    }

    // ---------------------------------------------------------------------
    // Phase 6: Derived Field Alignment (New — struct expansion)
    // ---------------------------------------------------------------------
    /// Populate Phase 6 derived invariant fields.
    /// These values become first-class struct fields in Phase 6.
    pub fn populate_phase6(&self, inv: &mut Invariant) {
        // --- curvature_energy ---
        inv.curvature_energy = compute_curvature_energy(
            inv.curvature,
            inv.spectral_energy,
            inv.membrane_tension,
        );

        // --- spectral_coupling ---
        inv.spectral_coupling = compute_spectral_coupling(
            inv.spectral_energy,
            inv.curvature,
            inv.slack_value(), // helper for scalar slack
        );

        // --- slack_stability ---
        inv.slack_stability = compute_slack_stability(
            inv.slack_value(),
            inv.spectral_energy,
        );

        // --- ndh_smoothing ---
        inv.ndh_smoothing = compute_ndh_smoothing(
            inv.ndh_displacement.clone(),
        );

        // --- membrane_tension (optional) ---
        inv.membrane_tension = compute_membrane_tension(
            inv.spectral_energy,
            inv.slack_value(),
            inv.curvature,
        );
    }
}

// -------------------------------------------------------------------------
// Phase 6 Compute Functions (Scaffolds — formulas added incrementally)
// -------------------------------------------------------------------------

fn compute_curvature_energy(
    curvature: f64,
    spectral_energy: f64,
    membrane_tension: Option<f64>,
) -> f64 {
    // Placeholder Phase 6 formula (to be refined)
    curvature.abs() * spectral_energy * 0.1
        + membrane_tension.unwrap_or(0.0) * 0.05
}

fn compute_spectral_coupling(
    spectral_energy: f64,
    curvature: f64,
    slack_scalar: f64,
) -> f64 {
    // Placeholder Phase 6 formula (to be refined)
    spectral_energy * curvature * (1.0 + slack_scalar)
}

fn compute_slack_stability(slack_scalar: f64, spectral_energy: f64) -> f64 {
    // Placeholder Phase 6 formula (to be refined)
    (slack_scalar.abs() + spectral_energy * 0.01).max(0.0)
}

fn compute_ndh_smoothing(ndh: Vec<f64>) -> f64 {
    // Placeholder Phase 6 formula (to be refined)
    ndh.iter().map(|d| d.abs()).sum::<f64>() * 0.1
}

fn compute_membrane_tension(
    spectral_energy: f64,
    slack_scalar: f64,
    curvature: f64,
) -> Option<f64> {
    // Placeholder Phase 6 formula (to be refined)
    Some(spectral_energy * 0.05 + slack_scalar * 0.02 + curvature.abs() * 0.01)
}
