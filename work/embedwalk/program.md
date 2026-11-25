---
id: embedwalk
kind: program
title: EMBEDWALK — the walk under the embedding certificate; a face's window and extent, the cells it drops, and where an ask stops
status: active
opened: 2026-11-22
group: api/assembly
prefix: embedwalk/
tag: (EMBEDWALK orchestrator)
paths: [crates/topo/src/clearance.rs]
keep_out: [cut from EMBED on 2026-11-22 after Ev asked in chat whether a chunk could be split off (EMBED at 118 against 30) along the seam between what a reading decides about one cell pair and the walk that brings cell pairs to it - the rows moved by git mv with ids and bodies unchanged, EMBED keeps crates/topo/src/injective.rs (the presentations and separators) and their margins, EMBEDGATE the gate that asks, CLEAR keeps topo::clearance's metric questions and shares clearance.rs by design, the region readers (sphere_region.rs, torus_region.rs, contain.rs, cell_region.rs) are seams this track touches by announced PR - run scripts/work.py territory on your branch and announce the seam in the PR]
---
**The walk under the embedding certificate.** `topo::clearance::embedded`
halves the two faces' windows into cells and hands each cell pair to a
reading (`crates/topo/src/injective.rs`, EMBED's). This track owns what
happens around the reading:

- **A face's window and extent**: which carriers have a window at all
  (a spindle sheet and an approximated face have none), where a
  coordinate line sits on a face of unknown extent, a face that covers
  its carrier more than once.
- **The cells the walk drops**: a cell that holds no point of its face
  is dropped by the region reading, and where the region reading
  answers nothing (a sphere or torus face bounded by a spline edge) the
  walk halves toward the carriers' crossing and spends its budget.
- **The budget and where an ask stops**: a failed local certificate
  answers "split" with no ground for expecting a smaller cell to do
  better, and the depth gap that parts faces sharing nothing.

Slate and order: `plan.md`. The measure is EMBED's: the per-solid
census through the gate (`origin/embed/gate-measure`) and the tour walk
(`cargo run --example embed_walk`, PR 7127).
