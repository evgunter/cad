---
id: share-nothing-face-pairs-are-halved-over-their-carriers-crossing-off-the-faces
kind: issue
title: Two faces of one solid that share no cell, centimetres apart, are halved to the depth floor over cells that hold their carriers' crossing and no point of a face, because the sphere and torus region readings read no face a spline edge bounds
status: closed
opened: 2026-11-22
priority: P0
cost: M
refs: [a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing, nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance]
branch: embedwalk/share-nothing-region
pr: 7772
closed: 2026-11-25
---


Traced and measured by the gate lane (`embed/gate-measure`, 2026-11-22),
for the refusing-ask row's evidence. No fix.

## What it costs

On the four tests behind the census's dearest refusing asks
(`germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint`,
`germ_torus_components::a_ball_round_the_donuts_axis_passes_the_germ_mint`,
`quadric_pair_sections::a_ball_and_a_rod_against_a_donut_build_on_periodic_loops`,
`germ_sphere_torus_placements::a_ball_against_a_donut_carries_onto_the_torus_and_builds`),
there are 11 refusing asks, costing about 3,470 s:
- 79.5% of the walk's 647,151 straddling distance splits, and 76% of
  the refusing face pairs' seconds, are on pairs of faces of one solid
  that share no cell of M;
- the other 20.5% descend from a touching pair whose local certificate
  split. Every straddle between faces sharing a cell is one of these;
  no straddle between faces sharing nothing is.

These are valid bodies the tests pin as built (below), refused at their
door by the gate, so the row is P0.

## Do the faces touch? No: they are 1.4 to 5.5 cm apart

`CAD_GATE_MINDIST` runs, for the three face pairs the box trace below
follows, a best-first branch and bound over the two faces' windows:
- **lower end:** the cells' enclosures, dropping each cell the walk's
  own region reader decides outside;
- **upper end:** the closest stations the point door places on the
  faces, and the faces' boundary arcs sampled (2,000 points an arc).

| face pair | lower end | upper end | the point door over a 49 × 49 grid |
|---|---:|---:|---|
| torus `1v3` × sphere `2v1` (`quadric_pair_sections`) | **1.38e-2 m** (60,000 pops, 24,470 torus cells dropped) | 5.45e-2 m (boundary to boundary) | torus: 1,128 in, 1,127 out; sphere: 2,169 of 2,401 `Trim(None)`, the rest on an edge or a vertex |
| torus `1v3` × sphere `5v1` (same) | the same to the digit: the same geometry, in the test's other result body | | |
| torus `5v3` × torus `6v3` (`two_donuts_in_general_pose`, placement 1) | 0: no reading of the engine's gives one | 4.46e-2 m (boundary to boundary) | 2,256 and 2,298 of 2,401 `Trim(None)`, the rest on an edge or a vertex |

- **Torus × sphere: certified embedded.** The lower end is certified,
  and it is six orders of magnitude above the band (1e-8). No cell of the
  sphere face is ever dropped, so the bound holds against the sphere's
  whole window patch, not just its face.
- **Torus × torus: embedded on an independent placement.** The engine
  cannot certify this pair, because the point door reads neither face.
  A union keeps each operand's boundary only where it lies outside the
  other operand. So face `5v3` lies on placement 1's carrier, outside the
  donut A; and face `6v3` lies on A's carrier, outside placement 1.
  - The two can meet only on the carriers' section curve, where each
    would have a boundary point.
  - Their boundaries are 4.46e-2 m apart, sampled.
  - This argument is not a certificate, but nothing here points to
    contact.
- **So no crossing is owed.** The verdict owed on all three is
  `Embedded`, which is what the tests assert. `quadric_pair_sections`
  asserts that A ∪ B and A ∩ B both build, the mesh then stopping at
  the faces' projected spline images. `two_donuts_in_general_pose`
  asserts:
  - placement 2 builds;
  - placements 1 and 3 pass the result gate and stop later, at the
    interior-loop guard.

## Where the 0.14 to 0.54 mm comes from: the carriers, off the faces

