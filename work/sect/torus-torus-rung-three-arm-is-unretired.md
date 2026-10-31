---
id: torus-torus-rung-three-arm-is-unretired
kind: issue
title: Torus × torus routes to rung 3 and refuses: no ℝ³ implicit-pair door or tangency decision for the pair
status: closed
opened: 2026-10-27
priority: P0
cost: M
refs: [torus-germ-pairs-have-no-section-frame, torus-certificate-runs-root-free-through-its-quartic, each-torus-pair-decides-tangency-on-one-pose]
branch: sect/torus-doors
closed: 2026-10-28
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

`crates/geom-brep/src/intersect.rs` `route(Torus, Torus)` answers
`Rung::General`, `implemented: false`, and `route_pose` keeps that
answer. No `ssi` door takes the pair.

## What the taker owes

A `torus_torus_ssi` door: the gate, the pair's tangency decision before
any rung (two tori can touch along a circle when coaxial, or at
isolated points; no closed-form gap like the sphere's is known here, so
that decision is the design work), a coincident pair refused as a
coincidence, and a measured certified pose (the germ row's donut ×
donut placements, the lily's stem and arch walls), then `route` and
`route_pose` flipped. Torus × torus waits on this before
`torus-germ-pairs-have-no-section-frame`'s torus half can join, so it
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
`torus_torus_ssi` door marches past a transverse gap; the section value's
`march_over_the_sphere` takes a sphere's box today, and this pair has
none, so the door also owes its domain.

## Closed

2026-10-28, by `sect/torus-doors`. `geom_brep::ssi::torus_torus_ssi` reads the
pose decided once (`TorusTorusPose::decide`), refuses a tangent gap as the
pair's own (`TangentPair`) before any rung, and marches a transverse
one through `implicit_pair_r3` (`past_the_gap`, every door's).
`route` answers `implemented`, and `route_pose` for every pose the
section value lists components for (a general pose it refuses is not
served). The march's domain is a
ball that holds the whole section (`PairPose::ball`: a torus lies in
`|x − c| ≤ R + r`, the smaller of two tori's), one construction for
every pose (`PairPose::domain`, `march_over_the_ball`). The symmetric poses' components
carry the general rung's carrier, which `PairSection::carry` marches;
`ClassCarrier::Unbuilt` is gone. Five of the germ row's six donut × donut placements certify (two loops
at three, one at two), the sixth a tangency the gap decides and the door
refuses before any rung. The lily's stem and arch tori decide a
transverse gap, but where the arch leaves the fork its tube runs 8 mm
inside the stem's, and the door refuses `CellBudget` over the arch's
ball (the smaller); this row closes with the lily pinned as that
refusal (`crates/sweep/tests/mate7a_r1_probes.rs`
`the_lilys_tori_exhaust_the_doors_cell_budget`), and a certified lily
pose moved to SECTRUNG's
`a-cylinder-sphere-march-near-its-node-costs-seconds-and-refuses-by-budget`.

A general pose still refuses at the section value
(`RoutesToGeneralRung`): it has no closed-form count of its components,
and what its classification lists is a design fork, filed as
`a-general-torus-pose-lists-no-components`.
