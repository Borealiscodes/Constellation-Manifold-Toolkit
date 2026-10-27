module EnvelopeSafety

export safety_envelope, is_safe_envelope

"""
    safety_envelope(M::Matrix)

D3 diagnostic safety envelope.
Altitude: D3 (diagnostic layer).
No operators, no invariants, no geometry creation.
Pure diagnostic safety extraction.
"""
function safety_envelope(M::Matrix)
    # Diagnostic altitude rule: envelope is a simple bounded scalar
    return maximum(abs.(M))
end

"""
    is_safe_envelope(M::Matrix)

Boolean safety predicate for envelope behavior.
Altitude-pure diagnostic logic.
"""
function is_safe_envelope(M::Matrix)
    return safety_envelope(M) ≤ 1.0
end

end # module
