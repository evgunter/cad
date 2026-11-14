---
id: the-profile-drops-the-unit-directions-its-verbs-made
kind: issue
title: The profile drops the unit directions its verbs made, so a door recovers an arc's start direction by dividing two boxed points' difference by its own length
status: parked
opened: 2026-11-14
priority: P0
cost: M
design: true
refs: [6936]
blocked_on: [6936]
---


Split from
`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`
by its design weighing (fork 268, `[ev]` PR 6936). It holds the tour's
certified stop, and so retires the box-scalar switch with the others.

**What is wrong.** The circle verb builds its two vertices as
(cx ± r, cy) (`profile::path::circle_kernel`), so each arc's start
direction is exactly ±x by construction, and the path verbs hold every
line's director exactly (`profile::path::Dir`). Validation drops them:
`ValidatedSegment::Line` and `Arc2` store vertices, centre, radius and
sweep. A door that needs the direction recovers it as
`place(cx + r, cy) − place(cx, cy)` divided by its own length, which
over a box is not recognisably unit: the tour's two hole extrudes refuse
check 1, `SurfaceDatumUndecided { kind: Cylinder, datum: URef, measure:
Length, margin: Enclosure { -2.971e-4, 2.412e-4 } }`, in every one of
512 leaves (measured, switch off; `probe/M1-REPORT.md` on
`boxfinish/probe-switch-off`). Re-deriving the direction in 2-D instead
divides the same bracket by its own norm (inferred).

**What waits on Ev.** The profile format
(`crates/profile/README.md`, "Profile format") gains a recorded unit
direction per segment: a line's director, an arc's radial at each end.
That clause is ratified, so PR 6936 asks. Parked on it.

**Open in the spec:** one direction per arc or one per end (the
designers lean per end, so reversing a loop swaps them); on `Arc2`
(shared with `geom-brep`'s sketch segment) or on the segment; what a
table-authored profile records (decided once at validate); how
`ValidatedSegment::lift` carries it across scalars.

**The test that says it is closed.** With the switch's box arm off, the
tour's certified stop certifies its share (`demo-tour certified`,
`eps_regression::certified_cells_run_green_at_default_eps`), with the
sweep doors reading the recorded directions.

**Not measured:** whether `CurveDatumUndecided` stands beside the face
findings on the holes' rim circles (their `u_ref` is the same mint,
`swept::placed_segment_spec`); and whether the same refusal is live on
main today through a taker (a plate with its holes subtracted, over a
macroscopic box).
