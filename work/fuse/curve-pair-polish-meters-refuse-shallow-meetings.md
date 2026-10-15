---
id: curve-pair-polish-meters-refuse-shallow-meetings
kind: issue
title: The curve-pair reader's polish meters refuse curves crossing in nearly parallel planes and curves crossing below the tangency floor
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [the-curve-pair-reader]
---


Found in the review of PR 5121 (stage (b) of `work/fuse/the-curve-pair-reader.md`).

**What refuses.** Across planes, `geom_brep::curve_pair::pose` places each candidate at the closest pair, polished by Gauss–Newton from where one curve crosses the other's plane (`pose.rs`, `Polish::closest`). Its basin meter (`curve_pair_pose_basin`) refuses `Unsettled::Meter` unless `sin φ − 2(κ₁ + κ₂)·2(ℓ + e)` reads positive. `φ` is the angle between the tangents, `ℓ` the arc about the crossing within the band's escalation `e` of the other's plane, and `κ` each carrier's curvature near the start (`Side::curvature_near`). Two windows refuse, measured on unit circles at `ε = 1e-9`, `K = 10` (`meet_rows`):

- **Wide-angle crossings in nearly parallel planes.** Two circles refuse at tilts from `1e-8` to about `5e-8`, and a line and a circle from `1e-8` to about `2e-8`. Below `1e-8` the planes read as one and the pose escalates. Above about `1e-7` both decide. `ℓ ≈ e/tilt` there is a sizeable fraction of a radius, so the curves turn too far over it for the polish to be certified to find their one closest pair.
- **Narrow crossings, below the tangency floor `√(κ·e)`.** Two circles cut from one sphere cross at an angle about the planes' tilt. They refuse below about `2·10⁻⁴` (`circles_on_one_sphere_meet_on_both_at_any_tilt`). There the curves stay within `e` of each other over a run about `e/φ` long. The oracle's tilted family refuses on about 70% of its draws (log-uniform tilts `1e-8 … 1e-2`), and its shallow family on about 19%.

**Why P2, below the unit's P1.** Every pose here refuses typed and never misanswers. The walk, the only other method in the kernel, refuses `Exhausted` on the same poses (the oracle's tilted family: 1150 of 1600 draws at effort 40). So stage (c)'s callers lose no answer they have today.

**Routes.**
- Below the tangency floor, the answer the band allows is a contact over a run, not an isolated point. The pose could carry that run (a `Pose` arm for it), as `Coincident` carries an overlap, and read the run's ends on both spans.
- For wide crossings in nearly parallel planes, the basin could read `κ` over the polish's actual travel rather than over the whole run `ℓ`, with a second certificate that no other stationary pair lies in the run.
