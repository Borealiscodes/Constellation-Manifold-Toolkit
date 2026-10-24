# Constellation Manifold Notebook 06

Membrane Stress & Envelope Analysis
Altitude: A2 (Diagnostic Surface)

This notebook analyzes membrane stress, envelope bounds, and violation
risk in the constellation embedding manifold. The membrane envelope M enforces
the ternary bounds {‑1, 0, +1} and ensures contamination‑safe behavior.

This notebook visualizes:

- membrane envelope  
- stress magnitude  
- stress heatmaps  
- violation risk  
- envelope‑bounded drift and collapse behavior  

It builds on Notebooks 01–05 and uses the A3 runtime defined in
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

2. Symbolic Input

`python
symbolic = ["+", "-", "+", "0", "-", "+", "+", "0", "-", "+"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z, E.M
`

---

3. Compute Diagnostics

`python
diag = rt.diagnose(E)
drift = diag["drift"]
roughness = diag["roughness"]
collapse = diag["collapse_grad"]
hol = diag["holonomy"]
`

---

4. Membrane Envelope Plot

`python
plt.figure(figsize=(10,4))
plt.plot(E.M, marker="o", color="black")
plt.title("Membrane Envelope")
plt.xlabel("Index")
plt.ylabel("Envelope Value")
plt.grid(True)
plt.show()
`

---

5. Stress Magnitude

Stress is defined as:

`
stress[i] = |z[i] - M[i]|
`

This measures how close the embedding is to violating the membrane.

`python
stress = np.abs(E.z - E.M)
stress
`

---

6. Stress Curve

`python
plt.figure(figsize=(10,4))
plt.plot(stress, marker="o", color="red")
plt.title("Membrane Stress Magnitude")
plt.xlabel("Index")
plt.ylabel("Stress")
plt.grid(True)
plt.show()
`

---

7. Stress Heatmap

`python
plt.figure(figsize=(10,2))
plt.imshow(np.expand_dims(stress, axis=0), cmap="Reds")
plt.title("Membrane Stress Heatmap")
plt.yticks([])
plt.colorbar()
plt.show()
`

---

8. Stress vs Drift

`python
plt.figure(figsize=(10,4))
plt.plot(stress, label="Stress", color="red")
plt.plot(drift, label="Drift", color="blue")
plt.title("Stress vs Drift")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

9. Stress vs Collapse Gradient

`python
plt.figure(figsize=(10,4))
plt.plot(stress, label="Stress", color="red")
plt.plot(collapse, label="Collapse Gradient", color="purple")
plt.title("Stress vs Collapse Gradient")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

10. Violation Risk Detection

Violation risk occurs when:

`
stress[i] > threshold
`

`python
threshold = 0.5 * np.max(stress)
risk_points = [i for i, s in enumerate(stress) if s > threshold]
risk_points
`

---

11. Reversibility Check

`python
E_prev = E.copy()
Erev = rt.reverse(Eprev)

(Eprev.v == Erev.v).all()
`

---

12. Provenance Log

`python
rt.provenance
`

---

Notes

- Membrane stress operates at altitude A2.  
- Envelope enforcement originates at altitude A3.  
- Julia geometry engine can compute stress with higher precision.  
- Rust kernels (future) will provide deterministic membrane enforcement.  

This notebook is part of the Constellation Manifold Toolkit.
`

---

