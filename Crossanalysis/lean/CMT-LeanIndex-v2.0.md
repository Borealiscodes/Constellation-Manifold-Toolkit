# 📘 CMT‑LeanIndex‑v2.0.md

Constellation‑Manifold‑Toolkit — Canonical Lean Index (Series II + Series III)

---

🧭 1. Altitude Map (Global Overview)

| Altitude | Module | Description |
|---------|--------|-------------|
| A0 | ConceptualManifold | Base reversible geometry |
| A1 | ConceptualMetric | Metric structure, reversible |
| Φ₇ | CurvatureEnvelope | Curvature‑operator envelope |
| D1 | CollapseSurface | Diagnostic collapse geometry |
| D2 | SoftDeformation | Diagnostic deformation tensor |
| D3 | EnvelopeSafety | Diagnostic safety predicates |
| Φ₈ | DriftSemigroup | Drift evolution operator |
| Φ₉ | InvariantExtraction | Trace/norm/det invariants |
| S1 | PhaseShiftOperator | Reversible steering operator |
| I1 | CollisionInvariant | Final integration layer |

This altitude map is the backbone of the entire toolkit.

---

🧱 2. Series II — Conceptual Modules (Lean Code)

Geometry Layer
- ConceptualManifold.lean
- ConceptualMetric.lean

Operator Layer
- CurvatureEnvelope.lean (Φ₇)
- DriftSemigroup.lean (Φ₈)
- InvariantExtraction.lean (Φ₉)
- PhaseShiftOperator.lean (S1)

Diagnostic Layer
- CollapseSurface.lean
- SoftDeformation.lean
- EnvelopeSafety.lean

Integration Layer
- CollisionInvariant.lean

These are the modules you fully populated in Series II.

---

🔍 3. Series III — Verification Modules (Lean Code)

Guard‑Macro Verification
- Verification.GuardCoverage.lean

Altitude‑Contract Verification
- Verification.AltitudeContracts.lean

Operator‑Safety Verification
- Verification.OperatorSafety.lean

Invariant‑Correctness Verification
- Verification.InvariantCorrectness.lean

Integration‑Safety Verification
- Verification.IntegrationSafety.lean

These modules validate the entire conceptual spine.

---

🔗 4. Dependency Graph (Canonical Order)

1. ConceptualManifold  
2. ConceptualMetric  
3. CurvatureEnvelope  
4. CollapseSurface  
5. SoftDeformation  
6. EnvelopeSafety  
7. DriftSemigroup  
8. InvariantExtraction  
9. PhaseShiftOperator  
10. CollisionInvariant

Verification modules depend on all of the above.

---

🛡️ 5. Guard‑Macro Map (v2.0)

| Module | Guard Type |
|--------|------------|
| ConceptualManifold | #guard |
| ConceptualMetric | #guard |
| CurvatureEnvelope | #guardCurvature |
| CollapseSurface | #guardTensor |
| SoftDeformation | #guardTensor |
| EnvelopeSafety | #guardDomain |
| DriftSemigroup | #guardRank |
| InvariantExtraction | #guardRank |
| PhaseShiftOperator | #guardRank |
| CollisionInvariant | #guardDomain |

This map is essential for the A6 Firewall Companion.

---

🧩 6. Operator Layer Map (Φ₇–Φ₉ + Steering)

| Operator | Module | Altitude |
|----------|--------|----------|
| envelope_op | CurvatureEnvelope | Φ₇ |
| drift | DriftSemigroup | Φ₈ |
| invariant_op | InvariantExtraction | Φ₉ |
| steering | PhaseShiftOperator | S1 |

This is the operator spine Julia will mirror.

---

🔒 7. Integration Spine

The integration layer (CollisionInvariant.lean) depends on:

- EnvelopeSafety  
- PhaseShiftOperator  
- InvariantExtraction  
- DriftSemigroup  
- SoftDeformation  
- CollapseSurface  

And is validated by:

- IntegrationSafety.lean

This is the closure of the entire conceptual system.

---

🚀 8. Ready for Series IV (Julia Runtime Mirror)

This index now provides:

- canonical module ordering  
- altitude contracts  
- operator boundaries  
- invariant rules  
- guard‑macro associations  
- integration‑layer constraints  

Everything Julia needs to mirror the Lean spine safely.

---

🔥 9. Ready for Series V (A6 Firewall Companion)

The firewall will use:

- Series III verification modules  
- this index  
- the Julia mirror  

to enforce runtime safety and operator‑boundedness.

---

📜 Provenance Footer

Constellation‑Manifold‑Toolkit  
Lean Index v2.0  
Generated as part of Series III completion and structural consolidation  
Dublin, Ireland — Tuesday, 27 October 2026, 22:47 GMT  

---
