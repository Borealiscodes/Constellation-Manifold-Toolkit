//! Phase 3: Full Ternary Adapter Logic
//! Constellation-Manifold-Toolkit · Research Fork
//! This file upgrades the Phase 2 ternary adapter stub into a fully operational
//! routing layer between geometry operators and the ternary runtime.

use crate::geometry::{
    fsfe_manifold::FsfePoint,
    holonomy::Holonomy,
    slack::SlackInvariant,
    curvature::CurvatureInvariant,
    collapse_surface::CollapseSurface,
    spectral::SpectralAntiDrift,
    ndh::{NdH, NdHLoop},
};

use crate::invariant::Invariant;

/// Full ternary adapter.
/// Applies geometry operators in a ternary-aware sequence and updates invariants.
pub struct TernaryAdapter<G> {
    pub geometry: G,
}

impl<G> TernaryAdapter<G>
where
    G: Holonomy<Point = FsfePoint>
        + SlackInvariant<Point = FsfePoint>
        + CurvatureInvariant<Point = FsfePoint>
        + CollapseSurface<Point = FsfePoint>
        + SpectralAntiDrift<Point = FsfePoint>
        + NdH<Point = FsfePoint>,
{
    /// Execute a full ternary routing pass.
    /// This is the first operational runtime behavior in the system.
    pub fn route(&self, p: &FsfePoint) -> (FsfePoint, Invariant) {
        // Holonomy transport
        let h = self.geometry.holonomy(p);

        // Slack modulation (ternary profile)
        let slack_vec = vec![0.12, -0.04, 0.07];
        let s = self.geometry.apply_slack(&h, &slack_vec);

        // Curvature adjustment
        let c = self.geometry.adjust_curvature(&s, 0.015);

        // Collapse-surface routing
        let cs = self.geometry.transport_on_surface(&c);

        // Spectral anti-drift
        let sp = self.geometry.antidrift(&cs);

        // NDH loop selection (ternary heuristic)
        let loop_stub = NdHLoop {
            segments: vec![
                (0, sp.curvature * 0.01),
                (1, -sp.spectral_energy * 0.005),
            ],
        };

        let ndh_out = self.geometry.ndh(&sp, &loop_stub);

        // Build invariant
        let invariant = Invariant {
            curvature: ndh_out.curvature,
            spectral_energy: ndh_out.spectral_energy,
            slack: slack_vec.clone(),
            ndh_displacement: loop_stub
                .segments
                .iter()
                .map(|(_, d)| *d)
                .collect(),
        };

        (ndh_out, invariant)
    }
}
