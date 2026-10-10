---
id: measure-is-an-operation
kind: issue
title: D10 stage 2 PR D: a Measure is one primitive defining one observed scalar; its arithmetic is a Defined variable, an Assertion reads a scalar variable, and a construction reading an observed variable refuses
status: dispatched
opened: 2026-10-07
priority: P0
cost: M
refs: [a-construction-reads-a-measured-value, error-design-e3-calls-a-measure-a-sink]
pr: 4355
branch: intent/s2-d-measure
---

INTENT stage 2, PR D. Spec: `docs/INTENT-STAGE2-SPEC.md` §5.

A `Measure` holds one primitive and defines one scalar variable. Measure arithmetic
is a `Defined` variable, which closes VR4's interim exception. `Assertion` reads a
scalar slot. `Node::measure` stays as an authored builder returning the edit list.

## FORK-5 ruled (2026-10-07, #4218)

A measure's output is observed. Only an assertion reads an observed variable, directly or through a definition. A construction reading one refuses `ConstructionReadsObserved` at the door, and `ObservedRead` at load. Driven dimensions are deferred. The design flag is cleared.

## Carried from unit B (PR 4342)

Fold `NodeErrorKind::UnresolvedSite` into `UnresolvedRead` when a measure's `at` becomes a read. Moved to unit E (`select-defines-face-and-edge-variables`): a measure's refs stay `SitedRef` until E (spec §5), so in D a site is still a node, not a read.
