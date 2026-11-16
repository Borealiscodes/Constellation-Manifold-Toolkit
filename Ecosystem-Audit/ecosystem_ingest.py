#!/usr/bin/env python3
"""
Ecosystem-Audit/ecosystem_ingest.py
Stage 1 & 2: Zero-dependency GitHub REST API metadata snapshotter, SHA-256
provenance anchor, and protocol-incidence matrix builder for `8b-is`.
"""

from __future__ import annotations

import hashlib
import json
import urllib.request
from fractions import Fraction
from pathlib import Path
from typing import Any

ORG_NAME = "8b-is"
CANONICAL_TOKENS = (
    "0x62",
    "bfp-64",
    "blake3",
    "q16.16",
    "spqt",
    "mem8",
    "mem|8",
    "sigmadelta",
    "b1.58",
    "wupz",
    "zeno",
    "aurora",
    "spherepop",
    "ultra-graph",
)


def sha256_bytes(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def fetch_org_repos(org: str = ORG_NAME) -> list[dict[str, Any]]:
    """Fetch all public repositories for `org` via GitHub REST API (zero dependencies)."""
    repos: list[dict[str, Any]] = []
    page = 1
    while True:
        url = f"https://github.com{org}/repos?per_page=100&page={page}&type=public"
        req = urllib.request.Request(
            url,
            headers={
                "Accept": "application/vnd.github+json",
                "User-Agent": "NDH-Ecosystem-Audit/1.0",
            },
        )
        with urllib.request.urlopen(req, timeout=20) as resp:
            batch = json.loads(resp.read().decode("utf-8"))
        if not batch:
            break
        repos.extend(batch)
        if len(batch) < 100:
            break
        page += 1
    repos.sort(key=lambda r: r["name"].lower())
    return repos


def build_snapshot_manifest(repos: list[dict[str, Any]]) -> dict[str, Any]:
    """Construct SHA-256 anchored manifest and binary protocol-incidence matrix B."""
    records: list[dict[str, Any]] = []
    incidence_matrix: list[list[int]] = []
    fork_count = 0

    for r in repos:
        is_fork = bool(r.get("fork", False))
        if is_fork:
            fork_count += 1

        searchable = " ".join([
            str(r.get("name", "")),
            str(r.get("description") or ""),
            " ".join(r.get("topics") or []),
        ]).lower()

        row = [1 if tok in searchable else 0 for tok in CANONICAL_TOKENS]
        incidence_matrix.append(row)

        records.append({
            "name": r["name"],
            "fork": is_fork,
            "language": r.get("language"),
            "stargazers_count": r.get("stargazers_count", 0),
            "pushed_at": r.get("pushed_at"),
            "matched_tokens": [tok for tok, bit in zip(CANONICAL_TOKENS, row) if bit == 1],
        })

    total = len(records)
    original_count = total - fork_count
    fork_ratio = Fraction(fork_count, max(total, 1))

    payload_without_hash = {
        "schema_version": "1.0.0",
        "target_org": ORG_NAME,
        "total_repos": total,
        "original_repos": original_count,
        "fork_repos": fork_count,
        "fork_ratio_pq": f"{fork_ratio.numerator}/{fork_ratio.denominator}",
        "tokens": list(CANONICAL_TOKENS),
        "incidence_matrix": incidence_matrix,
        "repositories": records,
    }
    canonical_bytes = json.dumps(payload_without_hash, sort_keys=True, separators=(",", ":")).encode("utf-8")
    payload_without_hash["manifest_sha256"] = sha256_bytes(canonical_bytes)
    return payload_without_hash


def main() -> None:
    out_path = Path(__file__).resolve().parent / "8b_snapshot_manifest.json"
    repos = fetch_org_repos(ORG_NAME)
    manifest = build_snapshot_manifest(repos)
    out_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(f"Anchored {manifest['total_repos']} repos ({manifest['original_repos']} original, {manifest['fork_repos']} forks)")
    print(f"Manifest SHA-256: {manifest['manifest_sha256']}")
    print(f"Saved to: {out_path}")


if __name__ == "__main__":
    main()
