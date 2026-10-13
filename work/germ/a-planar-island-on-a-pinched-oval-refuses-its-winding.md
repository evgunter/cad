---
id: a-planar-island-on-a-pinched-oval-refuses-its-winding
kind: issue
title: A plane face's island on a spiric oval pinched near the inner equator refuses RingIslandUnread: the winding's quadrature cannot bound its error there
status: closed
opened: 2026-10-12
closed: 2026-10-13
---


Found by U-T4 (`germ/plane-torus-chord-spiric`), 2026-10-12; a
residue of that unit's winding term.

## Measured

`crates/sweep/tests/germ_plane_torus_spiric.rs`
`a_plane_near_the_inner_equator_refuses_its_islands_typed`: the
half-space `x ≥ 1.5 − 1e-3` against the donut `(2, 0.5)` about `y` cuts
two ovals, each an island in the plane face, and every op refuses
`Join(RingIslandUnread { kind: Plane })` at `boolean::join`'s
`ring_run_ccw`.

The cause is `crates/topo/src/loop_winding.rs` `spiric_segment_term`:
the oval's bulge is an elliptic integral taken by a 16-point
Gauss–Legendre rule over 512 equal pieces, with its error bounded from
the integrand's analytic strip, `δ = min(1, f_min²/(10·r·(R + r)))`.
Near the inner equator `f_min` falls (`f_min² ≈ 2(R − r)·gap`), the
strip thins against the pieces, and the bound grows past the area
(`loop_winding::spiric_rows`'
`an_oval_near_the_inner_equator_bounds_its_error_by_its_bulge`: gaps
`1e-3`, `1e-6`, `1e-9`). The sum then reports no winding
(`bool_ring_run_winding_spiric_rule`), never a wrong one. On the fat
torus `(1, 0.6)` the same holds from offset `−0.35` (`f_min² = 0.0375`).

## Done when

Pieces graded toward the pinch (each piece no wider than its own
window's strip, which a generic scalar cannot size at run time without
a schedule fixed in advance: a geometric grading toward `t = π`, say),
or a closed form of the error near the pinch, decides these islands'
windings, and the row above builds its true bodies.

## Closed (U-T4's fix pass, 2026-10-13)

The winding no longer takes the spiric's bulge by its own quadrature:
`loop_winding` sums a cycle carrying a spiric at the interval scalar
over `geom_brep`'s certified enclosure (`props::spiric`'s
`spiric_vector_area`), refined by piece count until the sign is
decided. Measured on the fix pass: the half-space `x ≥ x₀` builds its
true body in all six ops on the donut at `x₀ = 1.5 − 1e-3` and
`1.5 − 1e-6`, and on the fat tori `(1, 0.9)` at `x₀ = 0.01, 0.02,
0.04, 0.06, 0.08` and `(1, 0.95)` at `0.01, 0.02, 0.04` (past its inner
equator, `x₀ ≥ 0.06`, the frame routes the pose to the general rung, a
different door). The row is
`germ_plane_torus_spiric::a_plane_near_the_inner_equator_builds_its_true_bodies`;
`loop_winding::spiric_rows::a_loop_on_an_oval_winds_one_way_round_and_the_other`
winds whole-oval loops `1e-3`, `1e-6` and `1e-9` short of the inner
equator.
