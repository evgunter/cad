---
id: cylinder-and-torus-edge-misses-are-read-back-about-the-axis
kind: issue
title: The cylinder's and torus's per-edge whole-turn misses are read back about the axis, so their Interval width is charged as disagreement on a far generic-pose body
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by the FLUX 1e-12 main-reds unit (PR 5400).

The cone's `props_chart_loop_whole` escalated a legal ring and a steep wide frustum at ε 1e-12. Each cone edge's per-edge miss (`closure::EdgeTurn`, `crates/geom-brep/src/props/closure.rs`) was read back about the surface axis against the edge's own ends. At the interval scalar that read is about 1e-12 wide per edge, even where the true miss is exactly 0 (a generator, an ellipse's thirds) or O(tilt²) (a rim). The sum over the loop then passed the 3ε slack.

PR 5400 fixed the cone:
- `EdgeTurn::exact` for edges that agree with their ends by construction;
- `EdgeTurn::carried` for rims, whose miss is read in the rim's own frame, where the carry cancels.

The cylinder (`cylinder_windings`) and the torus (`torus_chart_loop`) still read every edge's miss back about the axis. No row is red today. The same width grows with the body's distance from the origin, though, so a far generic-pose cylinder or torus face with many edges could escalate the same way.

What closes it: give the cylinder and the torus the cone's two constructors (rulings and meridians exact; rims and parallels carried). Add a row: a far, generic-pose, many-edged face at 1e-12 that the old reading escalates and the new one measures.

## Claimed by BOXFINISH (2026-11-14)

Moved here from NEARPOINT by `git mv`, id unchanged: it is what
BOXFINISH's `f64` row
(`f64-tier-3-reads-volume-uncomputable-in-band-on-hairline-revolves`)
measured as its one decidable loss, and "no row is red today" no longer
describes it.

**Measured, 2026-11-14, main `b363f5fa`, `f64` doors at ε = 1e-12.** In
every case the aimed rays read `Side(Out)`, the `f64` round 0 passes,
and the refusal is raised in the `Interval` re-derivation of the round
(`role_walk_set` → `RoleWalkRefusal::Refused` → `plus_v_verdict` →
`PlusVVerdict::Uncomputable`), at `closed_loop_turns`
(`crates/geom-brep/src/props/closure.rs`), as
`VolumeUncomputable { Face { Escalated { Indeterminate { margin:
Enclosure { lo: 0.0, hi: H }, band: (1e-12, 1e-11), predicate:
"props_chart_loop_whole" }, check: Inventory } } }`.

- **CARVE's witness, reconstructed** (its 20 poses are recorded
  nowhere): a 1 × 1 loop whose top side is an arc of radius uniform in
  [200, 400], sketched on a random frame, extruded 0.7. Of 20 poses, 6
  or 7 refuse here on the arc's cylinder (R 208 to 250; H from 2.47e-12
  to 9.01e-12), 13 refuse earlier in construction (below), and 0 or 1
  build. At ε = 1e-11 and 1e-9 all 20 build.
  - Pose 1 (R 213.08): the decided quantity is
    `excess(Σ lever·|miss|, 3ε)` with the cylinder's radius as lever.
    The `f64` lane reads Σ = 7.3e-15 m and 1.17e-14 m on the face's two
    loops and decides `Zero`. The `Interval` lane's per-edge miss
    enclosures are ±1.07e-14 rad on the arcs and ±5.8e-15 rad on the
    rulings, which the 213 m lever makes 2.3e-12 and 1.2e-12 m an edge.
    Inferred: the true Σ is of order 1e-14 m, so the enclosure is honest
    and several hundred times loose.
- **A tilted tube** (axis (1,1,1)/√3, outer 2.5, wall 0.001, inside the
  session box): major 3.7, 30 and 100 build; 300, 600, 990 and 1100
  refuse on FaceKey(1v5) with H = 2.63e-12, 8.10e-12, 1.52e-11 and
  1.71e-11, about 1.5e-14·R. At ε = 1e-9 all build. An axis-aligned tube
  of major 990 or 997 builds at 1e-12.
- **The same scale refuses earlier, typed, in construction**, and those
  are not this row's: the extrude refuses
  `AnalyticRung3(KernelLimit { operand: Cylinder, bound: 1.33e-12..1.73e-12,
  width: 2.2e-12..2.6e-12 })` (`upper_bound`,
  `crates/geom-brep/src/edge_nurbs.rs`) on arc sides of 270 to 400 m in
  13 of 20 poses, and a tilted tube of major 1200 refuses
  `pcurve_envelope` in band (margin 1.052e-12).

**Stale beside it.** `crates/sweep/tests/verbs_tubewall_r1_probes.rs`
still carries refusal arms for a major-1300 tube (`hollow`, `measured`,
the Interval arm, and doc text), which `CASES` no longer holds: 1300 m
is past the session box, and the door answers `OutsideSessionBox` there.
The arms are unreachable; the tilted tube above is an in-box witness.
