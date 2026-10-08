---
id: coincidences-are-recorded-at-one-door
kind: issue
title: D10 stage 4 PR B: every coincidence the kernel decides from values is a record carried into NodeValue; the door coincide::prove (rung 1, same source) and CheckId::UnprovenCoincidence
status: closed
opened: 2026-10-08
closed: 2026-10-08
branch: intent/s4-b-record
priority: P0
cost: H
design: true
---

INTENT stage 4, PR B. Spec: `docs/INTENT-STAGE4-SPEC.md` §3. Needs nothing from stage 2 or 3; dispatchable now. Design open: FORK-S4-3 (is the record `ContactRecords` or a new type), weighed before build.

The record (cells, relation, decision site, margin) emitted at every value decision that already glues, merges or touches: the declared rung of the plane and carrier ladders, the declared merge, margin-decided vertex fusions and covered pairs, the split's pinch (`splitting/classify.rs:256`), the blend battery's isosceles turn (`battery.rs:2033`, carried on `Blended`). Carried into `OpOut`/`NodeValue` named by `StableName`. The door `coincide::prove` with rung 1 (same source, what N6 proves today), and `CheckId::UnprovenCoincidence`. Nothing builds differently; no digest, content key or id moves. Closes `value-decided-coincidences-have-no-recording-door`.

## Closed (2026-10-08)

Built on `intent/s4-b-record`; the PR body carries the decisions, the
fork dependencies and the rows released.
