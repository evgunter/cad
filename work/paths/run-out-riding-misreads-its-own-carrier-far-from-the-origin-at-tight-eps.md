---
id: run-out-riding-misreads-its-own-carrier-far-from-the-origin-at-tight-eps
kind: issue
title: PendingRunOut::rides reads a segment on the fillet's own arrival carrier as off it, far from the origin at eps 1e-12
status: open
opened: 2026-09-29
---


## Finding

- **Where**: `crates/profile/src/path.rs` — `PendingRunOut::rides`
  (~:2272), which `Core::attribute` asks whether an emission is a
  pending fillet's run out.
- **What**: at `CAD_TOLERANCE_EPS=1e-12`, the scenes of
  `enclose_refusal_r2_probes::p3_no_emitted_fillet_swallows_a_carrier_off_the_prs_grid`
  (an `Open.arc_fillet_arc` corner at `(85000, 40000)` and the other
  off-grid scenes) emit the fused verb's own authored arrival arc and
  `rides` answers "not on the arrival circle". The record then named
  that arc `{step 0, Leg}`, a role `arc_fillet_arc` never draws; the
  role-list check added with `work/emit/python-spells-a-piece-by-its-authoring-calls-step-handle.md`
  caught it. That PR names a fused step's own unclaimed emission its
  `RunOut` structurally, so that case no longer asks `rides`.
- **What is left**: the radial misses `rides` decides are rounded at
  ε·|coordinate|, and they are decided against `linear_band(tol)`,
  which does not scale with the coordinates. A LATER step's segment
  riding a plain `fillet(r)`'s run out (a straight arrival, the `Ray`
  arm) is decided the same way, so far from the origin at a tight ε it
  can be named that step's `Leg` instead of the fillet's `RunOut`. Both
  roles are on their verbs' lists, so no check catches it, and the name
  moves. Not reproduced for the ray arm; this is the mechanism, read
  off the code.
- **Importance**: low (tight-ε rows, large coordinates), but it moves a
  durable name.
