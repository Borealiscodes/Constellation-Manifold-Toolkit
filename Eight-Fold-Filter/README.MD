# 🧭 `Eight-Fold-Filter/` — Non-Occult Pre-Crate Admission Filter (`sovereign_admission_filter.py`)

**Repository:** `Borealiscodes/Constellation-Manifold-Toolkit` (`Eight-Fold-Filter/`)  
**Maintainer:** Borealis S. Hedling (`NDH-RESEARCH-PILOT`)  
**Lineage Reference:** `NDH-Meta-Systems/ahso/series-audit` (`audit-artifact-1` & `audit-artifact-4`)  
**Posture:** Non-Accusatory • Non-Occult • Deterministic • Epistemically Humble  
**Membrane:** Pre-Ingestion Quarantine Seam (`Record ≠ Admit ≠ Commit`)  

---

## 🎨 Visual Gate Status Tiles

The filter evaluates candidate knowledge artifacts across six deterministic gates before they enter a canonical library crate (`sovereign_admission_filter.py`). Every gate emits a structured receipt rather than a moral judgment:

| Status Tile | Verdict State | Operational Meaning |
| :--- | :--- | :--- |
| 🟩 **`ADMIT`** | `compliance_score_pq == "6/6"` | All six provenance, anti-stigma, ecological, and altitude-separation predicates passed. Safe to proceed to commit review. |
| 🟨 **`QUARANTINE_FOR_REVIEW`** | `compliance_score_pq < "6/6"` | One or more structural boundaries require human maintainer review (e.g., missing primary citations or lifecycle context). Held cleanly without censorship or silent rewriting. |
| 🟦 **`OBSERVED_ANCHOR`** | `raw_sha256` Verified | Immutable `SHA-256` byte fingerprint recorded prior to any transformation or indexing (*Right Mindfulness*). |
| 🟪 **`ALTITUDE_SEPARATED`** | `Non-Activation` Verified | Descriptive metaphors and contemplative labels remain strictly isolated from executable runtime control flow (*Right Concentration*). |

---

## 1. Philosophical, Computational & Mathematical Grounding

In `NDH-Meta-Systems/ahso/series-audit` (`audit-artifact-4-academic-karmayoga-spectral-reframe.md`), contemplative traditions are treated strictly as **secular structural design patterns**—never as metaphysical claims or mystical authority.

### 1.1 Philosophical Grounding (*Pratītyasamutpāda*, *Ahiṃsā*, & *Niṣkāma Karma*)
* **Dependent Origination (*Pratītyasamutpāda*):** No software toolchain, monograph, or audio digest exists in a vacuum. Every technical artifact depends on upstream authors, institutional networks, and physical hardware supply chains. *Right View* requires acknowledging those dependencies honestly rather than presenting collaborative or resource-intensive work as solitary and weightless.
* **Non-Harm (*Ahiṃsā*) & Anti-Stigma:** *Right Intention* and *Right Speech* forbid turning clinical psychiatric labels into rhetorical shorthand for interpersonal conflict, and forbid presenting synthetic summaries as substitutes for primary sources.
* **Detached Stewardship (*Niṣkāma Karma* / Karma Yoga):** Action is separated from egoic extraction. Ingesting knowledge without stripping upstream attribution or exceeding host capacity ceilings preserves co-sovereign collaboration across repositories.

### 1.2 Computational Grounding (Zero-Dependency Deterministic Seam)
* **Zero External Dependencies:** Implemented in pure Python 3.10+ standard library (`hashlib`, `re`, `fractions`, `dataclasses`, `json`) so the filter runs identically on constrained ARM64 edge hardware without external model calls or floating-point drift.
* **Fail-Closed Quarantine (`Record ≠ Admit ≠ Commit`):** Flagged artifacts are never deleted, silenced, or rewritten by an LLM. They are routed to `QUARANTINE_FOR_REVIEW` with an itemized list of failed gate IDs so a human maintainer can supply missing context.

### 1.3 Mathematical Grounding (Boolean Predicates & Exact Rationals $\mathbb{Q}$)
Let an incoming candidate artifact be represented as a tuple $x = (c, m)$ where $c \in \Sigma^*$ is raw UTF-8 text and $m$ is structural metadata.
1. **Cryptographic Pre-Image Anchor:** Before evaluation, $h(c) = \text{SHA-256}(c) \in \{0,1\}^{256}$ fixes an immutable byte witness.
2. **Orthogonal Gate Predicates:** Six deterministic boolean functions $g_i : \Sigma^* \times \mathcal{M} \to \{0, 1\}$ ($i \in \{1, \dots, 6\}$) test independent structural invariants without cross-gate compensation.
3. **Exact Rational Compliance Score ($p/q \in \mathbb{Q}$):** Rather than emitting rounded floating-point percentages that hide sample size, compliance is represented as an irreducible fraction:

