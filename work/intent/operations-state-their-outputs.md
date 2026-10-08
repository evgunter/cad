---
id: operations-state-their-outputs
kind: issue
title: Stage 2 FORK-1: what an operation outputs (D10's kind list, REFERENCES DM3)
status: closed
opened: 2026-10-07
priority: P0
cost: E
closed: 2026-10-08
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

Raised as FORK-1 by the stage-2 spec (`docs/INTENT-STAGE2-SPEC.md` §11, PR 4216). It blocks unit A (`operations-define-output-variables`). A designer pair weighed it over several rounds and converged, leaving one stated choice. The question, the recommendation and both reports are in the `[ev]` PR that carries FORK-1 and FORK-3 together, since both rewrite D10's Variables and Operations paragraphs. Ev's answer closes this row.

**Ruled (Ev, 2026-10-08, PR 4222):** approved. Each operation states fixed, named, typed output ports; the shapes `Body`, `Bodies` and `Profile`; a split defines two bodies; an instance one `Body` per world placement of its part; assertions, mates and gauges define nothing. The revolve-axis split is answered by FORK-1b (`the-pose-kinds-are-one-order`): a revolve defines its body and its axis. Unit A builds it.
