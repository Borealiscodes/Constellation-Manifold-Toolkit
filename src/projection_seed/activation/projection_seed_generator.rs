//! Phase 9: Projection-Seed Generator (Populated)
//! Constellation-Manifold-Toolkit · Projection-Seed Geometry Layer
//!
//! Generates initial projection-seed geometry using Phase 8 collapse-surface
//! activation outputs and Phase 6 derived spectral/curvature fields.

use crate::invariant::Invariant;

pub struct ProjectionSeedGenerator;

impl ProjectionSeedGenerator {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 9 seed generation uses:
        // - Phase 8 collapse activation fields
        // - Phase 6 derived fields
        // - Phase 4/5 spectral + curvature base fields

        let activation = inv.collapse_activation_strength;
        let collapse_curv = inv.collapse_curvature;
        let collapse_bind = inv.collapse_spectral_binding;

        let spectral_energy = inv.spectral_energy;
        let curvature_energy = inv.curvature_energy;

        // Projection-seed energy (scalar)
        let seed_energy =
            activation * (0.5 + 0.2 * spectral_energy.abs()) * collapse_curv.abs();

        // Projection-seed curvature (scalar)
        let seed_curvature =
            curvature_energy * (0.3 + 0.1 * activation.abs()) * collapse_bind.abs();

        // Update invariant
        inv.projection_seed_energy = seed_energy;
        inv.projection_seed_curvature = seed_curvature;
    }
}
