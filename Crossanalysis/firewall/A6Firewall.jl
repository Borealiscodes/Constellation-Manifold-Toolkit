module A6Firewall

# Series V — A6 Firewall Companion
# A6Firewall.jl (v1.0)
#
# Purpose:
# Provide the public API for altitude enforcement, purity checks,
# non-generativity guards, boundedness guards, and full firewall protection.

export firewall_call, altitude_safe_call

# ---------------------------------------------------------------------
# Imports from Series V modules
# ---------------------------------------------------------------------

include("AltitudeEnforcer.jl")
include("JuliaBindings.jl")

using .AltitudeEnforcer
using .JuliaBindings

# ---------------------------------------------------------------------
# Public API: altitude-safe call
# ---------------------------------------------------------------------

"""
    altitude_safe_call(caller::Symbol, callee::Symbol, f)

Checks altitude legality between caller and callee, then executes `f`
only if the transition is allowed.
"""
function altitude_safe_call(caller::Symbol, callee::Symbol, f)
    enforce_altitude(caller, callee)
    return f()
end

# ---------------------------------------------------------------------
# Public API: full firewall call
# ---------------------------------------------------------------------

"""
    firewall_call(caller::Symbol, callee::Symbol, f)

Full A6 firewall enforcement:
- altitude transition check
- purity check
- non-generativity check
- boundedness check
Executes `f` only if all constraints pass.
"""
function firewall_call(caller::Symbol, callee::Symbol, f)
    a = altitude_of(caller)
    b = altitude_of(callee)
    return enforce_firewall(a, b, f)
end

end # module
