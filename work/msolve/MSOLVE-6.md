---
id: MSOLVE-6
kind: unit
title: The mate's lever is the mated parts' own extent, taken from each part's evaluated body
status: closed
opened: 2026-09-07
branch: msolve/6-part-extent
refs: [mate-lever-needs-the-parts-extent]
closed: 2026-09-19
pr: 2116
---


Spec: `docs/MSOLVE-6-SPEC.md`. Ev's ruling on `[ev]` PR 2086
(2026-09-07): option B — the extent resolved from the mated part's
own evaluated body, entering only as the lever; A11 amended by one
sentence. Lane dispatched on `msolve/6-part-extent`.

Stopped 2026-09-07 on the spec's clause (iii): the edit door's
maintenance (`reconcile`) solves with no resolver. Draft PR 2116
holds the whole unit but that caller; the fork is
`reconcile-solves-with-no-resolver`, on an `[ev]` PR.

Resumed 2026-09-08 on Ev's ruling (PR 2118): the edit door takes the
reach, replay re-applies recorded maintenance rows. The spec's
amendment section is binding on the lane.

## Closed (2026-09-19, PR 2116)

Landed: the lever is `(R_a + ‖a.origin‖) + (R_b + ‖b.origin‖) +
Σ|authored lengths|`, every term an upper bound, no floor and no
constant — `R` from each mated part's own evaluated body through one
part-keyed reach trait (`MateReach`; `PartReach` over the evaluation's
`PartCache`, `RefusingReach` for the doors with no parts), per-kind
face bounds walked by the measure site's own `reach_of`, a reach that
cannot be bounded refusing typed. The edit door takes the reach and
asks it only when a gauge moves; a solve with no verdict refuses the
edit (`MaintenanceRefused`), a decided no-pose keeps the cluster's
frame; the log records the maintenance rows (`LoggedEdit`), replay
re-applies them without solving, old logs that moved a gauge refuse at
load and migrate through `load_with`; every recorded row's frame is
held at load to the `SetPlacement` door's own predicate. Reviews on
`21f5a65ec`: correctness C1–C5 HOLD (PASS with MINORs), style no
MAJOR; the twenty-seven-item fix pass landed on the PR. The spec is
deleted into `docs/DOC-LEDGER.md` (recoverable at the unit head named
there). Closes `mate-lever-needs-the-parts-extent` and
`reconcile-solves-with-no-resolver`. MSOLVE-7's lane inherits
`msolve6-part-reach-double-evaluation` (`work/issues/`).
