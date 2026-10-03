---
id: ssi-a-step-capped-by-a-neighbouring-crossing-can-spend-the-step-budget
kind: issue
title: ssi: a plane × NURBS branch's step is capped by the nearest unused crossing, which may be another branch's, so a long branch beside a close crossing can spend SSI_MAX_STEPS
status: open
opened: 2026-10-02
priority: P2
cost: M
---


(SSI implementer on PR 3862, from reviewer B's review of that PR,
2026-10-02.)

## What

`ssi/ends.rs`'s `Ends::branches` marches each open branch from a
crossing `A` with its step capped at `min(SSI_STEP_MAX·extent, d/5)`,
`d` the distance from `A` to the nearest crossing not yet used. The
partner is not known before the march, so `d ≤ |AB|` and the branch is
cut into at least five steps. When another branch's crossing lies close
to `A` (two branches converging on a side, or meeting it a little apart),
`d` is that gap rather than the branch's length, and a branch of length
`L` takes about `5L/d` steps. Past `SSI_MAX_STEPS` (20 000) the march
refuses `StepBudget`, though the branch is ordinary. Reachable by valid
geometry; not yet met by a fixture.

## Open

Whether the cap should be re-derived once the partner is known (a second
march at `|AB|/5`, which is the re-march this lane retired), read from
the crossings on the same branch's side pair only, or grow as the march
leaves `A`'s neighbourhood (the step rule's own curvature rungs already
bind far from the ends). A stepper choice, priced M.
