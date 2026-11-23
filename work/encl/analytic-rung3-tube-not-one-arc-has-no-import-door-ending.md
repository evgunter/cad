---
id: analytic-rung3-tube-not-one-arc-has-no-import-door-ending
kind: issue
title: geom-brep: an AnalyticRung3 TubeNotOneArc refusal gets no ending at the import door
status: closed
closed: 2026-10-09
branch: encl/rung3-tube-door-ending
pr: 4432
opened: 2026-10-09
priority: P3
cost: E
---



(Filed by the ENCL orchestrator from the full review of PR 4427. This is pre-existing; that PR did not cause it.)

## What

`CertifyError::ending` special-cases `AnalyticRung3`: its `TubeNotOneArc` arm ends by `cause.ending(..)`. `CertifyError::ending_in_file` (`crates/geom-brep/src/certify.rs`) special-cases only `PlaneNurbs`, and `AnalyticRung3Refusal` (`crates/geom-brep/src/edge_nurbs.rs`) has no `ending_in_file`.

So at the import door, an `AnalyticRung3(TubeNotOneArc)` refusal goes through `decision()`, which gives `None`. `render_in_file` then prints the payload with no ending at all, which breaks D4 ¶1: one message and one recourse per decision.

## Repair shape

Add `AnalyticRung3Refusal::ending_in_file`, mirroring `PlaneNurbsRefusal::ending_in_file`, and add the matching arm in `CertifyError::ending_in_file`. Pin it with an import-door row that reaches `TubeNotOneArc`, or with a unit row over a constructed refusal if no corpus file reaches it.

## Closed

2026-10-09. PR 4432 merged at `510a375cf2` after a review (verdict: merge) and a small fix pass; hosted CI was green.
- **`AnalyticRung3Refusal::ending_in_file`** is the twin of `PlaneNurbsRefusal`'s, and `CertifyError::ending_in_file` has `ending`'s shape. One text moves: AnalyticRung3 `TubeNotOneArc` at the import door now ends with the one-arc at-rest ending in ε_in words, as PlaneNurbs already did.
- **Pins:** the geom-brep row `certify::tests::the_analytic_rung3_one_arc_refusal_ends_at_the_import_door` (red on main). Also the topo guard `every_certify_refusal_ends_alike_at_rest_and_at_the_import_door` over the full `certify_errors()` roster: both doors end, and alike but for ε_in.
- **Follow-up:** `refusal-endings-are-hand-written-twice-per-door`.
