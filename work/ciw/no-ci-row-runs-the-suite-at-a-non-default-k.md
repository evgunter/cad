---
id: no-ci-row-runs-the-suite-at-a-non-default-k
kind: issue
title: No CI row runs the suite at a non-default K — the eps axis is gated at three values and the K axis at one
status: open
opened: 2026-09-11
---


(FIX orchestrator) From the `literal-k-where-the-runs-k-belongs` lane,
PR 2346. Placed here because `.github/workflows/*` is CIW's glob.

## The hole, measured

`grep -rn "AMBIGUITY_K" .github/workflows/` returns **nothing**. Not
one job, row or matrix entry sets it, so every one of the twelve
`test (…)` jobs a code-tier run gates executes at `DEFAULT_K` = 10.

The asymmetry is the point. The gate treats ε as configuration and
draws it properly — three eps rows × two lanes × two shards — and
treats K as if it were a constant. It is not: `CAD_AMBIGUITY_K` is run
configuration with the same standing as ε, `Band::linear` scales the
coincidence threshold by it, and the tolerance module's only floor is
`k > 1.0` (`crates/geom-core/src/tolerance.rs:483`), so the legal range
is wide.

`scripts/k_probe_sweep.sh` does run on a code-tier run
(`.github/workflows/ci.yml:4816`) and is **not** this coverage. It
measures margins in order to inform the choice of K; it does not
execute the suite at a different one.

## Why it is worth a row rather than a note

PR 2346's lane ran the suite by hand at `CAD_AMBIGUITY_K` = 1.05, 3, 30
and 100 and found **three** K-dependent defects, two of them red on
`main`'s own tree at a legal K:

- `crates/editor-core/tests/dsc_checks.rs` — a slab `10ε` thick, so
  `V/A = 5ε` and the escalation it asserts happens only for K > 5. Red
  at K = 3 and at K = 1.05.
- `crates/sweep/tests/bool3_torus_doors.rs` — the shell measured at the
  run's band, the law computed from a literal `10·ε`. At K = 100 it
  fires its own law assertion — *a false accusation against the
  kernel*, which is the most expensive shape a test failure can take.
- `the_clamp_floor_clears_the_torus_tangency_shell` — a fixed floor
  against a shell growing as K^⅓. Red at K = 30, filed separately as
  `work/tint/torus-tangency-shell-floor-does-not-scale-with-k.md`.

Two were repaired in PR 2346. **None of them would have been found by
the gate**, and none was found by a reviewer reading code — all three
took running at another K. A defect class that only a configuration the
gate never draws can reveal is invisible for as long as nobody varies
it by hand, which is the same shape as the doc-link red that sat on
`main` for a week because no push happened to classify code-tier.

## What this does NOT claim

Not that the K axis should be drawn like the eps axis — twelve jobs ×
N K values is a cost question, and cost is CIW's and S-TCOST's to
answer, not FIX's. The candidates worth weighing are a single extra job
at one non-default K, a nightly row across several, or a K drawn per
run the way eps rows are. What the row asserts is only that **one**
is not a considered number: nothing in the tree argues for it, and
three live defects sat behind it.

## What was checked before filing

`main` at `0f07897e` (no workflow mentions `AMBIGUITY_K`), every open
PR by GitHub code search for `AMBIGUITY_K` (no matches), and CIW's two
open PRs by title (`ciw/apt-preamble-guard` #2345,
`ciw/criterion-selftest` #2330 — neither touches the test matrix).
Not checked: the full diffs of those two PRs.
