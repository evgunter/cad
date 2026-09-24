---
id: msolve6-part-reach-double-evaluation
kind: issue
title: A caller that solves and then evaluates resolves each mated part twice: PartReach's own cache beside the evaluation's
status: open
opened: 2026-09-12
priority: P1
cost: D
---


## Finding

Disclosed by `crates/editor-core/src/eval/mod.rs`'s `PartReach` doc
(MSOLVE-6, PR 2116): a reach built outside an evaluation
(`mate_reach(&opts, tol)` / `PartReach::with_resolver`) owns its own
`PartCache`, so a caller that solves a document through it and then
evaluates the document resolves and evaluates each mated part once in
the reach's cache and once in the evaluation's. The evaluation's own
solve shares its run's cache (`CacheReach`) and pays once; the sites
that solve first and evaluate after pay twice.

The sites, as of the PR:

- `crates/viewer/src/matetool.rs` — `MateTool::proposal` builds
  `mate_reach(opts, tol)` and calls `solve_document`, then the session
  evaluates the committed document through the seam.
- `crates/pncad-py/src/py/mate.rs` — `solve_document(doc, resolver=)`
  builds its reach over the workspace; a Python caller that then
  `evaluate(doc, resolver=)`s pays again.
- `demos/tour/src/assembly.rs` — `stand_scene`, `refusals`,
  `update_door` solve through `PartReach::with_resolver(Some(&store))`
  beside `run(doc, &with_store(ws), tol)`.
- The edit door itself (`apply` through a `PartReach`) followed by an
  evaluation of the accepted document: the maintenance solve's part
  evaluation is not the run's.

Correct today — the reach is a pure function of the part's body, and
each cache answers the same bits — so this is cost, not a defect, and
it is unscheduled. A fix that keeps the shape would let a run's
`Evaluation` hand out its `CacheReach` (or let `PartReach` be seeded
from a prior evaluation's part cache the way `evaluate` takes a
`prior`), so a caller with an evaluation in hand solves through it.
Not memoisation inside one maintenance solve (ruled harmless — one
evaluation through the cache per part) and not the reach per
`preview_gesture` tick (construction only; its comment says so).
