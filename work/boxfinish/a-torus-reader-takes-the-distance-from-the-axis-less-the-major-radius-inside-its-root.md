---
id: a-torus-reader-takes-the-distance-from-the-axis-less-the-major-radius-inside-its-root
kind: issue
title: A torus reader takes a point's distance from the axis less the major radius inside its root, which the rationalized gap alone does not make a polynomial
status: open
opened: 2026-11-15
---

Found by the audit of `a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root`
(`boxfinish/radial-readers`). A point's distance from a torus is
`√((ρ − R)² + h²) − r` with `ρ = ‖w‖` its distance from the axis. The
outer root less `r` takes the rationalized gap as a cylinder's does,
but its numerator `(ρ − R)² + h² − r²` still holds `ρ`: for a point
built at `c + (R + r·cos v)·x + r·sin v·axis` it is
`|R + r·cos v| − R` inside, and nothing in the form's syntax signs
`R + r·cos v`. So the gap alone does not make these readers
polynomial, and the unit left `chart_distance`'s torus arm as it was.
Not measured: no torus was read over a box on a radius in this lane.

**The sites** (a sweep by pattern, each classed by reading its
function and usually one caller):

- `geom-brep/src/pcurve_cache.rs`: `chart_distance`'s torus and
  spindle arm; the `incidence` envelope's torus-parallel `core`
  (`rho − major_radius`).
- `geom-brep/src/implicit.rs`: `implicit_residual`'s torus arm
  (`d = rho − major_radius`), read as an identity by
  `circle_torus_coaxial_residual` and as a margin by the contour
  search; `conic_torus_implicit`.
- `geom-brep/src/props/curved.rs`: `torus_classify`
  (`props_meridian_fit`); `torus_ends` and `torus_chart_loop` read
  `ρ − R` as a tube-angle coordinate, with no Zero asked.
- `geom-brep/src/curve_face/circle_torus.rs`: `meridian_deviation`
  (`circle_torus_meridian`).
- `topo/src/boolean/solid_contain.rs`: `torus_elevation`.
- `topo/src/boolean/slit.rs`: `torus_between` (the factored
  `((ρ − R)² + h² − r²)·((ρ + R)² + h² − r²)`, which may fold where the
  tier expands the product; not measured).
- `topo/src/offset_axial.rs`: `surface_residual`'s torus arm,
  `tube_centre_offset`.
- `geom/src/azimuth.rs`: `Azimuth::tube`, the one spelling of the raw
  `ρ − R` as a direction; `geom/src/curves/on_carrier.rs`: the spiric
  `clearance`.

**Owed:** a design answer first (which sign-free form a torus reader
holds: the factored quartic above is one candidate, since
`((ρ − R)² + h² − r²)·((ρ + R)² + h² − r²)` is a polynomial in `ρ²`),
then the readers.

**The tests that say it is closed.** A point built on a torus at
parameter radii reads a proven Zero from each identity reader above at
`Sym<Interval>` over a box on either radius.
