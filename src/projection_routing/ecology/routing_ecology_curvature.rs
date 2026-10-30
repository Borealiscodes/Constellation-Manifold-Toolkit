//! Phase 11: Routing Ecology Curvature Operator (Populated)
//! Constellation-Manifold-Toolkit · Projection-Routing Ecology Layer
//!
//! Refines routing-ecology directional vectors using Phase 11 routing-ecology
//! strength, Phase 10 collapse-surface ecology scalars, Phase 9 projection-seed
//! geometry, Phase 8 collapse-surface activation, and Phase 6 derived fields.

use crate::invariant::Invariant;

pub struct RoutingEcologyCurvature;

impl RoutingEcologyCurvature {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 11 curvature refinement uses:
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

        // Curvature-refined directional components
        let refined_x =
            vec_x * (1.0 + 0.25 * collapse_curvature.abs())
            + strength * 0.3
            + eco_curvature * 0.25
            + eco_binding * 0.2
            + seed_curvature.abs() * 0.15
            + curvature_energy * 0.1;

        let refined_y =
            vec_y * (1.0 + 0.2 * collapse_activation.abs())
            + strength * 0.35
            + eco_energy * 0.25
            + eco_binding * 0.2
