---
id: the-symbolic-door-proves-at-one-strength-at-rest
kind: unit
title: The coincidence door proves at one strength, a kernel constant; the viewer runs it at rest; the resident/full switch goes
status: open
opened: 2026-10-11
priority: P1
cost: M
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
  split. Where the residual keeps an opaque atom (a frozen form, or two
  separately computed values), the recourse names an assertion.
- **Cancellation.** Thread the cancel token through `coincide::prove`.
  `replay::rungs` mints a fresh `CancelToken` today, so a cancelled
  evaluation still runs the replay to the end.
- **Measure** replay time against discharge time per corpus document,
  and report the split in the PR body.
