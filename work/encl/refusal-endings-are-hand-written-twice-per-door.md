---
id: refusal-endings-are-hand-written-twice-per-door
kind: issue
title: geom-brep: every refusal level hand-writes an ending and an ending_in_file twin, and a missing twin silences the import door
status: dispatched
branch: encl/one-ending-per-door
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
