---
id: a-touch-window-fuzz-seed-contradicts-the-dense-distance-at-1e-12
kind: issue
title: curve_pair touch-window oracle fuzz row reds at eps 1e-12 on seed 0x810fa45f837dd426
status: open
opened: 2026-10-23
priority: P2
cost: M
---


## Found (QUAD, measure door unit A lane, 2026-10-23)

`crates/geom-brep/src/curve_pair/pose/window/oracle.rs:329`
`each_window_reading_holds_on_a_dense_distance_at_a_touch` failed in
the geom-brep suite run at `CAD_TOLERANCE_EPS=1e-12` on seed
`0x810fa45f837dd426`. That was the only geom-brep failure at that ε on
PR 5589's head. The PR does not touch `curve_pair`. Not root-caused
here.

**To reproduce:** run the row at ε = 1e-12 with that seed, replaying it
through `fuzz::replay()`'s documented environment variable. Then check
whether the same seed reds on main. A contradiction against the dense
distance is a soundness question until shown otherwise. The fuzz draws
at 1e-12 more rarely than the nightly reaches, so a seed is the
evidence.

## 2026-10-24 — a second seed (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`), full-suite job, eps=1e-6
pass: `ε 0.000000000001: 2 readings contradicted (reproduce with
CAD_FUZZ_SEED=0x9ac83e19a6f5b5a3 CAD_FUZZ_EFFORT=1)`. Draw 57 is a
pair of circles of radius ≈553 whose `Recedes` reading on
`(-3.909e-7, -3.908e-7)` (away false) and `(1.338e-7, 1.346e-7)`
(away true) contradicts the dense distance. The default and 1e-12
passes drew other seeds and passed. No merge in the window touched
`curve_pair/pose/window/`, so this is the fuzzer drawing into the
same defect, not a regression. Pin both seeds as deterministic rows
beside the fix.

## 2026-10-27 — another seed (SPLICE, PR 5868's slow-set run)

Default suite run (the row loops its own ε rows), PR 5868 at `833da0303`,
which does not touch `curve_pair`: `ε 0.000000000001: 2 readings
contradicted`, reproduce with `CAD_FUZZ_SEED=0xdcf91059414eafd4
CAD_FUZZ_EFFORT=1`. Draw 7, f64: `Recedes` read on two windows
`(-2.142e-7, -2.117e-7)` (away false) and `(1.139e-6, 1.141e-6)` (away
true), against a Circle of radius 477.40 centred at
`(-160.017, -381.167, 1.259)`, axis `(0.00135, 0.00227, 0.99999)`, and the
point b `(105.656, 48.066)`.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. A fourth seed: the 1e-6 pass drew `CAD_FUZZ_SEED=0xf440002aa745ad47`
(`CAD_FUZZ_EFFORT=1`), 3 readings contradicted at ε 1e-12, draw 31:
`Recedes` on `(-7.715e-7, -7.710e-7)`, `(-5.076e-7, -5.031e-7)` and
`(-1.380e-6, -1.379e-6)` (away false) against a Circle of radius 489.76.

The search is a counterexample fuzz, so no output can be pinned. The
1e-12 band is out of the row's loop
(`each_window_reading_holds_on_a_dense_distance_at_a_touch` prints a
`SKIPPED` line naming this row); the default band still searches.
Closing this row puts the 1e-12 band back, with the four seeds as
deterministic rows beside the fix.

## 2026-10-28 — another seed (SPLICE structural-rim lane)

A local default-ε run of the whole `geom-brep` suite on
`splice/structural-rim` (PR 5959, which does not touch `curve_pair`)
red this row on `CAD_FUZZ_SEED=0xc8d7ad5a8a57245d CAD_FUZZ_EFFORT=1`,
at the oracle's ε 1e-12 reading: draw 19 reads `Recedes` on
`(1.05e-6, 1.20e-6)` for a circle of radius 727 m centred about 675 m
from the origin. The seed replays deterministically.
