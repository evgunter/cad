---
id: tol-and-band-travel-as-a-redundant-pair-on-the-offset-fit-doors
kind: issue
title: (tol, band) travel as a redundant pair through the offset-fit doors and the transform's surface map, so a pair that disagrees is representable
status: open
opened: 2026-09-26
priority: P3
cost: D
---


## The class

The offset-fit doors take the run's ε twice: once as the `Tol` witness
and once as a `Band`. Every production caller derives the band from the
same witness with `Band::linear(tol)`. Nothing ties the two together, so
a call whose band came from a different ε than its `tol` type-checks.
Such a call would classify the limbs at one ε and meter the regularity
floor and the collapse reach at another.

Sites (cited by name):

- `geom_brep::OffsetFitLane`: the `mint`, `recertify` and `remap`
  methods and their bodies (`crates/geom-brep/src/offset_fit_lane.rs`).
- `geom_brep::offset_fit`'s `Tol` doors: `fit_offset`,
  `certify_offset`, `certify_offset_over`, `approx_offset_surface`,
  `recertify_approx` (`crates/geom-brep/src/offset_fit.rs`).
- `topo::transform`'s `map_surface` and `map_approx`, which receive both
  from `transform_rigid_via`. There the band is
  `Band::linear(tol)` (`crates/topo/src/transform.rs`).
- The tier-3 arm in `topo::validate` that calls
  `OffsetFitLane::recertify` with the band it built from `tol`.

## What is open

Pick one carrier for the run's ε on these doors. For example, the door
could derive the band from `tol` itself, or take a type that holds both
and can only be built from one witness. After that, a pair that
disagrees can no longer be constructed. Found in PR 3274's style review
(retiring `SurfaceSpec::tolerance`), where `map_surface` and
`map_approx` gained the `tol` beside their existing `band`.
