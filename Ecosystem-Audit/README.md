# 🛰️ `Ecosystem-Audit/` — Developer Guide & Epistemic Limitations

**Repository:** `Borealiscodes/Constellation-Manifold-Toolkit` (`Ecosystem-Audit/`)  
**Maintainer:** Borealis S. Hedling (`NDH-RESEARCH-PILOT`)  
**Target Surface:** `://github.com` (Multi-Repository Spectral Topology Audit)  
**Posture:** Non-Accusatory • Read-Only • Edge-Safe (`< 2 MB` Footprint)  

---

## 1. Overview & Two-Stage Pipeline

This directory implements a non-cloning, two-stage spectral health audit designed for edge workstations and CI runners. Instead of cloning hundreds of gigabytes of repository history, it partitions I/O ingestion from numerical eigenspectrum analysis across a `SHA-256` manifest seam:

```text
[ GitHub REST API (Metadata Only) ]
               │
               ▼
[ Stage 1 & 2: ecosystem_ingest.py (Python 3.10+ Stdlib) ]
 • Paginates public org metadata (zero external dependencies)
 • Extracts protocol incidence matrix B ∈ {0, 1}^{N × |T|}
 • Anchors snapshot with canonical SHA-256 digest -> 8b_snapshot_manifest.json
               │
  ═════════════╧═════════════  <-- SHA-256 MANIFEST SEAM
               │
[ Stage 3: EcosystemSpectralKernel.jl (Julia 1.10+) ]
 • Constructs protocol-coupling adjacency W = B Bᵀ - diag(B Bᵀ)
 • Computes symmetric Graph Laplacian L = D - W eigenspectrum (λ_1 ... λ_N)
 • Emits disconnected silo counts & Fiedler bridge hubs -> 8b_spectral_report.json
```

---

## 2. How to Run the Audit

### Prerequisites
* **Python 3.10+:** Uses standard library only (`urllib.request`, `json`, `hashlib`, `fractions`). No `pip` packages required.
* **Julia 1.10+:** Requires `LinearAlgebra` (built-in) and `JSON` (`julia -e 'using Pkg; Pkg.add("JSON")'`).

### Step-by-Step Execution

```bash
cd Constellation-Manifold-Toolkit/Ecosystem-Audit

# Step 1: Fetch public metadata & generate SHA-256 anchored incidence manifest
python3 ecosystem_ingest.py
# -> Outputs: 8b_snapshot_manifest.json

# Step 2: Compute Graph Laplacian eigenvalues & Fiedler hub rankings
julia EcosystemSpectralKernel.jl
# -> Outputs: 8b_spectral_report.json
```

*(Note: Unauthenticated GitHub REST API requests allow 60 requests per hour per IP, which easily covers 3 paginated calls for ~240 repositories. If auditing larger organizations, set `GITHUB_TOKEN` in your request headers.)*

---

## 3. Interpreting the Spectral Output (`8b_spectral_report.json`)

| Metric | Mathematical Definition | What It Tells You About the Ecosystem |
| :--- | :--- | :--- |
| **`connected_components`** | Multiplicity of $\lambda_1 = 0$ in $L = D - W$ | Total number of disconnected repository clusters (including single-repo islands). |
| **`isolated_silo_count`** | Count of nodes with degree $d_i = 0$ | Upstream forks or standalone experiments that reference zero canonical protocol tokens (`0x62`, `bfp-64`, `blake3`, `q16.16`, `spqt`, `mem8`, etc.). |
| **`first_nonzero_eigenvalue`** | Smallest $\lambda_k > 10^{-9}$ (algebraic connectivity of the main core) | Measures how tightly coupled the active substrate repositories are. Higher values indicate dense cross-repo protocol sharing; values near zero indicate fragile single-bridge dependencies. |
| **`spectral_radius_laplacian`** | $\lambda_{\max}(L)$ | Upper bound on coupling concentration around high-degree hub repositories. |
| **`top_protocol_hubs`** | Ranked by `(coupling_degree, fiedler_weight)` | Identifies which repositories act as the structural backbone connecting disparate sub-projects. |

---

## 4. Explicit Methodological & Epistemic Limitations

Auditors and reviewers interpreting `8b_spectral_report.json` must account for five explicit scope boundaries:

1. **Metadata-Only Visibility (Surface vs. Implementation):**  
   `ecosystem_ingest.py` inspects repository names, descriptions, and GitHub topics—**not** ASTs, `Cargo.toml` dependencies, or test execution logs. A repository mentioning `blake3` or `0x62` in its description scores a `1` in matrix $B$ even if its implementation is incomplete or experimental.
2. **Fork Sprawl Inflation:**  
   Unmodified upstream forks (`oxicode`, `deepseek-harness`, `SF-Mamba`) naturally lack `8b-is`-specific protocol tokens in their inherited descriptions and will appear in `isolated_silo_count`. Always compare `fork_repos` against `original_repos` before treating isolated nodes as architectural fragmentation.
3. **Vocabulary Sensitivity ($\mathcal{T}$):**  
   The incidence matrix $B$ depends directly on `CANONICAL_TOKENS` in `ecosystem_ingest.py`. Sub-projects using newer or unlisted codenames will appear disconnected until those tokens are added to the vocabulary.
4. **Static Snapshot vs. Live Velocity:**  
   Each run captures a point-in-time `SHA-256` snapshot. Measuring true architectural drift or convergence requires comparing multiple time-stamped manifests (`t_0`, `t_1`, `t_2`).
5. **Non-Activation & Non-Accusatory Clause:**  
   This audit toolkit is strictly descriptive, read-only, and constructive. Spectral metrics measure *lexical and structural coupling across public metadata*—they do not measure code quality, developer intent, or subjective system properties.
