---
id: the-symbolic-door-proves-at-one-strength-at-rest
kind: unit
title: The coincidence door proves at one strength, a kernel constant; the viewer runs it at rest; the resident/full switch goes
status: closed
opened: 2026-10-11
priority: P1
cost: M
closed: 2026-10-11
---


Ev's ruling on `the-symbolic-door-s-budget-and-the-theorem-cache`
(PR 4644, 2026-10-11) applies to the door PR 4628 built. Stage 4
spec §5b holds the shape.

- **One strength.** Delete `ChecksConfig.symbolic_pass` and Python's
  `full_symbolic_pass`, and the resident wording "unproven at the
  resident budget; the full symbolic pass decides". The door's budget
  becomes one kernel constant (today's full pass: 256 terms plus the
  kept-atom ladder). No setting and no dials type.
- **Synchronous callers** (`run_checks`, Python, CI) get the full
  verdict.
- **The viewer runs the door at rest.** It runs off the draw path as a
  generation-keyed job that the next edit cancels. Until the job lands,
  the rows read *pending*, which is a report state and not a finding.
- **One unproven finding.** There is no not-an-identity/undecided
  split. Ev: "if there's an opaque numerical computation like that
  it's pretty fair to ask the user to assert that the two things are
  identical rather than making a special undecided case for it", and,
  on the frozen mitre form, "not sure if it's worth a special case".
  So the recourse names an assertion where the residual keeps a value
  computed apart from the variables, or where the door has no form of
  it; a form frozen on the budget is no special case.
- **Cancellation.** Thread the cancel token through `coincide::prove`.
  `replay::rungs` mints a fresh `CancelToken` today, so a cancelled
  evaluation still runs the replay to the end.
- **Measure** replay time against discharge time per corpus document,
  and report the split in the PR body.

## Closed

Built as ruled. The door's one strength is `coincide/replay.rs`'s
`BUDGET` with the drive's `kept_atom` ladder; `SymbolicPass`,
`ChecksConfig::symbolic_pass`, Python's `full_symbolic_pass` and the
resident wording are gone. `coincide::prove` takes the caller's
`CancelToken` into the replay and checks it before each discharge
(`a_canceled_door_stops_in_its_replay`). An unproven row the door could not
replay, or whose residual keeps a value computed apart from the
variables (`geom_core::sym::deferred::Undischarged::KeepsAnOpaqueAtom`),
names an assertion as its recourse (`Recourse::Assert`); every other
unproven row, a form frozen on the budget included, names one
construction or an assertion (`Recourse::OneConstruction`). The viewer
runs the door at rest (`run_checks_at_rest_on`, `DoorJob`,
`viewer::DoorService`); its report reads `pending` until the job
settles.
