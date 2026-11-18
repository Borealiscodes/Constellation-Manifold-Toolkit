#!/usr/bin/env python3
"""
test_contract_bridge.py — Deterministic Unit Tests for Python-to-Coq Contract Bridge
Verifies fail-closed seam enforcement, exact rational (Q) equivalence,
and cryptographic .v / .vo Coq witness fingerprinting.

Posture: Zero-Dependency • Deterministic • Fail-Closed
Signed-off-by: Borealis S. Hedling <chguard-sentinel@vaked.dev>
"""

import json
import tempfile
import unittest
from pathlib import Path

from contract_bridge import (
    COQ_CONTRACT_SPECS,
    REQUIRED_GATES,
    validate_receipt_contract,
)


def _make_receipt(
    passed_gates: list[str] | None = None,
    verdict: str | None = None,
    score_override: str | None = None,
    raw_sha256: str = "b" * 64,
) -> dict:
    """Helper to construct valid or intentionally malformed AdmissionReceipts."""
    if passed_gates is None:
        passed_gates = list(REQUIRED_GATES)

    gate_results = {g: (g in passed_gates) for g in REQUIRED_GATES}
    passed_count = sum(1 for v in gate_results.values() if v)

    if verdict is None:
        verdict = "ADMIT" if passed_count == len(REQUIRED_GATES) else "QUARANTINE_FOR_REVIEW"

    score_str = score_override if score_override is not None else f"{passed_count}/{len(REQUIRED_GATES)}"

    return {
        "raw_sha256": raw_sha256,
        "verdict": verdict,
        "compliance_score_pq": score_str,
        "gate_results": gate_results,
    }


