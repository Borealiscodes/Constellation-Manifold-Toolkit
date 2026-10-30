pub struct Invariant {
    // Existing Phase 4/5 fields
    pub curvature: f64,
    pub spectral_energy: f64,
    pub slack: f64,
    pub ndh_displacement: f64,

    // Phase 6 derived fields
    pub curvature_energy: f64,
    pub spectral_coupling: f64,
    pub slack_stability: f64,
    pub ndh_smoothing: f64,

    // Optional: only present if membrane interaction is enabled
    pub membrane_tension: Option<f64>,

    // Phase 7 spectral stability fields
    pub spectral_stability: f64,
    pub basin_alignment: f64,
    pub collapse_pre_signal: f64,

    // Phase 8 collapse-surface activation fields
    pub collapse_activation_strength: f64,
    pub collapse_curvature: f64,
    pub collapse_spectral_binding: f64,

    // Phase 9 projection-seed geometry fields
    pub projection_seed_energy: f64,
    pub projection_seed_curvature: f64,
    pub projection_seed_binding: f64,

    // Phase 10 collapse-surface ecology fields
    pub collapse_ecology_energy: f64,
    pub collapse_ecology_curvature: f64,
    pub collapse_ecology_binding: f64,

    // Phase 11 routing-ecology fields
    pub routing_ecology_vector_x: f64,
    pub routing_ecology_vector_y: f64,
    pub routing_ecology_strength: f64,

    // Phase 12 projection-holonomy routing fields
    pub projection_holonomy_vector_x: f64,
    pub projection_holonomy_vector_y: f64,
    pub projection_holonomy_strength: f64,
}
