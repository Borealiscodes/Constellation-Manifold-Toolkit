module SoftDeformation

export deformation_tensor, deformation_magnitude

"""
    deformation_tensor(M::Matrix)

D2 diagnostic deformation tensor.
Altitude: D2 (diagnostic layer).
No operators, no invariants, no geometry creation.
Pure diagnostic tensor extraction.
"""
function deformation_tensor(M::Matrix)
    # Diagnostic altitude rule: simple reversible deformation tensor
    return abs.(M)
end

"""
    deformation_magnitude(M::Matrix)

Computes scalar magnitude of deformation.
Altitude-pure diagnostic logic.
"""
function deformation_magnitude(M::Matrix)
    return sum(abs.(M))
end

end # module