class TestContractBridge(unittest.TestCase):
    def test_valid_admit_6_of_6_receipt(self) -> None:
        """A clean 6/6 receipt with ADMIT satisfies all contract invariants."""
        receipt = _make_receipt(passed_gates=list(REQUIRED_GATES), verdict="ADMIT")
        envelope = validate_receipt_contract(receipt)

        self.assertTrue(envelope.contract_holds)
        self.assertEqual(envelope.violations, [])
        self.assertEqual(envelope.compliance_score_pq, "6/6")
        self.assertEqual(envelope.verdict, "ADMIT")

    def test_valid_quarantine_receipt_ecological_and_clinical_omission(self) -> None:
        """Simulates quarantining a flattened essay failing G3 (stigma) and G5 (ecology)."""
        passed = [
            "G1_RIGHT_MINDFULNESS",
            "G2_KARMA_YOGA_ATTRIBUTION",
            "G4_RIGHT_SPEECH_PROVENANCE",
            "G6_RIGHT_CONCENTRATION_ALTITUDE",
        ]
        receipt = _make_receipt(passed_gates=passed, verdict="QUARANTINE_FOR_REVIEW")
        envelope = validate_receipt_contract(receipt)

        self.assertTrue(envelope.contract_holds)
        self.assertEqual(envelope.compliance_score_pq, "4/6")
        self.assertEqual(envelope.verdict, "QUARANTINE_FOR_REVIEW")

    def test_fail_closed_breach_admit_below_6_of_6(self) -> None:
        """Rejects any receipt claiming ADMIT when fewer than 6/6 gates passed."""
        passed = [g for g in REQUIRED_GATES if g != "G5_RIGHT_VIEW_ECOLOGY"]
        receipt = _make_receipt(passed_gates=passed, verdict="ADMIT")
        envelope = validate_receipt_contract(receipt)

        self.assertFalse(envelope.contract_holds)
        self.assertTrue(any("FAIL_CLOSED_BREACH" in v for v in envelope.violations))

    def test_inconsistent_quarantine_on_6_of_6(self) -> None:
        """Flags inconsistent verdict if all 6 gates passed but verdict is QUARANTINE."""
        receipt = _make_receipt(
            passed_gates=list(REQUIRED_GATES),
            verdict="QUARANTINE_FOR_REVIEW",
        )
        envelope = validate_receipt_contract(receipt)

        self.assertFalse(envelope.contract_holds)
        self.assertTrue(any("INCONSISTENT_QUARANTINE" in v for v in envelope.violations))

    def test_rational_score_mismatch_detected(self) -> None:
        """Detects tampering or arithmetic mismatch in compliance_score_pq."""
        passed = ["G1_RIGHT_MINDFULNESS", "G2_KARMA_YOGA_ATTRIBUTION"]
        receipt = _make_receipt(
            passed_gates=passed,
            verdict="QUARANTINE_FOR_REVIEW",
            score_override="5/6",
        )
        envelope = validate_receipt_contract(receipt)

        self.assertFalse(envelope.contract_holds)
        self.assertTrue(any("RATIONAL_SCORE_MISMATCH" in v for v in envelope.violations))

    def test_invalid_preimage_hash_rejected(self) -> None:
        """G1 byte-integrity requires a valid 64-character SHA-256 hex string."""
        receipt = _make_receipt(raw_sha256="short_hash")
        envelope = validate_receipt_contract(receipt)

        self.assertFalse(envelope.contract_holds)
        self.assertTrue(any("INVALID_PREIMAGE_HASH" in v for v in envelope.violations))

    def test_missing_or_non_boolean_gate_rejected(self) -> None:
        """Orthogonal predicates must be strictly boolean without missing keys."""
        receipt = _make_receipt()
        receipt["gate_results"]["G3_RIGHT_INTENTION_NON_HARM"] = "true"  # type: ignore[assignment]
        envelope = validate_receipt_contract(receipt)

        self.assertFalse(envelope.contract_holds)
        self.assertTrue(
            any("MISSING_OR_NON_BOOLEAN_GATE: G3_RIGHT_INTENTION_NON_HARM" in v for v in envelope.violations)
        )

    def test_coq_witness_binding_with_compiled_vo_objects(self) -> None:
        """Verifies .v and compiled .vo SHA-256 witness hashing in a mock COQwrapped dir."""
        receipt = _make_receipt()

        with tempfile.TemporaryDirectory() as tmpdir:
            coq_path = Path(tmpdir)

            # Populate all .v source files, but omit .vo objects first
            for meta in COQ_CONTRACT_SPECS.values():
                (coq_path / meta["source_file"]).write_text("(* coq spec *)\n", encoding="utf-8")

            # Strict mode (require_compiled_vo=True) should fail when .vo is missing
            strict_env = validate_receipt_contract(
                receipt,
                coq_dir=coq_path,
                require_compiled_vo=True,
            )
            self.assertFalse(strict_env.contract_holds)
            self.assertEqual(len(strict_env.coq_witnesses), len(COQ_CONTRACT_SPECS))
            self.assertTrue(all(not w.coqc_verified for w in strict_env.coq_witnesses))

            # Now simulate coqc compilation by writing .vo artifacts
            for meta in COQ_CONTRACT_SPECS.values():
                (coq_path / meta["object_file"]).write_bytes(b"\x00COQ_VO_MOCK")

            verified_env = validate_receipt_contract(
                receipt,
                coq_dir=coq_path,
                require_compiled_vo=True,
            )
            self.assertTrue(verified_env.contract_holds)
            self.assertEqual(verified_env.violations, [])
            self.assertTrue(all(w.coqc_verified for w in verified_env.coq_witnesses))
            self.assertTrue(all(w.compiled_vo_sha256 is not None for w in verified_env.coq_witnesses))

            # Ensure output envelope serializes cleanly to JSON
            parsed = json.loads(verified_env.to_json())
            self.assertTrue(parsed["contract_holds"])
            self.assertIn("DESCRIPTIVE_AND_DIAGNOSTIC_ONLY", parsed["non_activation_clause"])


if __name__ == "__main__":
    unittest.main(verbosity=2)
