---
id: no-sign-decision-gives-a-conic-a-finite-size
kind: issue
title: No sign decision gives a conic a finite size: decide(+inf) is Positive and products overflow, so a decided or proven-positive radius can still panic the constructor
status: open
opened: 2026-11-23
priority: P1
cost: M
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

## Ruled: `[sizes]`

The design question this row asked (should a circle get a deciding
door, as `Curve3::ellipse` is one) is ruled by `[sizes]`
(`docs/DESIGN.md`, "A size is a type", PR 7645): a conic's sizes are
`Size`s, finite and strictly positive, with one fallible mint that
refuses a zero, a negative, a NaN or an unbounded value typed and never
panics, and finiteness is decided at that mint and nowhere else. This
row is the build of that clause for the conics.

## The per-site guards the build retires

The audit's PR (7658) closed the reproductions below with guards at
each site, which `[sizes]` says are a second place for the mint's fact.
The build replaces each with the mint's refusal and deletes it:

- `geom::conic_size_read` (`crates/geom/src/curves/conic.rs`), the
  constructors' finiteness predicate as a read, and its three readers:
  - `geom_brep::intersect::plane_sphere_section_read`'s poisoned read
    under `ps_center_gap`;
  - `topo::replace_face::moved_radius` (`offset_moved_radius`), which
    every transport arm (cylinder, cone, sphere, torus parallel and
    meridian) and `topo::offset_axial::moved_seam` read; its sign half
    is the producer's pose decision and stays;
  - `topo::offset_axial::latitude_circle`
    (`offset_axial_latitude_radius`), an escalation only.
- `geom_brep::intersect`'s `pc_cylinder_radius`, sharing
  `cylinder_radius_decided` with `coc_cylinder_radius`: the section
  arms' radius guards that `[sizes]`' surface bullet retires with the
  surface witness.
- `sweep::blend::build::nonpositive_size_gate`'s finite upper end
  (`BlendError::SizeBeyondRange`): the user's size enters as a `Size`
  at the door, whose mint is that refusal.
- `shell`'s `is_finite_length(thickness)`, the posture the blend gate
  copied.

## What the audit's PR closed, and what it did not

Closed, each with a row and a run mutant: the plane × sphere circle
(the radius is read through `geom::conic_size_read` under
`ps_center_gap` before the circle is stated), the offset transports and
seam circles (`topo::replace_face::moved_radius`,
`topo::offset_axial::moved_seam` and `latitude_circle`), each at `f64`
and at `Interval`, and the blend size gate at an infinite size
(`SizeBeyondRange`).

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

## Moved

The open sign beside this row, `Ring::parallel`'s `p.x` (the audit's
row 17), is SECTTORUS's file: filed there as
`a-torus-parallel-through-a-crossing-states-a-radius-no-row-decides`.
