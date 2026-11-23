---
id: a-face-on-several-arcs-of-one-conic-has-no-spanned-run-at-a-box-scalar
kind: issue
title: A face whose boundary is several arcs of one conic has no spanned run at a box scalar, so every revolved cap refuses at Interval
status: open
opened: 2026-11-23
priority: P1
cost: M
---


The embedding question levers a pair's parting at the narrower face's
run from the shared edge, a lower bound of how far the face stands from
the edge's whole circle or ellipse
(`geom_brep::BoundaryStations::run_from_edge`). A face whose whole
boundary lies on that one conic (a disc on its rim, an elliptic cut)
has every station on it and reads a rounding, so its run comes from the
spanned bound instead (`geom_brep::spanned_curve_run`): the face spans
the conic, meets the line through its centre, and stands at least the
conic's distance from that line away from it.

That bound needs the face's edges to be ONE conic. One edge is; several
are by `topo::injective::one_conic`, which at a box scalar admits only
carriers stored as points (a component held as a bracket is never one
value, even against itself). `revolve` mints every full rim as two half
circles, and a tilted cut of a rod as two half ellipses, so at
`Interval` the bound is not admitted, the face's run stays a rounding,
no cell beside the rim is signed, and the pair refuses.

Measured on `embed/verdict-read` (2026-11-23), suite file
`crates/sweep/tests/self_intersection_proves_injectivity.rs`, default
ε. Each `f64` twin holds; each edge is classed a convex crease by tier
3; each refuses `Uncertified/Departure/Edge` at `Interval` (the pot
`Budget(Pairs)`), where on `embed/remeter` it held:

| row | narrower face | its run | the other face's run |
|---|---|---|---|
| `a_revolved_cone_holds` | the disc, two circles | 3.2e-15 | 1.414 |
| `a_cone_frustum_and_its_cavity_hold_at_both_scalars` (2/64 frustum) | a cap, two circles | 5.9e-17 | 0.129 |
| `shell_4s_vessel_holds` | a cap, two circles | 1.9e-15 | 2.0 |
| `a_windowed_tubes_caps_hold_against_its_walls_at_both_scalars` | a cap, two circles | 1.3e-14 | 2.82 |
| `the_klein_elbows_cavity_holds_across_its_spiric_caps` | a cap, two circles | 2.0e-15 | 1.74 |
| `a_kernel_built_oblique_cut_holds_across_its_curved_corners` | the cut, two ellipses | 8.5e-15 | 1.44 |
| `a_rod_less_a_tilted_slab_holds_across_its_curved_corners` | the cut, two ellipses | 1.1e-14 | 3.02 |
| `a_rod_taller_than_a_turn_less_a_tilted_slab_holds` | the cut, two ellipses | 1.1e-14 | 5.64 |
| `the_corpus_pot_holds_at_both_scalars` | not probed | | |

None of these faces is inside the band of its neighbour: the reading is
missing, not in band.

**A fix that needs no equality.** The face spans the shared edge's
conic if its one loop stays inside a tube about that conic thinner than
the conic's least radius (each boundary edge's cover read about the
conic's circle, its far end below the radius) and winds once about the
conic's axis. That is sound for every member of a family. The winding
half is new argument: each arc's angular advance about the axis, summed
to one turn. The other route is provenance: the door that minted two
arcs from one circle knows they are one.
