---
id: analytic-exact-elevations-have-inline-homes
kind: issue
title: The exact-distance elevation of a cylinder, sphere and torus is written inline at several sites; the cone's now has one home in geom_brep::cone_elevation
status: closed
opened: 2026-09-28
priority: P3
cost: E
refs: [c5-gate-admits-every-pose-of-an-implemented-pair]
closed: 2026-10-26
---

## What

Found by the review of GERM's cone-containment / C5 pose-gate PR
(3322). That PR gave the cone's exact elevation `ρ·cos α − s·sin α` ONE
home, `geom_brep::cone_elevation` (`crates/geom-brep/src/implicit.rs`),
which `implicit_residual`'s cone arm, `offset_axial::surface_residual`,
the solid door and the face door now all call.

The same class for the other analytic kinds is still spelled inline —
the EXACT signed distance, as distinct from `implicit_residual`'s
linearized `(q² − r²)/2r` forms:

- **cylinder** `ρ − r`: `crates/topo/src/boolean/contain.rs`
  (`curved_face_placement`'s carrier test), `crates/topo/src/offset_axial.rs`
  (`surface_residual`), `crates/topo/src/validate.rs` (the `radial`
  near line 7335), `crates/geom-brep/src/props/curved.rs` (near line 1531);
- **sphere** `|p − c| − r`: `contain.rs` (`sphere_face_containment`),
  `offset_axial.rs` (`surface_residual`);
- **torus** `|(ρ − R, h)| − r`: `crates/topo/src/boolean/solid_contain.rs`
  (`torus_elevation`, already one home for the boolean module) and
  `offset_axial.rs` (`surface_residual`, spelled through `Vec3::norm`).

Each copy is right today; the risk is the one the cone home removes — a
fix to one spelling (a scale, an interval-tight square) that the others
do not get.

## Fix shape

`geom_brep::{cylinder,sphere,torus}_elevation` beside `cone_elevation`,
every site above calling them. Check bit-identity per site: the torus
copies differ in operation order.

## Closed (2026-10-26, tracker sweep)

**Superseded by `point-to-carrier-distance-is-spelled-seven-times`** (this slate). That row asks for one reading of a point against a carrier, with its residual, distance and on-carrier decision, and one arm per surface kind. The exact cylinder, sphere and torus elevations this row lists are spellings of that reading. Its unique facts (the cone's home, the extra sites, the bit-identity caution) are merged into that row's `## Merged from` section. `torus_elevation` is still the boolean module's home (`crates/topo/src/boolean/solid_contain.rs:1157`), and `geom_brep::cone_elevation` is still the only `*_elevation` home in geom-brep (`crates/geom-brep/src/implicit.rs:136`).
