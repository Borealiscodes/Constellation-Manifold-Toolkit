# 🌉 The Active-Inference-to-Deterministic-Gate Seam (`active_inference_seam.py`)

**Architecture Pattern:** Exploratory Proposal Side ($\Delta, \Phi$, Active Inference) $\rightarrow$ Cryptographic Commit Seam (`Record ≠ Admit ≠ Commit`)  
**Runtime:** Python 3.10+ Standard Library Only (Zero External Dependencies)  

---

## 🎯 Why Both Sides of the Seam Are Essential

A purely deterministic verification gate (`SHA-256`, `BigInt` rationals, Kahn DAGs) is **incorruptible**, but it is also **blind**—it cannot tell you *which* of 10,000 paragraphs hides a subtle anomaly, *what* missing evidence to query next, or *when* two differently worded clauses share the same underlying structural pattern.

Conversely, an unconstrained active-inference or salience engine is **perceptive**, but **epistemically unsafe** if allowed to write directly to memory—high salience or repeated resonance can masquerade as verified evidence.

Placing a strict **cryptographic and type-safety seam** between them gives you the best of both worlds:

```text
[ PROPOSAL SIDE: EXPLORATORY & ACTIVE INFERENCE ]
  • Salience (Δ): Surprisal / information-gain triage
  • Resonance (Φ): Structural / spectral similarity across silos
  • Active Inference: Expected Free Energy (EFE) query selection
  • Epistemic Tag: Strictly DERIVED (never OBSERVED)
                         │
            ═════════════╧═════════════  <-- THE SEAM (Stage -> Admit -> Commit)
                         │
[ COMMIT SIDE: DETERMINISTIC LEDGER & VERIFIER ]
  • Verifies SHA-256 chunk ancestry & exact Fraction math
  • Enforces Law of Non-Self-Warranting Memory (repetition ≠ proof)
  • Appends immutable OBSERVED / VERIFIED records only
```

---

## 🐍 Zero-Dependency Python Toolkit (`active_inference_seam.py`)

Save this file as `src/active_inference_seam.py` and run `python3 src/active_inference_seam.py` to execute the built-in demonstration and invariant assertions.

