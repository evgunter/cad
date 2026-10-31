---
id: nearpoint
kind: program
title: NEARPOINT — the closed forms' reference point and frame: sums about a far origin, rim levels that cancel, and reads that depend on the frame
status: ready
opened: 2026-10-27
group: kernel/props
prefix: nearpoint/
tag: (NEARPOINT orchestrator)
ab_band: 15300-15399
paths: [crates/geom-brep/src/props/curved.rs, crates/geom-brep/src/props/mod.rs, crates/geom-brep/src/props/closure.rs]
keep_out: [opened 2026-10-27 when FLUX (51 budget points) and FLUXTAIL (44.5) were joined and re-cut by logical thread rather than by priority so that the successors can run in parallel (Ev in chat 2026-10-27) - the rows moved by git mv with ids and bodies unchanged and both directories left the tracker (docs/doc-ledger/flux-leaves-the-tracker.md), crates/geom-brep/src/props/curved.rs is shared with CONEARM (which has the cone arm's region and section reads) and with QUAD and PRED - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**Sound closed forms whose readings are not.** Each curved and planar arm computes the right quantity, then reads it about a point or in a frame that costs it precision: a planar face summed about a far carrier origin, a cylinder wall about its axis origin, a cone wall about its apex, a sphere's flux cancelling R·Area against c·A. Rim levels are recovered by a dot product far from the origin or by √(1 − s²) near a pole. Rim incidence, the sphere side's in-arc test and vector-norm bounds are decided one coordinate or one lever at a time.

Slate, order and posture: `work/nearpoint/plan.md`; narrative in
`work/nearpoint/log.md`.
