module CollisionInvariant

export avoidance, distance_safe

"""
    avoidance(M::Matrix)

I1 integration predicate.
Altitude: I1 (integration layer).
Non-generative, no operators, no diagnostics, no invariants.
Pure integration logic only.
"""
function avoidance(M::Matrix)
    # Integration altitude rule: simple scalar predicate
    return minimum(abs.(M)) > 0.1
end

"""
    distance_safe(M::Matrix)

Integration safety predicate.
Altitude-pure integration logic.
"""
function distance_safe(M::Matrix)
    return maximum(abs.(M)) < 10.0
end

end # module
