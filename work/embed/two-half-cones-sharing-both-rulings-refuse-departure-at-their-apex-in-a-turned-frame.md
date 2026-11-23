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

From the second fix pass of PR 7115 (EMBED, lane pole, 2026-11-21 and
22), tracing what refuses in the review's turned-frame sweep
(`self_intersection_proves_injectivity::review_sweep_delta_shapes`)
once a cone's apex is placed and its meridians run on to it.

**Population.** Five cone tips, each on the plain sketch plane and on
three planes turned about a generic axis and moved (`turned_plane`),
each as a revolved body and as its axial-door cavity: 40 cone cases,
f64. The same sweep holds 12 ice-cream cones (a cone under a sphere
cap) on the plain and the first two turned planes, as body and cavity:
24 cases. Every plain-frame case holds on every main below, and so do
all 120 of the review's plain-frame sweep.

**Which turned cones refuse moves with main, with PR 7115's reading
unchanged.** Refusing cone cases of the 40, on PR 7115 over each main
(discharged / refused / splits):

| case | main `dc7059c445` | mains `94b6b284e`, `c3ffedeb2` (carrier frames in) | main `ae076e050` (PR 7120 in) |
|---|---|---|---|
| turned 1, `y0 = 0.3`, `(0.013, 0.9)`, apex down, cavity | 9,043 / 439 / 9,476 | 9,088 / 439 / 9,521 | 8,474 / 427 / 8,895 |
| turned 2, the same, cavity | `Budget(Pairs)`, 53,208 / 12,328 / 65,530 | `Budget(Pairs)`, 53,207 / 12,329 / 65,530 | `Budget(Pairs)`, 53,157 / 12,379 / 65,530 |
| turned 1, `y0 = 0`, `(20, 50)`, apex up, body | 39,304 / 17,479 / 56,777 | 37,763 / 12,271 / 50,028 | holds |
| turned 1, the same, cavity | 39,333 / 17,479 / 56,806 | 39,333 / 17,479 / 56,806 | holds |
| turned 2, `y0 = 101.3`, `(0.37, 2.3)`, apex up, body | 10,625 / 3,161 / 13,780 | 10,625 / 3,161 / 13,780 | holds |
| turned 2, the same, cavity | 10,652 / 3,161 / 13,807 | holds | holds |
| turned 3, `y0 = 0`, `(1, 1)`, apex up, body | holds | holds | 32,408 / 18,250 / 50,652 |
| turned 3, the same, cavity | holds | holds | 32,417 / 18,256 / 50,667 |
| turned 3, `y0 = 1.7`, `(1, 1)`, apex down, body | holds | holds | 32,186 / 18,248 / 50,428 |
| turned 3, the same, cavity | holds | holds | 32,045 / 18,244 / 50,283 |

Every refusal but the `Budget(Pairs)` one is
`Uncertified { Departure }` on the two half-cones across a shared
ruling. On main `ae076e050` the unit cone's and the slender cone's
verdicts on each turned plane are the same at ε 1e-9, 1e-6 and 1e-12;
the small cone's were read at the default ε alone.

On main `a3d5ce89c` (2026-11-22, 577 commits on) every case in the
last column reads the same, at the same receipt.

On the landing train (`origin/embed/wave3-land` at `f06547ccf`, merged
into PR 7115 on 2026-11-23) the same six cone cases refuse and no other
of the review's 68 turned cases does: the sixteen turned ice-creams
hold there, by the plane through the pair's one shared vertex
(`embed/apex-separator`), which does not part these half-cones. The
splits are unchanged; the receipts count one refusal, since the ask
ends at its first finding (turned 1: 6,567 / 1 / 6,681; turned 2:
`Budget(Pairs)`, 47,039 / 1 / 65,530; turned 3: 25,153 / 1 / 50,652,
25,154 / 1 / 50,667, 24,925 / 1 / 50,428, 24,795 / 1 / 50,283). The far
probe's one refuser is still `Budget(Depth { 40 })`, at 1,040 / 1 / 1,065.
The turned-3 unit cone, which held until PR 7120, has a row of its own:
`work/embedwalk/the-unit-cone-on-a-turned-plane-held-until-the-windows-came-from-covers-and-refuses-departure-since.md`.

**The verdict moved with ε too, on an earlier main.** On `c3ffedeb2`
the unit cone on the first turned plane held at the default ε and at
1e-12 after 10,507 splits and refused at 1e-6
(`Uncertified { 1v1, 2v1, Edge(4v1), Departure }`,
10,633 / 41 / 10,668, exit B below in 6,190 cell pairs). On `ae076e050` it
holds at all three, in about 5,200 splits. The plain-frame unit cone
holds in 173.

