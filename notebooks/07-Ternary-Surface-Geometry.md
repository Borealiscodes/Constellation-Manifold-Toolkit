# Constellation Manifold Notebook 07

Ternary Surface Geometry
Altitude: A2 (Diagnostic Surface)

This notebook visualizes ternary surface geometry by mapping ternary vectors
onto a 2D or 3D surface. The surface reveals curvature, membrane stress,
instability pockets, and geometric deformation induced by drift, roughness, and
collapse gradients.

It builds on Notebooks 01–06 and uses the A3 runtime defined in
/manifold/embedding.py.

---

1. Setup

`python
from manifold.embedding import ConstellationRuntime
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

rt = ConstellationRuntime()
`

---

2. Symbolic Input

`python
symbolic = ["+", "0", "-", "+", "+", "-", "0", "+", "-", "+"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z
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

4. Construct Surface Coordinates

We define a ternary surface:

`
x[i] = i
y[i] = z[i]
z[i] = drift[i] - collapse[i]
`

This produces a membrane‑bounded geometric surface.

`python
x = np.arange(len(E.z))
y = E.z
z = drift - collapse
`

---

5. 2D Surface Plot

`python
plt.figure(figsize=(10,4))
plt.plot(x, y, marker="o")
plt.title("Ternary Surface (2D)")
plt.xlabel("Index")
plt.ylabel("Surface Height")
plt.grid(True)
plt.show()
`

---

6. 3D Surface Geometry

`python
fig = plt.figure(figsize=(10,6))
ax = fig.add_subplot(111, projection='3d')

ax.plot(x, y, z, marker="o")
ax.set_title("Ternary Surface Geometry (3D)")
ax.set_xlabel("Index")
ax.set_ylabel("Embedding z")
ax.set_zlabel("Flow (drift - collapse)")
plt.show()
`

---

7. Curvature Approximation

Curvature is approximated by:

`
curv[i] = |z[i+1] - 2*z[i] + z[i-1]|
`

`python
curv = np.zeros_like(z)
for i in range(1, len(z)-1):
    curv[i] = abs(z[i+1] - 2*z[i] + z[i-1])
curv
`

---

8. Curvature Plot

`python
plt.figure(figsize=(10,4))
plt.plot(curv, marker="o", color="purple")
plt.title("Surface Curvature")
plt.xlabel("Index")
plt.ylabel("Curvature")
plt.grid(True)
plt.show()
`

---

9. Curvature Heatmap

`python
plt.figure(figsize=(10,2))
plt.imshow(np.expand_dims(curv, axis=0), cmap="Purples")
plt.title("Curvature Heatmap")
plt.yticks([])
plt.colorbar()
plt.show()
`

---

10. Membrane Stress Overlay

`python
stress = np.abs(E.z - E.M)

plt.figure(figsize=(10,4))
plt.plot(stress, label="Stress", color="red")
plt.plot(curv, label="Curvature", color="purple")
plt.title("Stress vs Curvature")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
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

- Ternary surface geometry operates at altitude A2.  
- Drift, roughness, holonomy, and collapse gradients originate at altitude A3.  
- Julia geometry engine can compute curvature with higher precision.  
- Rust kernels (future) will provide deterministic surface evolution.  

This notebook is part of the Constellation Manifold Toolkit.
`

---
