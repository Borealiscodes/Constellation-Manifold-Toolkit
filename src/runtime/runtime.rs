//! Phase 3: Runtime Coordinator
//! Constellation-Manifold-Toolkit · Research Fork
//! This file introduces the first unified runtime layer. It coordinates the
//! ternary and transport adapters, manages invariant propagation, and exposes
//! a public runtime API.

use crate::adapter::ternary_adapter::TernaryAdapter;
use crate::adapter::transport_adapter::TransportAdapter;
use crate::geometry::fsfe_manifold::FsfePoint;
use crate::invariant::Invariant;
use crate::invariant::populator::InvariantPopulator;

/// Unified runtime coordinator.
/// Combines ternary and transport flows into a single runtime step.
pub struct Runtime<T, U> {
    pub ternary: TernaryAdapter<T>,
    pub transport: TransportAdapter<U>,
}

impl<T, U> Runtime<T, U> {
    /// Execute a full runtime step.
    /// Applies ternary routing followed by transport routing.
    /// Returns the final point and merged invariant.
    pub fn step(&self, p: &FsfePoint) -> (FsfePoint, Invariant)
    where
        T: crate::geometry::holonomy::Holonomy<Point = FsfePoint>
            + crate::geometry::slack::SlackInvariant<Point = FsfePoint>
            + crate::geometry::curvature::CurvatureInvariant<Point = FsfePoint>
            + crate::geometry::collapse_surface::CollapseSurface<Point = FsfePoint>
            + crate::geometry::spectral::SpectralAntiDrift<Point = FsfePoint>
            + crate::geometry::ndh::NdH<Point = FsfePoint>,
        U: crate::geometry::holonomy::Holonomy<Point = FsfePoint>
            + crate::geometry::slack::SlackInvariant<Point = FsfePoint>
            + crate::geometry::curvature::CurvatureInvariant<Point = FsfePoint>
            + crate::geometry::collapse_surface::CollapseSurface<Point = FsfePoint>
            + crate::geometry::spectral::SpectralAntiDrift<Point = FsfePoint>
            + crate::geometry::ndh::NdH<Point = FsfePoint>,
    {
        // Phase 3 runtime: ternary → transport
        let (ternary_out, inv_t) = self.ternary.route(p);
        let (transport_out, inv_u) = self.transport.transport(&ternary_out);

        // Merge invariants (Phase 3: simple additive merge)
        let mut merged = Invariant {
            curvature: inv_t.curvature + inv_u.curvature,
            spectral_energy: inv_t.spectral_energy + inv_u.spectral_energy,
            slack: [inv_t.slack, inv_u.slack].concat(),
            ndh_displacement: [inv_t.ndh_displacement, inv_u.ndh_displacement].concat(),
        };

        // Phase 4: invariant population
        let populator = InvariantPopulator;
        populator.populate(&mut merged);

        (transport_out, merged)
    }
}
