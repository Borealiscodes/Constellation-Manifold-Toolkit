# Constellation Manifold Notebook 05

Manifold Flow Simulation
Altitude: A2 (Diagnostic Surface)

This notebook simulates manifold flow, evolving an embedding state across
multiple time steps using drift, roughness, holonomy, and collapse gradients.
Flow simulation reveals long‑range behavior, stability pockets, and membrane‑bounded
trajectory evolution.

It builds on Notebooks 01–04 and uses the A3 runtime defined in
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
symbolic = ["+", "-", "+", "+", "0", "-", "+", "0", "+", "-"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z
`

---

3. Flow Update Rule

We define a simple flow update:

`
znext = z + drift - collapsegrad
`

This is membrane‑bounded and reversible.

`python
def flow_step(E, diag):
    znext = E.z + diag["drift"] - diag["collapsegrad"]
    znext = np.clip(znext, -1, 1)  # membrane enforcement
    return z_next
`

---

4. Run Simulation

`python
steps = 20
trajectory = []

E_current = E.copy()

for _ in range(steps):
    diag = rt.diagnose(E_current)
    znext = flowstep(E_current, diag)
    Ecurrent.z = znext
    trajectory.append(z_next.copy())
`

---

5. Plot Trajectory

`python
plt.figure(figsize=(10,4))
for i, z in enumerate(trajectory):
    plt.plot(z, alpha=0.3)
plt.title("Manifold Flow Trajectory")
plt.xlabel("Index")
plt.ylabel("Coordinate")
plt.grid(True)
plt.show()
`

---

6. Heatmap of Flow Evolution

`python
plt.figure(figsize=(10,4))
plt.imshow(np.array(trajectory), cmap="viridis", aspect="auto")
plt.title("Flow Evolution Heatmap")
plt.xlabel("Index")
plt.ylabel("Time Step")
plt.colorbar()
plt.show()
`

---

7. Stability Detection

A stable region satisfies:

`
|z_next - z| < epsilon
`

`python
epsilon = 0.05
stable_steps = []

for i in range(1, len(trajectory)):
    diff = np.abs(trajectory[i] - trajectory[i-1])
    if np.all(diff < epsilon):
        stable_steps.append(i)

stable_steps
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

- Flow simulation operates at altitude A2.  
- Drift and collapse gradients originate at altitude A3.  
- Julia geometry engine can simulate flow with higher precision.  
- Rust kernels (future) will provide deterministic flow evolution.  

This notebook is part of the Constellation Manifold Toolkit.
`

---

