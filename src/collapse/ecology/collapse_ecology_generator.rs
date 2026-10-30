//! Phase 10: Collapse-Surface Ecology Generator (Populated)
//! Constellation-Manifold-Toolkit · Collapse-Surface Ecology Layer
//!
//! Generates initial collapse-surface ecology energy using Phase 9
//! projection-seed geometry and Phase 8 collapse-surface activation.

use crate::invariant::Invariant;

pub struct CollapseEcologyGenerator;

impl CollapseEcologyGenerator {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 10 ecology generation uses:
        // - Phase 9 projection-seed geometry
        // - Phase 8 collapse-surface activation
        // - Phase 6 derived fields

        let seed_energy = inv.projection_seed_energy;
        let seed_curvature = inv.projection_seed_curvature;
        let seed_binding = inv.projection_seed_binding;

        let collapse_activation = inv.collapse_activation_strength;
        let collapse_curvature = inv.collapse_curvature;
        let collapse_binding = inv.collapse_spectral_binding;

        let curvature_energy = inv.curvature_energy;
        let spectral_coupling = inv.spectral_coupling;

        // Initial ecology energy (scalar)
        let ecology_energy =
            seed_energy * (0.4 + 0.2 * collapse_activation.abs())
            + seed_curvature.abs() * (0.3 + 0.1 * collapse_curvature.abs())
            + seed_binding.abs() * 0.25
            + curvature_energy * 0.15
            + spectral_coupling.abs() * 0.1
            + collapse_binding.abs() * 0.2;

        // Update invariant
        inv.collapse_ecology_energy = ecology_energy;
    }
}
