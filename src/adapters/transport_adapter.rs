//! Phase 3: Full Transport Adapter Logic
//! Constellation-Manifold-Toolkit · Research Fork
//! This file upgrades the Phase 2 transport adapter stub into a fully operational
//! routing layer between geometry operators and the transport runtime.

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

/// Full transport adapter.
/// Applies geometry operators in a transport-aware sequence and updates invariants.
pub struct TransportAdapter<G> {
    pub geometry: G,
}

impl<G> TransportAdapter<G>
where
    G: Holonomy<Point = FsfePoint>
        + SlackInvariant<Point = FsfePoint>
        + CurvatureInvariant<Point = FsfePoint>
        + CollapseSurface<Point = FsfePoint>
        + SpectralAntiDrift<Point = FsfePoint>
        + NdH<Point = FsfePoint>,
{
    /// Execute a full transport routing pass.
    /// This is the second operational runtime behavior in the system.
    pub fn transport(&self, p: &FsfePoint) -> (FsfePoint, Invariant) {
        // Holonomy transport
        let h = self.geometry.holonomy(p);

        // Slack modulation (transport-specific profile)
        let slack_vec = vec![0.08, -0.03, 0.11];
        let s = self.geometry.apply_slack(&h, &slack_vec);

        // Curvature adjustment (transport-specific gain)
        let c = self.geometry.adjust_curvature(&s, 0.009);

        // Collapse-surface routing
        let cs = self.geometry.transport_on_surface(&c);

        // Spectral anti-drift
        let sp = self.geometry.antidrift(&cs);

        // NDH loop selection (transport heuristic)
        let loop_stub = NdHLoop {
            segments: vec![
                (0, sp.curvature * 0.012),
                (2, -sp.spectral_energy * 0.006),
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
