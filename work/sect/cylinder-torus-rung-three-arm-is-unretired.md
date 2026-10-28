---
id: cylinder-torus-rung-three-arm-is-unretired
kind: issue
title: Cylinder × torus routes to rung 3 and refuses: no ℝ³ implicit-pair door or pose classification for the pair
status: dispatched
opened: 2026-10-27
priority: P0
cost: M
refs: [torus-germ-pairs-have-no-section-frame, torus-certificate-runs-root-free-through-its-quartic, each-torus-pair-decides-tangency-on-one-pose]
---

Filed by SECT's `sect-toruscert` lane (branch `sect/torus-certificate`),
which retired sphere × torus at rung 3 and left this pair's arm unretired.

## What already exists

The torus's halves of the ℝ³ certificate are built and pair-independent:
`crates/geom-brep/src/ssi/enclose.rs` `implicit_enclosure` and
`implicit_gradient_enclosure` (the mean-value form of `g = F/(4rA)` and
its exact gradient, `Quartic`), `ssi/certify.rs` `composite_form`'s
torus arm with its per-span conversion (`ToMeters::Torus`,
`enclose::torus_meters_sup`), and `has_implicit_enclosure` lists the
torus. `ssi::sphere_torus_ssi` is the pattern: a pair gate, the pair's
own tangency decision before any rung, then `implicit_pair_r3`.

## Measured

`crates/geom-brep/src/intersect.rs` `route(Cylinder, Torus)` answers
`Rung::General`, `implemented: false`, and `route_pose` keeps that
answer. No `ssi` door takes the pair: `cylinder_sphere_ssi` and
`sphere_torus_ssi` refuse it `WrongLane`.

## What the taker owes

A `cylinder_torus_ssi` door: the gate, a cylinder × torus tangency (or
near-tangency) decision before any rung — the pair has no one-line gap
like the sphere's (a cylinder can touch a torus along a curve, or
coaxially around a circle), so that decision is the design work — and a
measured certified pose (the germ row's drum × donut placements, and
GERM's radial hole `germ_radial_hole.rs`), then `route` and
`route_pose` flipped. Cylinder × torus waits on this before
`torus-germ-pairs-have-no-section-frame`'s cylinder half can join, so it
carries that row's band.

## Designed (2026-10-28)

The tangency decision this row called "the design work" is the critical
gap of `each-torus-pair-decides-tangency-on-one-pose`, which this row now
waits on. What remains here is the door over a Positive gap (through
`implicit_pair_r3`), a measured certified pose, and `route`/`route_pose`
flipped. Also owed: the tube-circle tangency (equal minor radii with the
cores touching, or `r_c = r` with the axis touching the core) as a Zero
row, which neither this row's text nor its sibling's named.

## Opened (2026-10-28)

`each-torus-pair-decides-tangency-on-one-pose` closed: the pair's pose
and its critical gap are `geom_brep::pair_section::torus`'s, and the
tube-circle tangency rows are `crates/geom-brep/tests/torus_gap.rs`
`the_tube_circle_tangency_decides_zero`. What waits here is the door: a
general pose answers `SectionError::RoutesToGeneralRung` naming this
row, and a symmetric pose's components carry `ClassCarrier::Unbuilt`
naming it, so `PairSection::carry` refuses `CarryError::Unbuilt`. A
`cylinder_torus_ssi` door marches past a transverse gap; the section value's
`march_over_the_sphere` takes a sphere's box today, and this pair has
none, so the door also owes its domain.

