---
id: spliceseam
kind: program
title: SPLICESEAM — the seam reading between spline walls, and the doors that still read a spline by its kind
status: ready
opened: 2026-10-28
group: kernel/boolean/join
prefix: spliceseam/
tag: (SPLICESEAM orchestrator)
paths: [crates/geom-brep/src/dihedral.rs, crates/topo/src/boolean/rim_wedge.rs]
keep_out: [opened 2026-10-28 by SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28) - the rows moved by git mv with ids kept, each with a dated Moved note, SPLICE keeps lily wall 8's path (since 2026-10-29 the long leaf grown out of the stem - the curved crossing lane's spline dispatch then SECT's NURBS x torus section), crates/topo/src/boolean/* and crates/geom-brep/src/curve_face/* are shared ground with SPLICE SECT and the boolean programs by design - run scripts/work.py territory on your branch and announce the seam in the PR]
---

PR 5868 read the result's seam between two spline walls
(`classify_seam`), bracketing a spline face's arm between its certified
lower bound and the reach. Its residue is here: a smooth seam's
second-order reading, a creased chart's arm, the chart-curvature bound
that over-reads a flat chart at uneven speed, the arm's per-read cost,
and a closed chart's seam continuity. With it, the doors that still
refuse or misread a spline face or edge by its kind rather than its
geometry: the shared-rim routing, the slit pass's rider, the sweep's
curved arm, the section-area certificate, the split's section
boundary, the refusal that names no edge, the conic-named departure
types, and a lofted operand's quadrature budget at 1e-12.

**Witness:** none of the three demo walls stops here today; each row
names its own reproducer.

Opened from SPLICE, whose log (`work/splice/log.md`) carries the
history of every row here. This track owns the spline seam reader and the kind-based spline refusals.