The box trace's closest sampled points are carrier points in the walk's
deep cells. The hook places them at depth 26 (`min-implicit.py` places
them against the tests' own operands):
- **Torus × torus:**
  - The `5v3` cell's point lies on placement 1's carrier but
    1.43e-4 m inside donut A's solid, so it is on no face of the union.
    It is 0.159 m from `5v3`'s boundary.
  - The `6v3` cell's point lies on A's carrier, 9.0e-5 m outside
    placement 1. It is 1.0e-4 m from `6v3`'s boundary, a point of
    `6v3` by its section edge.
  - The cell pair holds the carriers' crossing, the section curve.
- **Torus × sphere:**
  - The torus cell's point is placed `Out` of `1v3` by the point door,
    1.6e-2 m from `1v3`'s boundary.
  - The sphere cell's point has no verdict, 0.317 m from `2v1`'s
    boundary.

In both, the two carriers cross within the cell pair, 63.8° and 62.7°
between their normals. So the separation's lower end of 0 is the
carriers' truth, and halving the cells moves toward that crossing, never
away from it. The walk keeps these cells because the region reader
decides none of them outside:
- `sphere_face_region` (`crates/topo/src/sphere_region.rs:187`) and
  `torus_face_region` (`crates/topo/src/torus_region.rs:153`, the kinds
  at `:184`) answer `None` for a face any of whose edges is not a circle
  (or, on the torus, a spiric). These booleans' section edges are
  splines (the tests' `ProjectedSpline`), and the faces' `Trim(None)`
  tallies above fit that reading. I did not read the edges' kinds one
  by one.
- `sphere_face_containment` and the torus arm then answer `Trim(None)`
  (`crates/topo/src/contain.rs:1056`).
- So `cell_region` leaves every cell `Undecided`, wholly off the face or
  not.
- Torus `1v3`'s region does read, so the branch and bound drops 24,470
  of its cells. But the walk's deep torus cells still straddle its trim at
  depth 26 to 28 (about 3 cm wide, 1.6 cm from the boundary).

## What reading would part them

- **At the faces' closest approach, one read.** Take the dyadic cells
  about the closest on-face stations, both windows halved alike. The
  engine's own support reading (`support_separation`: both normals and
  the centre line) parts them from the 5th level for torus × sphere
  (cells 0.14 and 0.10 m) and the 4th for torus × torus (0.32 and
  0.29 m). The support along the closest-point direction is no better.
  No normal-cone or monotone-distance reading is needed there.
- **Where the walk spends: no direction at any size.** The cell pairs
  hold a common point of the two carriers, so no plane parts them and
  no distance reading can. What parts them is a region reading: dropping
  a cell that holds no point of its face.
  - The owed reading is the sphere and torus region readings on faces
    bounded by spline section edges, which is what these booleans
    produce.
  - Failing that, a cell clipped to its face's trim by another route
    (`cell_region_lifted`'s boundary distance reads the arcs already:
    `BoundaryStop::Arc`).
  - PR 7119 (a share-nothing pair parted in depth by a gap along one
    direction) does not apply: there is no gap between the carriers to
    read.

## Trace: the boxes are tight

`CAD_GATE_BOXTRACE=1` logs, at each straddling split of two faces sharing
nothing at depth 12 or more:
- both cells' parameter rectangles;
- their boxes (`Surface::rect_enclosure`: an analytic carrier's `eval`
  over the rectangle's two intervals);
- the true extent sampled on a 5 × 5 grid;
- the separation;
- the closest sampled point pair on a 9 × 9 grid, the carrier's, not
  the face's.

| face pair | depth | cell (du × dv) | box / sampled extent | separation | closest sampled (carriers) |
|---|---:|---|---:|---|---:|
| torus `1v3` × sphere `5v1` | 12 | 0.39 × 1.57 rad | 1.01 | [0, 0.63] | 8.5e-2 m |
| | 27 | 0.025 × 0.098 | 1.000 | [0, 4.9e-2] | 6.4e-4 m |
| | 28 | 0.049 × 0.025 (sphere) | 1.000 | [0, 4.1e-2] | 5.4e-4 m |
| torus `1v3` × sphere `2v1` | 28 | as above | 1.000 | [0, 4.1e-2] | 5.4e-4 m |
| torus `5v3` × torus `6v3` | 12 | 0.39 × 1.57 | 1.000 | [0, 0.73] | 0.11 m |
| | 39 | 0.0031 × 0.012 | 1.000 | [0, 5.3e-3] | 1.4e-4 m |
| | 40 | 0.0031 × 0.012 | 1.000 | [0, 4.4e-3] | 1.4e-4 m |

Each cell's box is its sampled extent and halves as its rectangle does.
These boxes have no floor, pad, whole-span term or period wrap.

## What does not move it

Measured on the four tests, the counts are unchanged to the split:
- main `4d3402ecb1` (with PR 7120, a face's boundary box built from each
  edge's cover): 647,151 straddles, 79.5% sharing nothing;
- the same with `origin/embed/nested-tori` (PR 7119) merged: 647,151,
  79.5%.

## Reproduce

On `embed/gate-measure` (`measure/gate/README.md` has the build
environment):
- **The certified minimum:**
  `CAD_GATE_LOG=<dir> CAD_GATE_MINDIST=60000 cargo nextest run -p mesh
  -p sweep --profile ci --ignore-default-filter -E
  'test(=quadric_pair_sections::a_ball_and_a_rod_against_a_donut_build_on_periodic_loops)
  | test(=germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint)'`.
  The `MIN` rows are `measure/gate/min-dist-rows.tsv`. Torus × sphere
  appears in about 1 minute, torus × torus in about 6. The rest of each
  walk can be stopped.
- **The placement:** `python3 -I measure/gate/min-implicit.py`.
- **The box trace:** `CAD_GATE_WALK=1 CAD_GATE_BOXTRACE=1` on
  `quadric_pair_sections` (about 9 minutes). The table is the `BOX`
  rows.

## Closed

2026-11-25 by PR 7772 (after INSIDE's PR 7755): the walk's region reading
hands the certified spline reader (`props::certified_spline_face_crossings`,
the one body every certifying `AtRestPolicy` answers with) to the face
door, so a cell off a sphere or torus face a spline section edge bounds is
dropped. Asked alone, the row's 102 share-nothing pairs are `Embedded`
(the two donuts' `5v3 × 6v3` in 832 splits, from the depth floor). On
today's main no whole-solid ask of the row's four tests reaches them:
each stops at its first finding on a touching pair's departure arm,
which is what `a-failed-local-certificate-answers-split-with-no-ground`
is re-measured against. The cylinder and cone sibling is
`the-walk-drops-no-cell-of-a-cylinder-or-cone-face-a-spline-edge-bounds`.
