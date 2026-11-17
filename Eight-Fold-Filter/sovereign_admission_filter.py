#!/usr/bin/env python3
"""
sovereign_admission_filter.py
Non-occult, deterministic pre-crate admission filter grounded in Eightfold Path
and Karma Yoga engineering invariants (NDH-Meta-Systems / AHSO Series Foundation).
Zero external dependencies (Python 3.10+ standard library).
"""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import dataclass
from fractions import Fraction
from typing import Literal

Verdict = Literal["ADMIT", "QUARANTINE_FOR_REVIEW"]


@dataclass(frozen=True)
class GateFinding:
    gate_id: str
    guidepost: str
    passed: bool
    reason: str


@dataclass(frozen=True)
class AdmissionReceipt:
    artifact_id: str
    raw_sha256: str
    verdict: Verdict
    compliance_score_pq: str
    findings: list[GateFinding]


CLINICAL_STIGMA_PATTERNS = re.compile(
    r"\b(bpd|borderline personality|bipolar|narcissis\w*|psychopath\w*|diagnos\w+)\b",
    re.IGNORECASE,
)
HARDWARE_CLAIM_PATTERNS = re.compile(
    r"\b(gpu|gpus|chip|chips|accelerator|silicon|moat|simd|fpga|tpu)\b",
    re.IGNORECASE,
)
LIFECYCLE_CONTEXT_PATTERNS = re.compile(
    r"\b(lifecycle|mining|water|emission\w*|e-waste|externalit\w*|ecological|energy|zenodo\.23246401)\b",
    re.IGNORECASE,
)


def evaluate_pre_crate_admission(
    artifact_id: str,
    content: str,
    *,
    has_upstream_attribution: bool,
    has_non_activation_clause: bool,
    is_synthetic_digest: bool,
    cites_primary_source: bool,
) -> AdmissionReceipt:
    """
    Evaluate an artifact across 6 non-occult Eightfold Path & Karma Yoga gates
    prior to crate ingestion (Record != Admit != Commit).
    """
    raw_hash = hashlib.sha256(content.encode("utf-8")).hexdigest()
    findings: list[GateFinding] = []

    # 1. Right Mindfulness (Pre-ingestion cryptographic anchor)
    findings.append(
        GateFinding(
            gate_id="G1_RIGHT_MINDFULNESS",
            guidepost="Eightfold Path: Right Mindfulness (Pre-Ingestion Anchor)",
            passed=len(content.strip()) > 0,
            reason="Non-empty payload anchored with SHA-256 prior to crate admission."
            if content.strip()
            else "Empty payload rejected.",
        )
    )

    # 2. Karma Yoga: Non-Absorptive Attribution (Niskama Provenance)
    findings.append(
        GateFinding(
            gate_id="G2_KARMA_YOGA_ATTRIBUTION",
            guidepost="Karma Yoga: Detached Non-Absorptive Lineage",
            passed=has_upstream_attribution,
            reason="Upstream lineage and author sovereignty explicitly preserved."
            if has_upstream_attribution
            else "FLAG: Missing upstream attribution or network dependency disclosure.",
        )
    )

    # 3. Right Intention: Non-Harm & Anti-Stigma Boundary
    has_clinical_terms = bool(CLINICAL_STIGMA_PATTERNS.search(content))
    findings.append(
        GateFinding(
            gate_id="G3_RIGHT_INTENTION_NON_HARM",
            guidepost="Eightfold Path: Right Intention (Anti-Stigma & Non-Coercion)",
            passed=not has_clinical_terms,
            reason="No armchair clinical/psychiatric labeling detected."
            if not has_clinical_terms
            else "FLAG: Clinical/psychiatric terms detected; requires trauma-informed altitude separation.",
        )
    )

    # 4. Right Speech: Anti-Conflation for Synthetic / Audio Digests
    digest_ok = (not is_synthetic_digest) or cites_primary_source
    findings.append(
        GateFinding(
            gate_id="G4_RIGHT_SPEECH_PROVENANCE",
            guidepost="Eightfold Path: Right Speech (Derived Digest Traceability)",
            passed=digest_ok,
            reason="Primary source or anchored digest citations verified."
            if digest_ok
            else "FLAG: Synthetic/audio digest lacks explicit link to primary source text.",
        )
    )

    # 5. Right View & Right Livelihood: Hardware & Ecological Grounding
    mentions_chips = bool(HARDWARE_CLAIM_PATTERNS.search(content))
    mentions_lifecycle = bool(LIFECYCLE_CONTEXT_PATTERNS.search(content))
    ecology_ok = (not mentions_chips) or mentions_lifecycle
    findings.append(
        GateFinding(
            gate_id="G5_RIGHT_VIEW_ECOLOGY",
            guidepost="Eightfold Path: Right View & Livelihood (Substrate & Ecological Cost)",
            passed=ecology_ok,
            reason="Hardware/chip references include lifecycle or ecological boundary context."
            if ecology_ok
            else "FLAG: Hardware/chip moat claims omit manufacturing, mining, or ecological externalities.",
        )
    )

    # 6. Right Concentration: Altitude Separation (Non-Activation Clause)
    findings.append(
        GateFinding(
            gate_id="G6_RIGHT_CONCENTRATION_ALTITUDE",
            guidepost="Eightfold Path: Right Concentration (Altitude & Non-Activation Boundary)",
            passed=has_non_activation_clause,
            reason="Explicit Non-Activation & Altitude boundary present."
            if has_non_activation_clause
            else "FLAG: Missing explicit Non-Activation / altitude-separation clause.",
        )
    )

    passed_count = sum(1 for f in findings if f.passed)
    score = Fraction(passed_count, len(findings))
    verdict: Verdict = "ADMIT" if passed_count == len(findings) else "QUARANTINE_FOR_REVIEW"

    return AdmissionReceipt(
        artifact_id=artifact_id,
        raw_sha256=raw_hash,
        verdict=verdict,
        compliance_score_pq=f"{score.numerator}/{score.denominator}",
        findings=findings,
    )


if __name__ == "__main__":
    # Example audit of an ungrounded audio digest vs. a grounded artifact
    sample_receipt = evaluate_pre_crate_admission(
        artifact_id="software-toolchains-audio-digest",
        content="Software toolchains and GPU clusters are the real AI moat.",
        has_upstream_attribution=False,
        has_non_activation_clause=False,
        is_synthetic_digest=True,
        cites_primary_source=False,
    )
    print(json.dumps({
        "artifact_id": sample_receipt.artifact_id,
        "verdict": sample_receipt.verdict,
        "compliance_score_pq": sample_receipt.compliance_score_pq,
        "flagged_gates": [f.gate_id for f in sample_receipt.findings if not f.passed],
    }, indent=2))
