//! Phase 10: Collapse-Surface Ecology Activation Coordinator (Populated)
//! Constellation-Manifold-Toolkit · Collapse-Surface Ecology Layer
//!
//! Coordinates the Phase 10 operator triad:
//!   - CollapseEcologyGenerator
//!   - CollapseEcologyCurvature
//!   - CollapseEcologyCoupler
//!
//! Produces the complete collapse-surface ecology activation for Phase 10.

use crate::invariant::Invariant;
use crate::collapse::ecology::collapse_ecology_generator::CollapseEcologyGenerator;
use crate::collapse::ecology::collapse_ecology_curvature::CollapseEcologyCurvature;
use crate::collapse::ecology::collapse_ecology_coupler::CollapseEcologyCoupler;

pub struct CollapseEcologyActivation;

impl CollapseEcologyActivation {
    pub fn apply(&self, inv: &mut Invariant) {
        // 1. Ecology generation
        let generator = CollapseEcologyGenerator;
        generator.apply(inv);

        // 2. Ecology curvature refinement
        let curvature = CollapseEcologyCurvature;
        curvature.apply(inv);

        // 3. Ecology coupling
        let coupler = CollapseEcologyCoupler;
        coupler.apply(inv);

        // Coordinator introduces no new fields.
        // It ensures Phase 10 ecology is activated in altitude-correct order.
    }
}
