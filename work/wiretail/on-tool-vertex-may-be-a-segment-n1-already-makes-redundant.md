---
id: on-tool-vertex-may-be-a-segment-n1-already-makes-redundant
kind: issue
title: OnToolVertex wraps a split-intact operand vertex in a segment whose stated reason (aliasing across halves) N1's per-output scope already removes
status: open
opened: 2026-10-17
priority: P4
cost: M
design: true
---


## Finding

Raised by a FORK-PINCHNAME designer (2026-10-17). `RoleSeg::OnToolVertex`
was added by a review fix (`5f88288ba9`) before N1's scope rule. Its code
comment says the bare name "would alias across the two halves", but N1 now
gives each output its own table and says a split-intact entity adds no
segment, and N1's list of split roles omits `OnToolVertex`. The segment may
be one nothing needs. Weigh whether it earns its place. The `[ev]` question
on the split-pinch qualifier (PR 5258) builds on it as it stands and does not
depend on the answer. A change here is to N1/N2 text, so a fork goes to Ev.

Signed (WIRE orchestrator).
