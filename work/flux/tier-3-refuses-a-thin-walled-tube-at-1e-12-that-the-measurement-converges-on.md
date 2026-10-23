---
id: tier-3-refuses-a-thin-walled-tube-at-1e-12-that-the-measurement-converges-on
kind: issue
title: at 1e-12 tier 3 refuses the thin-walled hollow tubes (VolumeUncomputable, an escalated sign read) that the measurement door converges on, so the typed door cannot measure them
status: open
opened: 2026-10-23
priority: P2
cost: M
---


Found by F2 (the test-site migration onto the typed door), at
`CAD_TOLERANCE_EPS=1e-12`.

## Measured

`crates/sweep/tests/verbs_tubewall_r1_probes.rs`'s `closed_forms_at_varied_geometry`
and `the_bore_differences_out_against_the_solid_door` measure the hollow
torus tubes of their `CASES` (`:57`).

- At 1e-12, `AtRestBody::validate` refuses one case's body with
  `VolumeUncomputable { source: Face { face: FaceKey(3v3), source:
  Escalated { cause: Indeterminate { margin: Enclosure { lo: 0.0, hi:
  2.256e-11 } … } } } }`: check 7's sign read on that wall escalates.
- The raw walk over the same body converges.
- Both rows pass on main, where they measured unvalidated, and match
  their closed forms to 1e-9.

So at 1e-12 this body is admitted by the measurement and refused by the
gate in front of it. Since Ev's decision on PR 4822, the typed door reads
only finished bodies, so a caller cannot measure this tube at 1e-12.

The two rows read the raw door where the gate refuses and the typed door
otherwise, with a reason that names this row.

## What closes it

One of:
- check 7's sign read decides where the reporting walk converges;
- the gate says why it cannot, typed;
- the reporting door is shown to be wrong to converge here.

Then the two sites go back to the typed door at every ε.
