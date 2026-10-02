---
id: a-concave-graze-of-a-curved-wall-refuses-for-reasons-other-than-its-knife-edge
kind: issue
title: a concave graze of a curved wall refuses as a corrupt body, a degenerate cone or a join invariant instead of its knife edge
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## What

A plane tangent to a hole's wall from inside has an honest refusal:
the hole's piece would meet the cut face along a knife edge the split
cannot declare, `SplitFinishError::SectionCusp`
(`wedge_end_doors::a_split_tangent_to_a_hole_wall_refuses_the_knife_edge_it_would_mint`).
Rule (b) sends the graze's entries across for that reason
(`splitting/rules.rs`, `wall_graze`). Only some concave grazes reach
that refusal. The rest refuse with a payload that names something
else, and one names the body corrupt.

Measured with the fixtures of
`crates/sweep/tests/split_tangent_edge_curved.rs`. Every row below
gives the same payload with and without the convex-graze change, so
none of them comes from it:

- Round hole (radius 0.5 in a 4 × 4 plate), grazed at its seam
  (x = 0.5), either normal: `Finish(SectionCusp)`. This is the honest
  refusal.
- The same hole, grazed along a ruling (y = 0.5):
  `Join(DegenerateSection)` with +y, and
  `Join(SectionInvariant { what: "tangent section chord endpoints
  coincide along the ruling" })` with −y.
- Conical socket (`a_concave_graze_of_a_cone_never_answers_with_the_hole_on_the_wrong_side`),
  plane tangent along a ruling:
  - u = ±z, s = +1: `Finish(Corrupt)`, "the finish traversal failed
    (corrupt body)", on a valid revolved operand;
  - u = +x (the seam), s = −1: `Join(Section { source:
    DegenerateOperand { what: "the cone's half-angle does not
    definitely open off its axis, …" } })`, on a cone of half-angle
    atan(1/2);
  - u = +x, s = +1 and u = −x, s = +1: `Join(DegenerateSection)`;
  - otherwise: `Join(SectionInvariant)`, as for the hole.

`Finish(Corrupt)` is the worst of these, because it blames the
operand. The half-angle text names a property the cone does not have.

## Where to look

The raising sites have not been traced. The section the plane makes
there is a ruling line through the apex's side of the cone, so the
section lane's apex branch is the first suspect for the half-angle
text. `Finish(Corrupt)` comes from one of `finish.rs`'s
`SplitFinishError::Corrupt` sites.

## Found by

`cleave/convex-graze`'s measurement of the concave guards.
