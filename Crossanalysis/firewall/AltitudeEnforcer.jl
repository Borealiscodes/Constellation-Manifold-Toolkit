module AltitudeEnforcer

# Series V — A6 Firewall Companion
# AltitudeEnforcer.jl (v1.0)
#
# Purpose:
# Define altitude IDs, transitions, and runtime enforcement logic.
# This module is the altitude model used by the firewall.

export Altitude, altitude_of, check_transition, enforce_altitude

# ---------------------------------------------------------------------
# Altitude Identifiers (canonical Series IV → Series V mapping)
# ---------------------------------------------------------------------

@enum Altitude begin
    A0
    A1
    Phi7
    D1
    D2
    D3
    Phi8
    Phi9
    S1
    I1
end

# ---------------------------------------------------------------------
# Altitude assignment for Series IV modules
# ---------------------------------------------------------------------

function altitude_of(mod::Symbol)::Altitude
    if mod == :ReversibleGeometry
        return A0
    elseif mod == :ConceptualMetric
        return A1
    elseif mod == :CurvatureEnvelope
        return Phi7
    elseif mod == :CollapseSurface
        return D1
    elseif mod == :SoftDeformation
        return D2
    elseif mod == :EnvelopeSafety
        return D3
    elseif mod == :DriftSemigroup
        return Phi8
    elseif mod == :InvariantExtraction
        return Phi9
    elseif mod == :PhaseShiftOperator
        return S1
    elseif mod == :CollisionInvariant
        return I1
    else
        error("Unknown module altitude: $mod")
    end
end

# ---------------------------------------------------------------------
# Allowed transitions (same rules as LeanBindings)
# ---------------------------------------------------------------------

function allowed_transition(a::Altitude, b::Altitude)::Bool
    return (a == A0  && b == A1)  ||
           (a == A1  && b == Phi7) ||
           (a == Phi7 && b == D1)  ||
           (a == D1  && b == D2)   ||
           (a == D2  && b == D3)   ||
           (a == D3  && b == Phi8) ||
           (a == Phi8 && b == Phi9)||
           (a == Phi9 && b == S1)  ||
           (a == S1  && b == I1)
end

# ---------------------------------------------------------------------
# Transition checking
# ---------------------------------------------------------------------

struct AltitudeViolation <: Exception
    msg::String
end

function check_transition(a::Altitude, b::Altitude)
    if !allowed_transition(a, b)
        throw(AltitudeViolation("Forbidden altitude transition: $a → $b"))
    end
end

# ---------------------------------------------------------------------
# Enforce altitude for a given module call
# ---------------------------------------------------------------------

function enforce_altitude(caller::Symbol, callee::Symbol)
    a = altitude_of(caller)
    b = altitude_of(callee)
    check_transition(a, b)
end

end # module
