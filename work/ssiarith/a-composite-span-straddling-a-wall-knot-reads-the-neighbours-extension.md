---
id: a-composite-span-straddling-a-wall-knot-reads-the-neighbours-extension
kind: issue
title: surface_curve_residual bounds a span whose window straddles a wall knot on each cell's polynomial extended over the whole span, so a break a foot offset carries off a knot costs subdivision rounds
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [a-loose-hull-side-spends-limb-2s-budget-at-eps-1e-6]
---


Filed by ENCL's `encl/loose-hull-budget` lane (the
`a-loose-hull-side-spends-limb-2s-budget-at-eps-1e-6` row), measured.

`geom_core::spline::compose::tensor::surface_curve_residual` bounds each
shared t-span on every wall cell whose interior the pcurve's window
meets (`cells_touched`), each cell's polynomial composed with the
span's whole `P(t)` (`cell_residual`). Where the window crosses a wall
knot, the neighbour cell is read over the whole span through its
polynomial extension, though `P(t)` enters that cell only past the
crossing. On a C0 wall that extension is large: for the `(P, −N, P)`
quadratic of `ssi_limb3_one_arc`'s loose-side rows it reads
`2(P + N)·s(s − 1) + P` at `s` cell widths past the knot.

Measured on that wall (`a_side_whose_hull_is_loose_reads_clear_over_the_grid`,
limb 2 under an unbounded budget, 0.6ε/0.2ε rows): where the
composite's breaks land on the wall's knots (default ε, where the foot
points' offset rounds away) m = 1024 clears at 5908 spans of work and
1432 cuts; at ε 1e-6, where the feet sit about 6e-13 off the
identity and carry every break's image in `v` off its knot (the
breaks in `t` stay put), the same row costs 11 188
and 4058, because each round leaves one straddling span per wall knot.

A fix to weigh: break the composite where the pcurve crosses a wall
knot line (a structure cut, C6's `f64` lane, like `SSI_CERT_SPANS`), so
each piece reads one cell and the straddle left is the rounding of the
crossing. A degree-1 pcurve has the crossing in closed form; a
rational or higher-degree one needs a root-isolating structure step.
That is a change to which breaks C2 limb 2's composite carries, which
`ssi/certify.rs` decides (`nurbs_limbs`' hull closure hands
`surface_curve_residual` its extra breaks), so it is read against
`crates/geom-brep/README.md` C2 before it lands.
