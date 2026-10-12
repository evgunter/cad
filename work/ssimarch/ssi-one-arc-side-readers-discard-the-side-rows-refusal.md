---
id: ssi-one-arc-side-readers-discard-the-side-rows-refusal
kind: issue
title: ssi: one_arc's side_readers discards side_row's SplineError as an undecided side
status: open
opened: 2026-10-07
priority: P3
---


## Found (pcert lane `pcert/line-seam-row-refusal`, 2026-10-07, sweeping `boundary_iso_*` / `side_row` consumers)

`crates/geom-brep/src/ssi/one_arc.rs`, `side_readers` (near :387):
`super::boundary::side_row(boxes.surface(), side).ok()?` drops the
door's `SplineError` and leaves that side's reader `None`. `reader_of`'s
callers (`one_arc.rs` near :658 and :738) then answer
`Shortfall::Undecided` — "a walk resolved no count, or a linking or end
reading no sign" — so a wall whose net disagrees with its knots reads as
an undecidable sign rather than as the structural defect it is. That
path is unreachable for a wall `NurbsSurface::new` built, and it is
sound (it refuses), but the refusal names the wrong cause and loses the
payload.

The same PR converted the sibling site, `Pass::curve` in
`ssi/boundary.rs`, which carries the payload as `SsiError::ChartRow {
source }` now.

## Repair shape

`side_readers` returns the `SplineError` (or a `Shortfall` arm that
carries it) instead of `Option`, so `ChartRow` reaches the caller. The
`None` that a refused Bernstein form (`SectionReader::of`) yields is a
separate question, and it is out of this row's scope.
