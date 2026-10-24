# Constellation Manifold Notebook 02

Flow‑Field Explorer
Altitude: A2 (Diagnostic Surface)

This notebook visualizes the flow field induced by ternary vectors embedded
into the constellation manifold. Flow fields reveal directional tendencies,
local instabilities, and membrane‑bounded drift behavior.

It builds directly on Notebook 01 and uses the A3 runtime defined in
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
symbolic = ["+", "-", "+", "0", "+", "+", "-", "0", "+"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z
`

---

3. Compute Diagnostics

`python
diag = rt.diagnose(E)
drift = diag["drift"]
collapse = diag["collapse_grad"]
`

---

4. Construct Flow Field

The flow field is defined as:

`
F[i] = drift[i] - collapse_grad[i]
`

This highlights:

- stable regions  
- unstable regions  
- membrane‑bounded directional tendencies  

`python
flow = drift - collapse
flow
`

---

5. Flow‑Field Plot

`python
plt.figure(figsize=(10,4))
plt.plot(flow, marker="o")
plt.title("Constellation Flow Field")
plt.xlabel("Index")
plt.ylabel("Flow")
plt.grid(True)
plt.show()
`

---

6. Vector‑Field Arrows

`python
plt.figure(figsize=(12,2))
plt.quiver(range(len(flow)), np.zeros(len(flow)), flow, np.zeros(len(flow)),
           angles='xy', scale_units='xy', scale=1)
plt.title("Flow‑Field Arrows")
plt.yticks([])
plt.xlabel("Index")
plt.show()
`

---

7. Combined Diagnostic View

`python
plt.figure(figsize=(10,6))

plt.subplot(3,1,1)
plt.plot(drift)
plt.title("Drift")

plt.subplot(3,1,2)
plt.plot(collapse)
plt.title("Collapse Gradient")

plt.subplot(3,1,3)
plt.plot(flow)
plt.title("Flow Field")

plt.tight_layout()
plt.show()
`

---

8. Reversibility Check

`python
E_prev = E.copy()
Erev = rt.reverse(Eprev)

(Eprev.v == Erev.v).all()
`

---

9. Provenance Log

`python
rt.provenance
`

---

Notes

- Flow fields operate at altitude A2.  
- Drift and collapse gradients originate at altitude A3.  
- Julia geometry engine can replace Python for high‑precision flow fields.  
- Rust kernels (future) will provide deterministic flow‑field computation.  

This notebook is part of the Constellation Manifold Toolkit.
`

---

