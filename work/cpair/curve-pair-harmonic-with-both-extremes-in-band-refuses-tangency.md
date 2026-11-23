---
id: curve-pair-harmonic-with-both-extremes-in-band-refuses-tangency
kind: issue
title: The curve-pair reader refuses Tangency where a first harmonic's two extremes both read in the zero band
status: open
opened: 2026-10-14
priority: P3
cost: E
refs: [5227]
---


Found in the review of PR 5121 (stage (b) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)).

`geom_brep::curve_pair::pose`'s first-harmonic reader (`pose.rs`, `harmonic_roots`) refuses `Unsettled::Tangency` where both extremes read zero. The arms that reach it have already read the swing definite: the line in a conic's plane and the transverse height (`curve_pair_pose_swing`), or the circles' centres (`curve_pair_pose_centres`). So `hi − lo` is past `2K·ε`, and both ends can read zero only when each extreme's rounding charge (`Range::noise`) is itself past the band. That is a pose whose coordinates the band cannot resolve. The refusal names a tangency, but the cause is resolution.

**Why P3, below the unit's P1.** Only geometry far outside the model box reaches it, and the answer is a typed refusal.

**Route.** Refuse `Unsettled::Meter` there (the rounding charge is what fails). This is better taken with the split of `roots::Unsettled` into what consumers act on, which the unit's stage (c) list owes.
