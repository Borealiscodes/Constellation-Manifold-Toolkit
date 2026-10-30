//! Phase 7: Collapse-Surface Pre-Activation (Populated)

use crate::invariant::Invariant;

pub struct CollapsePreActivation;

impl CollapsePreActivation {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 7: collapse-surface pre-activation
        // Scalar-only precursor using Phase 6 fields.

        let curvature_energy = inv.curvature_energy;
        let spectral_coupling = inv.spectral_coupling;
        let slack = inv.slack_stability;
        let ndh = inv.ndh_smoothing;

        // Precursor magnitude (scalar)
        let precursor_mag =
            curvature_energy * 0.1 +
            spectral_coupling.abs() * 0.05 +
            slack * 0.02 +
            ndh * 0.03;

        // Soft activation curve (scalar)
        let pre_signal = precursor_mag / (1.0 + precursor_mag.abs());

        // Update invariant
        inv.collapse_pre_signal = pre_signal;
    }
}
