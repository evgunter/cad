---
id: iso
kind: program
title: ISO — the trimmed and rational lane: iso derivation, composite rounds, and the loft seam's exact compare
status: open
opened: 2026-09-20
area: kernel
prefix: iso/
tag: (ISO orchestrator)
ab_band: 7700-7799
paths: [crates/geom-brep/src/edge_nurbs.rs]
keep_out: [opened by CHART's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CHART measured 105 budget points and was cut into four tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CHART SSI PCERT ISO and they share crates/geom-brep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TRIM keeps nurbs_iso.rs and docs/PCURVE-P2-SPEC.md until its exit, SHELL owns editor-core/src/clearance.rs, TESS and CHORD own crates/mesh, QUAD owns props/quad.rs]
priority: P1
---
The trimmed lane and the rational surfaces under it: `nurbs_iso`'s
derivation arms, the trimmed quadrature's composite rounds, the
rational gates, and the exact knot compare that makes `loft_body`
refuse at degree >= 2 on generic parameterizations.

The unifying fact is that each of these assumes something simple about
a domain that is not simple: that an edge spans the chart's whole
domain, that a weight net is separable, that "is this rational" can be
answered by testing weights against 1.0 rather than against constancy,
and that two carriers are the same curve iff their knots compare
bit-equal.

Charter and order: `work/iso/plan.md`; narrative in `work/iso/log.md`.
