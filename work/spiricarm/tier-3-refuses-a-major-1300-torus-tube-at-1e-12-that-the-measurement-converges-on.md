---
id: tier-3-refuses-a-major-1300-torus-tube-at-1e-12-that-the-measurement-converges-on
kind: issue
title: at 1e-12 tier 3 refuses the major-1300 torus tubes, hollow and solid (VolumeUncomputable, an escalated sign read), that the measurement door converges on, so the typed door cannot measure them
status: open
opened: 2026-10-23
priority: P2
cost: M
---


Found by F2 (the test-site migration onto the typed door), at
`CAD_TOLERANCE_EPS=1e-12`.

## Measured

`crates/sweep/tests/verbs_tubewall_r1_probes.rs`'s `closed_forms_at_varied_geometry`
and `the_bore_differences_out_against_the_solid_door` measure the torus
tubes of their `CASES` (`:57`), hollow and solid.

- At 1e-12, `AtRestBody::validate` refuses exactly the case of major
  radius 1300 (outer 2.5, wall 0.001):
  - the hollow tube, windowed and full;
  - its solid tube (the bore row builds solid tubes for the windowed cases only).
- The error is `VolumeUncomputable { source: Face { face: FaceKey(3v…),
  source: Escalated { cause: Indeterminate { margin: Enclosure { lo: 0.0,
  hi: 2.256e-11 } …, predicate: "props_chart_loop_whole" } } } }`. Check
  7's sign read on one face escalates. The solid tube refuses as well, so
  the cause is the scale, not the thin wall.
- The raw walk over the same bodies converges.
- At 1e-9 and 1e-6 every case finishes. Both rows pass on main, where
  they measured unvalidated, and match their closed forms to 1e-9.

So at 1e-12 these bodies are admitted by the measurement and refused by the
gate in front of it. Since Ev's decision on PR 4822, the typed door reads
only finished bodies, so a caller cannot measure these tubes at 1e-12.

The two rows (three sites) read the raw door where the gate refuses and the typed door
otherwise, with a reason that names this row.

## What closes it

One of:
- check 7's sign read decides where the reporting walk converges;
- the gate says why it cannot, typed;
- the reporting door is shown to be wrong to converge here.

Then the three sites go back to the typed door at every ε.
