//! Phase 11: Projection-Routing Ecology Activation Coordinator (Populated)
//! Constellation-Manifold-Toolkit · Projection-Routing Ecology Layer
//!
//! Coordinates the Phase 11 operator triad:
//!   - RoutingEcologyGenerator
//!   - RoutingEcologyCurvature
//!   - RoutingEcologyCoupler
//!
//! Produces the complete routing-ecology activation for Phase 11.

use crate::invariant::Invariant;
use crate::projection_routing::ecology::routing_ecology_generator::RoutingEcologyGenerator;
use crate::projection_routing::ecology::routing_ecology_curvature::RoutingEcologyCurvature;
use crate::projection_routing::ecology::routing_ecology_coupler::RoutingEcologyCoupler;

pub struct RoutingEcologyActivation;

impl RoutingEcologyActivation {
    pub fn apply(&self, inv: &mut Invariant) {
        // 1. Routing ecology generation
        let generator = RoutingEcologyGenerator;
        generator.apply(inv);

        // 2. Routing ecology curvature refinement
        let curvature = RoutingEcologyCurvature;
        curvature.apply(inv);

        // 3. Routing ecology coupling
        let coupler = RoutingEcologyCoupler;
        coupler.apply(inv);

        // Coordinator introduces no new fields.
        // It ensures Phase 11 routing ecology is activated in altitude-correct order.
    }
}