$$\eta_{\text{gate}}(x) = \frac{1}{6}\sum_{i=1}^{6} g_i(x) \in \mathbb{Q}, \qquad \text{Verdict}(x) = \begin{cases} \texttt{ADMIT} & \text{if } \sum_{i=1}^6 g_i(x) = 6 \\ \texttt{QUARANTINE\_FOR\_REVIEW} & \text{otherwise} \end{cases}$$

---

## 2. Six-Gate Specification Matrix

| Gate ID & Tile | Contemplative Guidepost | Philosophical Principle | Computational & Mathematical Check |
| :--- | :--- | :--- | :--- |
| 🟦 **`G1_RIGHT_MINDFULNESS`** | **Right Mindfulness** (*Sammā Sati*) | Present-moment awareness of exact inputs before interpretation. | Verifies non-empty payload ($\|c\| > 0$) and computes immutable `SHA-256` digest prior to indexing. |
| 🟪 **`G2_KARMA_YOGA_ATTRIBUTION`** | **Karma Yoga** (*Niṣkāma Karma*) | Non-absorptive stewardship; action without erasing upstream lineage. | Boolean metadata predicate `has_upstream_attribution == True` preserving author sovereignty. |
| 🟩 **`G3_RIGHT_INTENTION_NON_HARM`** | **Right Intention** (*Sammā Saṅkappa*) | Non-harm (*Ahiṃsā*); refusing armchair clinical pathologization. | Regex boundary check ensuring psychiatric/diagnostic terms are not conflated with general relational/system conflict. |
| 🟨 **`G4_RIGHT_SPEECH_PROVENANCE`** | **Right Speech** (*Sammā Vācā*) | Truthful, non-conflated speech separating primary observation from summary. | Logical implication `is_synthetic_digest => cites_primary_source` (`RFC-007` anti-conflation rule). |
| 🟧 **`G5_RIGHT_VIEW_ECOLOGY`** | **Right View & Livelihood** (*Sammā Diṭṭhi*) | Dependent origination; recognizing material and planetary substrates. | Logical implication `mentions_hardware_chips => mentions_lifecycle_or_ecology` (e.g., mining, water, e-waste, `10.5281/zenodo.23246401`). |
| 🟥 **`G6_RIGHT_CONCENTRATION_ALTITUDE`** | **Right Concentration** (*Sammā Samādhi*) | Single-pointed clarity; keeping symbolic metaphors separate from runtime execution. | Boolean predicate `has_non_activation_clause == True` enforcing strict altitude separation. |

---

## 3. Epistemic Humility & Methodological Limitations

In keeping with `NDH-Meta-Systems` governance standards, users and reviewers must hold five explicit limits in view when interpreting `AdmissionReceipt` outputs:

1. **Lexical Heuristics Are Not Semantic Understanding:**  
   Regular expressions (`CLINICAL_STIGMA_PATTERNS`, `HARDWARE_CLAIM_PATTERNS`) detect surface keywords, not philosophical nuance or clinical context. An essay carefully critiquing psychiatric stigma (such as *The Charity Trap*) will legitimately trigger `G3_RIGHT_INTENTION_NON_HARM` for **human review**—which is why the verdict is `QUARANTINE_FOR_REVIEW` rather than automatic rejection.
2. **Contemplative Names Are Mnemonics, Not Authority:**  
   Using terms from the Noble Eightfold Path and Karma Yoga provides a structured ethical checklist for engineers. Passing `6/6` gates certifies *metadata and structural hygiene*—it does not confer moral perfection, spiritual authority, or empirical truth.
3. **Boolean Metadata Relies on Honest Caller Inputs:**  
   Flags such as `has_upstream_attribution` and `cites_primary_source` are asserted by the ingestion caller or upstream parser. The filter enforces structural consistency at the seam, not omniscient fact-checking.
4. **Complementary to Human-in-the-Loop (HITL) Discernment:**  
   Automated pre-crate gates exist to conserve human attention by catching obvious provenance, ecological, and attribution omissions early. Final editorial judgment always remains with sovereign human maintainers.
5. **Non-Activation Clause:**  
   This module and README are descriptive and diagnostic-only. They do not activate NDH geometry, holonomy engines, or autonomous governance authority.
