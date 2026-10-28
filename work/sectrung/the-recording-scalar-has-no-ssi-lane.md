---
id: the-recording-scalar-has-no-ssi-lane
kind: issue
title: The recording scalar has no SsiLane: Probe refuses GermSectionUnbuilt NoLane where f64 marches, and the door census admits no door formed at f64 and Probe
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Found by the dual review of `the-section-of-a-surface-pair-is-one-object`
(PR 5872, 2026-10-27).

## Measured

- `topo::AtRestPolicy::ssi_lane` is `None` at `geom_core::Probe`
  (`crates/topo/src/props.rs`), as `offset_fit_lane` and `section_lane`
  are, the marches being written at `f64` alone. So a Boolean whose
  crossing cylinder × sphere pair needs a carrier refuses
  `GermSectionUnbuilt { why: NoLane { scalar: "telemetry probe" } }` at the
  recording scalar, where `f64` marches it. The recording scalar is
  documented to carry exactly what `f64` carries.
- A lane at `Probe` is a second constructor of `SsiLane`
  (`SsiLane<Probe>::probed`, holding a march that reads the pose's
  values at `f64` and lifts the fitted branches back). It was written in
  the review's fix pass and taken out again, because the door census
  (`crates/topo/tests/certified_enclosure_impl_census.rs`) admits no
  such door: `every_door_fn_pointer_field_is_written_by_its_constructor_alone`
  requires each fn-pointer field written once, in a door's one
  constructor, and a door's `Formed` is either every certifying scalar
  or `f64` alone. A single constructor generic over a sealed "marches"
  trait is read by the door reader as an unbounded door, which agrees
  with neither.

## What a fix has to supply

The census taught a third shape, a door formed at `f64` and the
recording scalar, with one constructor per scalar and the wiring helper
pinning each, and then `SsiLane<Probe>::probed` restored (its body is in
PR 5872's history). Cost E: a `Formed` variant, its reader row and the
lane.
