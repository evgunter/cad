---
id: a-vertex-minted-on-an-edge-lying-on-a-curved-face-is-unread
kind: issue
title: A vertex a split mints on a conic lying in a plane is not read against that face
status: open
opened: 2026-10-15
priority: P2
cost: M
refs: [a-prism-crossing-a-face-along-a-contact-line-refuses-vertex-read-twice]
---

Raised in review of PR 5147 (m2, then m1 of the re-review).

`reduce::sweep_direction` reads an edge lying in a face at the edge's
ends only. When a later face splits that edge, the vertex the split
mints has to be read against that face too, or the two operand slots
disagree about it. PR 5147's witness was a line edge in a planar face,
split by a later face: one slot refused `VertexReadTwice`, the other
`Join(UnpairedLooseEnds)`.

The sweep's worklist entry (`Pending`, `reduce.rs`) carries the planar
faces the line lane's `(Zero, Zero)` arm read an edge in, and every
split reads its minted vertex against them (`read_then_requeue`). Two
kinds of edge lying in a face are not carried.

## An edge lying on a curved face: reached, refuses by slot

`curved_face_arm` takes such an edge by its ends only, in:

- the covered `(Sign::Zero, Sign::Zero)` arm, both ends on the carrier
  under a one-sided cover;
- the undeclared `(Sign::Zero, Sign::Zero)` arm's `SpanVerdict::LiesOn`
  (a conic arc on the carrier) and `SpanVerdict::Constant` (a ruling on
  a wall).

**Witness** (PR 5147's review lane, a sweep-crate probe, not committed;
the same on main and on the PR's head):

- the line body is the cylinder r = 1, z 0–2, unioned (cylinder first)
  with a wedge prism whose apex edge rests on the wall's ruling at
  (1, 0), z 0.5–1.5;
- the tool is a wedge prism with its apex edge on that ruling,
  z 0.2–1.8.
- **60°–120° tool:** line body first refuses `VertexReadTwice { reads:
  [Pierce(cylinder face), Pair(..)] }`; tool first refuses
  `Join(BooleanJoin(UnpairedLooseEnds { count: 2 }))`. That is PR 5147's
  slot-asymmetry signature, on the curved arm.
- **40°–80° tool:** line body first refuses `PartnerAlongFace { along:
  Curved }`; tool first builds, at the closed form (volume 6.434744 =
  cylinder + piece + tool), tier 3 clean.

Nothing builds wrong; the cost is an order-dependent refusal where one
order builds sound. Reading the minted vertex there means asking
`vertex_on_curved_face` and combining its `Placement` under that arm's
rule (`Placement::declared` or `Placement::undeclared_no_interior`), a
placement question with a frontier, not the planar side-then-
`vertex_on_face` read `read_lying_in` does.

## A conic lying in a plane: carrying it regresses

The conic lane's `ConicPlaneMeet::Parallel` arm takes a conic lying in
a face's plane by its ends only, as the line lane's coplanar arm does.
Recording that face in `lying_in` (PR 5147's fix pass did, at head
`7d34c7bd`) turned operations that build sound into refusals:
`sweep`'s `a_rim_in_a_face_beside_a_resting_ridge_answers_every_op_as_before`
pins the grid (a block ∪ a ridge resting on its top, against a cylinder
whose rim lies in the top). With the face carried, 8 cells that build
at their closed forms refused `PartnerAlongFace` or `VertexReadTwice`.
So it is not carried; whether a conic's split vertex needs reading
there, and how without the regression, is open.

## Done when

The curved witness above is a committed row, and both slots answer
alike in every op: the minted vertex is read against the curved face,
or the refusal is the same in both slots. The conic case is settled the
same way, with the ridge grid still building where it builds.

## The curved half, done (2026-10-15)

Branch `tang/minted-vertex-on-curved-face`. The sweep's worklist entry
carries each face an edge was read lying in, ends only, as
`reduce::LyingIn`: a plane (`Plane`), or a curved face with the rule its
arm read the edge under (`Curved(face, OnCurved)`). That rule is
`OnCurved::Declared { interior_clear }` from the covered `(Zero, Zero)`
arm, and `OnCurved::Undeclared` from the undeclared arm's `LiesOn` /
`Constant`. `read_lying_in` reads a minted vertex against a curved face
by the arm's own side question, then `vertex_on_curved_face`, and
combines the placement under that rule (`Placement::declared`, or
`Placement::undeclared_no_interior`). A placement the rule cannot
combine keeps the arm's frontier (`CurvedPierceUnsupported`).

The witness is committed as
`crates/sweep/tests/a_minted_vertex_on_a_curved_face.rs`'s
`a_wedge_on_the_ruling_answers_every_op_in_both_slots`. Its piece rests
outside the wall (−80°..−40°), with tools at 60°..120° and 40°..80°.
Every op builds in both slots at its closed form, tier 3 clean. On main
the tool-first slot refuses `Join(UnpairedLooseEnds)` at 60°..120°, and
`ClassificationInvariant` in the 40°..80° union.

The item's original line-first signatures, `VertexReadTwice` at
60°..120° and `PartnerAlongFace { along: Curved }` at 40°..80°, were not
reproduced on main in PR 5218's review over 102 probe poses. The
original probe was never committed. So the curved half's closure rests
on the committed substitute witness above: the same cylinder, ruling and
tool angles, with a resting piece, red on main with the item's tool-first
signature (`Join(UnpairedLooseEnds)`).

What is left in this item is the conic half below.

## The conic half, as it stands

`reduce::sweep_direction`'s conic lane, `ConicPlaneMeet::Parallel` arm
(`crates/topo/src/boolean/reduce.rs`, the arm commented "The conic's
plane is parallel to the face's"), still takes a conic lying in a
face's plane by its ends only. `LyingIn`'s doc says why it is not
recorded. Carrying it moves the ridge grid
(`crates/sweep/tests/a_rim_in_a_face_beside_a_resting_ridge.rs`,
`a_rim_in_a_face_beside_a_resting_ridge_answers_every_op_as_before`),
whose slot-asymmetric refusals this half is about. The grid did not
move with the curved half.
