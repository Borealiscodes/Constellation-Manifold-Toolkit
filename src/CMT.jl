module CMT

# Altitude 0–1: Geometry + Metric
include("ReversibleGeometry.jl")
include("ConceptualMetric.jl")

# Operator Altitudes Φ₇–Φ₉
include("CurvatureEnvelope.jl")
include("DriftSemigroup.jl")
include("InvariantExtraction.jl")

# Diagnostic Altitudes D1–D3
include("CollapseSurface.jl")
include("SoftDeformation.jl")
include("EnvelopeSafety.jl")

# Steering Altitude S1
include("PhaseShiftOperator.jl")

# Integration Altitude I1
include("CollisionInvariant.jl")

end
