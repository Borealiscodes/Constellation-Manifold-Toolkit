//! Phase 9: Projection-Seed Collapse Coupler (Populated)
//! Constellation-Manifold-Toolkit · Projection-Seed Geometry Layer
//!
//! Couples projection-seed geometry with collapse-surface activation to
//! produce the final Phase 9 binding scalar.

use crate::invariant::Invariant;

pub struct ProjectionSeedCoupler;

impl ProjectionSeedCoupler {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 9 coupling uses:
        // - Phase 9 seed-energy + seed-curvature
        // - Phase 8 collapse activation fields
        // - Phase 6 spectral-coupling + curvature-energy

        let seed_energy = inv.projection_seed_energy;
        let seed_curvature = inv.projection_seed_curvature;

        let collapse_activation = inv.collapse_activation_strength;
        let collapse_curv = inv.collapse_curvature;
        let collapse_bind = inv.collapse_spectral_binding;

        let spectral_coupling = inv.spectral_coupling;
        let curvature_energy = inv.curvature_energy;

        // Projection-seed binding (scalar)
        let binding =
            seed_energy * (0.3 + 0.2 * collapse_activation.abs())
            + seed_curvature * (0.25 + 0.15 * collapse_curv.abs())
            + spectral_coupling.abs() * 0.1
            + curvature_energy * 0.05
            + collapse_bind.abs() * 0.2;

        // Update invariant
        inv.projection_seed_binding = binding;
    }
}
