module CollapseSurface

export collapse_indicator

"""
    collapse_indicator(M::Matrix)

D1 diagnostic predicate for collapse-surface behavior.
Altitude: D1 (diagnostic layer).
No operators, no invariants, no geometry creation.
Pure diagnostic logic only.
"""
function collapse_indicator(M::Matrix)
    # Diagnostic altitude rule: only simple scalar predicates allowed
    return maximum(abs.(M)) > 1.0
end

end # module
