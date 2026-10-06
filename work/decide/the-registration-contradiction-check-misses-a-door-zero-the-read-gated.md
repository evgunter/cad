---
id: the-registration-contradiction-check-misses-a-door-zero-the-read-gated
kind: issue
title: door_zero reads a gated door zero as no zero, so a registration the read's arm hides is never counted contradicted
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [DECIDE-10]
---


Found by DECIDE-10's sweep, read from the code and not executed.

`door_zero` (`crates/geom-core/src/sym.rs`) is the release-checked
question a decision the numeric channel has proved NON-zero still asks:
is the DOOR form zero? If it is, a registration is false over this box,
and `SymCounts::registrations_contradicted` counts it. It answers
`d.is_zero() && !d.gated`. A door form that is zero only through the
decision read's arm is therefore read as "not zero", so a false
registration whose zero the read gated is never counted. With the read
shut, the same door form is an ungated zero and is counted.

DECIDE-10 made the decision path ask the door walk with the reads shut
behind a gated door form (`rungs`). It left this check alone, because
the check runs on every definite margin, and on R2's bracket a shut walk
behind every gated form there cost 70–95 % of the replay
(`the-read-at-its-node-relabels-a-cancellation-above-it`, "What Phase 1
found", candidate 1c). The fix is the same shut door walk behind a gated
door zero only, which on the measured documents is a handful of
decisions. It needs a row first: a registration false over its box,
under a read.
