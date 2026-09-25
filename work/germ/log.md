# GERM — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7000-7099 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-25 — an orchestrator picks the track up

Status `ready` → `active`. Orchestrator branch `germ/orchestrator`, unit
branches `germ/<unit>` (the #396 prefix convention, which holds over a
harness-pinned branch name per PR 3247, approved by Ev in chat).

Opening order, a sequencing call: two lanes run in PARALLEL from the start,
as the cut intended.

- `ray-torus-root-search-finds-a-counterexample-at-eps-1e-12` — the plan's
  first row and the only one with a recorded counterexample. Measure-first:
  replay the seed, and decide which of the three candidates is wrong (the
  root chain, the oracle, the agreement window in band units) before
  touching `line_torus_roots`.
- `torus-operand-gate-admission` — raised beside it rather than behind it,
  because Ev hit it as a user on 2026-09-17 (the dumbbell union in the viewer)
  and asked for it at high priority. Its first step is a measurement, not a fix:
  the row itself says to confirm that the (torus, plane) germ-pair join is
  what blocks before scoping, and a refusal's text is not its cause
  (`memories/refusal-text-is-not-cause.md`). That lane is read-only and
  reports a scope. The unit is cut from what it finds.

Why this beats the plan's strict serial order: the second lane's first step
is read-only and does not touch the root search the first lane may change.
Running it first gives the unit the user is waiting on a measured spec while
the root-search lane is still underway, and costs the root-search lane nothing.
