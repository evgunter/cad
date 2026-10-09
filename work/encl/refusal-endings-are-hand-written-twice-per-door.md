---
id: refusal-endings-are-hand-written-twice-per-door
kind: issue
title: geom-brep: every refusal level hand-writes an ending and an ending_in_file twin, and a missing twin silences the import door
status: closed
closed: 2026-10-09
branch: encl/one-ending-per-door
pr: 4457
opened: 2026-10-09
priority: P3
cost: M
---



(Filed by the ENCL orchestrator from the review of PR 4432, `encl/rung3-tube-door-ending`. Not taken in that PR.)

## What

Every level of the refusal stack hand-writes an at-rest `ending` and an import-door `ending_in_file`:
- `SizedDecision`;
- `geom_brep::recourse`;
- `OneArcRefusal` (`crates/geom-brep/src/ssi.rs`);
- `PlaneNurbsRefusal` and `AnalyticRung3Refusal` (`crates/geom-brep/src/edge_nurbs.rs` ~:342/354 and ~:904/916);
- `CertifyError` (`crates/geom-brep/src/certify.rs` ~:720/734).

The `TubeNotOneArc` early return alone appears four times: two lanes × two doors.

PR 4432's defect was one missing copy. `AnalyticRung3` had an `ending` special case and no `ending_in_file` twin, so the import door printed a payload with no ending. PR 4432's guard (`topo` `test_support_samples::tests::every_certify_refusal_ends_alike_at_rest_and_at_the_import_door`) now checks the pairs agree over the sample roster. The pairs are still written twice.

## Repair shape

Give each level one body that takes the door as a parameter, e.g. `enum ReadAt { Rest(Reading), File(FileCoincidence) }` threaded down to `recourse`/`recourse_in_file`. Then one body serves both doors, and the twin cannot be forgotten.

One catch: `Reading` is `Eq` and `FileCoincidence` is not, so the door cannot simply be a new `Reading` variant. Texts must not move; the at-rest and door rows plus the guard are the evidence.

## Closed

2026-10-09. PR 4457 merged at `dcff36e784` after a full review (verdict: merge) and a small fix pass; hosted CI was green.
- **`geom_brep::recourse::ReadAt { Run(Reading), File(FileCoincidence) }`.** `ReadAt::reading()` is the one home of "the import door reads at rest".
- **One body per level.** Each refusal level has one ending body taking `at: impl Into<ReadAt>`: `SizedDecision::recourse`, `certify::recourse`, `OneArcRefusal::ending`, `PlaneNurbsRefusal`/`AnalyticRung3Refusal::ending`, `CertifyError::ending`/`render` and topo `EulerOpError::render`. The `*_in_file` twins are gone, and callers drop the suffix.
- **Kept as pairs:** `Unsized::recourse` / `residual_in_file` (two roles) and geom-core's leaf sentences (gate-counted).
- **No text moved:** 371 renders (53 samples × 7 readings) are byte-identical, base vs branch. The topo guard now checks that the two doors end alike apart from ε_in.
