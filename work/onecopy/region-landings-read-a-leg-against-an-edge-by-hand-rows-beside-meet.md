---
id: region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet
kind: issue
title: topo::boolean's region landings read a leg against a boundary edge by hand rows (plane crossing, collinear test, first-harmonic roots) beside curve_pair::meet
status: dispatched
opened: 2026-11-03
priority: P1
cost: M
refs: [boundary-cuts-reads-line-against-conics-through-their-planes]
branch: onecopy/meet-pose
---

Found by the full review of PR 6451, which moved `boolean::boundary_cuts` onto the boundary pass.

The region walks' `landings` answer where a walk's leg meets a boundary edge with their own rows:
- `ruled_region.rs` `landings`;
- `region.rs` `landings`;
- `torus_region.rs` `landings`;
- `sphere_region.rs` `landings`.

The rows are a plane crossing, a collinear test and first-harmonic roots. That is the curve-pair meeting question, which `geom_brep::curve_pair::meet` answers (`LineLine`, `LineConic` and the walk) under DESIGN `[one-way.readings]` and C5, and the two readings can decide one meeting differently in the band.

Fix: read each leg against each edge through `meet` (or the boundary pass), after checking what each `Landing` carries that a `Meeting` does not.


## Decided (2026-11-03, two designers, agreed on first reports)

The design-fork procedure weighed this; the draw is on `analysis/design-fork/onecopy-landings`. Both designers recommend the same final state. Neither fork option PR 6477 named is taken: a surface argument to `meet`, or an on-surface reader beside it, would be a second predicate for one fact and would extend C5. This elaborates `[one-way.readings]` and C5 as written, and lands without Ev.

**Premise correction.** The hand rows do not decide "through the on-surface premise". `A ∩ B = A ∩ P_B` is a set identity: it changes the form of the candidate generator, not what the band resolves. One designer reproduced both reds on `meet` itself, in both argument orders:
- **Slim lune.** The second leg (a parallel tilted `off` from the equator) against the equatorial arc. The arc's plane is ≈6e-4 away, a plain miss. `meet` escalates `curve_pair_pose_swing` (swing 1.8e-6 … 8.7e-6, in the gap) before it reads the offset. The rows read the extremes `c0 ± A1`, which decide the miss whatever the swing reads.
- **Tiny cap.** A great-circle ray against the rim. `meet(ray, rim)` escalates `curve_pair_pose_extreme`: the ray's height above the rim's plane peaks at ~4e-9, in the gap. `meet(rim, ray)` decides both crossings. The rows read the edge over the leg's plane.

So `meet` loses only by its own decision order and its fixed parametrization.

**Final state.**
1. **`curve_pair::pose::conic_conic`: the parallel argument runs first, whatever the swing reads.** This is the line arms' own rule (commit 5d1b24e1fa). For each conic, `|c0| − swing − noise` decided Positive gives `Pose::Apart`. Only past that screen does the swing route: definite → transverse; zero → coplanar (unchanged); in the gap → escalate, which is then the honest answer.
2. **`transverse` reads the harmonic of the conic that crosses the other's plane less obliquely.** The choice is made by arithmetic before any decision: the larger `−lo·hi / r²`, with `r` the larger semi-axis. It is one method whose parametrization is a formula, the same kind of choice the torus door already makes (`bool_torus_region_family`). It is not a second reading reached because the first declined. `meet(a, b)` and `meet(b, a)` then agree.
3. **The four region `landings` read each leg against each edge through `meet`**, as the reverted attempt did: `Landing` from `Meeting`, and a shared run through `Meetings::shared`.
   - The hand rows go: the `circle_off_plane` crossings, the collinear test, the component rows, `FirstHarmonic` as the region doors use it, and the nappe/meridian/oval screens.
   - An escalation sets the path aside; a touch grazes. There is no fallback to any row.
   - Circle × spiric pairs on the torus go to `meet`'s certified walk. If they regress (budget, `Unsettled`), "conic against a planar curve as a closed arm" is an `[ev]` fork, because it changes C5's table.

**Rows to add:** an order-symmetry row (`meet(a, b)` ≡ `meet(b, a)` over the `meet_rows` fixtures); the slim lune's equator-against-tilted-parallel pair, which reads `Apart`; the tiny cap's ray × rim pair in both orders; and a torus meridian × parallel row near the tube's top circle. One designer's probe is at `~/.local/share/cad-work/oc-land-designer-a/oc_probe.rs` and can seed these.

Also: rename `Pose::Parallel` where it means two conics apart across a plane. Re-run every `meet` consumer's slow set (the boundary pass, `census`, `census/curved`, `ring_path`), since near-parallel conic pairs may move from an escalation to a decided `Apart`.
