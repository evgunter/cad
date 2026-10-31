---
id: spanfold
kind: program
title: SPANFOLD — the quadrature's span folds and loop-area arms: refusals dropped through f64::min, chord lengths rooted to nearest, and pieces no arm takes
status: ready
opened: 2026-10-27
group: kernel/props
prefix: spanfold/
tag: (SPANFOLD orchestrator)
ab_band: 15500-15599
paths: [crates/geom-brep/src/props/quad/trimmed.rs, crates/geom-brep/src/props/loop_area.rs, crates/geom-brep/src/props/rational_rim.rs]
keep_out: [opened 2026-10-27 when FLUX (51 budget points) and FLUXTAIL (44.5) were joined and re-cut by logical thread rather than by priority so that the successors can run in parallel (Ev in chat 2026-10-27) - the rows moved by git mv with ids and bodies unchanged and both directories left the tracker (docs/doc-ledger/flux-leaves-the-tracker.md), crates/geom-brep/src/props/quad/* is QUAD's ground and shared with it - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**Where a quadrature or loop-area fold loses a refusal or a sign.** The trim walk's span folds go through `f64::min` and drop a refused step or window; its chord lengths are rooted to nearest and read as a pad. A NURBS loop-area segment integrates an inverted window as zero, a spline carrier past the Gauss inventory has no composite arm, a rational rim with a near-zero weight is not live on equal pieces, and a cylinder split just off its rim vertex reads its sliver wide. The gating corpus reaches none of the collapsed-arm gates that would catch this class.

Slate, order and posture: `work/spanfold/plan.md`; narrative in
`work/spanfold/log.md`.
