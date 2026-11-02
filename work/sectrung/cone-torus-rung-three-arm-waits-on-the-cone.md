---
id: cone-torus-rung-three-arm-waits-on-the-cone
kind: issue
title: Cone × torus routes to rung 3 and refuses: the torus half is built, the cone's meters conversion is not
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

`crates/geom-brep/src/intersect.rs` `route(Cone, Torus)` answers
`Rung::General`, `implemented: false`, its note now naming only the
cone's meters conversion: `implicit_enclosure`,
`implicit_gradient_enclosure` and `composite_form` still refuse the
cone.

## What the taker owes

The cone's own unit first (its meters conversion and its `tan α`
exactness question), then a `cone_torus_ssi` door with the pair's
tangency decision and a measured certified pose, then `route` flipped.
