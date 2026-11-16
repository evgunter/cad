---
id: chart-region-refusals-that-mix-classes-await-their-split
kind: issue
title: ChartRegionError's TrimOffFace and DegenerateLoop each mix refusal classes, so their ladder standing aborts until they are split
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found by QML T4 (`onecopy/refusal-standing`), stating `topo::chart_region::ChartRegionError::refusal()` (orchestrator, 2026-11-09).

## What

Two variants of `ChartRegionError` each carry refusals of more than one `[failure.taxonomy]` class. By the ladder's rule (a payload that mixes classes aborts until it is split) both stand `Err(Aborts)`, which stops a ladder at what may be a reading a tighter band would decide.

- **`TrimOffFace { read: IncidenceRead }`**: a plane face's trim edge refused on its incidence on the plane. The read is one of:
  - `IncidenceRead::Off(_)`, decided off: an invalid input at a build, row 1(a), or a stored-data fault at rest, row 4;
  - `IncidenceRead::AtPose(_)`, read at a pose;
  - `IncidenceRead::Undecided(Indeterminate)`, in band, row 1(b), ranked by its margin.
- **`DegenerateLoop { face, loop }`** is raised at two sites (`chart_region.rs`):
  - a loop whose chart polygon has fewer than three points (`poly.len() < 3`), a structural fact no band moves (1(a) at a build, 4 at rest);
  - an orientation (`chart_region_orientation`, `twice_area` over its perimeter) the band decides `Zero`, in band, row 1(b): a tighter band may part it.

## Done when

Each variant is split by class, or carries its class:
- `TrimOffFace`'s standing reads its `IncidenceRead` (`Undecided` by its margin, `Off` an abort, `AtPose` as its own reading's standing);
- `DegenerateLoop`'s two raisings are two variants, or one variant with its cause.

`ChartRegionError::refusal()`'s abort arm then loses them, and its doc and `chart_region`'s standing row change with it.
