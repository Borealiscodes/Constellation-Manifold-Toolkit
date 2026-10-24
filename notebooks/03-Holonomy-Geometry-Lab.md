# Constellation Manifold Notebook 03

Holonomy Geometry Lab
Altitude: A2 (Diagnostic Surface)

This notebook explores holonomy in the constellation embedding manifold.
Holonomy measures cumulative phase change across the ternary vector and reveals
global geometric behavior such as winding, drift accumulation, and membrane‑bounded
phase collapse.

It builds on Notebooks 01 and 02 and uses the A3 runtime defined in
/manifold/embedding.py.

---

1. Setup

`python
from manifold.embedding import ConstellationRuntime
import numpy as np
import matplotlib.pyplot as plt

rt = ConstellationRuntime()
`

---

2. Example Symbolic Input

`python
symbolic = ["+", "+", "0", "-", "+", "-", "-", "+", "0", "+"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z
`

---

3. Compute Diagnostics

`python
diag = rt.diagnose(E)
hol = diag["holonomy"]
drift = diag["drift"]
collapse = diag["collapse_grad"]
`

---

4. Holonomy Curve

`python
plt.figure(figsize=(10,4))
plt.plot(hol, marker="o")
plt.title("Holonomy Accumulation Curve")
plt.xlabel("Index")
plt.ylabel("Phase")
plt.grid(True)
plt.show()
`

---

5. Holonomy vs Drift

Holonomy shows global phase, drift shows local directional tendency.

`python
plt.figure(figsize=(10,4))
plt.plot(hol, label="Holonomy")
plt.plot(drift, label="Drift")
plt.title("Holonomy vs Drift")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

6. Holonomy vs Collapse Gradient

Collapse gradient shows local instability.

`python
plt.figure(figsize=(10,4))
plt.plot(hol, label="Holonomy")
plt.plot(collapse, label="Collapse Gradient")
plt.title("Holonomy vs Collapse Gradient")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

7. Holonomy Phase Portrait

`python
plt.figure(figsize=(6,6))
plt.scatter(hol, drift, c=collapse, cmap="magma", s=80)
plt.title("Holonomy Phase Portrait")
plt.xlabel("Holonomy")
plt.ylabel("Drift")
plt.colorbar(label="Collapse Gradient")
plt.grid(True)
plt.show()
`

---

8. Holonomy Loop Detection

A holonomy loop occurs when cumulative phase returns near zero.

`python
loops = [i for i, h in enumerate(hol) if abs(h) < 0.01]
loops
`

---

9. Reversibility Check

`python
E_prev = E.copy()
Erev = rt.reverse(Eprev)

(Eprev.v == Erev.v).all()
`

---

10. Provenance Log

`python
rt.provenance
`

---

Notes

- Holonomy operates at altitude A2.  
- Drift and collapse gradients originate at altitude A3.  
- Julia geometry engine can compute holonomy with higher precision.  
- Rust kernels (future) will provide deterministic holonomy evolution.  

This notebook is part of the Constellation Manifold Toolkit.
`

---
