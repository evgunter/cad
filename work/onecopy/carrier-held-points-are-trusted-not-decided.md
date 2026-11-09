---
id: carrier-held-points-are-trusted-not-decided
kind: issue
title: Decide-and-fault for carrier-held points (OnCarrier): the trusting placement_on_carrier sites skip the on-plane decision and read a curved OffCarrier by their own policy
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
pr: 6669
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

## Resolution (`onecopy/held-points`): option (b), the trust stated as a type

Ruled (b) by the orchestrator, on this measurement. `topo::held::Held`
is a point its caller holds on a face's carrier, minted by a
constructor that names the premise (`Held::constructed(at, by)`,
`Held::decided(at, margin)`, `Held::in_band(at)`). The boolean's
`placement_on_carrier` takes one and checks it in debug builds
(`Held::check`, unlogged under F38, skipped where the residual is
poison: a spline or approximated carrier). `placement_unheld` is the
entry for a point not held, where `OffCarrier` is a reading:
`face_placement`, reduce's `spline_interior` ends, and vtxfac's spline
arm. Census `placed` and cell_region `place` check their own points
with the same `Held::check`.

### The measurement

An env-gated probe recorded, at every carrier-held placement, the
point's implicit residual against the face's carrier, decided unlogged
at the caller's band, and the site's answer. It ran over topo, sweep and
editor-core on the `ci` profile (9551 tests, all green), at each ε.
Branch `onecopy/held-points-probe`, not for merge.

| Site | Calls (1e-9 / 1e-12) | Kinds; scalars | Read |
|---|---|---|---|
| census `placed` | 40,379 / 40,379 | Plane, Cylinder, Sphere; f64 | 100% Zero |
| cell_region `place` | 10,970 (1e-12) | Plane, Cylinder, Cone, Sphere, Torus; f64, Interval, Sym | 100% Zero |
| fate, a cut's end | 174 / 174 | Plane; f64 | 100% Zero |
| fate, the ring ladder | 34 / 34 | Plane; f64 | 100% Zero |
| carrier_touch `ball_off_face` | 314 / 313 | Cylinder, Sphere, Torus, Plane; f64, Interval | 100% Zero |
| reduce `vertex_on_face` (caller decided) | 176,448 / 174,989 | f64, Interval, Dual | Zero but 2 Nurbs (poison) |
| reduce `vertex_on_curved_face_at` (caller decided) | 27,477 / 27,256 | f64, Interval | Zero but 1 Nurbs (poison) |
| vtxfac `held_reads`, plane arm | 3,881 / 3,889 | Plane; f64, Interval | Zero but 10 / 8 in band, the caller's intended in-band path |

No NURBS face reaches a trusting site. Deciding there would have moved
no outcome, and added about 51,000 logged decisions per run.

### What flips the choice to (a)

Decide-and-fault (an `OnCarrier` minted by one deciding door) is the
answer the day either happens:

- a NURBS or approximated face reaches a trusting site, where the debug
  check is skipped and the trust is unchecked;
- any trusting site reads a point non-`Zero` (the debug check panics on
  it), i.e. a construction stops holding its point on the carrier.
