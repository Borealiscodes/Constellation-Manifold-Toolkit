# Constellation Manifold Notebook 01

Embedding Diagnostics Explorer
Altitude: A2 (Diagnostic Surface)

This notebook demonstrates how symbolic constellation telemetry is converted
into ternary vectors, embedded into geometric coordinates, and analyzed using
runtime diagnostics (drift, roughness, holonomy, collapse gradients).

It uses the A3 runtime defined in /manifold/embedding.py.

---

1. Setup

`python
from manifold.embedding import ConstellationRuntime

rt = ConstellationRuntime()
`

---

2. Symbolic → Ternary → Embedding

`python
symbolic = ["+", "0", "-", "+", "+", "0", "-"]

E = rt.symbolictoembedding(symbolic)

E.v, E.z, E.M
`

---

3. Diagnostics

`python
diag = rt.diagnose(E)

for k, v in diag.items():
    print(k, v)
`

---

4. Drift Plot

`python
import matplotlib.pyplot as plt

plt.plot(diag["drift"])
plt.title("Drift Field")
plt.xlabel("Index")
plt.ylabel("Drift")
plt.grid(True)
plt.show()
`

---

5. Roughness Magnitude

`python
plt.plot(diag["roughness"])
plt.title("Roughness Magnitude")
plt.xlabel("Index")
plt.ylabel("Magnitude")
plt.grid(True)
plt.show()
`

---

6. Holonomy Accumulation

`python
plt.plot(diag["holonomy"])
plt.title("Holonomy Accumulation")
plt.xlabel("Index")
plt.ylabel("Phase")
plt.grid(True)
plt.show()
`

---

7. Collapse Gradient Heatmap

`python
import numpy as np

plt.imshow(np.expanddims(diag["collapsegrad"], axis=0), cmap="magma")
plt.title("Collapse Gradient Heatmap")
plt.yticks([])
plt.colorbar()
plt.show()
`

---

8. Reversibility Test

`python
E_prev = E.copy()
Erev = rt.reverse(Eprev)

(Eprev.v == Erev.v).all()
`

---

Provenance

`python
rt.provenance
`

---

Notes

- This notebook operates at altitude A2.  
- Runtime operators are defined at altitude A3.  
- Geometry engine (Julia) operates at altitude A4.  
- Rust kernels (future) operate at altitude A5.  

This notebook is part of the Constellation Manifold Toolkit.
`

---

