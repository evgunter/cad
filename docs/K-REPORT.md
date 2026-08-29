# K report — the ambiguity constant K = 10 (M2 telemetry, with the M3/M4/M5 snapshots, the #89 close, and the two M7 addenda)

**Status: FINAL** (M2 PR 7 deliverable; orchestrator-finalized
2026-07-21 after the adversarial review byte-reproduced the CSVs at all
three ε rows and independently re-derived every reported number. The
outcome is ratified into DESIGN.md's Q1 residue by the M2-exit sweep.
Per Evan on #41, the value needed no separate sign-off. That
byte-reproduction was a check against the tree of that day and is not
a standing property of the committed CSVs — see "Provenance of the M2
CSVs" under Methodology.)

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
    cargo test -p sweep --features probe --test all \
      -- --ignored --nocapture k_report::
  ```

  (`sweep` sets `autotests = false` and aggregates every suite into one
  `tests/all.rs` binary, so `--test k_report` names no target; the
  suite is selected by the `k_report::` module prefix instead. The
  `probe` feature is what compiles the file at all — it is
  `#![cfg(feature = "probe")]`.)

- Rows: ε ∈ {1e-6, 1e-9, 1e-12}. Raw CSVs: `docs/k-report-data/`.

  **Provenance of the M2 CSVs — what byte-reproduction does and does
  not mean here (corrected 2026-08-20, D15).** The status line above is
  a statement about 2026-07-21: the adversarial review byte-reproduced
  `docs/k-report-data/eps-*.csv` against the tree as it stood at
  finalization, and that check was real. It is **not** a standing
  claim: no tree since 2026-07-25 could have reproduced these files.
  Two things happened after the cut:

  - **2026-07-25, #101 (`548c9618`, declared-tangency discipline)**
    added `ProfileLoop::tangent_joints`, `judge_joints` and
    `ProfileError::UndeclaredTangency`. It migrated the corpora it
    exercised; `k_report.rs` is `#[ignore]`d and runs in no CI row, so
    its `rounded_prism` fixture — a rounded rectangle, tangent at all
    eight fillet-to-edge joints by construction — was missed and the
    harness has panicked at profile validation ever since. Nothing
    noticed for 26 days. Fixed by declaring the eight joints (D15); the
    fixture's coordinates and bulges are unchanged and the kernel
    verifies each declaration — a wrong declaration is refused as
    `TangencyContradicted`, so this is an assertion the kernel checks,
    not a silencer.

    **The migrator was in this directory that same afternoon.** #101's
    companion `0cda5f08` migrated ten files across six crates, among
    them `crates/sweep/tests/extrude_acceptance.rs` — where it declared
    **eight joints on a rounded square** — and
    `crates/mesh/tests/common/mod.rs`, whose rounded square is the same
    fixture down to `r = 0.5` and `tan(π/8)` and now reads
    `with_tangent_joints((0..n).collect())`, the identical idiom D15
    applied here. So the fix is not a judgement call reconstructed after
    the fact: it is the migration this file was supposed to get, and
    `rounded_prism` is conspicuously absent from a commit that
    enumerates the fixtures it converted. What singled it out was being
    `#[ignore]`d.
  - **2026-08-04 (`d8b8f6a8`, *collapse the remaining 122 test
    targets*)** folded `sweep`'s 60 test targets into one `tests/all.rs`
    binary, which retired the `--test k_report` target this section's
    command named. That command has therefore been **dead 16 days**, and
    it fails before compiling anything. The command above is the working
    one. Note the shape: **two independent breaks ten days apart**, and
    the second alone would have hidden the first — a reader who ran the
    documented command got `no test target named k_report`, never the
    panic.

  **The M2 CSVs are therefore a historical snapshot, not a
  reproducible artifact.**

  > **Measured against `9f559f6a` (2026-08-20), re-verified BYTE-IDENTICAL
  > after merging `origin/main` through `f382c4aa` (12 commits), and
  > still NOT GUARDED.** Every
  > figure in the next two paragraphs is a one-off observation of a
  > moving quantity: the next merge that adds a predicate name
  > falsifies the sample count, the name count and the ratio. It is not
  > registered, not asserted, and nothing will notice when it goes
  > stale — the harness runs in no CI row (D17). Guarding it would mean
  > committing a second baseline, which is the re-cut this unit
  > deliberately did not do. Read these as *dated evidence that the
  > committed files are stale*, never as current numbers.

  A fresh cut of the same ten shapes records **16 824 samples over 105
  predicate names** against the committed **13 282 over 63**. The
  breakdown matters, because the growth is **not purely additive**:

  - **+3 365** from **42 new names** — the `pcurve_*` chart/loop/trim
    family (PCURVE-UNIFY) 28, `tangent_*` 6, `props_rim_*` /
    `props_meridian_*` 7, `bool_ring_run_winding` 1;
  - **+177 of churn inside the original 63**, where **19 of the 63
    changed their own counts** — `chord_side` 216 → 245,
    `witness_on_surface_{1,2}` 178 → 194, `props_circle_axis_class`
    80 → 120 and others up, and **one down**:
    `carrier_matches_mapped_source` **1 296 → 1 224 (−72)**.

  "No name was lost" is true at the *name* level only. **The
  per-predicate counts in this report are stale for 19 of its 63 rows**,
  and one of them moved the wrong way — which is exactly the kind of
  thing a byte-reproduction check existed to surface. The committed
  files are left as cut: they are the M2-era record this report's
  numbers describe, and re-cutting them is the runbook's and the
  orchestrator's call, not a lane's.

  **The K = 10 conclusion survives the re-cut**, which is the part that
  decides whether anyone needs to hurry. The fresh sweep is ε-stable
  exactly as reported here (shape/predicate/outcome columns
  byte-identical across all three ε rows), and lands **0 samples in
  (ε, Kε), 0 within a decade of Kε, 0 indeterminate and 0 invalid** at
  every row. The definite-side minimum |m| is 1.0e-2 m — **250× the M7
  lint floor of 4.0e-5, i.e. 2.4 decades** (the "3 decades" figure in
  Finding 3 above is against *Kε at ε = 1e-6*, a different comparand;
  do not conflate them).

  **One claim does NOT carry over unscoped**, and it is named here
  rather than quietly left: the Zero-side bullet above reports the
  largest `zero`-classified |margin| as 8.9e-16 m, *"≥ 3 decades below
  even the tightest row's ε = 1e-12"*. On the fresh cut the largest is
  **1.378e-15 m** (`pcurve_map_residual`, one of the 42 new names) —
  **2.86 decades**, not ≥ 3. Restricted to the original 63 names it is
  still **8.882e-16 m**, so the sentence as written about *these CSVs*
  is intact and K = 10 is untouched; it is the *generalisation* to
  today's predicate set that fails, by 0.14 of a decade.

  **This break never touched the gate, and the gate reads no committed
  CSV at all.** `ci.yml`'s *K-telemetry probe sweep* runs
  `scripts/k_probe_sweep.sh` into `target/k-fresh` on every building
  merge, and `tools/k-lint` lints **that fresh sweep** against constants
  pinned in `tools/k-lint/src/lib.rs` (`BASELINE_FLOOR_MARGIN = 4.0e-5`
  and the rule set). Nothing under `docs/k-report-data/` is opened at
  gate time — the committed CSVs, M2's included, are a **record**, not
  an input. (Its neighbour `tess-lint` *does* diff against a committed
  baseline; k-lint deliberately does not.) So no staleness in any
  committed CSV can weaken the gate, and `k_report.rs` is the M2-era
  instrument only.

  **What CI now covers, stated precisely** (D17, closed 2026-08-20).
  `k_report.rs` is both **type-checked and run** on every building
  merge. The `k-lint` job's *"compile and list every probe-gated test target"*
  step covers the whole workspace — `scripts/gates/probe-suite-census.sh`
  derives the owning crates from the tree and the step `cargo check`s
  each `--features probe --all-targets`; the gate greps for that step
  name, so this paragraph cannot go quietly false — and
  `scripts/k_probe_sweep.sh` then *executes* this
  harness at all three ε beside the Band 4 corpus and the tour scenes.
  The two are not interchangeable: a type-check cannot see a panic, and
  running one harness says nothing about the suites nothing runs.

  **Which suites CI executes, rostered rather than counted.** The
  executed set is `scripts/gates/probe-suite-census.sh`'s `RUN_FLOOR`,
  and `scripts/k_probe_sweep.sh` is what runs it: two `--ignored` dump
  invocations (`m4_pr8_k_probe::` in `editor-core`, `k_report::` in
  `sweep`) inside the ε loop, and five default-selection runs before it
  (`m4_pr8_k_probe::` and `m5_pr5_corpus_probe::` in `editor-core`,
  `certified_door::` in `geom-core`, `k_report::` and
  `review_chamfer_r1_probes::` in `sweep` — `k_report::` runs no test
  under that selection and is rostered so its row reports the
  complement). Every other censused suite is compiled and not run, and
  each of them says so in its own header — the census gate refuses a
  probe suite that is on neither side, so a new one has to pick.

  **The roster is floored on what RAN, not on what a filter could
  match.** The sweep records the runner's own passed and ignored counts
  per invocation and `--check-executed` reads them back. Reachability
  would be the wrong key: `run_dump` passes `--ignored`, so a filter
  naming a suite of plain `#[test]`s matches the module and executes none
  of it — a floor built on "some filter names it" scores such a suite
  covered while it is inert. **The selection is part of the roster key**
  for the same reason: `--ignored` and the default selection run disjoint
  halves of a suite.

  **And the floor alone is not the whole check.** A floor catches a suite
  that stops running; it cannot catch one that grows a test no rostered
  selection reaches. Every rostered suite is therefore invoked once under
  the default selection, which reports how many `#[ignore]`d tests it
  skipped, and that number must equal what the `--ignored` invocation
  ran. The two selections then cover the suite with nothing left over,
  from the runner's own numbers rather than from a predicate over the
  source.

  **The distinction that was wrong, stated so it is not re-inferred.**
  Earlier prose put `editor-core`'s suites on the executed side because
  the sweep's invocation names that crate with `-p`. **Naming the crate
  is not naming the suite, and naming the suite is not naming the
  selection**: `crates/editor-core/tests/m5_pr5_corpus_probe.rs` was
  selected by no filter at all, and `m4_pr8_k_probe.rs`'s
  `corpus_evaluates_green_at_probe` sat inside a module the sweep DID
  name while carrying no `#[ignore]` — so the one filter that reached its
  module ran the other test in it and never that one. Both run now, as
  preconditions, once and outside the ε loop — **and the reason is
  redundancy, not ε-invariance.** What both actually assert is one-sided
  *greenness* at `Probe`; neither compares a `Probe` result against an
  f64 one, and greenness is tolerance-dependent. `m4_pr8_k_probe`'s
  `run_doc` asserts the same predicate over every corpus document at all
  three ε on every merge, so the ε sweep of that property is already
  paid. What running the default selection adds is that these bodies
  execute at all, and the `#[ignore]`d complement the floor reconciles.
  It runs at a stated ε (1e-9) rather than at whatever the ambient
  default happens to be.

  The total is deliberately not written here: it is that gate's derived
  tally, recomputed on every merge.

  **The M2 dump rides beside the gate, not inside it.** The sweep writes
  it to `<outdir>/m2/<prefix><ε>.csv`; `tools/k-lint` is handed the
  merged corpus+tour CSV only. Folding ten M2 shapes into the linted
  distribution would move what the thresholds are argued over, which is
  a K conversation and not a coverage one. So `k_report.rs` remains the
  M2-era instrument, and re-cutting `docs/k-report-data/eps-1e-*.csv` is
  now the same script CI runs rather than a hand-typed invocation.

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
   import-adoption time** (M7 under the 2026-08-03 renumbering;
   foreign geometry with real residuals and
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
`Tol::k`, env `CAD_AMBIGUITY_K`, default 10 — so future
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

## M3 addendum (snapshot, 2026-07-23 — M3 exit sweep, PR 6b)

**Scope: inventory only. This addendum records the M3 predicate crop
and the collection method for its future telemetry run; it does NOT
reopen the K = 10 decision (FINAL above), and no new margin data was
gathered — see "why no per-predicate data" below.**

M3 (splitting, booleans, tier 3′) added **59 predicate names** to the
unified `geom_core::k_stats` funnel — the richest crop yet, and the
first computed-intersection (rather than construction-controlled)
margin sources, exactly the pressure source Finding 4 anticipated:

- **24 `bool_*`** (boolean reduction/classification/join;
  `crates/topo`): `bool_contact_edge`, `bool_contact_edge_span`,
  `bool_contact_vertex`, `bool_dir_parallel`, `bool_dir_same`,
  `bool_ee_collinear`, `bool_faces_parallel`, `bool_germ_line`,
  `bool_join_facing`, `bool_join_nearest`,
  `bool_plane_offset`,
  `bool_plane_orient`, `bool_plane_parallel`,
  `bool_point_in_solid_{advance,denom,infinity,order,plane}`,
  `bool_sector_{arm,coplanar,reflex,straight,within}`,
  `bool_strut_order`.
- **19 `pm_census_*`** (the tier-3′ coincidence census, M3 PR 6a/#75;
  `crates/topo/src/census.rs`): `pm_census_vv_gap`,
  `pm_census_ve_line_gap`, `pm_census_ve_span`,
  `pm_census_vf_residual`, `pm_census_ef_residual`,
  `pm_census_ef_cut_gap`, `pm_census_ef_cut_span`,
  `pm_census_ee_parallel`, `pm_census_ee_gap`,
  `pm_census_ee_line_gap`, `pm_census_ee_span`,
  `pm_census_ee_overlap`, `pm_census_span_order`,
  `pm_census_span_gap`, `pm_census_bound_end`,
  `pm_census_bound_vertex`, `pm_census_confirm_vv`,
  `pm_census_confirm_vf`, plus the `pm_census_containment`
  escalation tag; the census also drives the existing
  `bool_contact_*` names through `contfp`.
- **10 `split_*`** (split reduction/classification/join):
  `split_bisector_side`, `split_edge_param_interior`,
  `split_join_frame_arm`, `split_section_area`,
  `split_sector_{arm,coplanar,extent,reflex,straight}`,
  `split_vertex_side`.
- **4 `point_in_loop_*`** (trilean containment, `laringmv`/F8 ray
  parity): `point_in_loop_{advance,arm,boundary,side}`.
- **2 `enters_material*`** (the F3 sign-chain primitive;
  `crates/geom-brep/src/enters.rs`): `enters_material`,
  `enters_material_arm`.

24 + 19 + 10 + 4 + 2 = 59, which is the count in the sentence above:
the bullets are the crop, and the arithmetic closing is how a reader
checks that neither has moved.

**Two names in these families were minted AFTER this snapshot and are
therefore NOT in it**, recorded here rather than folded into the
bullets — same rule as the seven orphans below, and for the same
reason: back-filling a dated crop makes it describe something it never
described.

- **`bool_join_chord`** (#719) splits the germ-chord LENGTH gate off
  `bool_join_nearest`, which keeps the nearest-candidate DIFFERENCE.
  The M4 addendum's decade-3 tail reads the split off the committed
  rows ("The `bool_join_nearest` 38 stay under that name", below).
- **`point_in_loop_segment`** (#712) splits the segment-length
  degeneracy gate off `point_in_loop_boundary`, which keeps the
  point-to-segment distance — one name that was deciding two
  questions. The family's samples and margins are unchanged: the
  49 290 `_boundary` samples of the M7 sweep split 24 645 / 24 645.
  So the M5 per-family table below correctly reads
  `point_in_loop_* | 4`, and a post-#712 sweep reads five.

(Inventory method: **superseded 2026-08-20 — see "The inventory
method, restated" below.** The method this addendum was cut with was
`grep -r 'decide("' crates/*/src` diffed against this report's M2 CSV
predicate column, plus the census's `gap_is_zero`/`signed_is_zero`
helper call sites, plus — since #712 — the row-name TABLES. It reaches
one spelling of one funnel entry point in one directory tree, and the
crop above is what it found. The three M2 refusal-path predicates dead
on the M2 corpus — `carrier_circles_internal`, `collinear_overlap`,
`extrusion_obliquity` — are M2-era, not counted here.)

### The inventory method, restated (2026-08-20)

**READ THIS FIRST: the roster is not complete, and was not.** The row
that ordered this restatement (§D's D19) described the roster as
*"complete today by luck of era"*. That premise is wrong, and it is
corrected here rather than worked around. **Seven predicate names that
the `k-lint`-gated corpus actually emits are recorded in neither this
document nor `docs/predicate-dimension-audit.md`, under any reading**
(tabled below). A reader who meets the old premise first and then finds
seven missing names will conclude the restatement is broken; it is the
premise that was.

**And a count of SITES was never the right measure of a NAME roster.**
The old method's blind spot was sized at *"37 sites across 24 files"*.
That figure reproduces exactly — and it understates the hole by more
than a factor of two, because one parameterised site carries many
names: those 37 sites carry **83 of the 233 names** in the committed M7
baseline. Size a name roster's hole in names.

**The rule.** A predicate name is in scope if it reaches the
`geom_core::k_stats` funnel — `decide`, `decide_flagged` or
`decide_invariant` — from anywhere the sweep can execute, **however it
is spelled at the call site**. That is the criterion
`docs/predicate-dimension-audit.md` already states for the same funnel
(*"every `classify`/`require_zero`/`require_extent`/`decide` funnel
call and every raw `sign_within` use"*); this document was using a
narrower one.

**Why the old rule under-reached, measured at `4a007a76`.** Of **349**
funnel call sites under `crates/*/src` (338 `decide`, 8
`decide_flagged`, 3 `decide_invariant`), **311 pass a bare string
literal** and carry **238 distinct names**. The other **37 sites, in 24
files**, do not — and the site count badly understates the consequence,
because one parameterised site carries many names: **83 of the 233
names in the committed M7 baseline have no `decide("<name>"` site
anywhere in the tree.**

Five ways a name escapes the old pattern, all live today:

1. **A different funnel entry — the sharpest instance, because the
   site satisfies the method's own criterion.** `decide("` does not
   match `decide_flagged("` or `decide_invariant("`. **Eleven sites,
   ten of them passing a bare string literal**: by the old rule's own
   description — "a name written as a literal at the funnel site" —
   these are covered, and they are not.
   `volume_backstop{,_operand,_violation}`, `bool_ray_cylinder_disc`,
   `revolve_axis_dir_in_plane`, `revolve_full_vs_partial`,
   `pcurve_cone_chart_nappe`, `bool_point_in_solid_denom`.
2. **A different wrapper spelling.** Names reach the funnel through at
   least `check_residual`, `classify`, `require_zero`, `coincident`,
   `zero`, `gap_is_zero` and `signed_is_zero`. The old method named the
   last two.
3. **A module-private `const &str`.** Five, not the three previously
   recorded: `sector_shape.rs`'s `SECTOR_{ARM,REFLEX,STRAIGHT}`, plus
   `editor-core/src/names/geompred.rs`'s `SEL_DATUM_DISTANCE`
   (`sel_datum_distance`) and `sweep/src/fillet/surgery.rs`'s
   `RING_CLEARANCE` (`fillet3_ring_clearance`).
4. **A struct field or a local table.** `ray_parity::ParityRows` (the
   one carrier this document already listed), `swept.rs`'s
   `CosurfaceNames`, and `transform.rs:129`'s seven-element
   `[(&'static str, T); 7]` array consumed by a loop variable.
5. **The scan root — a scope error in the method, not a missed site.**
   The pattern greps `crates/*/src`, while the corpus the gate is fed
   from is not confined to it: `demos/tour/src/booleans.rs` decides
   `demo_flush_{offset,orient,parallel}` through the same funnel, and
   `k_probe_sweep.sh` records them into the very CSV `k-lint` reads. A
   roster method that sweeps one tree and calls itself complete, while
   the gated corpus is fed from two, is wrong by construction — no
   amount of care at the sites it does scan would have found these.

**Both halves have a blind spot, and the union is the roster.** The
code scan misses names not written as a literal at a funnel site (the
83 above). The CSV column — "what the corpus actually emitted" — misses
names the corpus never reaches: **88** of the 238 literal names are
absent from the M7 baseline, `bool_join_chord` among them, and it is
named in the post-snapshot note under the M3 crop above. Neither is a
roster alone. Re-deriving:

```sh
# behavioural half — what a fresh sweep emitted
scripts/k_probe_sweep.sh target/k-fresh
tail -n +2 target/k-fresh/k-eps-1e-9.csv | cut -d, -f2 | sort -u
# code half — every funnel site, all three entries, all roots
grep -rnE '\b(decide|decide_flagged|decide_invariant)\s*\(' \
  crates/*/src demos/*/src
```

The second command is a **starting set, not an answer**: it reports
sites, and a site whose first argument is not a literal has to be read.
That residue is the roster's standing cost, and it is why this is
written down rather than discovered.

**What the restatement catches that the old one did not.** *Measured
against this document AS THE ROW FOUND IT (`ff5ad78e`, before this
section existed) — re-running them against the text below now returns
smaller numbers, because the seven orphans are tabled here.* Of the 83
carried names, **53 appear nowhere in the pre-restatement document**
verbatim, and **17** are not covered even by a `family_*` mention
there. Cross-checked
against `docs/predicate-dimension-audit.md`, which carries
`transform_rigid_*` and `transform_rigid_trans_finite_*` as family rows,
**seven names are recorded in neither document under any reading**:

| name | home | carrier |
|---|---|---|
| `arc_apex_identity` | `profile/src/seg.rs` | `coincident(…)` wrapper |
| `contact_at_shared_vertex` | `profile` | wrapper |
| `side_planes_cosurface` | `sweep/src/swept.rs` | `CosurfaceNames` field |
| `side_cylinders_cosurface` | `sweep/src/swept.rs` | `CosurfaceNames` field |
| `demo_flush_offset` | `demos/tour/src/booleans.rs` | outside the scan root |
| `demo_flush_orient` | `demos/tour/src/booleans.rs` | outside the scan root |
| `demo_flush_parallel` | `demos/tour/src/booleans.rs` | outside the scan root |

They are recorded here rather than folded into the M3 crop above: that
crop is a dated era snapshot of what M3 added, and back-filling it
would make it describe something it never described.

**Maintenance: this roster is a RECORD, and stays hand-maintained.**
The decision is on what the roster is *for*, and the evidence is that
nothing computes with it:

- No tool opens `docs/K-REPORT.md`. Every reference to it in `*.rs`,
  `*.sh`, `*.py`, `*.toml` and `*.yml` is a prose citation.
- `tools/k-lint` is handed CSV paths and lints rows against constants
  pinned in its own source. Its one name-keyed rule,
  `EPS_COUPLED_PREDICATES`, is a deliberate allow-list that fails
  **loud** — an ε-coupled predicate missing from it *keeps flagging*
  under the metre rules until someone rules. A roster omission
  therefore cannot silently weaken the gate. (Its neighbour `tess-lint`
  *does* diff a committed baseline; k-lint deliberately does not, and
  that difference is what makes this ruling possible.)
- A gate would have to be fed a machine-readable roster, which is the
  maintenance burden this decision declines; a reporting register would
  commit a second copy of a number the sweep already produces on every
  merge in `target/k-fresh`, one `cut -d, -f2 | sort -u` away.

So: **stated criterion, disclosed residue, no CI row.** What a future
reader is owed instead is above — the rule, the five escape routes, the
two blind spots, and the seven names measured outside both documents.
Adding a name carrier without recording it here still silently drops
its rows from the roster; that is now a disclosed cost rather than an
undetected one.

**Why no per-predicate margin data in this snapshot.** The recording
mechanism is the `Probe` scalar: per-predicate CSVs require running a
corpus end-to-end at `T = Probe` with `CAD_K_REPORT_OUT` set, one
process per ε (Methodology above). The only such harness,
`crates/sweep/tests/k_report.rs`, hard-builds the ten M2 sweep shapes
— it does not touch split/boolean/census code paths. The M3 corpora
(`m3_pr*`/`m3_pr6_tier3prime` and the promoted review suites) are
generic over `T` but instantiate only the f64 and Interval lanes; no
Probe instantiation exists. Producing M3 per-predicate data therefore
requires a new Probe-lane harness over the M3 corpus — new
infrastructure, deliberately out of the docs-only exit-sweep scope
(recorded per the PR 6b charter rather than built ad hoc).

**Collection method for the future run** (unchanged mechanics, ready
when a harness exists): instantiate the M3 corpus at `Probe`, run one
process per ε row with `CAD_TOLERANCE_EPS=<ε>
CAD_K_REPORT_OUT=docs/k-report-data/m3-eps-<ε>.csv`, and reuse this
report's normalization (`|m|/band_zero`, the escalation-band /
near-band-definite classes and the counterfactual-K table — no per-K
reruns needed). The natural trigger remains the one Finding 4 named:
refusal-path and near-coincidence statistics want an adversarial
corpus, whose first real instance is D7 import adoption.

## M4 addendum — the Band 4 + demo-scene telemetry run (2026-07-26, M4 PR 8b)

**Scope: the Probe run the M3 addendum specified, now executed** —
the harness gap is closed. This addendum reports the regenerated
distribution and the derivation of the large-K lint's thresholds
(`tools/k-lint`). It does NOT reopen K = 10 (FINAL above); on this
corpus, as on M2's, every candidate K in {3, 10, 30, 100} decides
identically (the band is empty; see the histogram).

- **Harnesses** (new in PR 8b): `m4_pr8_k_probe.rs` evaluates every
  Band 4 corpus document end-to-end at `T = Probe` (editor-core
  gained `ContentBits for Probe`); the demo tour gained scalar-generic
  scene constructors and a `k-probe` binary mode that rebuilds every
  scene at Probe through the SAME constructors the f64 tour runs.
  `scripts/k_probe_sweep.sh` runs both, one process per ε, and merges.
- **Data**: `docs/k-report-data/m4-eps-{1e-6,1e-9,1e-12}.csv.gz` —
  M2's columns exactly, shapes namespaced `corpus/<doc>` /
  `demo/<scene>`. Deviation from M2's convention (REPORTED): the raw
  rows are large — 203 MB at the 1e-6 row, ~162 MB at the tighter
  rows (2,562,157 samples vs M2's 13,282) — so the committed record
  is gzipped; `gzip -dc` reproduces the M2 format byte-for-byte
  row-wise.

| ε row | samples | zero | definite | indet. | invalid | in (ε, Kε) | definite within a decade of Kε |
|-------|--------:|-----:|---------:|-------:|--------:|-----------:|---:|
| each of 1e-6 / 1e-9 / 1e-12 | 2 562 157 | 458 734 | 2 103 423 | 0 | 0 | 0 | 0 |

Counts are identical at all three ε rows (margins are geometry; only
the bands move — the M2 ε-stability observation, reproduced at 193×
the sample count, now including computed-intersection margins: the
M3 `bool_*`/`pm_census_*`/`split_*` crops sample here for the first
time). The first refusal-path samples also land (the tour finale's
bowtie profile, 20 samples/row).

**The distribution stays sharply bimodal.** Normalized decade
histogram of |m|/ε for the DEFINITE population (ε = 1e-6 row; the
other rows are the same histogram shifted by exactly 3/6 decades):

```
decade 3  |          240   (1.7e3 .. 1e4 — composition below)
decade 4  |       47 622
decade 5  |    1 023 036
decade 6  |    1 031 564
decade 7  |           95
decade 39+|          866   (exact tie-break bands: canonical_order_*,
                            split_join_order_*, fillet_leg_fit at
                            band_zero 5e-324 / 1e-100 — order
                            decisions, excluded from the lint's ratio
                            rules)
```

The 240-sample decade-3 tail (the floor's neighborhood): 173
demo/az (bool_point_in_solid_plane 89, bool_join_nearest 38, and the
pm_census gap/residual family), 63 die `witness_at_mid_parameter`
(42 corpus + 21 demo — the same document through both paths), 3
demo/projectbox_cutaway `split_bisector_side`, 1 demo/table. All are
real millimeter-scale feature clearances, not noise.

**The `bool_join_nearest` 38 stay under that name after #719's split**
(which minted `bool_join_chord` for the germ-chord LENGTH gate and left
the name on the nearest-candidate DIFFERENCE). Two independent reads of
the committed M7 rows say so, identically at all three ε rows:

- **Sign.** Three of the 38 are negative
  (`-5.172658143638709e-3` ×1, `-5.88521089089028e-3` ×2), and across
  all of az's sub-1e-1 samples every magnitude appears with both signs
  at bit-identical values (`±5.474101278454191e-2` at 130/15). A chord
  norm cannot be negative; a difference of two of them can.
- **Recording order.** The CSV rows are in decision order, and the two
  sites have distinct signatures: the gate is followed by the facing or
  conic-section decision it guards, the selection is preceded by a
  facing decision. Partitioning M7's 41 745 `bool_join_nearest` samples
  that way is total (no unclassified row) and splits them **28 544
  chord-gate / 13 201 selection**; the gate half is entirely positive
  with `min |m| = 5.000e-2 m` and contributes **nothing** to this
  decade-3 tail and nothing to the zero cluster, while the selection
  half carries all 4 726 exact zeros, all 466 negatives, and all 38 of
  these. The partition rule was checked against ground truth on the
  twin boolean configurations at the post-split head, where the names
  are known: 656 of 656 correct, none ambiguous.

The corollary is that the split is **not** a clean cleave of the pooled
row into the report's two clusters: on the M7 corpus the chord row is
all-positive with a 5 cm floor while the selection row keeps positives,
zeros and negatives together. `docs/k-report-data/`
rule 1 stands — nothing in those files is renamed, and a `bool_join_*`
row there dates to its era.

Zero-side: 447 581 of 458 734 zero-classified margins are EXACTLY 0;
the rest are ≤ 5.33e-15 m (worst: `pm_census_ee_span`, demo/az).
Definite-side floor: **1.689e-3 m** (`pm_census_ee_gap`, demo/az —
a real 1.7 mm feature gap). The gap between the clusters spans ~12
decades and is EMPTY: 0 indeterminate, 0 invalid, nothing within a
decade of any band edge at any ε row.

### The large-K lint (Evan's ask, ruled 2026-07-25; spec D3)

*Historical, left as written. The thresholds and rule set below are the
M4 originals; both were revised on 2026-08-07 — see "M7 addendum: the
large-K lint's floor refresh" at the end of this report for the current
constants (floor 4.0e-5, rule 4, rule 2's cap) and for the CI row's
current posture: it is a **gate**, not the advisory row described
below.*

`tools/k-lint` (workspace-excluded tooling — thresholds are lint
policy, never kernel ε) scans freshly regenerated sweep CSVs and
FLAGS, **advisory-only in this first iteration**:

1. any `indeterminate`/`invalid` outcome (in-band = the kernel
   already refused; the lint makes it visible pre-merge);
2. band proximity within 10^2 at any supported ε row: definite
   |m| < 10²·Kε, or zero-classified |m| > ε/10²;
3. **the baseline floor**: definite |m| < 1.5e-3 (the constant in
   `tools/k-lint/src/lib.rs` with this provenance).

**Percentile choice (reported per spec): P0 — the observed baseline
minimum 1.689e-3, rounded down to 1.5e-3 (~11% of headroom below the
observed floor).**
Candidates from the baseline (1e-6 row, ratio units): P0.01 = 7.8e3,
P0.1 = 3.2e4, P1 = 6.3e4. Any P > 0 permanently flags the baseline's
own bottom tail (already at P0.01, the 240 real az census margins) on
every advisory run — pure noise, no signal, because the 12-decade
empty gap makes the population edge itself the maximally informative
threshold: a NEW margin below the floor sits in the no-man's land
between honest coincidence (≤ 5.3e-15) and honest feature (≥ 1.7e-3).
The committed baseline lints CLEAN at all three ε rows (verified,
2,562,157 × 3 samples, 0 flags), so every advisory line the CI row
ever prints is a real distribution change.

**The litmus (#99, replayed)**: `tools/k-lint/tests/litmus.rs`
resurrects the pre-#100 bracket (via point 1.146) at Probe and
re-measures its `carrier_line_circle` margin from the live
predicates: 2.315e-6 m. The lint fires at EVERY supported ε row —
in-band at 1e-6 (the row where #99 actually panicked), below the
baseline floor at 1e-9 and 1e-12, where the margin was a DEFINITE
outcome invisible to every pre-existing gate. The shipped
fillet-constructed bracket (margin < 1e-15, definite Zero) lints
clean at every row. The lint would have caught #99 before any
escalation band was entered — the motivating claim, demonstrated.

Observed headroom worth watching (honest caveat): the zero-side
proximity rule's closest baseline approach is `pm_census_ee_span`'s
5.3e-15 residual at the 1e-12 row — ratio 5.3e-3 against the 1e-2
threshold, only 1.9× clear. A future scene with ~1e-14 float noise
at model scale will advisory-flag at 1e-12; that is the intended
signal (ε = 1e-12 has thin noise headroom at unit scale), not a
false positive to tune away.

## M5 addendum — the curved-corpus telemetry snapshot (2026-08-03, M5 PR 14)

**Scope: the M5 exit sweep's T5 K-telemetry run, over the corpus and
demo scenes as they stand at main's tip (post-#166). This is the
#89 revisit that M2's Finding 4 named — and the FIRST snapshot in
which the counterfactual-K decision surface is not completely flat.
Its outcome: **#89 is CLOSED and K = 10 is the permanent ratified
default** (Evan, PR #169 comment 5171303851, 2026-08-03), with a
testable re-open trigger. See "Decision" below.**

- **Harnesses**: unchanged — `scripts/k_probe_sweep.sh`
  (`crates/editor-core/tests/m4_pr8_k_probe.rs` over every registered
  Band 4 corpus document + the tour's `k-probe` scene mode), one
  process per ε, merged. No new infrastructure; the M5 curved
  documents joined the corpus registry through their own PRs.
- **Data**: `docs/k-report-data/m5-eps-{1e-6,1e-9,1e-12}.csv.gz`,
  M2's columns exactly. *Reported naming deviation*: the sweep script
  hard-codes the `m4-eps-` output prefix, so the committed M5 baseline
  was renamed on commit; the rows are byte-identical to what the
  script wrote. Retiring the hard-coded prefix is a code change and
  was deliberately not made in this docs-only unit.
- **Reproducibility**: the hosted `k-lint (advisory)` row on main's
  tip (run 30835146557) reports the same sample counts to the unit
  — 1 758 387 / 1 758 411 / 1 758 435 — and the same flags. This
  snapshot is CI truth, not a local artifact.

| ε row | samples | zero | definite | indet. | invalid | in (ε, Kε) | definite within a decade of Kε |
|-------|--------:|-----:|---------:|-------:|--------:|-----------:|---:|
| 1e-6  | 1 758 387 | 420 422 | 1 337 965 | 0 | 0 | 0 | 0 |
| 1e-9  | 1 758 411 | 420 422 | 1 337 989 | 0 | 0 | 0 | 0 |
| 1e-12 | 1 758 435 | 420 422 | 1 338 013 | 0 | 0 | 0 | 0 |

Population split: 269 767 `corpus/<doc>` samples over 13 registered
documents + 1 488 620 `demo/<scene>` samples over 17 scenes.
**208 distinct predicate names** sample (M4: 145) — 63 new names,
0 retired. `<unnamed>`: 0 (harness-asserted). Zero-side: 404 975 of
420 422 zero-classified margins are EXACTLY 0; the worst is
5.329e-15 m (`pm_census_ee_span`, demo/az) — the identical M4 value,
so the noise cluster has not moved.

The total is LOWER than M4's 2 562 157 despite a richer corpus; the
demo tour's scenes were reworked across M5 (dual montage, #165) and
several M4-era scene rebuilds no longer run twice. Sample counts are
not a coverage metric and no claim rides on the direction.

### Finding M5-1: the ε-stability observation is RETIRED

M2 and M4 both reported decision counts *identical at every ε row*.
That is no longer true: 1 758 387 / 1 758 411 / 1 758 435, +24 per
row. The entire difference is one predicate —
`props_quad_converged` on `demo/tiltedcut` — which samples 8 / 32 /
56 times at 1e-6 / 1e-9 / 1e-12.

It is a **convergence-loop stopping test**: adaptive quadrature over
the tilted-cut face refines until its residual clears an ε-derived
target, so a tighter ε buys more refinement rounds and each round
records one more classification. Its margins form a visible ladder
(ε = 1e-12 row, one seed's rungs, m in meters):

```
1.833e-4 → 2.271e-5 → 2.823e-6 → 3.511e-7 → 4.293e-8 → 4.468e-9 → 3.360e-10
```

This is the first predicate in the project whose **margin is
ε-coupled** rather than model-scale. Every other margin in the
corpus is a distance or angle fixed by the geometry, so tightening ε
moves only the band; here it moves the margin too, and the ratio
|m|/ε does NOT grow as 1/ε. The bimodality claim survives — this
family is still definite everywhere — but the claim "the pipeline is
bitwise ε-stable in its decision COUNTS" does not, and should not be
repeated without this exception.

### Finding M5-2: the model-scale floor dropped ~0.5 decade

Definite floors by ε row (excluding the exact tie-break bands at
`band_zero` 5e-324 / 1e-100 — `canonical_order_*`,
`split_join_order_*`, `fillet_leg_fit` — which are order decisions,
not distances):

| predicate | shape | |m| (m) | note |
|---|---|--:|---|
| `bool_ring_run_winding` | demo/projectbox_cutaway | 5.086e-4 | ε-independent; the new model-scale floor |
| `props_rim_side` | corpus/die_pips | 5.760e-4 | the S13 pip rim — a real 0.58 mm feature |
| `props_quad_converged` | demo/tiltedcut | 8.395e-4 (1e-6) / 1.647e-7 (1e-9) / 3.360e-10 (1e-12) | ε-coupled, see M5-1 |
| `pm_census_ee_gap` | demo/az | 1.689e-3 | M4's floor, unchanged |

M4's floor of 1.689e-3 is now the FOURTH-lowest. The two new
ε-independent entries are honest sub-millimetre features (a ring-run
winding headroom and a die pip's rim clearance), not noise: the gap
to the zero cluster (≤ 5.3e-15) is still ~11 decades and still
completely empty.

**Consequence, stated plainly: the large-K lint's baseline floor is
stale.** `BASELINE_FLOOR_MARGIN = 1.5e-3` was the P0 of the M4
distribution. The M5 corpus sits below it, and the hosted advisory
row on main's tip prints **102 flags** (10 at 1e-6, 34 at 1e-9, 58 at
1e-12 — every one of them `props_quad_converged`, `props_rim_side`,
or `bool_ring_run_winding`). The lint is advisory-only by design
(M4 PR 8b D3: "printed, not failing … gate once the baseline is
trusted"), so nothing is red — but its first-iteration posture has
now expired: the threshold no longer distinguishes signal from
baseline. **Re-deriving the floor against this distribution (and
deciding whether the ε-coupled family belongs under a ratio rule
rather than a metre rule) is a code change and is therefore NOT made
here — it is carried as a named M6 pickup** (M6 = the main-path
completions under the 2026-08-03 renumbering).

> **DONE (2026-08-07)** — the pickup is discharged; see "M7 addendum:
> the large-K lint's floor refresh" at the end of this report. The
> floor was re-derived against a fresh sweep at the M7 tip
> (`BASELINE_FLOOR_MARGIN` 1.5e-3 → 4.0e-5, P0 of the ε-INDEPENDENT
> population) and the ε-coupled family got its own ε-relative rule
> (4). Two corrections to the paragraph above, for the record: the
> binding family at the M7 tip is `volume_backstop`, which did not
> exist at M5 — `props_rim_side` and `bool_ring_run_winding` have
> since risen to 2.2000e-2 and 6.5104e-3 and are no longer near the
> bottom; and the uncapped flag count at the M7 tip was 54, not 102,
> all at the 1e-6 row (Finding M7-F1).

### Finding M5-3: what this corpus STILL cannot show — no SSI margins

M2's Finding 4 named computed intersections as the pressure source.
M5 shipped the SSI marcher (#146) with 14 named predicates. **None
of them sample in this snapshot.** Neither do the sphere-class
boolean predicates, the cyl×cyl chord family, the cone family, the
NURBS span meter, or the second-order sector classifiers:

```
ssi_branch_open_end  ssi_closure_return  ssi_closure_tangent
ssi_cs_tangency  ssi_foot_orthogonality  ssi_hull_sup
ssi_hull_sup_chart  ssi_on_locus  ssi_on_locus_foot
ssi_step_progress  ssi_transversality  ssi_transversality_arm
ssi_tube_transversality
bool_sphere_extent_gap  bool_sphere_recut_align
bool_sphere_sphere_gap  bool_sphere_sphere_nested
bool_line_cylinder_clearance
cc_axes_coplanar  cc_axes_parallel  cc_coaxial
cc_declared_radius_equality  cc_parallel_gap  pc_parallel_gap
pn_apex_on_plane  pn_apex_section  pn_axis_normal
nurbs_span_meter  fillet3_chain_arm  fillet3_chain_g1
split_conic_departure  split_conic_inplane_mid
split_tangent_chord_forward
tangent_sector_order2  tangent_sector_order2_arm
tangent_sector_osculation
extrusion_obliquity   (still dead — M2-era refusal path)
```

The reason is structural and not a harness gap: the curved documents
that ARE registered — `cut_cylinder` (tilted plane × cylinder) and
`boss_union` (cylinder boss ∪ plate) — are exactly the cases M5
gave **exact analytic carriers**, so they resolve through the conic
lane and never enter the marcher. The SSI marcher's own acceptance
geometry lives in its test suites, which instantiate f64 and
Interval, not `Probe`. Reaching it needs either a Band-4 corpus
document whose boolean genuinely requires marching, or a Probe
instantiation of the SSI suites. Both are new work, deliberately out
of this docs-only unit.

So the honest scope line for this snapshot is: **it is the first
with computed TANGENCY and QUADRATURE margins** (`tangent_hull_sup`,
`tangent_normal_parallel`, `tangent_on_surface_1/2`,
`tangent_second_order`, `tangent_tube_margin`, 2 861 samples;
`props_quad_converged`, `props_quad_face_extent`), **and the first
with pcurve-certification margins** (18 `pcurve_*` names, 2 400
samples) **and fillet margins** (12 `fillet*` names, 410 samples —
including `carrier_circles_internal`, dead since M2, which finally
fires). It is **not** the SSI evidence Finding 4 asked for.

Per-family populations (ε = 1e-6 row; the distributions are
ε-invariant except `props_*`, per M5-1):

| family | names | samples |
|---|--:|--:|
| `pm_census_*` | 14 | 555 020 |
| `point_in_loop_*` | 4 | 389 802 |
| `bool_*` | 32 | 359 122 |
| `carrier_*` | 12 | 218 269 |
| M2-era construction (unprefixed) | 50 | 175 217 |
| `witness_*` | 3 | 23 543 |
| `interval_*` | 2 | 14 795 |
| `enters_material*` | 2 | 10 494 |
| `tangent_*` | 6 | 2 861 |
| `split_*` | 22 | 2 778 |
| `props_*` | 21 | 2 415 |
| `pcurve_*` | 18 | 2 400 |
| `canonical_*` | 2 | 1 018 |
| `fillet3_*` | 5 | 348 |
| `pc_*` / `fillet_*` / `ps_*` / `wall_*` / `ellipse_*` | 15 | 305 |
| `ssi_*` | 0 sampled (14 named) | 0 |

### Counterfactual K, re-run on the curved corpus

Derived post hoc from `|m|/band_zero` as before — no per-K reruns.
"Escalations" = definite samples a candidate K would convert to
refusals (`1 < |m|/ε < K`); "near" = definites within one decade
above that candidate's own boundary (`K ≤ |m|/ε < 10K`).

| ε row | K=3 esc / near | K=10 esc / near | K=30 esc / near | K=100 esc / near | min |m|/ε |
|---|--:|--:|--:|--:|--:|
| 1e-6  | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 10 | 508.6 |
| 1e-9  | 0 / 0 | 0 / 0 | 0 / 2 | 0 / 14 | 164.7 |
| 1e-12 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 8  | 336.0 |

**Every candidate still converts exactly zero decisions.** What has
changed is the clearance. The corpus minimum ratio is 164.7 (the
`props_quad_converged` ladder's bottom rung at ε = 1e-9), so:

| candidate K | clearance to the closest definite |
|---|--:|
| 3 | 55× |
| 10 | 16.5× |
| 30 | 5.5× |
| 100 | **1.65×** |

M2 and M4 could say "the decision surface is completely flat across
the candidate range." M5 cannot. K = 100 now has under a factor of
two of headroom against a real, shipped, every-run margin family;
K = 30 has 5.5×; K = 10 retains better than a decade.

### Decision: #89 CLOSED — K = 10 is the permanent ratified default

**Ruled by Evan on PR #169 (comment 5171303851, 2026-08-03):
"closing 89 makes sense."** This supersedes the continuation this
addendum originally recommended; the grounds below are the ones the
recommendation was built on and they support the close directly.

**K = 10 is the permanent ratified default.** Not "held pending
evidence" — decided. Three grounds:

1. **Nothing in three snapshots has ever pressured K in either
   direction.** Zero in-band landings, zero indeterminate, zero
   invalid, at every ε row of M2's, M4's and now M5's corpora — 5.3M
   decisions in this one alone. A free parameter should keep its
   ratified, documented default rather than churn.
2. **The first evidence with any discriminating power argues
   specifically against RAISING it.** The ε-coupled quadrature family
   puts a real, shipped, every-run population at ~1.6e2-8.4e2 × ε,
   which leaves K = 100 only 1.65× of clearance and K = 30 only 5.5×,
   against K = 10's 16.5×. Raising K toward those values would trade
   a currently-empty band for one an ordinary convergence loop enters
   under ordinary refinement — converting an honest definite into a
   refusal for no modelling reason.
3. **Waiting longer would not have improved the decision.** The
   evidence M2's Finding 4 named — computed intersections, then
   foreign geometry — has been "one milestone away" for three
   milestones. It is still absent here (Finding M5-3). Deciding on
   three corpora that agree, with a stated re-open trigger, is
   better than carrying an open question indefinitely against
   evidence that keeps not arriving.

**The re-open trigger, stated so it is testable rather than
aspirational: any corpus that shows IN-BAND LANDINGS** — a margin
classified indeterminate in `(ε, Kε)`, or an escalation the band
converts — reopens K. Not "definite margins that look close": the
counterfactual table already prices closeness, and closeness alone
has never changed a decision. The expected first source of such
landings is the **import corpus** (foreign geometry with real
residuals and near-coincidences not of our making), which under the
2026-08-03 renumbering arrives at **M7 (STEP adoption)**. The
`k-lint` advisory row is the standing detector: its rule 1 flags any
indeterminate or invalid outcome, so a landing surfaces at the next
hosted run without anyone re-reading this report.

Note that the machinery for a future change is already in place and
costs nothing to leave there: K is per-run configuration
(`Tol::k`, env `CAD_AMBIGUITY_K`, default 10), so a
future corpus can probe alternatives without code changes. Closing
#89 ratifies the default; it does not weld the dial.

**Named follow-ups (code, not this unit):**
- **M6 pickup — re-derive the k-lint baseline floor** against the
  M5 distribution, and decide whether `props_quad_converged`'s
  ε-coupled family belongs under a ratio rule rather than the metre
  floor. 102 advisory flags per run is a broken signal-to-noise
  ratio, and the lint's own charter said "gate once the baseline is
  trusted" — it cannot be gated in this state.
- **M6 pickup — an SSI Probe lane**: either a Band-4 corpus document
  whose boolean genuinely requires marching, or a `Probe`
  instantiation of the SSI acceptance suites, so the next K snapshot
  finally carries computed-intersection margins. Sequenced at M6
  under the 2026-08-03 renumbering, because that is where the SSI
  generic-`T` lift and the loft assembly land — the units that put
  marched geometry into a body at rest in the first place.

---

## M7 addendum (2026-08-05): the first in-band landing — fired,
## diagnosed, RETIRED as a dimensional-metering defect

Recorded by the docs-rot unit so this report's record matches what
happened; the section above is left as written.

**The trigger fired.** During M7-2 (FreeCAD foreign-corpus import,
#189), the hosted sweep produced the project's FIRST in-band K
landing: ε = 1e-7, fixture `cone_trunc`, predicate
`props_rim_level_group`, margin 5.590169943747308e-7 = √5/4 × 1e-6
— in Band{1e-7, 1e-6}. Verified bit-exact at review (the A3
attack), reported to Evan on the designated #89 thread with nothing
retuned.

**The diagnosis.** Evan's probe of the margin's DIMENSION broke the
case: the margin was an AREA (m², a two-length product, quadratic
in model scale) where a rim-level comparand should be a LENGTH.
Root cause in `geom-brep/src/props/curved.rs::du_of_rims`: every
rim-level comparand was metered by × arm — correct for the
sphere/torus payloads (sin v / cos v, dimensionless) but WRONG for
cylinder/cone, whose level v is already a length; the × arm(≈1e-3)
factor SHRANK a decisively-separated margin into the band (at
large scale it would inflate instead). The true rim separation on
`cone_trunc` is ~5.6e-4 m ≈ 5590ε.

**The retirement.** Fixed at PR #197 (RimLevel per-kind metering;
the fix corrects real verdict flips in both directions). Post-fix
the landing re-measures at √5/2 mm — length-dimensioned,
scale-linear, decisively OUT of band; the a3 sweep delta is exactly
one line. **K = 10 is unmoved and #89 stays closed** — the record
lives in the #89 thread comments; the follow-on dimensional sweep
is `docs/predicate-dimension-audit.md` (~120 rows, F-findings).

**The caveats this earns, stated for the next landing:**
1. **An in-band landing can be a dimensional-metering bug rather
   than K evidence.** The trigger protocol gains a step: before
   treating a landing as ε-vs-scale or K pressure, check the
   margin's DIMENSION against the predicate's comparand (the
   predicate-dimension audit is the checklist). The detector
   worked — it caught a real bug — but what it caught was not what
   this report forecast.
2. **This report's "without anyone re-reading this report" promise
   half-failed**: the landing surfaced at a non-CI ε row (1e-7,
   between the standard 1e-6/1e-9/1e-12 rows) during a milestone
   sweep, not via the k-lint advisory row; and its interpretation
   required exactly the re-reading the sentence hoped to avoid.
3. The two "M6 pickup" follow-ups above are hereby re-tagged
   **UNOWNED pickups** — M6's executed units (1–4) all merged with
   neither follow-up done, and M6 remains open awaiting Evan's exit
   walk (the k-lint baseline floor is still the stale M4-era 1.5e-3
   with ~102 advisory flags/run; the SSI Probe lane still has no
   owner). The k-lint floor refresh holds a promoted lull-queue
   spot per M6-LOG's status summary (docs/M6-LOG.md).
   **Update 2026-08-07: the k-lint floor-refresh pickup is DONE** (M7
   addendum, below); the SSI Probe lane pickup is still unowned.

---

## M7 addendum (2026-08-07): the large-K lint's floor refresh

**Scope: a fresh `scripts/k_probe_sweep.sh` run at main's M7 tip, the
re-derivation of `BASELINE_FLOOR_MARGIN` against it, and the ruling on
the ε-coupled family that M5-2 banked.** This closes the "UNOWNED
pickup" the M6-era section above names ("the k-lint baseline floor is
still the stale M4-era 1.5e-3 with ~102 advisory flags/run").

- **Harnesses**: unchanged (`crates/editor-core/tests/m4_pr8_k_probe.rs`
  over every registered Band 4 corpus document + the tour's `k-probe`
  scene mode, one process per ε, merged).
- **Data**: `docs/k-report-data/m7-eps-{1e-6,1e-9,1e-12}.csv.gz`, M2's
  columns exactly. **The M5-reported naming deviation is retired**:
  `k_probe_sweep.sh` no longer hard-codes the `m4-eps-` prefix — it
  defaults to the milestone-neutral `k-eps-` (what CI writes into its
  scratch dir) and takes `K_SWEEP_PREFIX` for the committed,
  milestone-stamped baselines. These rows are what the script wrote, no
  rename. The `m4-*` and `m5-*` baselines stay committed: they are the
  durable record.
- **Wall clock** (this box, warm deps, one build slot): 78 s to build
  the probe binaries, 32 s for the whole three-row sweep, ~60 s to lint
  all three rows. Not evidence of anything — a cold run (full rebuild
  after a main merge) is ~8–13 min on the same box.
- **Snapshot semantics, stated precisely.** Like `m4-*` and `m5-*`,
  these rows are a snapshot cut at a stated head, not a moving mirror
  of main. Every number in this addendum was reproduced BYTE-IDENTICALLY
  by a fresh sweep at the head where the baseline was cut. Main has
  since added one predicate — `path_junction_turn` (demo tour paths),
  293 samples/row across 10 demo scenes, every margin |m| ≥ 2.5 m, all
  definite. A fresh sweep at that later main is +293 rows and still
  lints **0 flags at all three ε rows**; the floor, the empty gap, the
  ε-independent population's P0 and the ε-coupled ratio are untouched,
  which is exactly the property a threshold snapshot is supposed to
  have. Re-cutting the baseline on every main merge is neither the M4/M5
  precedent nor useful; re-cut it when the DISTRIBUTION moves.
  **Drift since, in the other direction (2026-08-19).** #661 pooled the
  six `bool_sector_*` / `split_sector_*` names into three
  (`sector_{arm,reflex,straight}`), so a fresh sweep now also DROPS six
  names where until then drift had only added them. The **233** below is
  still the correct count for this committed snapshot, which still
  contains all six; a sweep at today's main carries **231**. Margins,
  bands, outcomes and order are untouched — only the `predicate` column,
  and only for those six values. Full treatment: the census note
  (2026-08-19) at the end of this report.

| ε row | samples | zero | definite (ambient) | indet. | invalid | in (ε, Kε) |
|-------|--------:|-----:|---------:|-------:|--------:|-----------:|
| 1e-6  | 1 792 902 | 443 183 | 1 348 473 | 0 | 0 | 0 |
| 1e-9  | 1 792 926 | 443 183 | 1 348 497 | 0 | 0 | 0 |
| 1e-12 | 1 792 950 | 443 183 | 1 348 521 | 0 | 0 | 0 |

284 178 `corpus/<doc>` samples + 1 508 724…772 `demo/<scene>`.
**233 distinct predicate names** (M5: 208, M4: 145). The +24/row ε
ladder is M5-1's `props_quad_converged`, unchanged in kind. Zero side:
418 711 of 443 183 zero classifications are EXACTLY 0; the ambient
worst is 5.3291e-15 m (`pm_census_ee_span`, demo/az) — the identical
M4 and M5 value, so the noise cluster still has not moved.

### The bottom of the distribution, by ε row

Ambient definite minima per predicate (excluding the exact tie-break
bands at `band_zero` 5e-324 / 1e-100, which the lint's ratio rules
never touched):

| predicate | shape | 1e-6 | 1e-9 | 1e-12 | class |
|---|---|--:|--:|--:|---|
| `props_quad_converged` | demo/tiltedcut | 8.3952e-4 | 1.6467e-7 | 3.3595e-10 | **ε-COUPLED** |
| `props_quad_converged` | corpus/loft_prism | 1.0240e-3 | 1.0240e-6 | 1.0240e-9 | **ε-COUPLED** |
| `volume_backstop` | corpus/die_pips, corpus/die_composed | 4.7965e-5 | = | = | ε-independent — **the new floor** |
| `volume_backstop` | demo/projectbox_cutaway | 1.3214e-4 | = | = | ε-independent |
| `volume_backstop` | corpus/die, demo/die | 1.4706e-4 | = | = | ε-independent |
| `volume_backstop` | demo/table | 1.4986e-3 | = | = | ε-independent |
| `pm_census_ee_gap` | demo/az | 1.6893e-3 | = | = | ε-independent (M4's floor) |
| `split_bisector_side` | demo/projectbox_cutaway | 3.6621e-3 | = | = | ε-independent |

Two changes since M5-2 matter. First, **`props_rim_side` (5.760e-4) and
`bool_ring_run_winding` (5.086e-4) — two of the three families in M5's
102-flag count — are no longer near the bottom**: their minima are now
2.2000e-2 and 6.5104e-3. Second, a family M5 did not have appears
underneath everything: **`volume_backstop`**, the boolean engine's
volume invariant, which #200 re-metered as a mean boundary displacement
`ΔV/(A_got + A_bound)` so its telemetry would be a length. Its floor
sample is the composed die's pip cavities — a real 48 µm quantity, 1.5
decades below M4's floor.

**Classification test used**: min |m| per predicate across the three
rows. `props_quad_converged` falls 6.4 decades from 1e-6 to 1e-12.
Every other predicate reproduces its minimum BIT-IDENTICALLY at all
three ε — margins are geometry, only the band moves — **with one
carve-out** (corrected on review; the first draft of this section
claimed there was no intermediate case, and that was wrong):
`props_quad_face_extent`, 12 ambient definite samples per row and 8 of
them differing across rows, has minima 4.0245003e-1 / 4.0256189e-1 /
4.0256210e-1. It is ε-DEPENDENT but not ε-proportional — it is the
CONVERGED quadrature's face extent, so a tighter ε buys more
refinement rounds and the recorded enclosure bound converges toward a
fixed ~0.4025621 m geometric value (total spread 2.8e-4 relative,
shrinking with ε rather than tracking it).

It moves nothing that matters: at 0.40 m it is 4 decades above the
floor, nowhere near the empty gap, and the ε-independent population
count (1 348 461), its P0 (4.7965e-5), and the gap's emptiness are all
identical whether this predicate is counted as ε-independent or set
aside. It also does not belong in rule 4: a margin converging to a
fixed length is a model-scale distance, so the metre rules are the
right ones for it. The honest statement is therefore not "no
intermediate case exists" but "the one intermediate case is
numerically inert, and no threshold in this refresh depends on which
side of the classification it lands."

Within a decade above the ε-independent bottom edge (4.7965e-5 …
4.7965e-4) there is exactly one family, `volume_backstop`. Within a
decade above the ε-coupled bottom edge at 1e-12 (3.3595e-10 …
3.3595e-9) there is exactly one, `props_quad_converged`.

### The new floor, and the percentile choice re-argued

**`BASELINE_FLOOR_MARGIN` = 1.5e-3 → 4.0e-5 m.** P0 of the
ε-INDEPENDENT ambient definite population (1 348 461 samples, the same
count at all three rows): observed minimum 4.7965e-5, rounded down with
16.6% of headroom.

Percentile candidates on the NEW distribution (identical at all three
rows, because the population is ε-independent by construction):

| percentile | value (m) | what it would strand |
|---|--:|---|
| P0 | 4.7965e-5 | nothing — **chosen** |
| P0.001 | 1.4916e-4 | the die / die_pips / die_composed backstop margins |
| P0.01 | 3.8393e-3 | the entire `volume_backstop` family, `pm_census_ee_gap` (M4's own floor), `split_bisector_side` |
| P0.1 | 2.4000e-2 | most of the corpus's millimetre-scale census work |
| P1 | 6.2500e-2 | the sub-centimetre corpus wholesale |

The M4 argument survives the corpus growth unchanged: any P > 0 flags
the baseline's own bottom tail on every advisory run — permanent noise,
no added signal — because the gap between the clusters is still empty.
Measured on this sweep, the ε-independent definite population has
**ZERO samples between the zero cluster's 5.3291e-15 and 4.7965e-5**,
at every ε row: a 10.0-decade no-man's land (M4 had ~12). That empty
gap is what makes the population edge the maximally informative
threshold, and it is why the floor stays P0.

Headroom is wider than M4's ~11% deliberately. M4's floor sample was a
rigid feature gap; `volume_backstop`'s margin is ΔV/(A+A), a smeared
quantity that moves with how fine a model's detail is relative to its
surface area, so it earns a wider skirt.

**The #99 litmus contract holds with room**: the datum is 2.315e-6 m,
1.2 decades below the new floor, so it still flags at every supported ε
row (in-band at 1e-6, `BelowBaselineFloor` at 1e-9 and 1e-12). The
litmus now ASSERTS that relation instead of documenting it —
`margin < BASELINE_FLOOR_MARGIN` is a test line, so no future refresh
can cut the floor under the datum silently.

### Ruling: the ε-coupled family (rule 4)

`props_quad_converged` records `1024·ε − width` — a headroom against an
ε-SCALED convergence target, not a distance. No fixed metre floor can
be clean at 1e-12 and informative at the same time, so it comes out
from under rule (3). **It comes out from under rule (2) as well**, and
that is the part worth arguing rather than assuming: its entire
operating range is `(0, 1024·ε] = (0, 102.4·Kε]` at the ratified
K = 10, so "within 10² of Kε" is its permanent state and carries no
information. Rule (2) applied to this family is a tautology, not a
finding — which is why M5's 102-flag count was dominated by it.

It is not muted. **Rule (4) is rule (2) recalibrated to the family's
own scale**: flag when the headroom falls below
`EPS_COUPLED_FLOOR_RATIO · ε = 1.5e2 · ε = 15·Kε`, the bottom ~15% of
the range. That is a real fragility statement — the stopping round
cleared its target by so little that at the interval scalar the
enclosure could straddle and escalate. The constant is P0 of the
baseline's own |m|/ε population (minimum 164.674 at demo/tiltedcut,
1e-9 row; the other rows sit at 839.524 and 335.953) with 8.9% of
headroom.

Its semantics differ from the metre floor's and the difference is
stated rather than glossed: this is a calibrated proximity rule on a
BOUNDED statistic, not the edge of an empty gap. The quadrature loop
stops at the first round whose width clears the target, widths fall
~8× per round, so the headroom has **no structural lower bound** — a
face whose width lands just under target will trip rule (4). That is
the intended signal.

Membership is an explicit allow-list (`EPS_COUPLED_PREDICATES`, one
entry today), never inferred: a new ε-coupled predicate is not on it,
stays under the metre rules, and flags loudly until someone rules on
it.

### Finding M7-F1: rule (2)'s definite arm degenerates at ε = 1e-6

Not banked, not forecast — it fell out of the fresh data. Rules
(2)-above and (3) are two thresholds on the same quantity, one in band
units (`10²·Kε`), one in metres. Rule (2) says something rule (3) does
not only while `10²·Kε < BASELINE_FLOOR_MARGIN`. At K = 10 that
inequality holds at 1e-9 (1e-6 < 4e-5) and at 1e-12 (1e-9 < 4e-5), and
FAILS at 1e-6, where `10²·Kε = 1e-3 m` is 25× the floor. There rule (2)
is no longer a proximity rule at all: it is a second, uncalibrated
floor sitting above the calibrated one, and it flags exactly the
corpus's known fine-feature population — including the samples the
floor was cut from.

Measured: with rule (2) uncapped, the fresh sweep prints **54 flags at
1e-6 and 0 at 1e-9 / 1e-12**. All 54 are `volume_backstop`, all
ε-independent, all ABOVE the new floor, spanning 4.7965e-5 … 9.5017e-4
on corpus/die (21), demo/die (21), demo/projectbox_cutaway (10),
corpus/die_pips (1), corpus/die_composed (1).

**Treatment**: rule (2)-above is capped at `BASELINE_FLOOR_MARGIN`. This
is a statement about the rule's discriminating power, not a family
exemption — every margin below the floor still answers to BOTH rules,
rule (1) still catches any sample that actually escalates, and the cap
is inert at both rows where rule (2) is the stronger statement. What is
genuinely true at ε = 1e-6 — that this corpus's finest honest features
sit less than a decade above the escalation band, i.e. that 1e-6 is a
loose ε for a sub-millimetre corpus — is a fact about the ε choice, not
about any one sample, so the CLI prints it as one `note:` line on every
file where the cap binds. Nothing is suppressed silently.

This is the definite-side twin of the M4 addendum's zero-side caveat
("ε = 1e-12 has thin noise headroom at unit scale"). Symmetrically:
**ε = 1e-6 has thin FEATURE headroom at millimetre scale**, and the
corpus has now grown fine enough to prove it.

### Acceptance (all rows executed locally at this tip)

| row | result |
|---|---|
| fresh sweep, 3 ε rows, new rules | **0 flags** (1 792 902 / 1 792 926 / 1 792 950 samples) |
| committed `m7-eps-*.csv.gz`, decompressed, 3 ε rows | **0 flags**, same counts |
| fresh sweep at a LATER main (post-`demos/tour` paths) | **0 flags** (1 793 195 / 1 793 219 / 1 793 243) |
| `cd tools/k-lint && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test` | green — 8 unit + 2 litmus |
| #99 litmus | fires at every supported ε row (in-band 1e-6; below floor 1e-9, 1e-12) |

### The row is a GATE

**`k-lint (gate)` fails on a finding** (ruled by the project owner, PR
#243). The CI row — hosted `.github/workflows/ci.yml`, local
`local-scripts/ci-local.sh` — is red whenever any margin in a fresh sweep
crowds a decision boundary; harness breakage still fails it in its own
distinct voice, and the two exit codes differ (2 vs 1) so they can
never be confused for one another.

**The failure message is the contract, not the exit code.** A fired
lint is evidence about the MARGIN DISTRIBUTION — the thresholds above
say a region should be empty and a sample landed there — and it is
just as likely to mean the threshold or the baseline is stale as it is
to mean anything about the geometry. Recourse, in order: re-derive the
baseline and thresholds per the snapshot contract this addendum
demonstrates (fresh sweep at a stated head, percentile choice
re-argued, committed rows byte-reproduced — the `EPS_COUPLED_FLOOR_RATIO`
constant is the likeliest to want it, being the minimum of 108 draws
with 8.9% of headroom); or, if re-derivation is not warranted, demote
the row to advisory in both wirings with a recorded justification.
**Changing geometry to get under a lint threshold is the one forbidden
move** — it destroys precisely the evidence the row exists to collect.
The CLI prints this on every failure; the three exit voices are pinned
by `tools/k-lint/tests/cli_contract.rs`.

## Census note (2026-08-19): the sector rungs are ONE population (#652)

Not a sweep and not a threshold change — a **name merge**, recorded here
because the census is the thing it changes.

Six K names became three. `bool_sector_{arm,reflex,straight}` (boolean
lane) and `split_sector_{arm,reflex,straight}` (splitting lane) are, since
#647, literally one implementation of one quantity —
`crates/topo/src/sector_shape.rs`, called from both lanes, with the name
set handed in as a parameter precisely so this decision could be taken
separately. It is taken: **pool them** (Evan, 2026-08-19, issue #652).
They now emit `sector_arm`, `sector_reflex`, `sector_straight`.

**Why, in one line that is not tidiness.** Coverage. Recomputed from
`m7-eps-1e-6.csv.gz`: all 64 `split_sector_reflex` samples are exactly
zero, so the splitting lane's wideness name had **no** corpus coverage of
a definite convex-or-reflex verdict, while `bool_sector_reflex` had 426
(418 positive + 8 negative) of 1880. Pooling gives the rung one
population with those 426 rather than two of which one is degenerate.
The precedent runs both ways and both directions are now on the record:
`docs/archive/M3-LOG.md:264` (PR #55 review MINOR-1) forced two margins
under one name to be **split**; `bool_planar_chord_spec` and `chord_spec`
deliberately **share** `split_arc_window`. This is the first time one
margin under two names was examined.

**Effect on the census count.** The M7 baseline's **233 distinct
predicate names** (`docs/k-report-data/m7-eps-*.csv.gz`, verified 233 at
all three ε rows) becomes **230** for any sweep cut after this change:
six names out, three in, nothing else touched. Main has since also added
`path_junction_turn` (recorded above), so a fresh sweep at this tip
carries **231**. No other predicate's name, margin, band or outcome
changes. The M7 addendum's own "233" is left as written — it describes
the committed snapshot, which still says 233 because it still contains
the six old names.

**Effect on the emitted stream.** Margins, order, bands and outcomes are
bit-identical; only the `predicate` column changes, and only for these
six values. Reproduced with the probe #647 left for exactly this —
`cargo test -p topo --features probe --test all -- --nocapture
probe_s5_sectors::sector_margin_stream | grep '^K '` on merge base
(`17b077f7`) and tip:

| | merge base | tip |
|---|--:|--:|
| recorded rows | 26 541 | 26 541 |
| rows that are NOT sector rungs | 26 121 | 26 121 — **byte-identical, no rewrite** |
| sector-rung rows | 420 | 420 |
| `bool_sector_arm` / `split_sector_arm` | 56 / 112 | `sector_arm` **168** |
| `bool_sector_reflex` / `split_sector_reflex` | 56 / 112 | `sector_reflex` **168** |
| `bool_sector_straight` / `split_sector_straight` | 56 / 28 | `sector_straight` **84** |

Rewriting only the predicate column of the base stream
(`s/^K (bool|split)_sector_(arm|reflex|straight)\|/K sector_\2|/`) makes
the two files **identical**, SHA-256
`7c0e4ee0efe0a60fb564bed3f049e2f097214c00c9bbd1dff8622065bee71aed`
(the base stream unrewritten is
`b1d84289d2f80db66be434b0c98451938628814b1a901336feec9e272dd8649f`).
Order, margins, bands and outcomes are untouched; the merge is exactly a
substitution on one column. `bool_sector_{coplanar,within}` and
`split_sector_{coplanar,extent}` appear in both streams unchanged, as
they should.

**Disposition of `docs/k-report-data/`: LEFT AS WRITTEN.** The committed
CSVs (`m4-`, `m5-`, `m7-`, and the M2-era `eps-*.csv`) are dated
snapshots of a stated head — "these rows are what the script wrote, no
rename" is already the standing rule for them (M7 addendum), and the
k-lint gate reads a *fresh* sweep, never these files, so nothing breaks
by leaving them. Regenerating them would be worse than useless: it would
destroy the historical record to make it agree with a name. A map of
that directory — the four eras, the two rules that govern it, and which
of the eleven names matching `grep sector` belong to which — now sits
at `docs/k-report-data/README.md`, so a reader who arrives at the CSVs
by grep does not have to reach section nine of this report to date a
row.

**How a future reader knows which era a row belongs to** — the one
sentence this note exists for. The pooled names are **new spellings, not
either lane's old one**, so the predicate column is self-dating: a row
reading `bool_sector_arm` / `split_sector_arm` (etc.) is **pre-#652**
data; a row reading `sector_arm` (etc.) is **post-#652**. No row in any
committed file silently changes meaning, because no committed row is
touched and no name is reused across the boundary.

That is the reason the merge did not simply keep `bool_sector_*`. The
29:1 majority spelling was the cheap choice — three fewer rows to
touch — and two things are wrong with it, in this order.

1. **It would be an actively FALSE name, not merely an uninformative
   one.** After pooling there is one population, so every
   splitting-lane decision would be recorded under a name whose prefix
   asserts `bool_`, on rows that carry nothing else to tell the lanes
   apart. That argument does not depend on a count. *How many* rows it
   would mislabel is a property of the corpus and not of the design,
   and the two numbers in this report differ by an order of magnitude:
   **64 of 1944** sector-arm samples in the M7 sweep (3.3% — the corpus
   is boolean-heavy), but **112 of 168** — two thirds — in the S5
   probe's fixture set, which is deliberately weighted to drive both
   walks. Neither number bounds the next sweep's.
2. **The era ambiguity**: 1880 pre-merge rows per ε would have become
   indistinguishable from post-merge ones. That one bites only a
   **cross-snapshot** comparison, since every committed file is dated by
   filename and cut at a single head, so no one file mixes eras. But
   cross-snapshot comparison is precisely what this report does — M4 →
   M5 → M7 in nearly every table above — so it is not hypothetical
   either.

The **M3 addendum's inventory** (the `bool_*` and `split_*` bullet lists
above) is likewise left as written: it is a dated 2026-07-23 record of the
crop M3 added, and it is accurate about that. Read alongside this note.

**Still forked, and correctly so.** `bool_sector_{coplanar,within}`,
`split_sector_{coplanar,extent}` are the `sector_face` twins and the
face-extent arm — different quantities, still two implementations, the
rest of smell-scan S5. Pooling does not reach them.
