---
id: certify-residual-predicates-still-name-a-slot
kind: issue
title: certify's residual predicate names (carrier_on_surface_1/2, witness_on_surface_1/2, tangent_on_surface_1/2) still name a slot of the surface pair
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself, rule-g-trades-sixteen-of-the-links-carrier-on-surface-2]
---



## Finding

- **Where**: `crates/geom-brep/src/certify.rs`, `EdgeCurve::certify`'s
  residual schedule — the `check_residual` calls whose predicate names
  are `carrier_on_surface_1` / `_2`, `tangent_on_surface_1` / `_2` and
  `witness_on_surface_1` / `_2`.
- **Raised by**: CARVE's `surface-pair-is-unordered` lane, 2026-10-06,
  while making `EdgeDescription::Intersection`'s pair a set.

The pair is now a `SurfacePair` and the checks those calls feed name
their surface by key (`CertCheck::SurfaceResidual { surface }`), but
the `k_stats` predicate names were left as they were: `_1` and `_2`
now mean "the lower key" and "the higher key" of the pair. Before, they
meant whichever surface the minting builder happened to name first.

So the names are deterministic now, and still say a slot. Two things
read them by name:

- tallies in prose and tests: `demos/tour/src/tolerance.rs`'s
  prose (`carrier_on_surface_2`, 72; `witness_on_surface_2`, 8),
  `crates/editor-core/src/drive.rs`'s doc, and
  `crates/editor-core/tests/m10_bulge_interval.rs` and
  `edit_refusal_recourse.rs`.
- DECIDE's measured rows, e.g.
  `work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2.md`,
  which tabulate one slot's decisions. Where an edge's builder named
  the higher key first, its decisions now count under the other name,
  so those tables were measured under an order that no longer exists.

Not renamed in the unit because a rename moves every one of those
tallies at once and has nothing to do with the pair being a set.

## The question

Either merge each pair into one predicate name (`carrier_on_surface`,
its count the sum, which is what a set-shaped pair reads as), or keep
two names and say in `certify.rs` that they are key order. The first
fits the type; it re-baselines every per-predicate tally above.
