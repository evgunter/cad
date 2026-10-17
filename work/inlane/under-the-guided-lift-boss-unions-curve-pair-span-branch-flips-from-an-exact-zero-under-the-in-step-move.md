---
id: under-the-guided-lift-boss-unions-curve-pair-span-branch-flips-from-an-exact-zero-under-the-in-step-move
kind: issue
title: Under the guided lift, boss_union's curve_pair_span_branch margin is exactly zero at the nominal and pi at p1, so a 1e-7 in-step move flips it (#5749) and curve_pair_span_end_side decides another margin after it
status: closed
opened: 2026-10-17
priority: P2
cost: M
closed: 2026-10-17
---


Found by the shared replay session unit (`inlane/sym-replay-session`), guided lift, in-step move (every variable by the same relative 1e-7).

- **#5749 `curve_pair_span_branch`:** margin 0.0 exactly at p0, 3.141592653589793 at p1. The sign flips: a branch angle sits exactly on its cut at the nominal and jumps a half turn.
- **#5752 `curve_pair_span_end_side`:** p0 −0.7330382858376184, re-valued at p1 −1.8325958978536172, p1 build 0.3665191795707234. #5753: p0 1.4660765716752366, re-valued 2.565634256995064, p1 0.36651917957072355. After the flip the builds decide this predicate over different margins: two margins under one name, not a laundering.
- **Repro:** `REVALUE_DETAIL=1 REVALUE_DOC=boss_union cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

The gating row `revalue_boss_union` pins exactly this, through its `flips` entry. The flipping predicates are {`curve_pair_span_branch`, `curve_pair_span_end_side`}, and the predicate re-valued away after them is {`curve_pair_span_end_side`}. The row goes red if the flip goes, so the entry comes out with this fix, and also if any other decision flips.

**Owed:** decide `curve_pair_span_branch` away from its cut at a held configuration, by name, or at a margin that is not exactly zero at the nominal. Then take the entry out.

## Closed

**Root cause: a place read at an undefined foot.**
- **The point.** The boolean's role resolution reads a planar face for a point certified inside it (`topo::stands::planar_interior_point`). Its first candidate is the centroid of three consecutive loop vertices. boss_union has a disc whose rim is three 120° arcs, so that centroid is the circle's centre, on the axis of every arc.
- **The walk.** `point_in_loop_projected` reads it against each arc through `geom_brep::curve_pair::Span::read`, which placed the point's foot against the span's ends before asking whether the point is on the carrier at all.
- **The foot.** On a circle's axis every parameter is a foot, and `angle_near` returns the seed. At the nominal the foot sat exactly antipodal to the span's middle, so `curve_pair_span_branch` decided a half turn's cut from an exact zero. After a 1e-7 move the point sits a hair off the axis, its foot swings to the middle, and the branch and both `curve_pair_span_end_side` margins are other margins under the same names.

**The decision, certified.** The point is off the carrier by the radius, so its place decides nothing. `Span::read` now reads the carrier first, and a point the carrier reads off is off the span with no place read. No end can be within the band of a point the whole carrier is past the band from, so every reading's answer is unchanged; only the place's decisions go.
- **On the frame.** The carrier's recording still goes after the place's (`k_stats::detached`, then spliced), so a refusal of the carrier's is the last on the frame. At an end, which no carrier reading decides, its escalation is superseded.
- **Neither a band refusal nor a held decision.** It is not that the predicate should refuse in band, nor that it should take the coincidence by name: the predicate is not asked where its answer cannot matter.

**Rows.**
- `revalue_boss_union` compares 11,366 decisions with 6,472 moved, nothing laundered and no flip. Its `flips` entry is out, and its floor is 3,236 (half the moved, was 3,243).
- `revalue_corpus::holds` now holds flips exactly whether or not a row names any, so a row that names none owes none, and this flip coming back is red. That tightening turned `revalue_declared_tangency` red on its two exact-zero `pcurve_chart_azimuth_frame` flips, which nothing pinned. They are now owed under `under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop`.
- `geom_brep::curve_pair::span_rows::a_point_on_a_circles_axis_is_off_with_no_place_read` (probe) reads a circle's centre against a third of a turn at five seeds: `Off` every time, and no `curve_pair_span_*` sample. With main's order it records two branch, two end-chord and two end-side decisions at seed 0.

**Not the same class as plate_param's exact zero.** That one is `walk_schedule`'s ray-schedule gate, a margin zero because a schedule member is an axis-aligned face's normal. Its item says so.
