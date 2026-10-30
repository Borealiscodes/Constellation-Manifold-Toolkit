//! Phase 8: Collapse-Surface Activation Coordinator (Populated)
//! Constellation-Manifold-Toolkit · Collapse Activation Layer
//!
//! Coordinates the Phase 8 operator triad:
//!   - CollapseCurvature
//!   - CollapseSpectralCoupler
//!
//! Produces the complete collapse-surface activation output for Phase 8.

use crate::invariant::Invariant;
use crate::collapse::activation::collapse_curvature::CollapseCurvature;
use crate::collapse::activation::collapse_spectral_coupler::CollapseSpectralCoupler;

pub struct CollapseActivation;

impl CollapseActivation {
    pub fn apply(&self, inv: &mut Invariant) {
        // Phase 8 operator triad

        // 1. Collapse-surface curvature activation
        let curvature = CollapseCurvature;
        curvature.apply(inv);

        // 2. Collapse-spectral coupling
        let coupler = CollapseSpectralCoupler;
        coupler.apply(inv);

        // Coordinator does not produce new fields itself.
        // It ensures Phase 8 activation is executed in correct altitude order.
    }
}
