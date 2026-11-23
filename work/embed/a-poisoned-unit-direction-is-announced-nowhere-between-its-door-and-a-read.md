---
id: a-poisoned-unit-direction-is-announced-nowhere-between-its-door-and-a-read
kind: issue
title: A poisoned UnitVec3 is announced nowhere between checked and a read: the embedding certificate's line readers refuse it one by one, by their own comparisons, and no other reader was audited
status: open
opened: 2026-11-23
priority: P3
cost: E
---


## Finding

From lane `embed/tangent-corner` (PR 7320, 2026-11-23).

`geom_core::UnitVec3::checked` answers a direction every component of
which is poison (NaN) where its measurement refutes the vector, in a
release build as in a debug one, and says nothing. At an enclosure
scalar that is any box whose bracket of `v·v − 1` does not hold zero,
which a point box of a rounded quotient usually is. Nothing between
that door and a read announces it: a `geom::Line` is built on it and
handed on.

## The trace

Main's PR 7349 migrated the test helper `segment(..)` in
`crates/topo/src/injective.rs` to state a line's direction as
`UnitVec3::checked(Vec3::new(iv0(dx / len), iv0(dy / len), iv0(0.0)))`.
For `the_graph_does_not_run_on_along_an_edge_that_turns_back_along_t`
(a chord at 45°) the rounded components' squares sum to
`1.0000000000000002`, the direction was `(NaN, NaN, NaN)`, and the row
failed "not presented: Local" once that migration reached the landing
branch. In `graph_read`'s run branch: `straight` returned the line's
chart ends as NaN; `no_other_copy` passed; `leaving` answered `None`
at `on_vertex`, whose `decide(SELF_INTERSECTION_CORNER_VERTEX)` does
not read NaN as `Zero`. A refusal, so nothing was accepted wrongly.

Fixed in the fixtures (that PR): every hand-normalised or literal
direction in that file's test modules is now `UnitVec3::chord` (an
enclosure built in the scalar's own arithmetic) or an `exact_x/y/z`.

## What was checked, by reading the comparisons

Each consumer of a line edge's evaluated ends or direction in
`injective.rs`'s production half refuses a poisoned line:

- `present_plane`, one edge: `y.is_certified()` before its `Vacuous`,
  then `interval_sign_as(inside, SELF_INTERSECTION_SPAN)`;
- `present_plane`, two edges: `interval_sign_as(lean, SELF_INTERSECTION_CORNER)`
  on both leans, `x.hi() <= 0.0 || y.hi() <= 0.0` (false on NaN), two
  more span decisions;
- `present_plane_corner`: `cell[..].is_certified()`, then
  `leaves_line`'s decisions;
- `leaving`: `on_vertex`;
- `line_onward`: `interval_sign_as(advance, SELF_INTERSECTION_COLUMN)`,
  one-sided comparisons of the far end, `share.is_finite()`,
  `Real::is_bounded(reach)`;
- `graph_of` / `graph_run`: `unit(..)?`, `bisector(..)?`, then
  `interval_sign_as(.., SELF_INTERSECTION_GRAPH)`.

**`no_other_copy`** deserves its own sentence. Its last test,
`meets_rect`, is false on NaN, which would read as "no other copy
meets the rectangle". It is not reached with a poisoned edge: on a
chart with no periodic axis there is no other copy to test (the pass
in the trace above is that, not NaN), and on a periodic one NaN ends
make `reach_of` return `(+∞, −∞)` (`f64::min` passes over NaN) and
`turn_ladder` refuses on `k0.is_finite()`. So it is safe by what
stands before `meets_rect`, not by `meets_rect`. A caller that reached
`meets_rect` with NaN and was not followed by a NaN-refusing decision
would accept.

## What was not checked

- No NaN was injected: the list above is a reading of the comparisons.
- No reader of a line's direction outside the embedding certificate
  (the boolean, the band, props, the pcurve mint) was audited.
- Test sites elsewhere that pass `T::from_f64(..)` components to
  `checked` at a generic scalar were not classified by instantiation;
  each is the same shape where `T` is `Interval`:
  `crates/sweep/src/blend/reach.rs` (about 3557),
  `crates/sweep/src/blend/surgery.rs` (about 6947),
  `crates/geom-brep/tests/iso_parse_incidence.rs` (370, 382),
  `crates/geom-brep/tests/closed_form_limb2.rs` (197).
- The 12 production `checked` sites
  (`editor-core/src/trace.rs`, `geom-brep/src/curve_face/pose.rs`,
  `geom-brep/src/intersect.rs` ×6, `geom-brep/src/pair_section/cones.rs`
  ×2, `topo/src/boolean/section_cert.rs`, `…/section_cert/ruling.rs`)
  each state a construction in the scalar's own arithmetic; that each
  caller upholds it was read at the site and not re-derived.

## What is needed

A decision, not a patch here: whether a poisoned direction is announced
at the door (a typed refusal, or a debug assertion at an enclosure
scalar), or each reader keeps refusing it by its own comparison, in
which case `no_other_copy`'s `meets_rect` wants a NaN-refusing spelling
so that it is safe on its own.
