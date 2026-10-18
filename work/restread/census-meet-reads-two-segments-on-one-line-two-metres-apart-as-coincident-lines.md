---
id: census-meet-reads-two-segments-on-one-line-two-metres-apart-as-coincident-lines
kind: issue
title: The edge-face cut's meet of two straight segments on one line, 2 m apart, escalates curve_pair_pose_offset: the level-lines pose reads each span's ends against the other's line (132 escalations left at ε = 1e-6, all pass 4)
status: closed
opened: 2026-10-18
priority: P1
cost: M
closed: 2026-10-18
---

## What

Measured by the edge-face lane fix pass (PR 5261, head after the
review fixes), on the near-tangent probe's default grid
(`crates/sweep/examples/near_tangent_census_probe.rs`).

After that PR, `curve_pair_pose_offset` still escalates 132 times at
ε = 1e-6 and 12 times at ε = 1e-12 (T3 lines; 0 at 1e-9). Every one
comes from pass 4. The PR's fix pass instrumented both census callers
of `geom_brep::curve_pair::meet`: `census::meetings`, behind
`edge_crossings`, logged 264 / 24 escalations of that row (each body is
validated twice). `census::curved::reads_meet` (pass 5′) logged none.

The poses are `w345 nt e0 a4 d±1e-7` and `vee300 nt e1 a3 d±1e-7`
(4 each per run), plus single ones at `w345 e1 a6`, `e1 a2`, `e0 a9`.

Witness: `w345 nt e0 a4 d1e-7`, ε = 1e-6, prism ∪ cube. The census edge
is `EdgeKey(21v1)`, from `(2.0, -0.53590, 1)` to
`(1.99936, -0.53573, 1)`, 6.7e-4 long. The face's straight boundary
edge is `EdgeKey(29v3)`, from `(0, 0, 1)` to `(-1.99936, 0.53572, 1)`.
They lie on one line through the origin and are about 2 m apart along
it. `meet` reads them as level lines, and its `Coincident` test reads
each span's two ends against the other's carrier
(`crates/geom-brep/src/curve_pair/pose.rs`, *Two lines*). An end
1.06e-6 off the other's LINE lands in band, so it escalates, though the
two segments are definitely apart. The reverse pair (29 against 21's
face) does the same.

The edge-face lane's shared-end skip (`edge_crossings`) does not apply,
because the two edges share no point.

## The shape to give

Read the two segments' gap before asking whether their lines coincide.
Segments whose projections on the common line are definitely apart
(or whose nearest points are) meet nowhere. That is the curve-pair
reader's own question, so the fix belongs in
`geom_brep::curve_pair::pose`, not at the census call site. It is the
segment-vs-line proxy of
`the-census-parallel-test-reads-two-edges-through-one-point-end-to-end-as-a-near-parallel-pair`,
inside `meet`.

## Closed

Closed by branch `restread/meet-apart-on-one-line`. The router in
`geom_brep::curve_pair::pose` (`route`) now reads the gap between the
two spans' projections on the first line (`curve_pair_pose_gap`) right
after the parallel argument and before the tilt or the offset:
projections apart by a gap that decides positive are `Pose::Apart`,
since a meeting's two points project within the zero band of each
other. The fix sits in the curve-pair reader, so every caller of
`meet` has it, and it covers both arms the router serves, two lines
and a line against a conic's extent. Pinned by
`meet_rows::two_segments_on_one_line_two_metres_apart_meet_nowhere`
(the census's own pair at `ε = 10⁻⁶`), and by
`a_gap_along_one_line_past_the_band_meets_nowhere` and
`a_line_far_along_from_a_circle_in_its_plane_meets_nowhere`. On the
near-tangent probe's grid, `curve_pair_*` escalations go from 132 / 0 /
12 to 0 / 0 / 0 at ε = 1e-6 / 1e-9 / 1e-12, and 97 and 12 `OK BAD`
outcomes read `OK SOUND`. The conic arms' sibling is filed on CPAIR
(`the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc`),
and check 9's on RESTFRONT
(`check-9-lines-meet-reads-each-end-against-the-other-line-before-the-overlap`).
