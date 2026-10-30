//! Phase 11: Routing Ecology Coupler (Populated)
//! Constellation-Manifold-Toolkit · Projection-Routing Ecology Layer
//!
//! Couples routing-ecology directional vectors and strength with Phase 10
//! collapse-surface ecology scalars, Phase 9 projection-seed geometry,
//! Phase 8 collapse-surface activation, and Phase 6 derived fields to produce
//! holonomy-ready routing ecology for Phase 12.

use crate::invariant::Invariant;

pub struct RoutingEcologyCoupler;

impl RoutingEcologyCoupler {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 11 coupling uses:
        // - Phase 11 routing-ecology vector + strength
        // - Phase 10 ecology-energy, ecology-curvature, ecology-binding
        // - Phase 9 projection-seed geometry
        // - Phase 8 collapse-surface activation
        // - Phase 6 derived fields

        let vec_x = inv.routing_ecology_vector_x;
        let vec_y = inv.routing_ecology_vector_y;
        let strength = inv.routing_ecology_strength;

        let eco_energy = inv.collapse_ecology_energy;
        let eco_curvature = inv.collapse_ecology_curvature;
        let eco_binding = inv.collapse_ecology_binding;

        let seed_energy = inv.projection_seed_energy;
        let seed_curvature = inv.projection_seed_curvature;

        let collapse_activation = inv.collapse_activation_strength;
        let collapse_curvature = inv.collapse_curvature;

        let curvature_energy = inv.curvature_energy;
        let spectral_coupling = inv.spectral_coupling;

        // Coupled directional components
        let coupled_x =
            vec_x * (1.0 + 0.3 * eco_binding.abs())
            + vec_y * (0.15 * collapse_curvature.abs())
            + strength * 0.35
            + eco_energy * 0.25
            + eco_curvature * 0.2
            + seed_curvature.abs() * 0.15
            + curvature_energy * 0.1;

        let coupled_y =
            vec_y * (1.0 + 0.25 * eco_binding.abs())
            + vec_x * (0.15 * collapse_activation.abs())
            + strength * 0.3
            + eco_energy * 0.25
            + eco_curvature * 0.2
            + seed_energy * 0.15
            + spectral_coupling.abs() * 0.1;

        // Coupled routing strength
        let coupled_strength =
            strength * (1.0 + 0.35 * eco_binding.abs())
            + eco_energy * 0.3
            + eco_curvature * 0.25
            + seed_energy * 0.15
            + seed_curvature.abs() * 0.1;

        // Update invariant
        inv.routing_ecology_vector_x = coupled_x;
        inv.routing_ecology_vector_y = coupled_y;
        inv.routing_ecology_strength = coupled_strength;
    }
}
