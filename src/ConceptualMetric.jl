module ConceptualMetric

export Metric, metric_tensor, metric_distance

"""
    Metric(dimension)

Reversible metric structure.
Altitude: A1 (reversible geometry).
"""
struct Metric
    dimension::Int
end

"""
    metric_tensor(M::Metric)

Returns the identity metric tensor for reversible geometry.
No operators, no diagnostics, no invariants.
"""
function metric_tensor(M::Metric)
    return Matrix{Float64}(I, M.dimension, M.dimension)
end

"""
    metric_distance(M::Metric, p::Vector{Float64}, q::Vector{Float64})

Computes reversible metric distance between two points.
Altitude-pure, non-generative.
"""
function metric_distance(M::Metric, p::Vector{Float64}, q::Vector{Float64})
    @assert length(p) == M.dimension
    @assert length(q) == M.dimension
    diff = p .- q
    return sqrt(sum(diff .* diff))
end

end # module
