module ReversibleGeometry

export Manifold, manifold_point, manifold_dimension

"""
    Manifold(dimension)

Reversible geometric manifold structure.
Altitude: A0 (reversible geometry).
"""
struct Manifold
    dimension::Int
end

"""
    manifold_point(M::Manifold, coords)

Constructs a reversible point on the manifold.
No operators, no tensors, no diagnostics.
"""
function manifold_point(M::Manifold, coords::Vector{Float64})
    @assert length(coords) == M.dimension
    return coords
end

"""
    manifold_dimension(M::Manifold)

Returns the dimension of the manifold.
Pure reversible geometry.
"""
manifold_dimension(M::Manifold) = M.dimension

end # module
