---
id: tilted-plane-torus-rung-three-arm-is-unretired
kind: issue
title: A tilted plane × torus pose routes to rung 3 and refuses: no ℝ³ door marches a plane against a torus
status: open
opened: 2026-10-27
priority: P2
cost: M
refs: [torus-certificate-runs-root-free-through-its-quartic]
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

`crates/geom-brep/src/intersect.rs` `plane_torus_section` refuses
`RoutesToGeneralRung` on every tilted pose and on an axis-parallel
plane at or past the inner equator, and `route_pose` reads those poses
as `implemented: false`.

## What the taker owes

An ℝ³ plane × torus door over `implicit_pair_r3` (the plane's arms of
the enclosures and of `composite_form` exist), the pair's tangency
decision before any rung (a plane is tangent to a torus where its
distance from the core circle's nearest or farthest point in the
plane's normal direction equals `r`, the bitangent Villarceau pose
included), and the routing of those poses to it from
`plane_torus_section`'s refusals.
