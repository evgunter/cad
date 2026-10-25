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