**Not this row's: the ice-cream cones and turned balls.** On
`dc7059c445` two ice-cream cases refused (turned 2, `y0 = -2.9`, body
and cavity), on their CONE's halves, exits B and C below. Since main's
carrier frames, all 16 turned ice-creams and every turned ball refuse
on their SPHERE cap's halves (`1v1`, `2v1`, 4 to 6 cells refused),
which is
`a-balls-seam-meridians-in-one-bracket-are-not-one-plane-so-its-pole-refuses-at-interval`.
They held with PR 7115 before the carrier frames; with main's own
`injective.rs` they run out of pairs, the pole unplaced.

Each refuser's apex is placed in every cell that holds it: the apex's
collapsed side is among the clips, and the extents end at `0`.

## Trace

The pair is always the two half-cones, which share BOTH rulings and
the apex vertex. The cells are at the apex. Counts are of one run.

- **A. `zero_of` reads `Local` at the apex.** Its last arm, with the
  cell's clips the face's two meridians, `XLo` and `XHi`, both shared,
  with or without the apex's side: 64,393 `Local`s on the turned-1
  `(20, 50)` body (`dc7059c445`), 37,049 of them with the two meridians
  alone. A cell at the apex is small in space while it spans the
  half-cone's whole `u`, and it is not split in `u`. On the turned-3
  unit cone (`ae076e050`): 58,517 at the last arm and 32,332 at the arm
  where a shared vertex is no one corner of two clips.
- **B. The plane through both rulings signs one face only.**
  `departure` reaches its last `split` with both faces presented, one
  by its ruling's side and the other by the corner of its ruling and
  the apex's side. `planes_through` gives the one plane through the two
  rulings (`n = d × e`), and `sign_on` reads `Some(Negative)` on the
  first face and `None` on the second. 9,547 times on the turned-2
  `y0 = 101.3` body (`dc7059c445`), down to cells `6.8e-4` by `1.5e-3`.
- **C. Two rulings near and no vertex.** In 6,707 cell pairs of the
  same run both rulings are in the shared cell and the apex vertex is
  not, both faces are presented by a side, and `planes_through` has no
  plane: its two-line arm needs a shared vertex in the cell.
- **D. `vacuous` reads `Local`.** On the turned-2 small cone's cavity
  35,823 `Local`s are `present_chart`'s "outside every near line and
  not vacuous" arm, which is
  `vacuous-reads-a-cell-at-a-corner-local-at-every-depth`.

Traced cases: the turned-1 `(20, 50)` body (A); the turned-1 small
cavity (A, B, C); the turned-2 `y0 = 101.3` body (B, C); the turned-2
small cavity (D, then A, B, C); the turned-2 ice-cream's cone (B, C);
the turned-1 unit cone at ε 1e-6 (B, A); the turned-3 unit cone (A).
The cavities beside a traced body are not traced.

**Against the plain frame.** The `y0 = 101.3` cavity in the plain frame
holds at 367 / 0 / 361. Its trace shows B's reading 34 times, resolved
by a smaller cell, and C never.

**A fifth exit, the same on main: `Budget(Depth { 40 })`.** The
review's `review_probe_far_witnesses` puts four cone tips a hundred
units off the origin, along the axis, across it, and on a plane turned
and moved (100, -70, 30), each as a body and a cavity: 24 cases at
f64. On main `ae076e050` with PR 7115, 23 hold. The slender cone's
(`r = 0.37`, `h = 2.3`, apex up) cavity on the turned and moved plane
refuses `Budget(Depth { max_cell_depth: 40 })` at 3,095 / 486 / 3,575,
which is none of A to D, and it refuses identically with main's own
`injective.rs`. Its exit is not traced. (With main's own code 17 of
the 24 hold; why each of the other six refuses there is not traced.) At ten
thousand units the revolve itself refuses every case,
`OutsideSessionBox`.

**Not traced:** why a turn, or a change elsewhere in main, moves a cone
between holding and refusing. The two rulings' stored frames are
rounded in a turned frame, and the half-cones' shared meridian is
stored a rounding apart on the two faces; whether that is the cause is
not measured. PR 7120 changed how a face's boundary box is built, which
changes which cells are near which edges.

## What is needed

A reading of two faces that share two straight edges through one
singular vertex, which holds in any frame and does not turn on which
cells the walk happens to visit. A plain cone that refuses is a body
the at-rest gate would refuse at its door, hence P0.

Pinned: `the_unit_cone_on_the_third_turned_plane_refuses_departure_at_its_apex`
(a refuser on `ae076e050`), and the cones that hold there in
`singular_points_hold_far_from_the_origin_and_in_turned_frames`. A pin
that goes red on a later main is this row's evidence, not a regression
of the pole and apex reading.
