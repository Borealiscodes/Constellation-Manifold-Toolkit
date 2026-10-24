# Constellation Manifold Notebook 08

Provenance Telemetry Explorer
Altitude: A2 (Diagnostic Surface)

This notebook explores provenance telemetry, the structured log of all
runtime events in the constellation embedding system. Provenance provides a
contamination‑safe record of:

- operator dispatch  
- embedding transitions  
- diagnostic evaluations  
- membrane guard interventions  
- reversals  
- symbolic → ternary → manifold evolution  

It builds on Notebooks 01–07 and uses the A3 runtime defined in
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
symbolic = ["+", "-", "+", "0", "+", "-", "+", "0", "-", "+"]
E = rt.symbolictoembedding(symbolic)

E.v, E.z
`

---

3. Run Diagnostics

`python
diag = rt.diagnose(E)
diag
`

---

4. Inspect Provenance Log

`python
rt.provenance
`

You should see entries like:

- activation+transition  
- diagnostic  
- guard  
- reversal  

Each entry is a structured dictionary.

---

5. Provenance Timeline

`python
events = [p["event"] for p in rt.provenance]

plt.figure(figsize=(10,3))
plt.plot(events, marker="o")
plt.title("Provenance Timeline")
plt.xlabel("Step")
plt.ylabel("Event")
plt.grid(True)
plt.show()
`

---

6. Provenance Event Types

`python
from collections import Counter

Counter(events)
`

This reveals the distribution of operator activity.

---

7. Provenance Detail View

`python
for i, entry in enumerate(rt.provenance):
    print(f"Step {i}: {entry}")
`

---

8. Provenance Heatmap (Event Frequency)

Convert event types into numeric categories:

`python
event_map = {e: i for i, e in enumerate(set(events))}
numericevents = [eventmap[e] for e in events]

plt.figure(figsize=(10,2))
plt.imshow(np.expanddims(numericevents, axis=0), cmap="viridis")
plt.title("Provenance Event Heatmap")
plt.yticks([])
plt.colorbar()
plt.show()
`

---

9. Provenance + Drift Overlay

`python
plt.figure(figsize=(10,4))
plt.plot(diag["drift"], label="Drift", color="blue")
plt.scatter(range(len(events)), numeric_events, color="orange", label="Events")
plt.title("Drift vs Provenance Events")
plt.xlabel("Index")
plt.legend()
plt.grid(True)
plt.show()
`

---

10. Provenance + Collapse Gradient Overlay

`python
plt.figure(figsize=(10,4))
plt.plot(diag["collapse_grad"], label="Collapse Gradient", color="purple")
plt.scatter(range(len(events)), numeric_events, color="orange", label="Events")
plt.title("Collapse Gradient vs Provenance Events")
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

12. Final Provenance Dump

`python
rt.provenance
`

---

Notes

- Provenance operates at altitude A2.  
- Operator dispatch originates at altitude A3.  
- Julia geometry engine can emit provenance for high‑precision geometry.  
- Rust kernels (future) will provide deterministic provenance logging.  

This notebook is part of the Constellation Manifold Toolkit.
`

---
