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
stale key that the kernel is defective, and no conditional text is
true at every raise site either, so they state the fact and claim
neither a recourse nor a defect
(`euler::tests::corruption_refusals_end_in_the_kernel_defect_ending`
pins both halves).

## The driver case

The caller is usually a kernel driver, not the person at the GUI. In
all 11 feature-tree chains that carry one of these variants
(`crates/editor-core/tests/refusal_concision_chains.rs`'s
`FILED_NO_RECOURSE`: `Boolean/Join/Euler`, `Split/Finish/Euler`,
`Split/Join/Euler`, `Split/Reduce/CrossingInsertion`,
`Split/Reduce/Euler`, `Blend/Op`, `Extrude/Op`, `Loft/Euler`,
`Revolve/Op`, `Shell/Partition`, `Shell/Rim`) the keys come from the
extrude, revolve, loft, blend, split, Boolean or shell driver. A
recourse to "pass keys this body holds" speaks to nobody the user can
be, and `Shell/Partition`'s wrapper (`ShellError::Partition`, "could
not be partitioned out (kernel bug)") contradicts it outright.

Sampling 12 `StaleKey` raise sites (PR 3621's review, C2) found three
shapes:

- caller-only: `mekr_both_empty`'s `get_loop(target)`,
  `set_null_face_pair`'s face and loops (`null.rs`), `split_edge`'s
  `get_edge` (`split.rs`);
- torn-only: `euler_ring.rs`'s `parent_loop` reads, `split.rs`'s row
  `Stale`, `Body::rebased_carrier`, `splitting/classify.rs`'s
  half-edge read;
- driver-minted: `sweep`'s `extrude.rs` (`base.hes[j]`, which the
  driver minted itself, and its rim `get_edge`) and
  `revolve/full.rs`'s seam `c_plus`. Here the body is sound and the
  driver's bookkeeping is the defect, so "the body is torn" is false
  as well.

Neither direction below fixes the driver case: keys a kernel driver
mints are a caller's keys under both, so the caller's variant would
still answer them. Only the wrapper (`ExtrudeError::Op`,
`ShellError::Partition`, …) knows the caller was the kernel, so the
wrapper has to say so, whichever way the variant splits.

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
