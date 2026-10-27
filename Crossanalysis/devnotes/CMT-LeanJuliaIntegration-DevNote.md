# 🧠 Best Way to Populate Lean Modules (Zero‑Hallucination Workflow)

Lean hallucination happens when:

- definitions are implicitly inferred  
- types are not explicitly constrained  
- modules import too much  
- proofs rely on unverified lemmas  
- operator signatures drift between files  
- geometry and algebra mix without guard rails  

Your Lean‑Hallucination‑Guard solves this by enforcing:

- explicit operator signatures  
- explicit tensor ranks  
- explicit manifold smoothness assumptions  
- explicit curvature bounds  
- explicit algebraic identities  
- explicit module‑local invariants  

To populate Lean modules safely, follow this exact workflow.

---

🧩 1. Create the Lean Modules First (Empty Skeletons)

Before writing any definitions, create the module skeletons:

`
crossanalysis/lean/
  ConceptualManifold.lean
  ConceptualCurvature.lean
  SoftDeformation.lean
  PhaseShiftOperator.lean
  EnvelopeSafety.lean
  CollisionInvariant.lean
  OperatorDrift.lean
  KrausInvariant.lean
`

Each file should begin with:

`lean
import LeanHallucinationGuard.Guard
`

This ensures every definition is validated.

---

🛡️ 2. Use Guard Macros on Every Definition

Every operator, tensor, manifold, and transformation must be guarded:

`lean

guard conceptualManifold

guard conceptualMetric

guard conceptualDensity

guard conceptualCurvature

guard deformationTensor

guard phaseShiftOperator

guard envelopeOperator

guard collisionInvariant

guard pseudoKrausOperator
`

This prevents Lean from accepting malformed definitions.

---

📐 3. Populate Definitions Only After Guard Hooks Are in Place

The correct order inside each module:

1. import guard  
2. declare types  
3. declare structures  
4. guard each structure  
5. declare invariants  
6. prove invariants  

Example:

`lean
structure ConceptualManifold :=
  (M : Type)
  (metric : M → Tensor2)
  (density : M → ℝ)
  (smooth : Smooth metric)

guard ConceptualManifold
`

This prevents Lean from silently inferring wrong tensor ranks or smoothness assumptions.

---

🧮 4. Never Mix Geometry and Operator Algebra in the Same Lean File

This is critical.

Lean hallucination spikes when:

- curvature tensors  
- operator algebra  
- pseudo‑Kraus operators  
- deformation tensors  

are defined in the same module.

Your altitude‑separated module structure prevents this.

---

🧭 5. Prove Invariants Before Implementing Julia

Lean must prove:

- non‑degeneracy  
- curvature bounds  
- envelope energy bounds  
- collision‑avoidance distance invariants  
- pseudo‑Kraus trace‑preservation  
- drift semigroup properties  

Julia must not implement these until Lean proves them.

This prevents runtime drift.

---

🔗 6. Julia Integration Notes (Critical)

Julia modules should mirror Lean modules:

`
crossanalysis/julia/
  ConceptualManifold.jl
  ConceptualCurvature.jl
  SoftDeformation.jl
  PhaseShiftOperator.jl
  EnvelopeSafety.jl
  CollisionRuntime.jl
  OperatorDrift.jl
  KrausRuntime.jl
`

Integration Rules

- Lean defines invariants; Julia enforces them.  
- Lean proves operator identities; Julia implements them.  
- Lean proves safety; Julia checks safety.  
- Lean modules map 1‑to‑1 to Julia modules.  
- Julia must not define new invariants.  
- Julia must not infer tensor ranks or operator signatures.  
- Julia must not mix altitudes.

This keeps the runtime aligned with the formal altitude.

---

📜 Provenance Footer

`
---
Provenance:
Generated as part of the Constellation-Manifold-Toolkit cross-analysis series.
Provides formal development guidance for populating Lean modules using
Lean-Hallucination-Guard and integrating Lean proofs with Julia runtime
implementations for conceptual manifold collision analysis.
Supports altitude-aligned operator geometry and formal verification workflows.
Not part of TISD governance, Anima-Vectorium runtime, or Vectorium A7–A9.
Serves exclusively as technical developer documentation.

Author:
Borealis S. Hedling — Dublin, Ireland.

Version:
CMT-LHJ-v1.0 — 27 October 2026, 20:41 IST
---
`

---

