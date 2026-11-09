---
id: carrier-held-points-are-trusted-not-decided
kind: issue
title: Decide-and-fault for carrier-held points (OnCarrier): the trusting placement_on_carrier sites skip the on-plane decision and read a curved OffCarrier by their own policy
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Filed by `onecopy/face-placement` (row
`the-face-placement-dispatch-has-four-more-copies`), as the
outcome-moving half that row's fix left, by the orchestrator's ruling
(option (b) there).

## The finding

`boolean::contain::placement_on_carrier` is the one dispatch for a
point the caller holds on a face's carrier. On a plane it decides no
on-plane margin; on a curved face it returns `OffCarrier` as data, and
each caller states its policy at the call. Grep `OffCarrier →` for
every policy line. The row's proposal: a point type that says it is
held on its carrier (`OnCarrier<T>(Point3<T>)`), minted only where the
construction holds it, read by a door that decides the on-plane margin
once and maps a definite `OffCarrier` to a typed fault carrying that
margin (as `rim_wedge`'s `RimUnread::Fault` does today).

**The trusting sites** (no on-plane decision; curved `OffCarrier` read
as Out):
- `topo/src/census.rs` `placed` and `topo/src/cell_region.rs` `place`.
  These two still spell their own split over `contfp` and
  `curved_face_containment`, because the dispatch is engine-private
  until the containment door moves to the root
  (`work/boxes/the-face-covers-sphere-arm-reads-the-booleans-chart-trim-and-face-placement.md`).
  `crate::boolean::face_placement`, which census already reads from
  the root (`census::face_holds`, `census/in_plane.rs`), is not
  theirs: it decides the on-plane margin, and these two trust the point
  on the plane (the follow-up row's subject); making them decide it is
  that row's outcome-moving change, not a reroute.;
- `topo/src/boolean/fate.rs`, a fate cut's end in a divided face's
  pieces;
- `topo/src/boolean/fate.rs`, the ring-vertex ladder;
- `topo/src/boolean/carrier_touch.rs` `ball_off_face` (OffCarrier →
  not clear).

**The sites whose caller already decided the margin** (to be handed
in, not decided twice):
- `topo/src/boolean/reduce.rs` `vertex_on_face` (the caller's
  vertex-vs-plane classification, `on_plane`) and
  `vertex_on_curved_face_at` (`on_carrier`);
- `topo/src/boolean/vtxfac.rs` `held_reads`' two arms (its `read_face`)
  (`bool_vertex_face_side`, `bool_curved_contain_carrier`).

**What it costs.** A new logged decision on every trusting planar arm.
At `Interval`, a constructed point's enclosure can sit in the band of
its own plane, so new escalations and refusals appear where nothing is
decided today. Curved sites that read `OffCarrier` as Out would fault
instead. Outcomes and k-stats move at every site; read each.

## Fix

Mint `OnCarrier` at each construction above, give
`placement_on_carrier` that argument and the decision, and hand the
already-decided margins in at reduce and vtxfac.
