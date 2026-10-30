//! Phase 11: Routing Ecology Generator (Populated)
//! Constellation-Manifold-Toolkit · Projection-Routing Ecology Layer
//!
//! Generates initial routing-ecology directional vectors using Phase 10
//! collapse-surface ecology scalars, Phase 9 projection-seed geometry,
//! Phase 8 collapse-surface activation, and Phase 6 derived fields.

use crate::invariant::Invariant;

pub struct RoutingEcologyGenerator;

impl RoutingEcologyGenerator {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 11 routing ecology uses:
        // - Phase 10 ecology-energy, ecology-curvature, ecology-binding
        // - Phase 9 projection-seed geometry
        // - Phase 8 collapse-surface activation
        // - Phase 6 derived fields

        let eco_energy = inv.collapse_ecology_energy;
        let eco_curvature = inv.collapse_ecology_curvature;
        let eco_binding = inv.collapse_ecology_binding;

        let seed_energy = inv.projection_seed_energy;
        let seed_curvature = inv.projection_seed_curvature;

        let collapse_activation = inv.collapse_activation_strength;
        let collapse_curvature = inv.collapse_curvature;

        let curvature_energy = inv.curvature_energy;
        let spectral_coupling = inv.spectral_coupling;

        // Routing ecology directional components
        let vector_x =
            eco_energy * (0.45 + 0.15 * collapse_activation.abs())
            + eco_curvature * 0.25
            + eco_binding * 0.2
            + seed_energy * 0.15
            + curvature_energy * 0.1;

        let vector_y =
            eco_energy * (0.35 + 0.1 * collapse_curvature.abs())
            + eco_curvature * 0.3
            + eco_binding * 0.25
            + seed_curvature.abs() * 0.2
            + spectral_coupling.abs() * 0.1;

        // Routing ecology strength (scalar)
        let strength =
            eco_binding * (0.4 + 0.2 * collapse_activation.abs())
            + eco_energy * 0.3
            + eco_curvature * 0.25
            + seed_energy * 0.15
            + seed_curvature.abs() * 0.1;

        // Update invariant
        inv.routing_ecology_vector_x = vector_x;
        inv.routing_ecology_vector_y = vector_y;
        inv.routing_ecology_strength = strength;
    }
      }
