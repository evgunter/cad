---
id: local-point-direction-line-types-restate-geom-line
kind: issue
title: Five local (point, direction) line types restate geom::Line
status: dispatched
opened: 2026-11-02
priority: P2
cost: M
branch: onecopy/frame-readings
rides_with: the-frame-carries-every-placement-reading
---


Found by the review of PR 6328, which made `geom::Line { origin, dir }` the line `Curve3::Line` holds and the home of a point's foot and parameter on a line (`Line::foot`, `Line::param`).

Five local types carry the same two fields under their own names, and their readers read them by hand (DESIGN `[one-way]`: two types carrying the same fields are a sign of a cleaner arrangement to find):

- `crates/topo/src/boolean/rim_wedge.rs` `Locus::Line { origin, dir }`: its `at` and parameter readers (`origin + dir·s`, `(p − origin)·dir`) restate `Curve3::eval` and `Line::param`.
- `crates/geom-brep/src/locus.rs` `TangentLocus::Line { origin, dir }`.
- `crates/geom-brep/src/pair_section/gap.rs` `Spine::Line { origin, axis }`.
- `crates/topo/src/census/depth_gap.rs` `Depth::Axis { origin, axis }`.
- `crates/topo/src/ring_path.rs` `LoopArc::Line { origin, dir, span }`: the line plus its span.
- `crates/editor-core/src/pose.rs` `AxisLine { foot, dir }`: its `dir` is a `UnitVec3` and its `foot` the line's point nearest the origin, both facts it adds; it reads a foot through `geom::Line` (`AxisLine::foot_of`) but builds that line each time.
- `crates/topo/src/boolean/region.rs` `Line { start, dir }`, a leg's course: already owed by charttail's `the-region-legs-spell-a-circle-and-a-line-beside-curve3` (that row is the one to work there; this one only cross-references it).

Fix: each variant holds a `geom::Line` (`Locus::Line(geom::Line<T>)`, …) and reads its foot, parameter and point through it. Where a variant adds a fact (a certified unit `dir`, a leg's start), it holds that beside the line, not instead of it.

Not here: the analytic surfaces' bare axis fields (`origin`/`axis` on `Surface::Cylinder`, `apex`/`axis` on `Cone`, …) are the parked row `analytic-surfaces-spell-their-frame-field-by-field`.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
