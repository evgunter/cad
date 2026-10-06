---
id: replace-face-zero-span-reanchor-reads-as-a-kernel-defect
kind: issue
title: an offset that collapses an untouched edge to zero span refuses at the attach door with "a kernel defect worth reporting", which is false for a user-reachable input
status: open
opened: 2026-10-06
priority: P3
cost: E
rides_with: replace-face-refusals-open-with-a-stage-prefix-and-name-keys
---


Filed by the `shell/lofted-wall-seam` lane (PR 4117), from its review's
C2 NOTE. The refusal is typed; its TEXT is the defect.

## Measured

The straight square prism (`crates/sweep/tests/common/approx.rs`,
`prism()`: walls 1 m tall), `topo::replace_face_offset(top cap, -1.0)`
— an inward move of exactly the walls' height. The re-anchor reads each
vertical seam's moved end at `t_new == t0` (the edge collapses to a
point), the gate passes, and the attach door refuses:

```
replace_face_offset: the attach door refused EdgeKey(5v1): geometry
attachment gate: the stored parameter interval spans no length at this
tolerance — a degenerate zero-span interval, which the forward gate
refuses. Recourse: move the geometry so this edge is not vanishingly
short; an edge of no length, or a reversed one, is one no kernel
construction mints, so this is a kernel defect worth reporting
```

(`Op { edge: Some(EdgeKey(5v1)), error: Certification { error:
IntervalNotForward { verdict: Zero(..) } } }`.) The input is a user's
— an offset as deep as the body — so "a kernel defect worth reporting"
is false. The review lane measured the same on its vase; the line
seams presumably share it (unmeasured), and PR 4117 makes it newly
reachable on spline seams.

## Fix

`replace_face_offset` refuses an offset that collapses an untouched
edge before the attach door does, naming the edge and the offset (the
re-anchor knows `t_new == t_other`), with this refusal unit's text
conventions.
