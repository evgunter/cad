---
id: projected-hull-bounds-are-operands-of-t-arithmetic
kind: issue
title: The projected and spline-incidence hull bounds enter T arithmetic as operands, not as a decision's allowance
status: open
opened: 2026-11-20
---

Found building `a-bound-to-decide-against-never-becomes-a-t` (DL5.2, `crates/geom-core/README.md`).

## What

These lift a hull bound into `T` and then compute with it, so neither a decision at `f64` nor an allowance on the margin fits:

- `pcurve_cache/projected.rs`: `axial_floor` (`(σz)_min` from a part's `z` bracket), `Meters::sheet_apex_reach` and `sheet_at_height` (squares, quotients and roots of `z` ends against the twin's radii), `Meters::over_at`'s spindle arm (`z2`), `Meters::apex_part` (`span.meters`), and `net_incidence` (`span.rho_lo`, `rho_hi`, `f_sup` divided by levers at `T`; its `(incidence, rho_floor)` are returned as `T`). `round_floored`'s caller returns `Offset::Miss { floor: T::from_computed(floor) }`.
- `pcurve_cache/spline_incidence.rs`: `Net::floor` is `topo::spline_trim`'s `Patch::floor` written a second time: `max(gap − read − cut, 0)`, in the same order since the unit's fix pass, which moved `Patch::floor` onto `Margin::less_floored`. This one lifts the cut through `T::from_computed`: its floor is min-folded over cells and returned as `Read::Off(floor: T)`, and `past_zero` subtracts the band's zero from it, so the cut cannot ride as one margin's allowance. When it moves, it moves onto the same door, so the two stay one rule.

## What would move them

The arithmetic is certification arithmetic over `f64` brackets: it can run at `Interval` (`Meters<Interval>` already exists for `SpanHull::meters`) with the twin's radii read as brackets, returning `f64` bounds, which the callers decide through `Margin::bound` / `Margin::less`. That reads brackets of the twin's scalars, which is the marker-bound unit's ground (`certification-interiors-are-closed-to-a-dual-by-a-marker-bound`): these interiors are dual-free once it lands, so the lift is wrong only at `Sym`, where it is an unknown and proves nothing it should not.
