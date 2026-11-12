---
id: a-ridge-end-on-an-arc-wall-tangent-to-its-floor-refuses-unreadable
kind: issue
title: A ridge's ends on an arc wall tangent to its floor refuse Unreadable on a valid body
status: review
opened: 2026-11-12
priority: P1
cost: M
pr: 6878
branch: restread/self-touch-valid-bodies
---


Found by the remainder-refuses lane (PR 6854), which makes every
self-touch point the reading cannot decide refuse.

## What

`crates/sweep/tests/an_edge_on_a_ruling_lying_in_its_own_face.rs`,
`a_ridge_on_a_cylinders_ruling_resting_on_its_floor_passes_tier_3_prime`,
the case "an arc wall tangent to the floor". It fails at both offsets
(origin, and `(990, −990, 990)`), at default ε and at 1e-12.

- The ridge's two ends on the floor, `(1.5, 1, 0.5)` and
  `(1.5, 1, 2.5)` at the origin, each refuse
  `BoundaryCrossesItselfUndecided` with `why: Curved(Unreadable)`.
- Before PR 6854 the self-touch sweep (`sweep_self_touch_points`,
  `crates/topo/src/census.rs:8888`) skipped them. The edge-in-own-face
  pass reads the tip line itself, and it passes.
- The body is valid: the ridge's wedge lies outside the floor's
  material.
- The three other ridge walls (bowed out, bowed in, short arc) read
  `Rest` at the same points. So this refusal belongs to the wall that
  is tangent to the floor.
- **Where it refuses is not yet localised.** At each end the cylinder
  wall's tangent plane is the floor's plane along the tip ray. That is
  a first-order tie the reading sends one order down
  (`rest::first_order`, `crates/topo/src/link.rs:4649`;
  `second_order::parts`), or two tangent sectors on one plane that no
  lens or lune reads (`notes.coincident`,
  `crates/topo/src/link.rs:5023`).
- The row now pins the refusal, citing this file.

## The fix

Find which arm refuses at the ridge's end. Then read the end, whether as
a lune between the wall and the floor capped at the end cap's plane, or
one order down along the tip ray. When it lands, the pinned row goes
back to asserting pass.

## Read (branch `restread/self-touch-valid-bodies`)

**Which arm refused.** Neither first order nor `second_order::parts`:
the link passes `rest::first_order` (the tip's line edge lies in the
floor's plane as the faces keep it, and `parts` reads the arc's ray,
where the floor, the wall and the end cap part in the cut). It refused
in `rest::sectors_material`: the wall's tangent sector, a quarter of
the floor's plane, lies on the floor's whole plane the other way round,
and `second_order::lenses` found no pair (it paired a sheet only with
the first or last wedge of a sector that has bounds), so the probe read
two sectors on each other (`notes.coincident`).

**What the bodies hold.** At each end: the floor, a planar face holding
the point inside it; the cylinder wall's corner from the tip's line edge
to the cap's arc edge, tangent to the floor along the whole tip; the end
cap and the plane wall.

**The reading** (`second_order::along_a_line`, `Close::Lying`; the
argument is in `crates/topo/README.md` beside the lune's). The pair is a
lune whose one end is a line both faces hold, read Zero as a pinch of
two lines is, the plane running on past it, and whose other end is a
mouth, read decided. The remainder holds the floor less the wedge,
bounded by the wall's edges. No other sector may hold the line's ray
inside it.

**Still refused**, filed as
`a-sheet-on-a-whole-tangent-plane-is-read-only-along-a-line-on-a-planar-face`:
a sheet on a curved face's whole tangent plane, one with no line end or
two, and one wider than a wedge.

The tangent wall is back in
`a_ridge_on_a_cylinders_ruling_resting_on_its_floor_passes_tier_3_prime`,
asserting pass at both offsets;
`a_ridge_on_an_arc_wall_tangent_to_its_floor_refuses_at_its_ends` is
gone, from the slow set too.

**Measured** by PR 6854's instrument (key: test, `census::witness`,
verdict; topo's lib and `topo::all`, the nine sweep families and the two
rows above, default ε, slow set included), head `ce153ff56a` against its
merge base `f562166146`. The later commits change rows and one
equivalent spelling (the sheet held on the plane by
`rest::in_plane_of`).

- 495 triples at the base, 491 at the head.
- 5 triples refused at the base read `Rest` at the head: the shared apex
  (1, `NoTangentPlane`) and the ridge's two ends at both offsets (4,
  `Unreadable`). The 4 ridge triples key onto triples the merged test
  already holds (the other ridges' ends at the same points, `Rest`),
  which is the drop of 4.
- No other triple changes verdict, and the head reaches no point the
  base does not.
- Reds, the same three both sides, all on the known-main list:
  `offer_rows::every_offered_tolerance_passes_just_below_it`,
  `face_normal::tests::no_source_file_folds_the_bit_by_hand` and the
  needle cavity.
