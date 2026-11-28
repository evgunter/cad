---
id: on-surface-curve-doors-state-a-raw-surfaces-radius-undecided
kind: issue
title: SurfaceCurve::parallel, meridian, great_circle and about state a circle at a raw surface's radius with no decision, and panic on a surface outside its convention
status: open
opened: 2026-11-23
priority: P2
cost: M
design: true
---


Filed by GERM's `germ/conic-constructions-audit` lane (2026-11-23), from
`work/germ/conic-constructions-from-computed-sizes-unaudited.md`.

## Measured

`geom_brep::curve_pair::SurfaceCurve`'s doors take a `Charted` surface,
which is any `geom::Surface` (`Charted::record`, `Charted::held`), and
mint through one private `SurfaceCurve::circle` that hands its radius to
`geom::Circle::new`, which panics on a radius that is negative or not
finite. `ChartInverse::of` (`crates/geom/src/chart_point.rs`) copies the
radii as stored and refuses only a spline. A surface's radii are public
fields, positive by convention and unchecked (`crates/geom/src/surfaces.rs`);
tier 3 admits the tube and the plain radius at rest and never the major
radius. By reading, each of these panics:

- `SurfaceCurve::parallel` on `Surface::Cylinder { radius: -1.0, .. }`;
- `parallel` on `Surface::Torus { major_radius: 1.0, minor_radius: 2.0, .. }`
  at `v = π` (`major + minor·cos v = −1`);
- `meridian` or `great_circle` on a sphere of radius `−1` or NaN;
- `about` on a sphere of radius `0` (a zero vector normalized).

`topo` reaches `parallel` on a torus, `meridian` on a torus,
`great_circle` and `about` behind its own rows
(`geom::require_ring_torus` in `torus_region.rs`,
`bool_sphere_region_arm` and `bool_sphere_region_parallel` in
`sphere_region.rs`). It reaches `parallel` on a cylinder
(`ruled_region.rs`) and `meridian` on a sphere (`boolean/ops.rs`,
`cut_in`) on the body's tier-3 standing alone.

## What a fix has to supply

Whether these doors decide the radius they state, or take only a
surface something has admitted: the class is
`work/germ/no-sign-decision-gives-a-conic-a-finite-size.md`'s, and
`work/sect/section-arms-off-the-value-read-radii-unguarded.md` is the
same question at the section arms.
