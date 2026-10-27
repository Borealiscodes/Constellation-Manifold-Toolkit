module InvariantExtraction

export trace_invariant, norm_invariant, det_invariant

"""
    trace_invariant(M::Matrix)

Φ₉ invariant: trace.
Altitude: Φ₉ (operator layer).
Invariant purity: no geometry, no diagnostics, no steering, no integration.
"""
function trace_invariant(M::Matrix)
    return sum(diag(M))
end

"""
    norm_invariant(M::Matrix)

Φ₉ invariant: Frobenius norm.
Altitude-pure invariant logic.
"""
function norm_invariant(M::Matrix)
    return sqrt(sum(abs.(M).^2))
end

"""
    det_invariant(M::Matrix)

Φ₉ invariant: determinant.
Altitude-pure invariant logic.
"""
function det_invariant(M::Matrix)
    return det(M)
end

end # module
