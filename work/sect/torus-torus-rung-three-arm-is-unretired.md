---
id: torus-torus-rung-three-arm-is-unretired
kind: issue
title: Torus × torus routes to rung 3 and refuses: no ℝ³ implicit-pair door or tangency decision for the pair
status: open
opened: 2026-10-27
priority: P0
cost: M
refs: [torus-germ-pairs-have-no-section-frame, torus-certificate-runs-root-free-through-its-quartic]
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
