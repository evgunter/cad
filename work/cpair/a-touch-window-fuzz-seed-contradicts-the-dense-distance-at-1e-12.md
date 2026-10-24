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
