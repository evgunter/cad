# K report (DRAFT) — the ambiguity constant K = 10 under M2's telemetry

**Status: DRAFT** (M2 PR 7 deliverable; the recommendation below is the
implementer's draft — the orchestrator finalizes it and ratifies the
outcome into DESIGN.md's Q1 residue.)

## Background

Q1/D4 classify every topology-determining margin `m` against a band
`(ε, K·ε)`: `|m| ≤ ε` is coincident, `|m| ≥ K·ε` is definite, and the
open interval between is the **ambiguity band** — a semantic sliver
zone, indeterminate even under exact arithmetic (`geom-core`'s
predicate module docs). `K = 10` was picked at M0 as an honest guess,
explicitly pending multi-ε telemetry. This report is that telemetry,
gathered across M2's full pipeline.

## Methodology

- Recorder: the unified `geom_core::k_stats` funnel (this PR): every
  shipped decision funnels through `k_stats::decide`, which names the
  predicate; runs at the `Probe` scalar record every classification
  (predicate, margin, band, outcome) with decisions bit-identical to
  f64.
- Harness: `crates/sweep/tests/k_report.rs` (`dump_k_samples`,
  `#[ignore]`d) builds ten M2 acceptance shapes end-to-end at `Probe`
  — profile validation → extrude/revolve → tier 1–3 validation
  (incl. the new +V invariant) → exact mass properties — and dumps
  every `MarginSample` as CSV. One process per ε
  (`Tolerance` is a OnceLock):

  ```sh
  CAD_TOLERANCE_EPS=1e-9 CAD_K_REPORT_OUT=docs/k-report-data/eps-1e-9.csv \
    cargo test -p sweep --test k_report -- --ignored --nocapture
  ```

- Rows: ε ∈ {1e-6, 1e-9, 1e-12}. Raw CSVs: `docs/k-report-data/`.
- Normalization: the K-relevant statistic is `|margin| / band_zero`
  (how many ε the margin clears the coincidence threshold by).
  Flagged classes:
  - **escalation-band landings**: `1 < |m|/ε < K` — the samples K
    actually converts from "definite" to "refuse";
  - **near-band definites**: `K ≤ |m|/ε < 10·K` — definite outcomes
    within a decade of the boundary, the population a larger K would
    start refusing.

## Results

<!-- RESULTS -->

## Findings

<!-- FINDINGS -->

## Draft recommendation

<!-- RECOMMENDATION -->