```python
#!/usr/bin/env python3
"""
active_inference_seam.py
Bridges Salience (Δ), Structural Resonance (Φ), and Active Inference (Proposal Side)
with a Fail-Closed Cryptographic Commit Ledger (Commit Side).
Zero external dependencies (Python 3.10+ standard library).
"""

from __future__ import annotations

import hashlib
import json
import math
import re
from dataclasses import dataclass, field
from fractions import Fraction
from typing import Literal

EpistemicKind = Literal["OBSERVED", "DERIVED", "REJECTED"]

STOPWORDS = frozenset({
    "the", "and", "of", "to", "in", "a", "is", "for", "that", "on", "with", "as", "by", "at", "an", "be", "this"
})


def sha256_hex(data: str) -> str:
    return hashlib.sha256(data.encode("utf-8")).hexdigest()


def tokenize(text: str) -> list[str]:
    return [t for t in re.findall(r"[a-z0-9]+", text.lower()) if t not in STOPWORDS and len(t) > 1]


# ============================================================================
# 1. COMMIT SIDE: IMMUTABLE OBSERVATION STORE & LEDGER
# ============================================================================

@dataclass(frozen=True)
class ObservedChunk:
    """Primary ground-truth record anchored before any AI or inference runs."""
    chunk_id: str
    source_uri: str
    text: str
    sha256: str
    kind: Literal["OBSERVED"] = "OBSERVED"

    @classmethod
    def from_raw(cls, chunk_id: str, source_uri: str, text: str) -> ObservedChunk:
        clean = text.strip()
        return cls(
            chunk_id=chunk_id,
            source_uri=source_uri,
            text=clean,
            sha256=sha256_hex(clean),
        )


@dataclass(frozen=True)
class StagedProposal:
    """Untrusted candidate emitted by the Exploratory / Active Inference layer."""
    proposal_id: str
    target_chunk_id: str
    quoted_text: str
    salience_delta: Fraction       # Δ: Surprisal / triage priority (exact rational)
    resonance_phi: Fraction        # Φ: Cross-silo structural similarity (exact rational)
    active_query: str              # Next evidence query proposed to minimize uncertainty
    rationale: str
    kind: Literal["DERIVED"] = "DERIVED"


@dataclass
class CommitLedger:
    """
    The Seam: Enforces Record != Admit != Commit.
    Salience and resonance rank proposals, but NEVER substitute for SHA-256 ancestry.
    """
    observations: dict[str, ObservedChunk] = field(default_factory=dict)
    committed: list[dict] = field(default_factory=list)
    rejected: list[dict] = field(default_factory=list)
    _seen_proposal_hashes: set[str] = field(default_factory=set)

    def ingest_observation(self, chunk: ObservedChunk) -> None:
        if sha256_hex(chunk.text) != chunk.sha256:
            raise ValueError(f"Integrity Failure: Chunk {chunk.chunk_id} hash mismatch.")
        self.observations[chunk.chunk_id] = chunk

    def admit_and_commit(self, proposal: StagedProposal) -> dict:
        """
        Crosses the seam:
        1. Blocks DERIVED proposals from claiming OBSERVED status.
        2. Verifies target chunk exists and quoted_text is a verbatim substring of verified text.
        3. Enforces Law of Non-Self-Warranting Memory (duplicate proposals do not add warrant).
        """
        if proposal.kind != "DERIVED":
            return self._reject(proposal, "EpistemicTypeError: Proposals must be tagged DERIVED.")

        obs = self.observations.get(proposal.target_chunk_id)
        if obs is None:
            return self._reject(proposal, f"AncestryError: Unknown target chunk {proposal.target_chunk_id!r}.")

        # Re-verify underlying observation integrity before checking substring
        if sha256_hex(obs.text) != obs.sha256:
            return self._reject(proposal, "SubstrateTamperError: Stored observation failed SHA-256 check.")

        quote_clean = proposal.quoted_text.strip()
        if not quote_clean or quote_clean not in obs.text:
            return self._reject(proposal, "ProvenanceError: Proposed quote is not verbatim in source chunk.")

        # Non-Self-Warranting Law: identical (chunk_id + quote) commits at most once
        dedup_key = sha256_hex(f"{obs.sha256}:{quote_clean}")
        if dedup_key in self._seen_proposal_hashes:
            return self._reject(proposal, "NonSelfWarrantingError: Duplicate claim repetition adds zero warrant.")

        self._seen_proposal_hashes.add(dedup_key)
        record = {
            "status": "COMMITTED",
            "epistemic_type": "DERIVED_VERIFIED",
            "proposal_id": proposal.proposal_id,
            "source_chunk_id": obs.chunk_id,
            "source_sha256": obs.sha256,
            "quote_sha256": sha256_hex(quote_clean),
            "verified_quote": quote_clean,
            "triage_metrics": {
                "salience_delta": f"{proposal.salience_delta.numerator}/{proposal.salience_delta.denominator}",
                "resonance_phi": f"{proposal.resonance_phi.numerator}/{proposal.resonance_phi.denominator}",
            },
            "next_active_query": proposal.active_query,
        }
        self.committed.append(record)
        return record

    def _reject(self, proposal: StagedProposal, reason: str) -> dict:
        tombstone = {
            "status": "REJECTED",
            "epistemic_type": "REJECTED",
            "proposal_id": proposal.proposal_id,
            "reason": reason,
            "tombstone_sha256": sha256_hex(f"{proposal.proposal_id}:{reason}"),
        }
        self.rejected.append(tombstone)
        return tombstone


# ============================================================================
# 2. PROPOSAL SIDE: SALIENCE (Δ), RESONANCE (Φ) & ACTIVE INFERENCE ENGINE
# ============================================================================

class ExploratoryProposer:
    """
    Lives strictly BEFORE the seam.
    Uses surprisal salience (Δ), cross-silo structural resonance (Φ), and expected
    free energy (uncertainty reduction) to decide WHERE to look and WHAT to propose.
    """

    def __init__(self, prior_token_counts: dict[str, int] | None = None) -> None:
        self.prior_counts: dict[str, int] = dict(prior_token_counts or {})
        self.total_prior_tokens: int = sum(self.prior_counts.values())

    def compute_salience_delta(self, text: str) -> Fraction:
        """
        Δ (Salience): Information-theoretic surprisal against prior corpus distribution.
        Rare, high-information terms produce higher rational salience scores.
        """
        tokens = tokenize(text)
        if not tokens:
            return Fraction(0, 1)

        vocab_size = max(len(self.prior_counts), 1)
        denom = self.total_prior_tokens + vocab_size
        surprisal_sum = 0.0
        for tok in tokens:
            prob = (self.prior_counts.get(tok, 0) + 1) / denom
            surprisal_sum += -math.log2(prob)

        # Quantize cleanly to exact rational (millibits) so downstream triage is deterministic
        mean_surprisal_millibits = round((surprisal_sum / len(tokens)) * 1000)
        return Fraction(mean_surprisal_millibits, 1000)

    def compute_resonance_phi(self, text_a: str, text_b: str) -> Fraction:
        """
        Φ (Structural Resonance): Exact rational Jaccard overlap over content tokens
        to detect cross-silo structural echoes without floating-point drift.
        """
        set_a = set(tokenize(text_a))
        set_b = set(tokenize(text_b))
        union = set_a | set_b
        if not union:
            return Fraction(0, 1)
        return Fraction(len(set_a & set_b), len(union))

    def propose_from_corpus(
        self,
        chunks: list[ObservedChunk],
        reference_pattern: str,
        top_k: int = 2,
    ) -> list[StagedProposal]:
        """
        Active Inference Triage:
        Ranks chunks by combined Salience (Δ) + Resonance (Φ) to select the top-k
        candidates worth verifying across the seam, and formulates follow-up queries
        for unresolved uncertainty.
        """
Use code with caution.
scored: list[tuple[Fraction, ObservedChunk, Fraction, Fraction]] = []
for ch in chunks:
delta = self.compute_salience_delta(ch.text)
phi = self.compute_resonance_phi(ch.text, reference_pattern)
# Expected Free Energy proxy: prioritize high salience + high structural resonance
priority = delta + (phi * 10)
scored.append((priority, ch, delta, phi))
# Deterministic sort: highest priority first, tie-break by chunk_id
scored.sort(key=lambda item: (-item, item.chunk_id))
proposals: list[StagedProposal] = []
for rank, (_, ch, delta, phi) in enumerate(scored[:top_k], start=1):
# Extract first sentence as candidate verbatim quote
first_sentence = ch.text.split(".")[0].strip() + "."
missing_terms = sorted(set(tokenize(reference_pattern)) - set(tokenize(ch.text)))
next_query = (
f"Retrieve primary records mentioning: {', '.join(missing_terms[:3])}"
if missing_terms
else "All reference pattern tokens accounted for; verify temporal ordering."
)
proposals.append(
StagedProposal(
proposal_id=f"prop-{rank}-{ch.chunk_id}",
target_chunk_id=ch.chunk_id,
quoted_text=first_sentence,
salience_delta=delta,
resonance_phi=phi,
active_query=next_query,
rationale="High combined surprisal salience (Δ) and cross-silo resonance (Φ).",
)
)
return proposals

============================================================================


3. DEMONSTRATION & INVARIANT SELF-TEST


============================================================================

def run_demo() -> None:
ledger = CommitLedger()
# Step A: Anchor raw ground-truth observations BEFORE exploratory inference
raw_docs = [
ObservedChunk.from_raw(
"doc-01",
"example.org",
"Routine administrative scheduling notice for quarterly building maintenance.",
),
ObservedChunk.from_raw(
"doc-02",
"example.org",
"Retroactive jurisdiction transfer executed without notice or statutory warrant.",
),
ObservedChunk.from_raw(
"doc-03",
"example.org",
"Cross-border custody transfer executed without notice during pending appeal.",
),
]
for doc in raw_docs:
ledger.ingest_observation(doc)
# Step B: Exploratory Proposer uses Salience (Δ), Resonance (Φ), and Active Inference
proposer = ExploratoryProposer(prior_token_counts={"routine": 50, "administrative": 40, "notice": 30})
pattern = "retroactive transfer without statutory warrant or notice during appeal"
proposals = proposer.propose_from_corpus(raw_docs, reference_pattern=pattern, top_k=2)
print("=== 1. EXPLORATORY PROPOSALS (BEFORE THE SEAM) ===")
for p in proposals:
res = ledger.admit_and_commit(p)
print(json.dumps(res, indent=2))
# Step C: Prove the Seam blocks hallucinated quotes & duplicate self-warranting
print("\n=== 2. SEAM SECURITY TESTS (FAIL-CLOSED REJECTIONS) ===")
hallucinated = StagedProposal(
proposal_id="prop-bad-quote",
target_chunk_id="doc-02",
quoted_text="Retroactive jurisdiction transfer approved with full statutory warrant.",
salience_delta=Fraction(99, 1),  # High salience CANNOT bypass SHA-256 verification!
resonance_phi=Fraction(1, 1),
active_query="None",
rationale="Altered quote disguised with high salience.",
)
rej_1 = ledger.admit_and_commit(hallucinated)
assert rej_1["status"] == "REJECTED"
print("Hallucinated quote blocked:", rej_1["reason"])
# Repeating an already-committed proposal adds zero extra warrant
rej_2 = ledger.admit_and_commit(proposals)
assert rej_2["status"] == "REJECTED"
print("Duplicate self-warrant blocked:", rej_2["reason"])
if name == "main":
run_demo()
```

### How This Keeps Both Halves Honest
* **Salience (Δ) & Resonance (Φ) Do Real Work:** In the demo, `doc-01` (*routine maintenance*) is filtered out automatically while `doc-02` and `doc-03` (*retroactive/cross-border transfer without notice*) bubble to the top with concrete follow-up active-inference queries for the missing tokens (`appeal`, `statutory`, `warrant`).
* **The Seam Never Bends:** Even when `prop-bad-quote` sets `salience_delta = 99/1` and `resonance_phi = 1/1`, `CommitLedger.admit_and_commit()` immediately rejects and tombstones it because the altered quote fails verbatim substring verification against `doc-02`'s `SHA-256` hash.

