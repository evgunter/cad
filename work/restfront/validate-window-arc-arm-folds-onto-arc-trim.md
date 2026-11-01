---
id: validate-window-arc-arm-folds-onto-arc-trim
kind: issue
title: validate::window's arc arm and splitting::containment::arc_trim are two homes of one construction (end distances, then a chordal-defect sum): fold check 9's onto arc_trim
status: closed
opened: 2026-09-26
closed: 2026-09-28
priority: P3
cost: E
---


Filed by CONTACT-4 (review MINOR-3).

`validate::window`'s `MeetSegment::Arc` arm (check 9's arm 5, from
ATREST-11) decides whether a point on a circle lies on an arc in two
steps:
1. the distance to either end (`ring_outer_arc_end`);
2. the chordal-defect sum `(|a − m| − |p − m|) + (|p − m′| − |a − m′|)`
   (`ring_outer_arc_trim`).

It works in metres, on a circle.

`splitting::containment::arc_trim` is the same two steps in a conic's
unit coordinates, levered by the smaller semi-axis. It serves:
- the carrier walk's boundary pre-pass;
- `contain`'s boundary pre-pass, through `ConicArc::hit`;
- ellipses as well as circles.

So there are two homes for one construction. The argument for why the
margin is uncompressed is written out in `arc_trim`'s doc; `window`'s
doc states it in outline. A change to one is a change to both, and
nothing enforces that.

**Repair:** give `validate::window` a `ConicRows`-shaped pair of its own
names and call `arc_trim` in the circle's unit frame, with `lever =
radius`. Its `Window::{In, Out, Unsure}` maps from `ArcTrim::{End | On,
Off}` and the escalation. ATREST-12, which moves check 9 onto the
carrier walk, is the natural taker.

## Closed by CONTACT-4's reconciliation with ATREST-12

ATREST-12 (#3325) made `splitting::containment::arc_trim` the one home
of the distance trim, and check 9's arm 5 calls it (`RING_OUTER_ARC_TRIM`).
In the reconciliation, CONTACT-4 deleted its second copy and reads the
same function:
- a circle edge's boundary trim;
- a ray's crossing trim.

An ellipse's boundary reads its ends as exact distances and then only
`arc_trim`'s step 2, factored out as `arc_trim_margin` so it keeps one
body. Nothing is left to fold.
