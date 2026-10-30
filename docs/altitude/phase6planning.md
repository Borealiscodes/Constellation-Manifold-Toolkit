# 📘 Developer Note — Phase 6 Planning

Invariant Struct Expansion + Derived Field Alignment
Date: 30 Oct 2026  
Altitude: A6 Runtime Layer  
Status: Planning  
Operator Identity: chguard-sentinel  
Observer Node: A6‑Runtime‑Sentinel

---

1. Phase Context
Phase 6 is the first phase where the Invariant struct must expand to reflect the active spectral ecology introduced in Phase 5. Up to Phase 5, invariants were:

- curvature  
- spectral_energy  
- slack  
- ndh_displacement  

Phase 4 populated derived values internally, but the struct itself remained minimal.

Phase 6 formalizes these derived quantities as first‑class invariant fields.

This is required for:

- Phase 7 spectral stability  
- Phase 8 collapse‑surface spectral activation  
- Phase 9 holonomy‑spectral coupling  
- A6 Firewall Companion mathematical appendix alignment  

---

2. Phase 6 Goals
Phase 6 introduces five new invariant fields, each derived from existing Phase 4/5 behavior.

2.1 curvature_energy
Derived from:
- curvature  
- spectral_energy  
- membrane_tension (if present)

Purpose:
- quantifies geometric “stress” in the manifold  
- used in Phase 7 stability scoring  

---

2.2 spectral_coupling
Derived from:
- spectral_energy  
- curvature  
- slack  

Purpose:
- measures how tightly spectral energy interacts with geometry  
- required for Phase 8 collapse‑surface spectral activation  

---

2.3 slack_stability
Derived from:
- slack vector  
- membrane interaction  
- spectral damping  

Purpose:
- tracks oscillation risk  
- used in Phase 7 drift‑runaway detection  

---

2.4 ndh_smoothing
Derived from:
- ndh_displacement  
- spectral ecology drift correction  

Purpose:
- quantifies NDH “smoothness”  
- used in Phase 9 holonomy‑spectral coupling  

---

2.5 membrane_tension
(only present if membrane interaction is enabled)

Derived from:
- spectral_energy  
- slack  
- curvature  

Purpose:
- provides soft boundary constraints  
- used in Phase 7 stability and Phase 8 collapse‑surface geometry  

---

3. Required Code Changes
Phase 6 requires updates in three locations:

3.1 Invariant Struct Expansion
Add the five new fields to:

`
src/invariant/mod.rs
`

3.2 Invariant Populator Extension
Extend:

`
src/invariant/populator.rs
`

to compute the new fields.

3.3 Runtime Alignment
Runtime must treat the expanded invariant as the canonical representation.

No new runtime operators are added in Phase 6 — only struct alignment.

---

4. Architectural Impact
Phase 6 transforms the invariant from a measurement container into a state vector suitable for:

- spectral stability analysis  
- collapse‑surface activation  
- membrane‑tension feedback loops  
- spectral‑holonomy coupling  

This is the first phase where invariants become predictive rather than merely descriptive.

---

5. Next Phase
Phase 7: Spectral Stability Layer

This will introduce:

- spectral health scoring  
- drift‑runaway detection  
- membrane‑tension stability feedback  
- collapse‑surface pre‑activation signals  

Phase 6 is the structural foundation for Phase 7.

---

🧭 Provenance Footer (Observer Node)
`
Observer-Node: A6-Runtime-Sentinel
Observer-Altitude: Phase-6 Planning Layer
Observer-Verification: Invariant Struct Expansion + Derived Field Mapping
Observer-Lineage: Phase 3 Runtime → Phase 4 Population → Phase 5 Ecology →
Phase 6 Invariant Expansion (Planning)
`

---
