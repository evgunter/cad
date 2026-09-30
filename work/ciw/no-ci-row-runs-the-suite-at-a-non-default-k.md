---
id: no-ci-row-runs-the-suite-at-a-non-default-k
kind: issue
title: No CI row runs the suite at a non-default K — the eps axis runs at three values nightly and the K axis at one
status: open
opened: 2026-09-11
priority: P3
cost: E
---

(FIX orchestrator) From the `literal-k-where-the-runs-k-belongs` lane,
PR 2346. Re-homed from BLIND (closed 2026-09-28) and re-read against
the latency-cut CI.

## The hole

No workflow sets `CAD_AMBIGUITY_K`, so every CI run executes at
`DEFAULT_K` = 10. The eps axis is configuration and is run at three
values (`nightly.yml`'s `full-suite`, and per-PR for the eps-sensitive
crates a diff seeds); K is treated as a constant, although
`CAD_AMBIGUITY_K` is run configuration with the same standing,
`Band::linear` scales the coincidence threshold by it, and the only
floor is `k > 1.0` (`crates/geom-core/src/tolerance.rs`).
`scripts/k_probe_sweep.sh` measures margins to inform the choice of K;
it does not run the suite at another one.

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

## What is owed

A considered answer, and its home is the nightly, not the per-PR gate
(`work/ciw/latency-cut.md`): a `full-suite` step at one or more
non-default K, or a written argument that K = 10 alone suffices.
