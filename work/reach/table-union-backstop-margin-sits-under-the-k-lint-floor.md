---
id: table-union-backstop-margin-sits-under-the-k-lint-floor
kind: issue
title: k-lint (dev-probe) flags demo/table's volume_backstop margin 1.5039e-5 under the 4e-5 floor at every eps row
status: open
opened: 2026-10-03
priority: P3
cost: M
---


Found by the REACH lane `reach/lily-leaf-1e12`. Until that lane, the nightly
`k-lint (dev-probe)` row never reached its lint step: `scripts/k_probe_sweep.sh`
panicked in its 1e-12 demo pass (`work/reach/lily-leaf-b-mass-exhausts-the-quadrature-budget-at-eps-1e-12.md`).
With the sweep exiting 0, `tools/k-lint` over the three fresh CSVs exits 2.
Most of its flags already have rows (`work/chart/chart-bound-outer-span-decides-a-poisoned-margin.md`,
`work/germ/circle-torus-root-slack-crowds-the-zero-band-at-1e-12.md`,
`work/cleave/lily-walls-curved-clearance-crowds-the-band-under-k-lint.md`); this is one that had none.
The flag is main's: the demo CSV built from `origin/main` 55ba6226 (whose
1e-6 and 1e-9 demo passes run) carries the same row.

## The flag

```
FLAG demo/table:volume_backstop: |m|=1.5039173041239659e-5 band_zero=1e-6 — definite within 10^2 of the escalation threshold
FLAG demo/table:volume_backstop: |m|=1.5039173041239659e-5 band_zero=1e-6 — definite below the baseline distribution's floor (4e-5)
```

The same margin, bit for bit, at 1e-9 and 1e-12 (rule 3 only there):
an eps-independent backstop margin (`topo::boolean::ops`, `decide_invariant("volume_backstop", ..)`)
on the tour's `table` scene, 2.7x under the floor. Unmeasured: which op
of the table it is and whether the margin is a real thin feature (then the
K-REPORT runbook's recourse) or a lever that reads small.
