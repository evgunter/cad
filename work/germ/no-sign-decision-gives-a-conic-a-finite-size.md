---
id: no-sign-decision-gives-a-conic-a-finite-size
kind: issue
title: No sign decision gives a conic a finite size: decide(+inf) is Positive and products overflow, so a decided or proven-positive radius can still panic the constructor
status: open
opened: 2026-11-23
priority: P1
cost: M
design: true
---


Filed by GERM's `germ/conic-constructions-audit` lane (2026-11-23), from
`conic-constructions-from-computed-sizes-unaudited`.

## Measured

`geom::Circle::new` and `geom::Ellipse::new` panic on a size that is
negative **or not finite** (`crates/geom/src/curves/conic.rs`,
`stored_size`, which asks `is_bounded`). The audit's dispositions
("decided before construction", "provably non-negative") are arguments
about the size's **sign**. None of them gives a finite size:

- `geom_core`'s `decide` reads `+inf` as `Positive`
  (`crates/geom-core/src/predicate.rs`, "an infinite margin is maximally
  definite"); only NaN escalates. So a radius guard, a ring guard or a
  gap row passes an infinite datum.
- A size that is a product or a root of a product overflows on finite
  data: `geom_brep::plane_sphere_section_read` stated
  `√((r − |s|)(r + |s|))`, which is `inf` from `r ≈ 1e154` up.

Reproduced on main through public doors, each a panic in `Circle::new`:
`topo::replace_face_offset` on a sphere face of `sweep::test_support`'s
dome and sphere zone at `d = f64::INFINITY`, `f64::MAX` and `1e306`;
`topo::offset_charts_together` on a ball at `f64::INFINITY`;
`sweep::blend::build::fillet_edges` on the lantern's lip rim at
`f64::INFINITY`, `f64::MAX` and `1.5e308`.

## What the audit's PR closed, and what it did not

Closed, each with a row and a run mutant: the plane × sphere circle
(the radius' width is read under `ps_center_gap` before the circle is
stated), the offset transports and seam circles
(`topo::replace_face::moved_radius`, `topo::offset_axial::moved_seam`
and `latitude_circle`), and the blend size gate at an infinite size.

Open: every other site in the PR's table whose finiteness column reads
"open". On read, the ones a surface literal reaches are
`geom_brep::intersect`'s `sphere_sphere_section_read`
(`Circles::half_chord`), `plane_torus_read` (`big_r ± w`, `big_r`),
`coaxial_cone_sphere`, `coaxial_cone_torus`, `cone_cylinder_section`
(`big_r`), `pair_section`'s `cylinder_sphere` and `Ring::parallel`, and
`geom_brep::pair_section::torus::Ring::of`, where
`Surface::Torus { major_radius: f64::INFINITY, .. }` passes both ring
rows and reaches `unreachable!("Ring::of mints a ring of finite data
only")` in `CylinderTorusPose::decide` and `TorusTorusPose::decide`, and
`Circle::new(frame, inf)` in `gap::Core::of` and `winds.rs`.

## What a fix has to supply

One statement, not a check per site. `Curve3::ellipse` is already a
deciding door: it refuses `EllipseInvalid::SemiAxisNotFinite` and
`MinorNotPositive` typed before it builds, and `geom_brep` carries the
refusal as `SectionError::Carrier`. A circle has no such door, so each
site that mints one from a computed radius either re-reads finiteness
by hand or panics. Whether the circle gets that door, with the error
type and each layer's mapping it needs, is a design choice the lane
reported to the orchestrator and did not make.

## One sign left open beside it

`geom_brep::pair_section::torus::Ring::parallel` is handed `p.x` by
`crossing_parallels` and `tangent_parallel`: a point of
`Circles::crossings`, or of the tangent foot, `R + û.x·a ∓ û.y·h`. In
real numbers it is at least `R − r`, which the ring row holds past the
band, but that is a different expression from the size stored, and no
row decides the sum itself. No input that turns it negative was built.
