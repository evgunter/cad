---
id: a-cap-slivers-rim-arcs-whole-circle-arms-have-no-cap-row
kind: issue
title: blend: the cap sliver's reach and floors rest on its rim arcs' whole-circle arms (CircleFrame far/low), which no cap row reaches
status: open
priority: P3
cost: M
opened: 2026-10-14
---


## Finding

Found by PR 5110's review. Once the end curve's own terms were dropped
from `CapSliver::of` (`crates/sweep/src/blend/open/end_face.rs`, the
`extent` closure; proof in `CapSliver`'s doc), `reach` and the floors
come only from `piece_distance(..).far` and `piece_along(..).low` over
the two rim pieces. On a rim arc whose window holds its circle's
critical point, those values come from `CircleFrame`'s whole-circle
arms. With every arm forced to its window's ends, scoped to `of`, all
578 cap rows stay green. A log over 1,164 rows saw 356 circle and 20
ellipse rims, and none had a whole-circle-versus-ends gap above 1e-9.
The 20 ellipse gaps are `piece_distance`'s loose `d + major`, not an arm.
This predates PR 5110.

The `near` and `low` arms likewise have no cap row (`near` was disclosed
by PR 5110's lane; `low` is read only in `of`). The unit row
`a_pieces_extremes_are_its_ends_unless_it_holds_the_critical_point`
stays the only pin.

## What the taker owes

An assembly row where a rim arc's window holds its circle's farthest
point from the section centre, or its lowest point along a floor
direction, so the whole-circle arm binds: a rim piece from foot to old
vertex spanning more than π, or one whose arc bulges outward past the
section. Or a proof at `of`, for each arm, that it cannot bind through
any door, and the arm dropped there.
