//! Phase 9: Projection-Seed Activation Coordinator (Populated)
//! Constellation-Manifold-Toolkit · Projection-Seed Geometry Layer
//!
//! Coordinates the Phase 9 operator triad:
//!   - ProjectionSeedGenerator
//!   - ProjectionSeedCurvature
//!   - ProjectionSeedCoupler
//!
//! Produces the complete projection-seed geometry activation for Phase 9.

use crate::invariant::Invariant;
use crate::projection_seed::activation::projection_seed_generator::ProjectionSeedGenerator;
use crate::projection_seed::activation::projection_seed_curvature::ProjectionSeedCurvature;
use crate::projection_seed::activation::projection_seed_coupler::ProjectionSeedCoupler;

pub struct ProjectionSeedActivation;

impl ProjectionSeedActivation {
    pub fn apply(&self, inv: &mut Invariant) {
        // 1. Projection-seed generation
        let generator = ProjectionSeedGenerator;
        generator.apply(inv);

        // 2. Projection-seed curvature refinement
        let curvature = ProjectionSeedCurvature;
        curvature.apply(inv);

        // 3. Projection-seed collapse coupling
        let coupler = ProjectionSeedCoupler;
        coupler.apply(inv);

        // Coordinator introduces no new fields.
        // It ensures Phase 9 geometry is activated in altitude-correct order.
    }
}
