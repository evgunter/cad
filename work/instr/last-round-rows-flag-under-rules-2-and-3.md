---
id: last-round-rows-flag-under-rules-2-and-3
kind: issue
title: k-lint (dev-probe) flags props_quad_last_round rows under rules 2 and 3 at 1e-9 and 1e-12, the family whose rule-4 floor is unruled
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

## The flags

`props_quad_last_round` on `demo/lily` and `demo/lily_walls`: at 1e-9
positive margins of 1.0e-6 (the swept leaves) and 1.1e-5 to 2.6e-5 (walls
15 and 16), each under the 4e-5 metre floor (rule 3); at 1e-12 the
leaves' negative margins -1.44e-8, -2.87e-9, -7.40e-9 and -8.72e-10 —
the budget exit refusing, the samples the demo sweep now records
instead of panicking — under rule 3, the last also under rule 2.

k-lint's own note says these rows are judged by rules 2 and 3 only
because rule 4's floor for this eps-coupled family is unruled, and that
a flag on them "points at the baseline floor and its recourse does NOT
apply". So every row of this family that sits within 4e-5 of the target
flags, by construction, on every sweep that records one. That ruling is
the open item; `work/instr/k-lint-last-round-is-eps-coupled-but-unrostered.md`
(closed) and `work/quad/quad-last-round-margin-has-no-sign-premise.md`
are its neighbours.
