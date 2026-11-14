---
id: a-rings-lift-is-placed-against-its-outer-loop-in-one-place
kind: unit
title: a ring's lift is placed in its outer loop's sheet by one rule for every chart, where spline charts alone have one today
status: spec
opened: 2026-11-14
priority: P0
cost: M
---


Unit 3a of the chart lane (`work/tesslane/plan.md`, The chart lane). Independent of units 1 and 2; unit 3 waits on it.

## What it is

Each loop's lift starts on one of its own rows (`pcurves::Lift`'s doc calls the start gauge), so on a chart with a closed channel a ring can be stored whole periods from the region its outer loop's lift bounds. Measured on door-built bodies (`crates/mesh/tests/lift_probes.rs`, `a_ring_sits_in_its_outer_loops_polygon_but_on_the_ball_less_the_tube`): 23 of 24 rings are stored inside, and ball ∖ tube's whole-circle ring is one `u` period off.

`Trim::with_rings` in `topo::boolean::spline_region` already places a ring, for spline charts only: by the one whole number of periods per closed channel that puts the ring's first point in the outer loop's cover, decided against the outer loop alone, `None` where no shift or more than one is decided. The unit makes that one rule the lift's, for every chart, with the spline reader calling it:

- a face's rings are answered in its outer loop's sheet by one door beside `Body::loop_lift`;
- the placement is decided the way the lift's joints are, through the kernel's band, and a ring no shift places, or two place, answers typed rather than being left where it is stored;
- a face whose outer loop does not close in its chart (a seamless band, built through the Euler doors) has no sheet to place a ring in, and answers that typed;
- the other multi-loop readers that give up on a ring today (`cover::TorusChartWindow::open_loop`, `chart_region::extract_face_uv`) are named in the PR with whether this door serves them; they are not rebuilt here.

## Seams

`crates/topo/src/pcurves.rs` is CHARTTAIL's and `spline_region.rs` is shared by SPLICE and the boolean programs. Announced on both logs at dispatch.

## Unit (2026-11-14, TESSLANE)

**Letter M. Tier: single review, FULL.** A whole-period placement that is wrong is silent: the ring lands a period away and every reader triangulates or integrates the wrong region. The review carries claims to falsify.

Branch `tesslane/ring-placement`, cut from `tesslane/measure-lift` so the lift probes are in hand. Done when the displaced ring's probe is re-pinned to read the ring inside through the new door, the 23 others still do, the spline reader answers what it answered before on its own suite, and a seamless band answers typed.
