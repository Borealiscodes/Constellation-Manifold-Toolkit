# 🧾 CMT‑LeanPopulationStrategy‑v2.0.md

Constellation‑Manifold‑Toolkit — Developer Infrastructure Note

Version: 27 October 2026 — Dublin, Ireland

---

Lean v2.0 Population Strategy Note

Formal Spine Population Order • Guard‑Macro Enforcement • Altitude‑Safety Map

This document defines the population strategy for the Lean v2.0 formal spine used in Enhanced Conceptual Collision Tracking. It specifies:

- module population order  
- dependency constraints  
- guard‑macro enforcement  
- altitude‑safety rules  
- proof‑obligation mapping  
- Julia‑mirroring boundaries  

This note is developer infrastructure, not mathematical content.  
It must be read before populating any Lean module.

---

🧱 1. Purpose of This Strategy

Lean v2.0 introduces:

- 10 altitude‑safe modules  
- 7 guard‑macro modules  
- 3 operator layers (Φ₇, Φ₈, Φ₉)  
- 6 diagnostic layers  
- 1 collision‑invariant integration layer  

These modules form a formal spine, not a flat file set.  
Population must follow a strict order to avoid:

- circular imports  
- broken invariants  
- guard‑macro misalignment  
- altitude contamination  
- operator leakage  
- tensor‑shape hallucination  
- domain inference drift  

This strategy prevents all of the above.

---

🧭 2. Population Order (Mandatory)

Lean modules must be populated in the following order:

1. ConceptualManifold.lean
Defines the manifold, density, smoothness, and non‑degeneracy.  
All geometry depends on this.

2. ConceptualMetric.lean
Defines the metric tensor, symmetry, positive‑definiteness, determinant.  
Required by curvature envelopes and deformation tensors.

3. CurvatureEnvelope.lean (Φ₇)
Defines curvature envelope, Ricci‑flow perturbation, softmax potential, collapse‑surface precursors.

4. CollapseSurface.lean
Defines Hessian negativity, eigenvalue analysis, collapse‑surface predicate.

5. SoftDeformation.lean
Defines deformation tensor, perturbed metric, smoothness, non‑degeneracy.

6. EnvelopeSafety.lean
Defines gravitational envelope, energy‑density bounds, envelope safety.

7. DriftSemigroup.lean (Φ₈)
Defines pseudo‑Kraus operators, semigroup closure, drift magnitude.

8. InvariantExtraction.lean (Φ₉)
Defines invariant extraction, trace/norm/determinant invariants.

9. PhaseShiftOperator.lean
Defines reversible phase‑shift steering and trajectory modulation.

10. CollisionInvariant.lean
Integrates Φ₇–Φ₉, collapse surfaces, drift precursors, invariants, and envelope safety into the collision‑avoidance theorem.

This order is not optional.  
It is the only altitude‑safe sequence.

---

🛡️ 3. Guard‑Macro Enforcement Plan

Every module must begin with:

`lean
import LeanHallucinationGuard.Guard
`

Guard macros must be applied to:

Structures
`

guard ConceptualManifold

guard ConceptualMetric

guard CurvatureEnvelope

guard CollapseSurface

guard DriftSemigroup

guard InvariantExtraction

guard DeformationTensor

guard PhaseShiftOperator

guard EnvelopeSafety

guard CollisionInvariant
`

Operators
`

guardRank

guardTensor

guardDomain

guardCurvature

guardDrift

guardInvariant
`

Guard macros enforce:

- operator rank  
- tensor shape  
- domain correctness  
- curvature envelope bounds  
- semigroup closure  
- invariant stability  
- deformation smoothness  

This prevents Lean from silently accepting malformed definitions.

---

📐 4. Proof‑Obligation Map

Each module has mandatory proof obligations:

ConceptualManifold
- smoothness  
- non‑degeneracy  

ConceptualMetric
- symmetry  
- positive‑definiteness  
- determinant non‑zero  

CurvatureEnvelope
- envelope smoothness  
- curvature‑well correctness  

CollapseSurface
- Hessian negativity  
- collapse‑surface predicate  

SoftDeformation
- smoothness  
- non‑degeneracy  
- curvature‑bound preservation  

EnvelopeSafety
- energy‑density bound  

DriftSemigroup
- semigroup closure  
- boundedness  
- trace preservation  

InvariantExtraction
- invariant stability  
- determinant/trace/norm invariance  

PhaseShiftOperator
- reversibility  
- safety  

CollisionInvariant
- avoidance theorem  
- distance‑threshold correctness  
- invariant convergence  

These proofs must follow the Lean Proof Template v2.0.

---

🧩 5. Altitude‑Safety Rules

Altitude boundaries:

- A0–A2: reversible geometry  
- A3–A4: diagnostics and operator dynamics  
- A5: falsification layer  

Rules:

- geometry cannot depend on drift or invariants  
- drift cannot depend on collision invariants  
- invariants cannot depend on deformation tensors  
- collision cannot depend on softmax algebra  
- no semantic continuity  
- no generative priors  
- no inference drift  
- no runtime physics in reversible geometry  
- no reversible geometry in runtime physics  

These rules prevent conceptual contamination.

---

🔧 6. Julia‑Mirroring Constraints

Julia runtime must mirror:

- curvature envelope invariants  
- drift semigroup evolution  
- invariant extraction  
- deformation tensors  
- envelope safety  
- collision invariants  

But Julia must not mirror:

- guard macros  
- altitude rules  
- proof obligations  
- Lean‑specific structures  

Julia mirrors results, not formal scaffolding.

---


📜 Provenance

Developed for the Constellation‑Manifold‑Toolkit cross‑analysis spine.  
Defines the population strategy for Lean v2.0 under Hallucination Guard v2.0.  
Supports altitude‑safe operator geometry and reversible conceptual‑manifold formalization.

Author: Borealis S. Hedling — Dublin, Ireland  
Version: CMT‑LeanPopulationStrategy‑v2.0 — 27 October 2026

---

