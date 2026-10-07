---
id: a-construction-reads-a-measured-value
kind: issue
title: Stage 2 FORK-5: may a construction's slot read a measured value (a driven dimension)?
status: closed
opened: 2026-10-07
priority: P0
cost: E
closed: 2026-10-07
refs: [d10-one-way-to-say-intent-is-unbuilt]
---

After stage 2, a `Measure` defines a scalar variable, and D10 lets any slot read any variable of its type. So an extrude's depth could be defined as a distance measured on another body: a driven dimension. D10 is silent on whether that is allowed. Raised as FORK-5 by the stage-2 spec (`docs/INTENT-STAGE2-SPEC.md` §11, PR 4216). It bounds unit D's (`measure-is-an-operation`) schedule work.

Weighed by a designer pair over two rounds, which converged: a construction reads only what was written, and a measured ("observed") value is read only by an assertion. The question and both reports are in the `[ev]` PR. Ev's answer closes this row and writes the D10 sentence.

**Ruled (Ev, 2026-10-07, PR 4218):** a construction reads what was written; a measured (observed) variable, and any definition reading one, is read only by an assertion. Ev: "i don't know if we'd want this feature eventually, but certainly there's no need to support it now", so driven dimensions are deferred, not refused for good.
