---
id: offset-fit-composite-elevates-a-rational-direction-at-f64
kind: issue
title: offset_fit::Composite::build elevates degree-1 directions at f64 before the ring composite; a rational net's elevation ratio rounds
status: closed
closed: 2026-10-11
opened: 2026-10-10
priority: P3
cost: E
rides_with: offset-fit-refuses-every-degree-1-base-direction
branch: encl/offset-fit-degree-1
pr: 4616
---

Found by NURBS fork3 (designer B), off its question.

## Measured

The finding holds. On a rational bilinear with weight `1` on one ruling and `0.6` on the other, the `f64`-elevated middle control point lands up to 8.5e-17 off the exact one (checked against exact rationals). The composite's stored weight channel then carries the point `0.8`, where the exact middle weight `(1 + 0.6)/2` of the stored `0.6` is 5.6e-17 below it. So that coefficient's enclosure misses the supplied net. `Ẽ` and `M̃` do not show it: their own outward rounding (widths up to about 3e-15 on the same face) covers the error. `offset_fit::tests::a_rational_degree_1_composite_encloses_the_supplied_net` is red on main's `Composite::build` at the weight coefficient, and green with the ring raise.

`crates/geom-brep/src/offset_fit.rs` `Composite::build` (`:2343`) elevates degree-1 directions at `f64` before it builds the ring composite. For a rational net the elevation runs through the projective applier with a stored `f64` λ, so the composite encloses an elevated net with rounded coefficients rather than the described one.

NURBS fork3's decision makes the projective knot algebra unreachable at `Interval` (`[ev]` PR 4539; build on `nurbs/knot-algebra-off-the-ring`). That does not cover this site, because the elevation runs at `f64` and is then lifted. The first step is to measure whether the lifted elevation is padded, or whether the composite is certified about a neighbour of the base.
