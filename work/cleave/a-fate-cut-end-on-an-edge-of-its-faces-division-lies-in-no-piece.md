---
id: a-fate-cut-end-on-an-edge-of-its-faces-division-lies-in-no-piece
kind: issue
title: A fate cut's pierced corner on an edge of its kept face's division, at no vertex of it, refuses ClassificationInvariant
status: review
opened: 2026-11-01
priority: P1
cost: M
refs: [a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece, a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece, a-fate-cut-touch-end-on-an-edge-of-its-faces-division-is-unplaced, a-fate-cut-placement-limit-refuses-in-the-voice-of-a-kernel-bug]
branch: cleave/fate-cut-end-on-a-division-edge
pr: 6237
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

## Built (branch `cleave/fate-cut-end-on-a-division-edge`)

`fate::lies` now returns `Lies::OnEdge(sides)`, each piece with the
edge of its loops the point lies on, where it lay in no piece and at no
vertex (`Unplaced::OnEdge` is gone). The pierced corner's arm reads it
with `fate::piece_beside`: the piece whose edge's half-edge, read as a
straight corner at the point by `corner_holds` (the reading `place`
uses), holds the dropped sector's direction. Where no one piece holds
it (the direction along the edge, or an edge other than a line) it
refuses `fate::Limit::BesideNoPiece`, which `ops::fallback_fates` keeps
uncut and the section path refuses in `Limit`'s voice (the P2 voice
row). The touch placement's on-edge refusal is unchanged (its own row).

Measured at the site on main: the corner `(1, 1, 1)` or `(1, 0.5, 1)`
reads `OnEdge` in two pieces, through two coincident edges (a null
face between them), so the pieces are read by their own half-edges,
not one edge's two sides.

Pins (`a_keel_on_the_floor_subtracts_as_a_slit.rs`):
`a_fate_corner_inside_an_edge_of_its_faces_division_builds_at_rest`
(gate) and `…_builds` (slow: three lump pairs, both orders, every pose,
two sides, four footprints, `y ∖ x` and `y ∪ x`; 28 of each pose and
order's 48 cells reach the arm). Both check tier 3, 3′, the closed-form
volume and that each substituted fragment's kept face is in the result.
Red on main (the refusal) and under a mutant that takes the piece the
direction lies outside (the kept face named is one the result drops).

Outcome over the touch-end probe's grid (pt, base, s2, widened; 7728
cells per ε), main against head at default, 1e-6 and 1e-12: 104 cells
per ε refused→built (all this refusal: `pt S edge short` 80, `S partial
line` 12, `S wider in y` 12), all sound under the probe's checks; 0
built→refused, 0 →invalid, 0 cells reaching `BesideNoPiece`.

Filed: `a-fate-corner-inside-a-curved-edge-of-its-faces-division-refuses-as-a-placement-limit`
(P3), `a-sphere-cut-in-circle-across-a-cut-of-its-face-refuses-in-the-voice-of-a-kernel-bug`
(P3, the `ops.rs` cut-in voice noted above).
