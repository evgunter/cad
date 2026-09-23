---
id: ssi
kind: program
title: SSI — the plane×NURBS surface intersection: its bounds, its tubes and the diagnoses that survive a bad one
status: ready
opened: 2026-09-20
area: kernel
prefix: ssi/
tag: (SSI orchestrator)
ab_band: 7500-7599
paths: [crates/geom-brep/src/ssi.rs, crates/geom-brep/src/ssi/*]
keep_out: [opened by CHART's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CHART measured 105 budget points and was cut into four tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CHART SSI PCERT ISO and they share crates/geom-brep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TRIM keeps nurbs_iso.rs and docs/PCURVE-P2-SPEC.md until its exit, SHELL owns editor-core/src/clearance.rs, TESS and CHORD own crates/mesh, QUAD owns props/quad.rs]
priority: P0
---
The marching-and-certifying lane, as its own track. DESIGN.md's banked
principle is that **marching finds and subdivision certifies
exhaustiveness** — the outcome is "every branch found" or a typed
failure, never silence — so the bounds this lane computes are the
proof, not an optimisation.

Three of its rows say the proof does not hold: the curvature lever arm
is folded with `f64::min`, so a poisoned operand can pass as the
smaller of two; the chart-stretch divisor is a raw `sqrt` of an `f64`
fold, so it is not rounded outward; and the uniqueness tubes are padded
by `max(su, sv)` on BOTH chart axes where limb 3 proved a per-axis
bound. A fourth says wrong diagnoses survive at finite-but-unusable
speeds, which is the usability boundary nobody has drawn.

Charter and order: `work/ssi/plan.md`; narrative in `work/ssi/log.md`.
