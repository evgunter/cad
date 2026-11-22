---
id: two-half-cones-sharing-both-rulings-refuse-departure-at-their-apex-in-a-turned-frame
kind: issue
title: Two half-cones that share both rulings refuse Departure at their apex once the cone is built in a turned frame: a cell holds both rulings as zero sides, or the plane through both signs one face only
status: open
opened: 2026-11-21
priority: P0
cost: M
---


## Finding

From the second fix pass of PR 7115 (EMBED, lane pole, 2026-11-21),
tracing what still refuses in the review's delta sweep
(`self_intersection_proves_injectivity::review_sweep_delta_shapes`)
once the apex is placed and a meridian runs on to it.

**Population.** 20 revolved cones in four frames (the plain one and
three rigid turns with translations), each as a body and as its
axial-door cavity, f64, and 12 ice-cream cones (a cone under a sphere
cap) in three frames. On PR 7115's final head:
- every plain-frame case holds, and so do all 120 of the review's
  plain-frame sweep;
- **eight turned-frame cases refuse.** Seven are
  `Uncertified { Departure }` on the two half-cones across a shared
  ruling; one is `Budget(Pairs)`.

| case (discharged / refused / splits) | exit |
|---|---|
| turned 1, `y0 = 0`, `(20, 50)`, apex up, body: 39,304 / 17,479 / 56,777 | A |
| turned 1, the same, cavity: 39,333 / 17,479 / 56,806 | not traced; the body's receipt |
| turned 1, `y0 = 0.3`, `(0.013, 0.9)`, apex down, cavity: 9,043 / 439 / 9,476 | A, B and C |
| turned 2, `y0 = 101.3`, `(0.37, 2.3)`, apex up, body: 10,625 / 3,161 / 13,780 | B and C |
| turned 2, the same, cavity: 10,652 / 3,161 / 13,807 | not traced; the body's receipt |
| turned 2, ice-cream `y0 = -2.9`, `(0.37, 2.3)`, body: 10,985 / 3,161 / 14,136 | B and C, on its cone's halves |
| turned 2, the same, cavity: 11,145 / 3,161 / 14,296 | not traced |
| turned 2, `y0 = 0.3`, `(0.013, 0.9)`, apex down, cavity: `Budget(Pairs)`, 53,208 / 12,328 / 65,530 | D, then A, B and C |

**The verdict moves with ε.** The unit cone tip on the first turned
plane (`(0, 0) (1, 0) (0, 1)` revolved) holds at the default ε and at
1e-12, after 10,592 discharged and 10,586 splits, and refuses at 1e-6:
`Uncertified { 1v1, 2v1, Edge(4v1), Departure }`, 10,712 / 41 / 10,747.
On the second and third planes it holds at 1,747 and 1,921 splits. The
plain-frame unit cone holds in 151. At 1e-6 its exit is B (4,604 cell
pairs with the other half-cone at a corner and 1,586 at a side, the
plane signing one half-cone only), with A's `zero_of` Locals beside it
(2,838). Pinned at each ε by
`the_unit_cone_on_the_first_turned_plane_holds_or_refuses_by_eps`.

Each refuser's apex is placed in every cell that holds it: the apex's
collapsed side is among the clips, and the extents end at `0`.

## Trace

The pair is always the two half-cones, which share BOTH rulings and
the apex vertex. The cells are at the apex. Counts are of one run.

- **A. Both rulings are zero sides of one cell.** `zero_of`
  (`crates/topo/src/injective.rs`) returns `Local` from its last arm:
  the cell's clips are the face's two meridians, `XLo` and `XHi`, both
  shared, with or without the apex's side. 64,393 `Local`s on the
  turned `(20, 50)` body, 37,049 of them with the two meridians alone.
  A cell at the apex is small in space while it spans the half-cone's
  whole `u`, and it is not split in `u`.
- **B. The plane through both rulings signs one face only.**
  `departure` reaches its last `split` with both faces presented, one
  by its ruling's side and the other by the corner of its ruling and
  the apex's side. `planes_through` gives the one plane through the two
  rulings (`n = d × e`), and `sign_on` reads `Some(Negative)` on the
  first face and `None` on the second. 9,547 times on the turned
  `y0 = 101.3` body, down to cells `6.8e-4` by `1.5e-3`.
- **C. Two rulings near and no vertex.** In 6,707 cell pairs of the
  same run both rulings are in the shared cell and the apex vertex is
  not, both faces are presented by a side, and `planes_through` has no
  plane: its two-line arm needs a shared vertex in the cell.
- **D. `vacuous` reads `Local`.** On the turned small cone's cavity
  35,823 `Local`s are `present_chart`'s "outside every near line and
  not vacuous" arm, which is
  `vacuous-reads-a-cell-at-a-corner-local-at-every-depth`.

**Against the plain frame.** The same `y0 = 101.3` cavity in the plain
frame holds at 367 / 0 / 361. Its trace shows B's reading 34 times,
resolved by a smaller cell, and C never.

**Not traced:** why a turn makes B persist and C appear. The two
rulings' stored frames are rounded in a turned frame, and the half-cones'
shared meridian is stored a rounding apart on the two faces
(`u = -8.7e-17` on one, `2π` less a rounding on the other); whether
that is the cause is not measured.

## What is needed

A reading of two faces that share two straight edges through one
singular vertex, which holds in any frame. A plain cone that refuses
is a body the at-rest gate would refuse at its door, hence P0.

The sphere's twin, two lunes sharing both halves of a great circle at
a pole, holds in the frames measured (the ice-cream's cap is not the
refusing pair), by `planes_through`'s conic arm.
