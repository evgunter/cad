# K report — the ambiguity constant K = 10 under M2's telemetry

**Status: FINAL** (M2 PR 7 deliverable; orchestrator-finalized
2026-07-21 after the adversarial review byte-reproduced the CSVs at all
three ε rows and independently re-derived every reported number. The
outcome is ratified into DESIGN.md's Q1 residue by the M2-exit sweep.
Per Evan on #41, the value needed no separate sign-off.)

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
- Scope: the corpus is all-valid by construction, so refusal-path
  predicates that only fire on invalid input never sample here (dead
  on this corpus: `carrier_circles_internal`, `collinear_overlap`,
  `extrusion_obliquity`) — refusal-path margin statistics await an
  adversarial corpus (D7 / M3).
- Normalization: the K-relevant statistic is `|margin| / band_zero`
  (how many ε the margin clears the coincidence threshold by).
  Flagged classes:
  - **escalation-band landings**: `1 < |m|/ε < K` — the samples K
    actually converts from "definite" to "refuse";
  - **near-band definites**: `K ≤ |m|/ε < 10·K` — definite outcomes
    within a decade of the boundary, the population a larger K would
    start refusing.

## Results

Headline numbers (identical structure at every ε row — the pipeline is
bitwise ε-stable in its decision COUNTS, as the determinism suites
predict):

| ε row | samples | predicates | indeterminate | invalid | in (ε, Kε) | definite within decade of Kε |
|-------|--------:|-----------:|--------------:|--------:|-----------:|-----------------------------:|
| 1e-6  | 13 282  | 63         | 0 | 0 | 0 | 0 |
| 1e-9  | 13 282  | 63         | 0 | 0 | 0 | 0 |
| 1e-12 | 13 282  | 63         | 0 | 0 | 0 | 0 |

Outcome classes: 8 136 `zero` + 5 146 definite per row; 0
indeterminate, 0 invalid. `<unnamed>` samples: 0 (asserted by the
harness — the unification closed the tagging gap).

The margin distribution is sharply **bimodal**:

- **Zero-side** (residual checks — carrier-on-surface, endpoint pins,
  Newell/planar residuals, the `props_*` consistency fits): the
  largest `zero`-classified |margin| across all 8 136 samples is
  **8.9e-16 m** (`carrier_matches_mapped_source`); most are ≤ 2.3e-16.
  That is ≥ 3 decades below even the tightest row's ε = 1e-12.
- **Definite-side**: the smallest definite |margin|/ε at ε = 1e-6 is
  **1e4** (`interval_span_winding`, margin ≈ 1e-2 m·rad — the
  near-full wedge's winding headroom (τ − θ)·r at θ = τ − 0.01; the
  deliberately adversarial shape of the set), with
  `revolve_angle_headroom` at 2e4 and everything else ≥ 1e5. At
  ε = 1e-12 the minimum ratio is 1e10.

Per-predicate tables: 63 predicates spanning every deciding layer
(profile validation, extrude, revolve incl. the two-band machinery,
certification, tier-3 dihedral/residual sweeps, and the new `props_*`
mass-properties classifications + `positive_volume`). Full per-row
CSVs in `docs/k-report-data/eps-{1e-6,1e-9,1e-12}.csv`
(columns: shape, predicate, margin, band_zero, band_escalate,
outcome).

### Counterfactual K (Evan-requested, #41)

Every `MarginSample` records the margin and `band_zero`, so outcome
counts for ANY candidate K are derivable post hoc from the normalized
ratios — no per-K reruns. For candidates K ∈ {3, 10, 30, 100}, over
the samples with |m|/ε > 1 (the population a K converts):

| ε row | would-be escalations (any K in {3,10,30,100}) | definites within a decade above the candidate | min |m|/ε |
|-------|--:|--:|--:|
| 1e-6  | 0 | 0 | 1e4  |
| 1e-9  | 0 | 0 | 1e7  |
| 1e-12 | 0 | 0 | 1e10 |

Even K = 100 converts nothing at ε = 1e-6, and no definite margin sits
within a decade of any candidate's boundary. The decision surface is
completely flat across the candidate range on this corpus.

## Findings

1. **The band is empty in practice.** Across 39 846 decisions ×
   3 ε rows, not one margin landed in (ε, Kε), and no definite margin
   came within a **decade** of Kε. The gap between the noise cluster
   (≤ 1e-15, honest coincidences) and the model cluster (≥ 1e-2·scale,
   honest features) is ~13 decades wide at these shapes' unit scale.
   K = 10 sits comfortably inside that gap at every tested ε.
2. **K's exact value is unexercised.** Any K from ~2 to ~1e3 would
   have produced identical decisions on this corpus. The data
   therefore neither *demands* K = 10 nor argues against it — it
   confirms the sliver-band design statement is cheap at M2's
   construction-only workload (margins are construction-controlled;
   nothing yet *computes* near-coincident geometry).
3. **The stress comes from angle headroom, not residuals.** The
   closest definite approaches are the full-period winding headroom
   `(τ − θ)·r` on near-full partial revolves — a margin the USER
   controls (their θ), not evaluation noise. Even θ = τ − 0.01 clears
   Kε by 3 decades at ε = 1e-6.
4. **What this corpus cannot show** (scoping, per Evan): M2's native
   constructions are a **well-conditioned corpus** — profile-validated
   inputs, sweep-generated geometry, margins the modeler controls.
   The expectation is that the strongest K evidence arrives at **D7
   import-adoption time** (foreign geometry with real residuals and
   near-coincidences not of our making), with M3's booleans/SSI
   (computed intersections) the other pressure source. This report's
   claims are scoped to the native-construction corpus accordingly.

## Recommendation (final)

**Keep K = 10 as the default.** The M2 telemetry gives no empirical
pressure to move it in either direction: the band converted zero
decisions at any tested ε — and the counterfactual table shows every
candidate in {3, 10, 30, 100} behaves identically on this corpus — so
the value is currently free, and a free parameter should keep its
ratified, documented default rather than churn. (Per Evan's #41
direction, K is now ε-style per-run configuration —
`Tolerance::get().k`, env `CAD_AMBIGUITY_K`, default 10 — so future
corpora can probe alternatives without code changes.)
Retaining a full decade of escalation headroom above ε remains the
right *a-priori* posture for M3, where boolean/SSI margins will be
computed quantities with real conditioning error; revisit with the
same harness once M3's intersection predicates exist (the funnel is
now unified, so M3 predicates join the telemetry for free). If a
future corpus shows definite margins crowding the band from above
while the zero cluster stays at machine noise, the data would then
support *shrinking* ε (or K) rather than growing it — the current gap
is overwhelmingly on the definite side.
