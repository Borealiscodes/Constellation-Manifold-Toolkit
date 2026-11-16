#!/usr/bin/env julia
"""
Ecosystem-Audit/EcosystemSpectralKernel.jl
Stage 3: Computes the Graph Laplacian eigenspectrum (λ_1 ... λ_N), disconnected
component count, algebraic connectivity (Fiedler value λ_2), and bridge hub rankings
from the SHA-256 anchored `8b_snapshot_manifest.json`.
"""

module EcosystemSpectralKernel

using JSON
using LinearAlgebra

export audit_ecosystem_spectrum

"""
    audit_ecosystem_spectrum(manifest_path::AbstractString) -> Dict{String, Any}

Load `8b_snapshot_manifest.json`, construct W = B * B' - diag(B * B') and L = D - W,
and extract spectral health diagnostics via symmetric eigendecomposition.
"""
function audit_ecosystem_spectrum(manifest_path::AbstractString)
    data = JSON.parsefile(manifest_path)
    repos = data["repositories"]
    raw_B = data["incidence_matrix"]

    N = length(repos)
    if N == 0
        error("Empty repository manifest: cannot compute Laplacian spectrum.")
    end

    M = length(data["tokens"])
    B = zeros(Float64, N, M)
    for i in 1:N
        row = raw_B[i]
        for j in 1:M
            B[i, j] = Float64(row[j])
        end
    end

    # Protocol-coupling adjacency matrix W (zero self-loops) & Graph Laplacian L = D - W
    W = B * B'
    for i in 1:N
        W[i, i] = 0.0
    end
    degrees = vec(sum(W, dims=2))
    L = Diagonal(degrees) - W

    # Full symmetric eigendecomposition (sorted ascending: λ_1 <= λ_2 <= ... <= λ_N)
    eig = eigen(Symmetric(L))
    vals = eig.values
    vecs = eig.vectors

    # Zero-eigenvalue multiplicity = number of disconnected components / isolated silos
    tol = 1e-9
    zero_count = count(v -> abs(v) <= tol, vals)

    # First strictly positive eigenvalue (algebraic connectivity of the main connected core)
    pos_idx = findfirst(v -> v > tol, vals)
    lambda_pos = isnothing(pos_idx) ? 0.0 : vals[pos_idx]
    fiedler_vec = isnothing(pos_idx) ? zeros(Float64, N) : vecs[:, pos_idx]

    # Rank repositories by degree centrality (protocol hub density) and Fiedler bridge magnitude
    hub_ranking = sort(
        [(repos[i]["name"], Int(round(degrees[i])), round(abs(fiedler_vec[i]), digits=6), repos[i]["fork"]) for i in 1:N],
        by = x -> (-x[2], x[1]),
    )

    isolated_names = [repos[i]["name"] for i in 1:N if degrees[i] == 0.0]

    return Dict{String, Any}(
        "target_org" => data["target_org"],
        "manifest_sha256" => data["manifest_sha256"],
        "total_repos" => N,
        "connected_components" => zero_count,
        "isolated_silo_count" => length(isolated_names),
        "first_nonzero_eigenvalue" => round(lambda_pos, digits=6),
        "spectral_radius_laplacian" => round(maximum(vals), digits=6),
        "top_protocol_hubs" => [
            Dict("repo" => r, "coupling_degree" => d, "fiedler_weight" => f, "is_fork" => fk)
            for (r, d, f, fk) in hub_ranking[1:min(10, N)]
        ],
        "sample_isolated_repos" => isolated_names[1:min(15, length(isolated_names))],
    )
end

function main()
    manifest_path = joinpath(@__DIR__, "8b_snapshot_manifest.json")
    out_path = joinpath(@__DIR__, "8b_spectral_report.json")

    if !isfile(manifest_path)
        error("Missing $manifest_path — run `python3 ecosystem_ingest.py` first.")
    end

    report = audit_ecosystem_spectrum(manifest_path)
    open(out_path, "w") do io
        JSON.print(io, report, 2)
    end

    println("=== 8b-is SPECTRAL ECOSYSTEM AUDIT COMPLETE ===")
    println("Manifest SHA-256         : ", report["manifest_sha256"])
    println("Total Repositories       : ", report["total_repos"])
    println("Disconnected Components  : ", report["connected_components"])
    println("Isolated Zero-Token Repos: ", report["isolated_silo_count"])
    println("First Non-Zero λ (Core)  : ", report["first_nonzero_eigenvalue"])
    println("Max Laplacian Eigenvalue : ", report["spectral_radius_laplacian"])
    println("Saved report to          : ", out_path)
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end

end # module
