---
id: curve-pair-line-beside-a-thin-conic-is-unsupported
kind: issue
title: The curve-pair reader refuses a line in a conic's plane when the conic is as thin as the band about the line
status: open
opened: 2026-10-14
priority: P3
cost: E
refs: [5227]
---


Found in the review of PR 5121 (stage (b) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)).

`geom_brep::curve_pair::pose`'s line × conic arm (`pose.rs`, `line_conic`) handles a line that lies in the conic's plane. It reads the conic's signed distance from the line, a first harmonic in the conic's angle. Where that harmonic's swing reads zero (`curve_pair_pose_swing`), the arm refuses `MeetFault::Unsupported` ("a conic the band reads as thin as the line it lies beside"). That pose is an ellipse whose short semi-axis, seen across the line, is inside the band: the whole conic lies along the line within `ε`. It then meets the line over a run, as one curve does.

**Why P3, below the unit's P1.** The pose needs a conic thinner than the band about a line in its own plane, which the band reads as a segment: geometry that almost always indicates a modelling mistake (D4 commitment 3). No caller answered it before stage (c): `carrier_cross::meetings`, which `meet` replaced, had no in-plane conic × line arm that placed such a run, and `carrier_cross` answers `Unread` on this refusal.

**Route.** Read it as `Pose::Coincident` over the run where the line's foot falls inside the conic's extent along it, with the conic's ends along the line as witnesses.
