# Constellation Manifold Notebook 04

Collapse Gradient Analyzer
Altitude: A2 (Diagnostic Surface)

This notebook analyzes collapse gradients, the local instability operator of
the constellation embedding manifold. Collapse gradients reveal where ternary
vectors produce sharp transitions, membrane stress, or altitude‑bounded
instability.

It builds on Notebooks 01–03 and uses the A3 runtime defined in
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
symbolic = ["+", "-", "-", "+", "0", "+", "-", "+", "+", "0", "-"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z
`

---

3. Compute Diagnostics

`python
diag = rt.diagnose(E)
collapse = diag["collapse_grad"]
drift = diag["drift"]
roughness = diag["roughness"]
hol = diag["holonomy"]
`

---

4. Collapse Gradient Curve

`python
plt.figure(figsize=(10,4))
plt.plot(collapse, marker="o", color="crimson")
plt.title("Collapse Gradient")
plt.xlabel("Index")
plt.ylabel("Gradient")
plt.grid(True)
plt.show()
`

---

5. Collapse Gradient Heatmap

`python
plt.figure(figsize=(10,2))
plt.imshow(np.expand_dims(collapse, axis=0), cmap="inferno")
plt.title("Collapse Gradient Heatmap")
plt.yticks([])
plt.colorbar()
plt.show()
`

---

6. Collapse vs Drift

Collapse gradient shows instability.  
Drift shows directional tendency.

`python
plt.figure(figsize=(10,4))
plt.plot(collapse, label="Collapse Gradient", color="crimson")
plt.plot(drift, label="Drift", color="steelblue")
plt.title("Collapse Gradient vs Drift")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

7. Collapse vs Roughness

`python
plt.figure(figsize=(10,4))
plt.plot(collapse, label="Collapse Gradient", color="crimson")
plt.plot(roughness, label="Roughness", color="darkgreen")
plt.title("Collapse Gradient vs Roughness")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

8. Collapse vs Holonomy

`python
plt.figure(figsize=(10,4))
plt.plot(collapse, label="Collapse Gradient", color="crimson")
plt.plot(hol, label="Holonomy", color="purple")
plt.title("Collapse Gradient vs Holonomy")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

9. Instability Detection

An instability occurs when:

`
|collapse_grad[i]| > threshold
`

`python
threshold = 0.5 * np.max(np.abs(collapse))
instabilities = [i for i, c in enumerate(collapse) if abs(c) > threshold]
instabilities
`

---

10. Reversibility Check

`python
E_prev = E.copy()
Erev = rt.reverse(Eprev)

(Eprev.v == Erev.v).all()
`

---

11. Provenance Log

`python
rt.provenance
`

---

Notes

- Collapse gradients operate at altitude A2.  
- They originate from altitude A3 operators.  
- Julia geometry engine can compute collapse gradients with higher precision.  
- Rust kernels (future) will provide deterministic collapse‑gradient evolution.  

This notebook is part of the Constellation Manifold Toolkit.
`

---

