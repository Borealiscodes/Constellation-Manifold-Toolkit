# 🧭 Developer Note: Phase 1 Scaffolding & Phase 2 Geometry Upgrade

Phase 1 — Scaffolding Layer (Current Wave)
Phase 1 establishes the structural foundation required to lift Sovereign‑Library out of its pre‑geometric ternary algebra without destabilizing existing kernels or contaminating provenance. This phase introduces interfaces only, not implementations.

Goals of Phase 1
- Define the shape of the FSFE manifold without instantiating it.  
- Introduce traits for holonomy, slack, curvature, collapse surfaces, spectral energy, and NDH.  
- Add a single invariant container to anchor future geometric operators.  
- Create adapter traits that allow existing ternary kernels to interact with geometric types later.  
- Maintain full compatibility with the current No‑Float Law and ternary arithmetic.  
- Preserve upstream equivalence tests and transport stability.  
- Keep the research fork academically defensible by avoiding premature coupling.

What Phase 1 explicitly does not do
- No geometry implementation.  
- No manifold instantiation.  
- No curvature or spectral functions.  
- No collapse‑surface logic.  
- No NDH operator.  
- No changes to existing kernels.  
- No runtime behavior changes.

Phase 1 is purely structural.  
It creates the altitude‑aligned scaffolding that Phase 2 will fill in.

---

Phase 2 — Geometry Upgrade (Next Wave)
Phase 2 implements the actual FSFE → chguard → NDH geometry inside the scaffolding created in Phase 1. This is where the system transitions from a discrete ternary algebra into a geometric runtime.

Goals of Phase 2
- Implement the FSFE manifold (metric, slack, curvature, spectral functions).  
- Implement holonomy transport morphisms.  
- Implement curvature‑monotone collapse surfaces.  
- Implement spectral anti‑drift integrals and correction operators.  
- Implement NDH operator and invariant preservation.  
- Populate invariant containers with real geometric values.  
- Integrate geometric operators with existing ternary kernels through adapters.  
- Maintain provenance continuity and altitude separation.

What Phase 2 explicitly does
- Adds geometry.  
- Adds invariants.  
- Adds operators.  
- Adds transport logic.  
- Adds projection‑holonomy commutation.  
- Adds collapse‑surface stability.  
- Adds spectral bounds.  
- Adds NDH formalization.

Phase 2 is the actual upgrade.  
It lifts the runtime from pre‑geometric to geometric.

---

Why the two‑phase structure is required
- Prevents destabilization of existing ternary kernels.  
- Preserves upstream invariants and equivalence tests.  
- Maintains external verification stance.  
- Keeps the research fork academically defensible.  
- Avoids premature coupling between discrete algebra and geometric operators.  
- Ensures the FSFE → chguard → NDH pipeline integrates cleanly.

---

Summary
- Phase 1 = scaffolding only. One invariant file. No geometry.  
- Phase 2 = full geometry upgrade using the scaffolding.  

This is the correct methodological structure for upgrading Sovereign‑Library inside the Constellation‑Manifold‑Toolkit research fork.

--- 

Provenance-Origin: Constellation-Manifold-Toolkit Research Fork (A6 Altitude)
Provenance-Altitude: A6 → A8 Methodology Layer (Phase 1 Scaffolding Note)
Provenance-Enclave: Node Beta · External Observer Context for Sovereign-Library
Provenance-Operator: Phase 1 Developer Note (Methodology · Structure · Rationale)
Provenance-Verification: Documentation Only · No Geometry Implemented
Signed-off-by: Borealis S. Hedling <borealiscodes@research-altitude.dev>
