---
id: stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body
kind: issue
title: StaleKey, StaleGeometry and NotSameEdge are one variant each for a caller's key and a torn body, so reports_tier1_corruption answers true for a caller's mistake
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
refs: [euler-op-corruption-refusals-end-in-a-tag]
---


(TOPO implementer, from `euler-op-corruption-refusals-end-in-a-tag`.)

## What

Three `EulerOpError` variants (`crates/topo/src/euler.rs`) are raised
both for a caller's mistake on a sound body and for a torn body, and
nothing in the value says which:

- `StaleKey`: an argument key that does not resolve (a key the caller
  kept past a kill, or another body's), and a key the plan read from a
  record (`next`, `prev`, a spine parent, a start). The shared lookups
  serve both: `Body::resolve_half_edge`, `Body::require_live`
  (`live.rs`), `Body::resolve_vertex_point`, and the `get_*`
  `.ok_or(StaleKey { .. })` sites, about 170 production raises across
  `euler.rs`, `euler_kill.rs`, `euler_ring.rs`, `attach.rs`,
  `movefac.rs`, `merge_faces.rs`, `null.rs`, `split.rs` and
  `splitting/classify.rs`, plus `sweep`'s own.
- `StaleGeometry`: a `FaceSurface::Shared` key the caller passed, and
  a point or surface key a record holds. `readback::DanglingRef` keeps
  the two apart (`Entity` against `Geometry` reached from a live
  entity) and its `From` maps both across onto these two variants.
- `NotSameEdge`: `kemr`'s two arguments equal, or halves of two edges
  (`Body::kemr`, `euler_ring.rs`, before `require_halves`), and every
  arm `require_halves` refuses through `Body::proven_mate` (`kev`'s
  and `kef`'s plans, `movefac`), which only a torn bijection reaches.

`EulerOpError::reports_tier1_corruption` answers `true` for all three,
which is right for a driver passing keys it read from the body
(`merge_faces`' `OpPlacement` delegates them) and wrong for a caller's
stale key. The unit that unified the corruption endings
(`euler-op-corruption-refusals-end-in-a-tag`) could not give these
three `geom_core::KERNEL_DEFECT_ENDING` without telling a caller with a
stale key that the kernel is defective, so they end in the caller's
repair and then the report ("Recourse: pass keys this body holds; if
the call did, the body is torn, which is a kernel defect: report it"),
pinned by `euler::tests::corruption_refusals_end_in_the_kernel_defect_ending`.
The repair reads oddly where the caller is a kernel driver (an
`ExtrudeError::Op` wrapping a `StaleKey` tells the person at the GUI to
pass keys).

## The question

Split each into a caller's variant and a torn one, so
`reports_tier1_corruption` answers by variant and the torn half ends in
`KERNEL_DEFECT_ENDING` with the rest of the class. Two directions:

- a caller's variant raised only where an operator resolves its own
  arguments, leaving the existing name on every key read from a record
  (fewer sites, but each public door's argument resolution has to be
  found, and a missed one keeps saying "torn");
- a torn variant raised wherever a key comes from a record (every
  shared lookup takes a provenance).

`NotSameEdge` is the cheap one: the caller's arm is `kemr`'s pair
alone. Either direction has to place the new variant in
`merge_faces`' `OpPlacement`, which calls `kemr` and the kills with
keys it read from the body.
