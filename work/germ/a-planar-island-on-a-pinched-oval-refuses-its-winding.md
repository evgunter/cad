---
id: a-planar-island-on-a-pinched-oval-refuses-its-winding
kind: issue
title: A plane face's island on a spiric oval pinched near the inner equator refuses RingIslandUnread: the winding's quadrature cannot bound its error there
status: open
opened: 2026-10-12
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
