//! Phase 10: Collapse-Surface Ecology Coupler (Populated)
//! Constellation-Manifold-Toolkit · Collapse-Surface Ecology Layer
//!
//! Couples ecology-energy and ecology-curvature with Phase 9 projection-seed
//! geometry and Phase 8 collapse-surface activation to produce the final
//! Phase 10 ecological binding scalar.

use crate::invariant::Invariant;

pub struct CollapseEcologyCoupler;

impl CollapseEcologyCoupler {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 10 coupling uses:
        // - Phase 10 ecology-energy + ecology-curvature
        // - Phase 9 projection-seed geometry
        // - Phase 8 collapse-surface activation
        // - Phase 6 derived fields

        let ecology_energy = inv.collapse_ecology_energy;
        let ecology_curvature = inv.collapse_ecology_curvature;

        let seed_energy = inv.projection_seed_energy;
        let seed_curvature = inv.projection_seed_curvature;
        let seed_binding = inv.projection_seed_binding;

        let collapse_activation = inv.collapse_activation_strength;
        let collapse_curvature = inv.collapse_curvature;
        let collapse_binding = inv.collapse_spectral_binding;

        let curvature_energy = inv.curvature_energy;
        let spectral_coupling = inv.spectral_coupling;

        // Final ecology binding (scalar)
        let ecology_binding =
            ecology_energy * (0.4 + 0.2 * collapse_activation.abs())
            + ecology_curvature * (0.3 + 0.15 * collapse_curvature.abs())
            + seed_binding.abs() * 0.25
            + seed_curvature.abs() * 0.2
            + seed_energy * 0.15
            + curvature_energy * 0.1
            + spectral_coupling.abs() * 0.1
            + collapse_binding.abs() * 0.2;

        // Update invariant
        inv.collapse_ecology_binding = ecology_binding;
    }
}
