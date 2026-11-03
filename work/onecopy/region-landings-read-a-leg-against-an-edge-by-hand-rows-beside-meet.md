---
id: region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet
kind: issue
title: topo::boolean's region landings read a leg against a boundary edge by hand rows (plane crossing, collinear test, first-harmonic roots) beside curve_pair::meet
status: open
opened: 2026-11-03
priority: P1
cost: M
refs: [boundary-cuts-reads-line-against-conics-through-their-planes]
---

Found by the full review of PR 6451, which moved `boolean::boundary_cuts` onto the boundary pass.

The region walks' `landings` answer where a walk's leg meets a boundary edge with their own rows:
- `ruled_region.rs` `landings`;
- `region.rs` `landings`;
- `torus_region.rs` `landings`;
- `sphere_region.rs` `landings`.

The rows are a plane crossing, a collinear test and first-harmonic roots. That is the curve-pair meeting question, which `geom_brep::curve_pair::meet` answers (`LineLine`, `LineConic` and the walk) under DESIGN `[one-way.readings]` and C5, and the two readings can decide one meeting differently in the band.

Fix: read each leg against each edge through `meet` (or the boundary pass), after checking what each `Landing` carries that a `Meeting` does not.


## Tried on `onecopy/meet-reads`: `meet` is not a drop-in, because the landings read a premise it does not take

What the callers need is what a `Meeting` carries. `region::offer` takes each landing's point, the edge's parameter there, and whether it lies within the band of an edge end. That is `Meeting::point`, `on_b`'s `Hit::On { t, .. }`, and `Hit::End`. A shared run is `Meetings::shared`, which today's `Roots::On` → `Leg::along` arms read. The leg is a bounded span once a ruling's far end is set past the edge's cover along it. A bounded leg's meeting at its own end is no crossing, as a cone's apex and an aimed path's corner already are.

So the lane routed all four doors through one `region::landings` over `meet`, gave each `Course` a `carrier`, and dropped `Legs::landings`. 216 of 218 region and point-in-solid rows passed. Two pinned rows went red:
- `sphere_region::tests::a_slim_lunes_far_pole_reads_out_by_the_two_leg_path` (the blend4 fillet corner: sphere radius 8.726e-4, band (1e-6, 1e-5)) reads `Err(Escalated)` at 3e-3 rad, predicate `curve_pair_pose_swing`.
- `sphere_region::tests::the_aim_lies_in_the_face`, its "tiny cap" (rim 1e-4 rad).

**Why.** The leg and the lune's meridian are two great circles at a small angle. `meet` reads them as free carriers in 3D. Each circle's swing above the other's plane is R·sin θ ≈ 3e-6, inside the band, so the conic-conic pose escalates. The door's row uses the premise that both curves lie on the sphere. Under that premise the meeting is the edge crossing the leg's plane, a first harmonic in the edge's own angle with its roots decided. Under the same premise a cone's nappe and a torus's two meridians per plane are screens that `meet` would not need.

**What is owed.** This is a design fork, for designers:
- `curve_pair::meet` takes a common surface, and reads the on-surface pair through it;
- or the four doors' hand rows become that reading's home in `geom_brep` (an on-surface curve pair), and the doors call it.

The hand rows are not a copy of `meet`'s question. They answer the narrower one, where two curves on one known surface meet, and only the premise lets them decide where `meet` escalates. A fallback from `meet` to the rows where `meet` refuses would answer where a reading declined, and is barred.
