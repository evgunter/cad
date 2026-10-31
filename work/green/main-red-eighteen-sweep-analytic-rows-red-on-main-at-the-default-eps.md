---
id: main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps
kind: issue
title: Main red: eighteen sweep analytic rows (rulings, cylinder_sphere_frame, torus_doors_join, …) are red on main at the default ε
status: open
opened: 2026-10-31
priority: P0
cost: M
---

Found by SPLICE's curved-crossing lane (PR 6136) while validating locally
on 2026-10-31. The lane re-ran the failing rows on bare `origin/main`,
and all 18 are red there too, so they are main's, not 6136's. The run
used the sweep slow set at the default ε. Not yet bisected.

## The rows (sweep)

- `a_ruling_lying_on_a_wall` ×2
- `a_ruling_through_a_box_corner_edge` ×2
- `a_pierce_held_in_a_curved_face` ×2
- `bisector_nodes::the_near_node_poses_at_the_witness_band`
- `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_*` ×6
- `torus_doors_join` ×3
- `conic_edge_curved_face::a_rim_crossing_reaches_the_join`
- `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`

## What closing it takes

Reproduce each row on current main and bisect it. The candidates are the
merges of 10-30 and 10-31, which include SECTARMS's cylinder × cylinder
unit (PR 6139), the touch-or-tangency split and VSEAM PR 6129. Then fix
each row or route it to the program whose merge caused it. Some of these
may already be covered by an existing row:
`charttail/main-red-ringed-curved-doors-reds-four-sweep-rows` lists
`bisector_nodes` at 1e-12. If one is, add this evidence to that row
rather than duplicating it.


## 2026-10-31 (SECT, lane `sect-onesix`)

Measured on main `b719d39ad1` at opt-level 2. Two of these rows also
read the same at 1e-6 and 1e-12 as at the default ε:

- `torus_doors_join::a_square_rod_on_the_outer_equator_stops_at_the_join`
  stops `Projected`, where the row wants `Join { closed: false }`.
  `a_rod_across_the_inner_bound_…` stops `Projected` and
  `a_rod_off_the_axis_line_…` stops `ConicTrim`, at every ε.
- `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`:
  for β 0.5 the five ops stop `ring volume`, `quadrature volume`,
  `quadrature volume`, `ring volume`, `quadrature volume`. For β 0.6 and
  0.7 all five stop `quadrature volume`. `OTHER_SPINS_STOPS` wants
  `spline join` on ∪ and on c ∖ h.

Until SECT's PR on
`sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot`
lands, both rows are pinned at 1e-6 to an earlier stop, the germ
section's `Winds(Unseparated)`. That PR removes the two pins. With the
pins gone, at 1e-6 both rows stop exactly as listed above, so all three
ε now share this one red.
