module Diagnostics

export resonance_cluster, drift_corridor, twist_vector, holonomy_distortion

using LinearAlgebra

function resonance_cluster(a::Vector{Float64}, b::Vector{Float64}; ϵ=1e-3)
    return findall(i -> abs(a[i] - b[i]) < ϵ, eachindex(a))
end

function drift_corridor(A::Diagonal, tmax::Float64)
    return [exp.(t .* diag(A)) for t in range(0, tmax, length=50)]
end

function twist_vector(h::Vector{Float64})
    return h .- 1
end

function holonomy_distortion(h1::Vector{Float64}, h2::Vector{Float64})
    return norm(h1 .- h2)
end

end # module
