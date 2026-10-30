//! Phase 2: Invariant Container Stub
//! Constellation-Manifold-Toolkit · Research Fork
//! This file introduces the invariant container required for Phase 3 adapter
//! logic and Phase 4 invariant population. No invariant logic or population
//! occurs in Phase 2.

#[derive(Clone, Debug, Default)]
pub struct Invariant {
    /// Curvature value after geometry processing.
    pub curvature: f64,

    /// Spectral energy after anti-drift stabilization.
    pub spectral_energy: f64,

    /// Slack vector applied during slack modulation.
    pub slack: Vec<f64>,

    /// NDH displacement values extracted from NDH loop segments.
    pub ndh_displacement: Vec<f64>,
}
