---
id: splice
kind: program
title: SPLICE — a body built by loft or sweep enters every boolean: the spline-edged operand, its carrier's parameter inverse, the spline face read on its own sheet, and the seam and kind-based readings between spline walls
status: ready
opened: 2026-10-25
group: kernel/boolean/join
prefix: splice/
tag: (SPLICE orchestrator)
paths: [crates/topo/src/boolean/reduce.rs, crates/topo/src/boolean/sectors.rs, crates/topo/src/boolean/join.rs, crates/topo/src/chord_join.rs, crates/editor-core/src/names/emit_union.rs, crates/geom/src/curves.rs, crates/topo/src/boolean/vtxfac.rs, crates/geom-brep/src/curve_face/*, crates/topo/src/boolean/spline_region.rs, crates/geom-brep/src/dihedral.rs, crates/topo/src/boolean/rim_wedge.rs]
keep_out: [opened 2026-10-25 by a refold approved in chat - the rows moved by git mv with ids and bodies unchanged from ORBIT ROOTS GAUGE REACHHOLD HONE APEX CLEAVE and SEAMRANK, each row carries a dated Moved note, split 2026-10-28 into SPLICE SPLICECARRIER SPLICEFACE and SPLICESEAM (Track size; Ev, in chat) and merged back 2026-11-02 when the budget came to count only P0 P1 and unbanded rows (docs/doc-ledger/splice-successors-merge-back.md), SECT owns the section-first join and the per-kind-pair sections it reads (fork row 100, "[ev] PR 4313) - a NURBS face against a curved face is a section at C5's rung 3 and is built there (SECTARMS)", not here as a stopgap, so SPLICE's demo rows that pierce a curved wall (klein wall 3, teapot wall 3, lily wall 8) need SECTARMS' rung-3 section as well as this slate, spiric operand edges stay deferred on the NURBS/spiric operand fork (REACHHOLD spiric-operand-edges-reopen-with-their-first-producer), crates/topo/src/boolean/* and crates/geom-brep/src/curve_face/* are shared ground with SECT SECTARMS ORBIT FLUSH ZIP FUSE GERM CONTACTHOLD TANG CLEAVE BOXES PIN by design - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**A body built by loft or sweep can enter every boolean.** Every loft
or sweep has spline (NURBS) seam edges, and until 2026-10-25 the
boolean's operand gate (`boolean::reduce::gate_operand_edges`) refused
an operand with any spline edge, whether or not that edge was anywhere
near the other body. The gate is gone, and the slate is what a spline
edge or face still meets at the doors behind it. Three demo joins were
its witnesses: klein wall 3 (the loop onto the bulb), lily wall 8 (since
`[ev]` PR 5990, the long leaf grown out of the stem) and teapot wall 3
(the spout onto the pot). At the 2026-11-01 hand-back lily wall 8 stops
at a plane × torus spiric germ (SECT's
`c5-plane-torus-cone-cylinder-arms`) and klein wall 3 at the flare cone
seam on the loop tube's double knot (CARVETAIL's
`the-skin-cuts-an-arc-wall-at-each-sub-arc-joint`): neither is SPLICE
ground.

The track runs four threads. They were four tracks from 2026-10-28
(Track size; Ev, in chat) until they merged back on 2026-11-02, when the
budget came to count only the rows a sitting dispatches:

- **The operand's own path through the doors**: the interior-loop
  guard's spline × curved analytic arm, a rim end inside its carrier,
  doors that carry a shared row, the pair walk's domain corners and the
  two-chart tube.
- **A NURBS carrier has no parameter inverse** (teapot wall 3's
  thread). Several of the boolean's later doors ask the carrier for
  something only an analytic carrier answers: a parameter for a point
  (`Curve3::param_near`), a period, a run-on along the curve. The join
  cannot run a kept edge on along it, a meeting the boundary pass found
  on it has no split parameter
  (`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`, where
  teapot wall 3 stopped at `PointSplitCarrierUnsupported`), the flush
  rule's track reads every point as off it, and the touch floor reads
  its arc at the carrier's whole speed bound. This thread owns
  `Curve3::param_near` and its callers.
- **A spline face read on its own sheet** (klein wall 3's thread). An
  end placed in space rather than on the sheet, a derivative read at a
  foot no one placed, a meeting on a closed chart's seam or a pole, a
  crease at a G1 rational-arc joint, a creased spline's second-order
  clearing, the analytic crossing doors outside the curve × face table,
  a projection that accepts a zero-speed stationary point, and a spline
  wall that cannot say whether it holds a pierce point.
- **The seam between spline walls, and the doors that read a spline by
  its kind** (each row names its own reproducer). PR 5868 read the
  result's seam between two spline walls (`classify_seam`); its residue
  is a smooth seam's second-order reading, a creased chart's arm, the
  chart-curvature bound that over-reads a flat chart at uneven speed,
  the arm's per-read cost and a closed chart's seam continuity. Beside
  it are the doors that still refuse or misread a spline face or edge by
  its kind rather than its geometry: the shared-rim routing, the slit
  pass's rider, the sweep's curved arm, the section-area certificate,
  the split's section boundary, the refusal that names no edge, the
  conic-named departure types, and a lofted operand's quadrature budget
  at 1e-12.

Slate, order and posture: `work/splice/plan.md`; narrative in
`work/splice/log.md`.
