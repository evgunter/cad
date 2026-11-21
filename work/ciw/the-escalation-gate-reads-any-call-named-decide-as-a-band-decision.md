---
id: the-escalation-gate-reads-any-call-named-decide-as-a-band-decision
kind: issue
title: The escalation-question-marks gate reads any call whose name starts with decide as a band decision, so a function that decides nothing against a band must be renamed to pass
status: open
opened: 2026-11-15
priority: P3
cost: E
---


Filed by the BOXFINISH orchestrator from PR 6982 and its review.

**What happened.** `topo::props::quad_lane` gained a helper that reads
whether a rate's certified bracket lies wholly on one side of zero: a
comparison of an enclosure's bounds, no call to `decide`, no band. Its
author named it `decided_way` and
`scripts/gates/escalation-question-marks.sh` refused the file: the gate
matches a call spelled `decide…(…)?` inside a loop and takes it for a
band decision whose escalation is propagated with `?`. The helper was
renamed `way_along` to pass.

**Why it is filed.** The gate matches a name, not the door. A helper
that is no decision is refused for its spelling, and a real decision
reached through a function with another name is not seen. The review
judged the rename fair vocabulary, so nothing is wrong in that PR; the
gate's rule is what is loose.

**Also from the same PR,** for whoever owns the census gate: a test's
struct literal `LiftedRow { lift: … }` was read as a write of
`QuadLane`'s field `lift`, so the suites build their rows through a
`#[cfg(test)]` constructor instead. The review judged that a fair
constraint; it is noted here because it is the same shape (a gate
matching a field's name without its type).

**Where:** `scripts/gates/escalation-question-marks.sh`; the PR's
`crates/topo/src/props/quad_lane.rs` (`way_along`).

Moved from GUARD to CIW on 2026-11-20, by `git mv`, when GUARD merged into CIW: GUARD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.
