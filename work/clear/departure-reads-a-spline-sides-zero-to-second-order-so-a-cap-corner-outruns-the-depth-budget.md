---
id: departure-reads-a-spline-sides-zero-to-second-order-so-a-cap-corner-outruns-the-depth-budget
kind: issue
title: At a cap corner no presentation reads the cap, and the wall-departs-the-cap's-plane reading that would certify it reads the wall's zero side through the centred form over a rational spline net, second order in the cell: it decides only at depth 51-73, past the budget of 40
status: open
opened: 2026-11-01
priority: P0
cost: M
---


## Finding (SEAMDIAG, 2026-11-01)

The cap-against-wall locus of
`injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams`.

**Where.** Every refusing cell pair sits at a vertex of the cap's
boundary where two circular arcs meet:

- `lily_leaf_a` and `lily_leaf_b`: the lens tip, where two different
  circles meet at a sharp corner.
- `klein::bottle().top`: where a wall seam lands on the section circle,
  which is split into four arcs of one circle meeting tangentially.

The cap is `Plane` and the wall `Nurbs`. The shared edge is a `Circle`
on the cap and the wall's `v = 0` or `v = 1` iso. Klein
`1v1 × 3v3`'s depth-40 cells, for example, are
`u ∈ (0.27393, 0.275) × v ∈ (−0.00107, 0)` on the cap and
`u ∈ (0, 0.00195) × v ∈ (0.99988, 1)` on the wall, at vertex `5v1`.

**What fails.**

- `present_plane` → `curved_walls` → `beside_curve`. The cap cell
  holds both arcs, and `corner_of` and `reading` read no corner of two
  curved edges. So the cap is `Structural` at the vertex, and `Local`
  in the cells beside it, which hold both arcs but not the vertex.
- The wall presents (`Zero::Side(YHi)` or `YLo`), so the pair splits
  rather than refusing.
- The one reading that can certify without the cap is the carrier
  arm: the cap's plane as `ψ`, the wall departing it. Its across sign
  is definite at every depth (`across V ≈ +9.2e-4` at depth 40). But
  its `zero_on` test of the wall's side reads `±8.0e-6` at depth 40,
  `±1.4e-7` at depth 50 and `±7.8e-9` at depth 60.

That residual is the `centred` form on a rational spline side. `jet_on`
reads a zero-extent side through the whole rectangle's `rect_jet`, and
the restricted net's hull of `n·S_u` is first-order wide even on the
iso. So the side value is second order in the wall cell.

**Measured.** With `max_cell_depth` 80, every cap–wall pair of all
three bodies `Holds` (deepest 51–73, 0.2–0.5 s). Reading the side over
a strip `2⁻²⁰` of the span wide inside it, rather than the whole
rectangle, barely helps (`±3.7e-5` at depth 40). The natural interval
extension `surface.jet(x, y)` is looser still (`±3e-4`).

**Confirms the row's guess in part.** This is a case of
`cleartail/departure-reads-no-curved-corner-off-its-shared-edge`, but
its second bullet (a corner of two curved edges), not the plane's
line-edge bullet the row guessed. And presenting that corner alone
would not obviously certify here: the binding residual is on the
wall's side.

**`m8_14`'s half-turn helix at 33 stations no longer reproduces.** All
eight cap–wall pairs `Hold` at depth 6–8 at this branch's base. CARVE
measured it before the carried frame (`40a4f9c9`) and POTDOC's corner
run-on (`1ee245d7`, `21aeef58`) landed. Not bisected between the two.

## Fix shapes

1. **The zero side of a linear separator read off the side's own net**
   (recommended). For `ψ = n·(p − q)` and a NURBS iso side, enclose
   `ψ` along the side by the hull of `n·(P_i − q)` over the side's
   restricted iso control points. The convex-hull property holds with
   positive weights. The iso lies in the cap's plane to rounding, so
   the side reads zero at any depth.
   - Where: `sign_on`'s `zero_on` (or `centred` on a side), in the
     departure arm's `ON_SHARED` reading.
   - Argument: none new. It is a tighter enclosure of the same
     quantity, with no new arm and no ε.
   - Size: S–M, about 40 lines, plus an iso-net door on
     `NurbsSurface<Interval>` if none is public.
   - It should flip every cap–wall pair here well under depth 40, but
     that is not measured.
2. **The CORNER extension the cleartail row describes**: a plane's
   corner of two arcs, read through the vertex by the continued arc
   for Klein's arcs of one circle, and by the other arc's height over
   the continued curve at the lens tip.
   - Size: M. It needs a new corner reading argued.
   - On its own it leaves the wall-side residual, so it is not
     sufficient here.
3. **Raise `DEFAULT_MAX_CELL_DEPTH`.** Not recommended: a line locus
   (the seams, SLIVER) then refuses later and dearer.

Pinned, today's refusal:

- `demos/tour/tests/lily.rs`
  `cap_corner_departure::each_cap_of_a_lens_blade_refuses_departure_at_its_tip`;
- `demos/tour/tests/klein.rs`
  `cap_corner_departure::each_cap_of_the_loop_refuses_departure_at_its_arc_vertices`;
- `a_lens_loft_cap_refuses_departure_at_its_tip_at_both_scalars` in
  `crates/sweep/tests/self_intersection_proves_injectivity.rs`.

The helix's closure is pinned by
`the_half_turn_helixs_caps_hold_against_its_walls_at_both_scalars`.
