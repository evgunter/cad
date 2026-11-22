---
id: faces-sharing-nothing-within-a-cell-of-each-other-are-not-parted-by-the-depth-floor
kind: issue
title: Faces of one solid that share no cell but come within a fraction of a millimetre of each other are halved by box distance to the depth floor and the pair budget: the boxes are tight, the cells never get smaller than the gap
status: open
opened: 2026-11-22
priority: P0
cost: M
refs: [a-refusing-ask-runs-on-to-its-whole-budget-looking-for-a-verified-crossing, nested-tori-a-thin-gap-apart-spend-the-pair-budget-on-box-distance]
---


Traced by the gate lane (`embed/gate-measure`, 2026-11-22), for the
refusing-ask row's evidence. No fix; measured, not argued.

## What it costs

On the four tests behind the census's dearest refusing asks
(`germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint`,
`germ_torus_components::a_ball_round_the_donuts_axis_passes_the_germ_mint`,
`quadric_pair_sections::a_ball_and_a_rod_against_a_donut_build_on_periodic_loops`,
`germ_sphere_torus_placements::a_ball_against_a_donut_carries_onto_the_torus_and_builds`),
11 refusing asks, about 3,470 s: 79.5% of the walk's 647,151 straddling
distance splits, and 76% of the refusing face pairs' seconds, are on pairs
of faces of one solid that share no cell of M. The rest (20.5%) are
descendants of a touching pair whose local certificate split; every
straddle between faces sharing a cell is one, and no straddle between
faces sharing nothing is. These are valid bodies the tests pin as built,
refused at their door by the gate, so the band is P0.

## Trace: the boxes are tight, the faces are that close

`CAD_GATE_BOXTRACE=1` logs, at each straddling split of two faces sharing
nothing at depth 12 or more, both cells' parameter rectangles, their boxes
(`Surface::rect_enclosure`: an analytic carrier's `eval` over the
rectangle's two intervals), the true extent sampled on a 5 × 5 grid, the
separation, and the closest sampled point pair on a 9 × 9 grid. Three face
pairs reach depth 12 in these asks:

| face pair | depth | cell (du × dv) | box / sampled extent | separation | closest sampled |
|---|---:|---|---:|---|---:|
| torus `1v3` × sphere `5v1` (`quadric_pair_sections`) | 12 | 0.39 × 1.57 rad | 1.01 | [0, 0.63] | 8.5e-2 m |
| | 27 | 0.025 × 0.098 | 1.000 | [0, 4.9e-2] | 6.4e-4 m |
| | 28 | 0.049 × 0.025 (sphere) | 1.000 | [0, 4.1e-2] | 5.4e-4 m |
| torus `1v3` × sphere `2v1` (same) | 28 | as above | 1.000 | [0, 4.1e-2] | 5.4e-4 m |
| torus `5v3` × torus `6v3` (`two_donuts_in_general_pose`) | 12 | 0.39 × 1.57 | 1.000 | [0, 0.73] | 0.11 m |
| | 39 | 0.0031 × 0.012 | 1.000 | [0, 5.3e-3] | 1.4e-4 m |
| | 40 | 0.0031 × 0.012 | 1.000 | [0, 4.4e-3] | 1.4e-4 m |

- **Each cell's box is its sampled extent** (ratio 1.000; the torus 1.01
  at depth 12) and halves as its rectangle does. There is no floor, pad,
  whole-span term or period wrap in these boxes.
- **The separation's lower end is 0 because the two boxes overlap, and
  they overlap because the faces come within a cell's size of each
  other**: at depth 40 the cells are about 2 mm across and the closest
  sampled points are 0.14 mm apart (0.54 mm for the torus and the sphere
  at depth 28). The walk halves one cell along one axis per level, so 40
  levels leave each axis about 2⁻¹⁰ of its window: millimetres here. No
  cell the floor allows is smaller than the gap, so box distance cannot
  part these faces.
- **Whether the faces touch is not settled.** The sampled distances are
  upper bounds (0.14 mm and 0.54 mm). If the gap is above K·ε the bodies
  are embedded and the engine owes a reading that parts two faces
  sharing nothing without subdividing to the gap; if it is within the band,
  the faces meet off every shared cell and record, and the verdict owed is
  a crossing (none is verified now: the verifier's stations sit at the
  cells' sampled points). The next step is a local minimisation of the
  distance between the two faces in these cells.

## What does not move it

Measured on the four tests, counts unchanged to the split:
- main `4d3402ecb1` (with PR 7120, a face's boundary box from each edge's
  cover): 647,151 straddles, 79.5% sharing nothing;
- the same with `origin/embed/nested-tori` (PR 7119, a face pair sharing
  nothing read apart in depth) merged: 647,151, 79.5%. PR 7119 is for
  faces a 0.025 gap apart (`nested-tori-a-thin-gap-apart-…`); these are
  a fraction of a millimetre apart.

## Reproduce

On `embed/gate-measure`: `CAD_GATE_LOG=<dir> CAD_GATE_WALK=1
CAD_GATE_BOXTRACE=1 cargo nextest run -p mesh --profile ci
--ignore-default-filter -E 'test(=quadric_pair_sections::a_ball_and_a_rod_against_a_donut_build_on_periodic_loops)'`
(about 9 minutes); the `BOX` rows are the table, the `STRADDLE` rows the
split.
