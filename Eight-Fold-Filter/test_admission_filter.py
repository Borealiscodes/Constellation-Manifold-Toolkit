#!/usr/bin/env python3
"""
Eight-Fold-Filter/test_admission_filter.py
Standard-library unit test suite for `sovereign_admission_filter.py` (zero dependencies).
Run with: python3 -m unittest discover -s Eight-Fold-Filter -v
"""

from __future__ import annotations

import hashlib
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from sovereign_admission_filter import evaluate_pre_crate_admission  # noqa: E402


class TestSovereignAdmissionFilter(unittest.TestCase):
    def test_clean_grounded_artifact_achieves_six_of_six_admit(self) -> None:
        content = (
            "This study analyzes GPU cluster energy usage alongside rare-earth mining "
            "and e-waste lifecycle externalities (DOI: 10.5281/zenodo.23246401). "
            "Non-Activation Clause: Descriptive and forensic-only."
        )
        expected_sha256 = hashlib.sha256(content.encode("utf-8")).hexdigest()

        receipt = evaluate_pre_crate_admission(
            artifact_id="grounded-hardware-lifecycle-audit",
            content=content,
            has_upstream_attribution=True,
            has_non_activation_clause=True,
            is_synthetic_digest=False,
            cites_primary_source=True,
        )

        self.assertEqual(receipt.verdict, "ADMIT")
        self.assertEqual(receipt.compliance_score_pq, "6/6")
        self.assertEqual(receipt.raw_sha256, expected_sha256)
        self.assertTrue(all(f.passed for f in receipt.findings))

    def test_empty_payload_quarantined_by_right_mindfulness(self) -> None:
        receipt = evaluate_pre_crate_admission(
            artifact_id="empty-stub",
            content="   \n\t  ",
            has_upstream_attribution=True,
            has_non_activation_clause=True,
            is_synthetic_digest=False,
            cites_primary_source=True,
        )
        failed = {f.gate_id for f in receipt.findings if not f.passed}
        self.assertEqual(receipt.verdict, "QUARANTINE_FOR_REVIEW")
        self.assertEqual(receipt.compliance_score_pq, "5/6")
        self.assertEqual(failed, {"G1_RIGHT_MINDFULNESS"})

    def test_missing_attribution_quarantined_by_karma_yoga(self) -> None:
        receipt = evaluate_pre_crate_admission(
            artifact_id="unattributed-ingestion",
            content="Descriptive technical specification with no external hardware claims.",
            has_upstream_attribution=False,
            has_non_activation_clause=True,
            is_synthetic_digest=False,
            cites_primary_source=True,
        )
        failed = {f.gate_id for f in receipt.findings if not f.passed}
        self.assertEqual(receipt.verdict, "QUARANTINE_FOR_REVIEW")
        self.assertEqual(receipt.compliance_score_pq, "5/6")
        self.assertEqual(failed, {"G2_KARMA_YOGA_ATTRIBUTION"})

    def test_clinical_terms_quarantined_for_human_review(self) -> None:
        receipt = evaluate_pre_crate_admission(
            artifact_id="relational-model-with-diagnostic-labels",
            content="Relational boundary drift modeled via BPD and bipolar diagnostic categories.",
            has_upstream_attribution=True,
            has_non_activation_clause=True,
            is_synthetic_digest=False,
            cites_primary_source=True,
        )
        failed = {f.gate_id for f in receipt.findings if not f.passed}
        self.assertEqual(receipt.verdict, "QUARANTINE_FOR_REVIEW")
        self.assertEqual(receipt.compliance_score_pq, "5/6")
        self.assertEqual(failed, {"G3_RIGHT_INTENTION_NON_HARM"})

    def test_ungrounded_synthetic_digest_quarantined_by_right_speech(self) -> None:
        receipt = evaluate_pre_crate_admission(
            artifact_id="orphan-audio-overview",
            content="Audio overview summarizing abstract system dynamics.",
            has_upstream_attribution=True,
            has_non_activation_clause=True,
            is_synthetic_digest=True,
            cites_primary_source=False,
        )
        failed = {f.gate_id for f in receipt.findings if not f.passed}
        self.assertEqual(receipt.verdict, "QUARANTINE_FOR_REVIEW")
        self.assertEqual(receipt.compliance_score_pq, "5/6")
        self.assertEqual(failed, {"G4_RIGHT_SPEECH_PROVENANCE"})

    def test_chip_moat_without_lifecycle_quarantined_by_right_view(self) -> None:
        receipt = evaluate_pre_crate_admission(
            artifact_id="disembodied-gpu-moat-digest",
            content="Software toolchains and GPU accelerators form the ultimate AI moat.",
            has_upstream_attribution=True,
            has_non_activation_clause=True,
            is_synthetic_digest=False,
            cites_primary_source=True,
        )
        failed = {f.gate_id for f in receipt.findings if not f.passed}
        self.assertEqual(receipt.verdict, "QUARANTINE_FOR_REVIEW")
        self.assertEqual(receipt.compliance_score_pq, "5/6")
        self.assertEqual(failed, {"G5_RIGHT_VIEW_ECOLOGY"})

    def test_missing_non_activation_clause_quarantined_by_right_concentration(self) -> None:
        receipt = evaluate_pre_crate_admission(
            artifact_id="unbounded-spec",
            content="Pure mathematical analysis of discrete transition graphs.",
            has_upstream_attribution=True,
            has_non_activation_clause=False,
            is_synthetic_digest=False,
            cites_primary_source=True,
        )
        failed = {f.gate_id for f in receipt.findings if not f.passed}
        self.assertEqual(receipt.verdict, "QUARANTINE_FOR_REVIEW")
        self.assertEqual(receipt.compliance_score_pq, "5/6")
        self.assertEqual(failed, {"G6_RIGHT_CONCENTRATION_ALTITUDE"})


if __name__ == "__main__":
    unittest.main()
