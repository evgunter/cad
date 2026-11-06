---
id: whole-body-side-reads-both-sides-at-a-pinch
kind: issue
title: whole_body_side reads vertices on both sides of the split plane at a pinch: production takes the first, so the side depends on vertex order
status: open
opened: 2026-11-06
priority: P1
cost: M
---


Found by `a-ladder-that-checks-its-answers-agree-has-no-home`
(`onecopy/refusal-standing`), wiring `geom_brep::ladder::climb_any` in as
the debug check on `Answering::Any` ladders.

## What

`splitting::finish::whole_body_side` (`crates/topo/src/splitting/finish.rs:2048`)
reads which side of the split plane a body that the plane does not cut
lies on. It climbs `Answering::Any` over the body's vertices (their
`PlaneSide`, `On` passing) and then its curved edges. Its premise: a
body the plane does not cut lies wholly on one side, so the first vertex
off the plane names it. **At a pinch that premise is false: the body's
vertices read on both sides.**

**Production takes the first answer** (`climb(Answering::Any, ..)`
stops at the first rung that answers), so the side the whole body is
filed on depends on the order of the body's vertices, not on the
geometry.

The site stays on plain `climb` with a pointer to this row
(`finish.rs`, above `let side = match climb(`): `climb_any`'s debug
check is off there, not loosened.

## The disagreement shape

Rungs are vertices (then curved edges). Under `climb_any` the debug run
printed **`Below` then `Above`** (twice over the suite). On
`census::split_wedge_tests::a_split_through_a_pinch_keeps_its_wedges_disjoint`
(the union of bricks `[0,1]×[0,1]×[0,1]` and `[1,2]×[1,2]×[0,1]`,
which meet along the line x = y = 1) a probe printed, in climb order:
vertex 1 `Below` (the answer), then vertices 4, 5 and 8 `Below`, and
**vertex 10 `Above`**. The split planes in that test all pass through
the pinch line (`across the line`, `along a diagonal through it`,
`along a face through it`), so the "uncut" body reaching
`whole_body_side` has vertices on both sides of the plane: either the
plane does cut it at the pinch and the cut was missed upstream, or the
pinch leaves a body the plane touches only along the line, and one side
is not the answer.

## The failing tests

With `climb_any` at `whole_body_side` (debug, topo `ci` profile):

- census::split_wedge_tests::a_split_through_a_pinch_keeps_its_wedges_disjoint

To reproduce: swap the site's `climb(Answering::Any, vertices.chain(edges), ..)`
for `geom_brep::ladder::climb_any(vertices.chain(edges), .., PartialEq::eq)`
and run that test.

## Done when

The body that reaches `whole_body_side` lies on one side (the cut at a
pinch is made upstream), and the site moves to `climb_any`; or a body
read on both sides refuses (`ladder::climb_agreeing`) and the split
takes the refusal.
