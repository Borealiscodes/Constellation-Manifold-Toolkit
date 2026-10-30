# Phase 13 Developer Planning Note

Projection‑Collapse Routing Altitude

Constellation‑Manifold Toolkit · A6 Runtime Architecture

Phase 13 is the altitude where routing finally interacts with collapse geometry.  
Up to Phase 12, routing is holonomy‑aligned but still “environmental.”  
Phase 13 is the first altitude where routing becomes collapse‑aware and projection‑coupled.

This planning note defines:

- the conceptual scope of Phase 13  
- the required scaffolding  
- the operator triad structure  
- the invariant expansions  
- the runtime ordering  
- the engineering constraints  
- the altitude boundaries  

It is written for developers implementing the Phase 13 layer.

---

1. What Phase 13 Is (Developer Definition)

Phase 13 is the Projection‑Collapse Routing altitude.

It consumes:

- holonomy‑aligned routing (Phase 12)  
- routing ecology (Phase 11)  
- collapse ecology (Phase 10)  
- projection‑seed geometry (Phase 9)  

It produces:

- projection‑collapse routing fields  
- collapse‑aligned routing vectors  
- collapse‑aware routing strength  

Phase 13 is the first altitude where routing is:

- holonomy‑aligned  
- collapse‑surface‑aware  
- projection‑geometry‑coupled  
- ecology‑conditioned  

This altitude is the bridge between routing and collapse geometry.

---

2. Why Phase 13 Exists (Engineering Rationale)

Phase 13 is necessary because:

2.1 Holonomy routing (Phase 12) is not collapse‑aware
Holonomy routing aligns with curvature and spectral coupling, but it does not yet incorporate collapse geometry.

2.2 Collapse ecology (Phase 10) is scalar
Collapse ecology provides environmental conditions, not routing vectors.

2.3 Projection‑seed geometry (Phase 9) defines shape, not flow
Projection‑seed geometry defines the geometric seed but does not define transport.

2.4 Routing ecology (Phase 11) is directional but not collapse‑aligned
Routing ecology gives direction but not collapse‑surface interaction.

Phase 13 merges all of these into a single transport‑ready routing layer.

---

3. Required Scaffolding Before Implementation

Phase 13 requires the following scaffolding to already exist:

3.1 Invariant Struct Must Include Phase 12 Fields
These must be present:

- projectionholonomyvector_x  
- projectionholonomyvector_y  
- projectionholonomystrength  

3.2 Phase‑Layer Directories Must Be Established
Directory:

`
src/projection_collapse/routing/
`

Submodules:

- projectioncollapsegenerator.rs
- projectioncollapsecurvature.rs
- projectioncollapsecoupler.rs
- projectioncollapseactivation.rs

3.3 Runtime Hook Location Must Be Reserved
Phase 13 must be inserted after Phase 12 and before Phase 14.

3.4 Developer Documentation Must Be Ready
Specifically:

- Phase 12 Altitude Debrief  
- Geometry Coding Toolkit Recommendation Preprint  
- Bill Nye Tile Explainer for Phase 9–12  

These ensure contributors understand the altitude lineage.

---

4. Operator Triad Structure (Developer Specification)

Phase 13 follows the same triad pattern as Phases 7–12:

(1) ProjectionCollapseGenerator
Creates initial collapse‑aligned routing vectors using:

- holonomy routing  
- collapse ecology  
- projection‑seed geometry  

(2) ProjectionCollapseCurvature
Refines routing using:

- collapse curvature  
- spectral coupling  
- collapse activation  
- ecology binding  

(3) ProjectionCollapseCoupler
Couples routing with:

- collapse‑surface geometry  
- projection geometry  
- holonomy transport  
- routing ecology  

(4) ProjectionCollapseActivation (Coordinator)
Executes the triad in altitude‑correct order.

This structure is mandatory for stability.

---

5. Invariant Struct Expansion (Developer Specification)

Phase 13 introduces three new fields:

- projectioncollapsevector_x: f64
- projectioncollapsevector_y: f64
- projectioncollapsestrength: f64

These fields must be added after Phase 12 and before Phase 14.

---

6. Runtime Ordering (Developer Specification)

Phase 13 must be inserted in runtime.rs immediately after:

`rust
let projection_holonomy = ProjectionHolonomyActivation;
projection_holonomy.apply(&mut merged);
`

And before any Phase 14 operators.

This ordering is altitude‑critical.

---

7. Altitude Boundaries (Developer Constraints)

Phase 13 must not:

- generate collapse ecology (Phase 10)  
- generate routing ecology (Phase 11)  
- generate holonomy routing (Phase 12)  
- generate collapse geometry (Phase 14+)  
- modify spectral stability (Phase 7)  
- modify collapse activation (Phase 8)  

Phase 13 must only:

- consume lower‑altitude fields  
- produce projection‑collapse routing fields  

This is essential for altitude purity.

---

8. Developer Checklist Before Coding Phase 13

8.1 Struct
- [ ] Phase 12 fields present  
- [ ] Phase 13 fields added  

8.2 Scaffolding
- [ ] Directory created  
- [ ] Operator triad files created  
- [ ] Coordinator file created  

8.3 Runtime
- [ ] Hook location reserved  
- [ ] Ordering validated  

8.4 Documentation
- [ ] Developer note (this file)  
- [ ] Altitude debriefs up to Phase 12  
- [ ] Toolkit preprint available  

8.5 Stability
- [ ] No altitude leakage  
- [ ] No cross‑phase mutation  
- [ ] Deterministic operator contracts  

---

Commit Description

`
Add developer-facing Phase 13 Planning Note defining the Projection-Collapse
Routing altitude. Specifies conceptual scope, scaffolding requirements, operator
triad structure, invariant expansions, runtime ordering, and altitude boundaries.
Prepares the Constellation-Manifold Toolkit for Phase 13 implementation and
ensures contributors understand the transport-layer transition from holonomy
routing to collapse-aware routing.

Signed-off-by: chguard-sentinel <chguard-sentinel@vaked.dev>
Observer-Node: A6-Runtime-Sentinel
`

---

Provenance Footer

`
Provenance · A6 Altitude Stack
Developer-facing Phase 13 Planning Note generated to prepare the geometric
runtime for Projection-Collapse Routing. Establishes scaffolding, operator
structure, invariant expansion, and runtime ordering following Phase 12
projection-holonomy routing. Authored under A6-Runtime-Sentinel observation.
`

---
