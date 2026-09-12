---
id: profile-fillet-radius-off-at-eps-1e-6
kind: issue
title: profile: the fuzzed offset-carrier fillet recovers its radius 2.6e-7 off at CAD_TOLERANCE_EPS=1e-6 (seed 0x063fda568e08fb0f, iter 380)
status: open
opened: 2026-09-04
refs: [1877]
---


(S-CERT orchestrator) Filed from CERT-M3's gate (PR #1877, run 33925337156,
job `test (eps = 1e-6, 2/2)`), where it is a red INHERITED from main: the
PR touches nothing under `crates/profile`, and the failure reproduces
byte-for-byte on main's own tree.

## The failure

`crates/profile/tests/review_s2.rs:673` —
`review_s2::fuzz_offset_carrier_construction_tangency_and_bulge`, oracle
(a) of `check_corner`:

```
recovered radius 0.07525678837520786 vs 0.07525705177877821 — iter 380:
reproduce with CAD_FUZZ_SEED=0x063fda568e08fb0f CAD_FUZZ_EFFORT=1
```

The fillet circle re-derived from the emitted `(t1, t2, bulge)` has a
radius 2.6e-7 off the requested one, against the oracle's 1e-9. Iteration
380 of the seed's sweep, at effort 1.

## Reproduction (2026-09-04)

Replayed with the seed, one test, separate build directories per tree:

| tree | `CAD_TOLERANCE_EPS=1e-6` | default eps |
|---|---|---|
| main `b7f347254` | **FAIL**, identical message | pass |
| PR #1877 head `19a775c0e` | **FAIL**, identical message | pass |

```
CAD_FUZZ_SEED=0x063fda568e08fb0f CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p profile --test all \
  -E 'test(fuzz_offset_carrier_construction_tangency_and_bulge)'
```

So the defect is (i) tolerance-dependent — the construction at the
`1e-6` band emits a `(t1, t2, bulge)` whose implied radius misses by
~3.5e-6 relative — and (ii) seed-dependent, which is why the eps=1e-6 row
is green on most runs of main: the fuzz draws a fresh seed per run and
this draw is one of the ones that finds it. Not measured here: which of
the emitted three carries the error (the tangent points against their
carriers, oracle (b), or the bulge), and whether the 1e-9 oracle is the
right bar at a 1e-6 band or the construction is genuinely off. Both are
the owning program's to measure at the site.

## Debt

The row is red on main independent of #1877; the S-CERT program annotated
the PR and did not absorb the fix (crates/profile is outside its fence).
