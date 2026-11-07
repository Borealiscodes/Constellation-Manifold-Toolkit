# tinkering/gateaux_instrument.jl
using StaticArrays
using LinearAlgebra

# Match the 80D (40 Position, 40 Momentum) structural boundaries from Rust
const DIM_80 = 80
const StateVector = SVector{DIM_80, Float64}
const PerturbationVector = SVector{DIM_80, Float64}

"""
    evaluate_manifold_curvature(x::StateVector, scale_factor::Float64)
Simulates the local non-linear coordinate field behavior of our symplectic substrate.
"""
function evaluate_manifold_curvature(x::StateVector, scale_factor::Float64)::StateVector
    # Stack-allocated, vector-parallel sin/cos transformations
    return sin.(x) .* scale_factor
end

"""
    Tier 3: Gâteaux Variational Susceptibility Monitor
Computes the immediate directional variation rate under a tiny informational nudge (h).
"""
function compute_gateaux_susceptibility(
    x::StateVector, 
    h::PerturbationVector, 
    epsilon::Float64, 
    scale_factor::Float64
)::Float64
    # 1. Inject perturbation entirely on the CPU Stack
    perturbed_state = x + (epsilon .* h)
    
    # 2. Evaluate both coordinates on the manifold layout
    f_perturbed = evaluate_manifold_curvature(perturbed_state, scale_factor)
    f_original  = evaluate_manifold_curvature(x, scale_factor)
    
    # 3. Calculate directional derivative tensor: (F(x + eps*h) - F(x)) / eps
    variation_vector = (f_perturbed - f_original) ./ epsilon
    
    # 4. Compute L2 Norm to output a single, objective stability metric
    return norm(variation_vector)
end

# =============================================================================
# LIVE OPERATIONAL TELEMETRY CHECK
# =============================================================================
println("🔬 Initializing Julia Manifold Perturbation Instrument...")

# Generate matching sample coordinates
x_state = StateVector(rand(DIM_80))
h_push  = PerturbationVector(rand(DIM_80) * 0.1) # Directional nudge
eps     = 1e-6
scale   = 1.58

# Run with ZERO heap allocations
stability_score = compute_gateaux_susceptibility(x_state, h_push, eps, scale)

@assert stability_score > 0.0 "💥 DECAY DETECTED: Manifold collapsed to zero!"
println("🟩 TELEMETRY ACTIVE. Current Gâteaux Susceptibility Score: ", round(stability_score, digits=6))
