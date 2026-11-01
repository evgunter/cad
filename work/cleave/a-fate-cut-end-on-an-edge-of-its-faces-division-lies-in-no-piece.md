---
id: a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece
kind: issue
title: A fate cut's pierced corner on an edge of its kept face's division, at no vertex of it, refuses ClassificationInvariant
status: open
opened: 2026-11-01
priority: P1
cost: M
refs: [a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece, a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece, a-fate-cut-touch-end-on-an-edge-of-its-faces-division-is-unplaced, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug]
---


Found by the sweep of PR 6185 (`cleave/fate-touch-end-at-a-division-vertex`),
the blind spot of `fate::lies`' two callers that its own fix leaves:
the same on main `336465c32b` and on the branch.

## What

`x` is the box `[0, 1]³` ∪ the box `[1, 2] × [0.5, 1.5] × [1, 2]`,
touching along `x = 1, z = 1, y ∈ [0.5, 1]` (an S cut short, so the
second lump's corner `(1, 0.5, 1)` lies inside the box's top edge
`x = 1, z = 1`, and the box's corner `(1, 1, 1)` inside the second
lump's bottom edge), either union order. `y` is a box over `[1, 1.5]`
(above) or `[0.5, 1]` (below), flush with a face of each lump on
`z = 1`. With `y`'s footprint `[−0.5, 2.5]²`, `y − x` and `y ∪ x` both
refuse, above and below; with `[−0.5, 2.5] × [0.5, 1.5]` or
`[0.5, 1.5]²`, `y − x` refuses above and `y ∪ x` below. That is 16
cells per pose (8 per union order), at every pose of
`test_support::meeting::poses`, at ε default, 1e-6 and 1e-12. On main
the refusal reads `ClassificationInvariant` "a fate cut's end lies
inside no one piece of its face"; since PR 6185 it reads "a fate
cut's end lies on an edge of its face's division", the state it
measured (`fate::Unplaced::OnEdge`).

The same, 4 cells per pose each (one op per partner, both union
orders), at rest, turned and flipped and turned, at all three ε, with
the S `[1, 2] × [0.3, 0.7] × [1, 2]` (its whole bottom edge inside the
box's top edge; `y ∪ x` above, `y − x` below) and `[1, 2] × [0, 2] ×
[1, 2]` (the box's top edge inside its bottom edge; `y − x` above,
`y ∪ x` below), the partner `[−0.5, 2.5] × [−0.5, 1.5]` over `[1, 1.5]`
or `[0.5, 1]`.

## Cause

The refusing site is the fate cut's pierced corner (`fate::cut`, the
`KeptAt::Pierced` arm with no pierce ring, `Lies::Nowhere`). Measured
there: the point lies inside none of the kept face's pieces and at no
vertex of their loops, but on an edge of two of them (`contfp` reads
`OnEdge` in both). `lies` places a point inside one piece or at
division vertices (`piece_holding` reads the latter by direction); a
point inside an edge of the division has no reading.

## Owed

Read the piece whose side of the division's edge holds the dropped
sector's direction, or refuse typed.

Where the fix refuses, it refuses as a placement limit, not as
`ClassificationInvariant`: a `fate::Limit` (beside `NoCorner`), which
the containment fallback already keeps uncut and records
(`ops::fallback_fates`) and which
`a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug`
(P2) types on the section path. That row covers `Limit` only; the
on-edge refusal is `Unplaced::OnEdge`, outside it, so moving it under
`Limit` is this row's change. `Unplaced::InSeveral` and `Unplaced::Off`
stay `ClassificationInvariant`: those are broken invariants.

The same voice for a placement the reading does not part:
`boolean/ops.rs` "cut-in: the section circle lies in no one piece of
its face" (`ops.rs:7397`), a sphere-section circle across a cut of its
face, refused as `ClassificationInvariant`.

The touching end's on-edge case is a separate unit (it splits the
division's edge rather than reading a piece):
`a-fate-cut-touch-end-on-an-edge-of-its-faces-division-is-unplaced`.
