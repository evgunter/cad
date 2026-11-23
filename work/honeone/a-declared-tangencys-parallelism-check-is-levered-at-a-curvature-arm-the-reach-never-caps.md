---
id: a-declared-tangencys-parallelism-check-is-levered-at-a-curvature-arm-the-reach-never-caps
kind: issue
title: Certification's tangent_normal_parallel is levered at 1/|κ_rel|, an arm the faces' reach never caps, so it refuses a declared tangency the wall join reads smooth
status: open
opened: 2026-11-23
priority: P2
cost: M
design: true
refs: [the-wedge-reach-counts-a-faces-run-along-its-edge]
---


Found by ONEROUND's turn-lever measurement lane (2026-11-23; `analysis/oneround/turn-lever/g2.md` on `oneround/turn-lever`, `7b62af0248`, the sections "The open row's extrude table again" and "`tangent_normal_parallel`'s arm"), read in the code by the lane and not by me.

In `run_checks` (`crates/geom-brep/src/certify.rs`), certification's normal-parallelism check for a declared tangency decides `Margin::levered_inv(sin θ, |κ_rel|)` under the name `tangent_normal_parallel`: sin θ over the relative curvature, an arm of 1/|κ_rel|. Nothing takes the lesser of that arm and the reach of the faces it is read on, so the arm can be longer than the region. Only the renaming path (`named_by_parallelism`, through `folded_parallelism`) uses the folded arm.

**What it does today, and what it will do.** The lane's probe is a profile of a line 0.05 long running into an arc of radius 1 that sweeps 0.05 rad, φ off tangent, extruded and revolved, at ε 1e-9. On main the wall join's wedge is levered from the edge's middle (`the-wedge-reach-counts-a-faces-run-along-its-edge`) and at φ = 15 ε and 19 ε a depth-4 extrude builds a sharp edge that tier 3 passes. With the wedge levered from the edge's line (ONEROUND's turn-lever fork, both designers' recommendation, not yet with Ev), the join reads the true lever, about 0.05, and reads the declared tangency smooth; this check then refuses the body ("definitely exceeds the tolerance band") at its own arm of 1 m, at both depths. So the wedge row does not close with the line site: these bodies go from wrongly built to refused, where the profile accepts them.

**The question (`design`).** The form is D4's tangency lever, so it is ratified form and not a slip. What is open is which displacement a declared tangency holds against the tolerance: the locus's offset, sin θ / κ, or the faces' separation over the reach they actually have. `crates/geom-core/README.md` ("Float rounding", the lever bullet) says a lever is "no longer than the site's own measure of it". One of ONEROUND's two designers expects this to end as an `[ev]` question of its own.
