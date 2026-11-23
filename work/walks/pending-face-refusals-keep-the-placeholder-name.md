---
id: pending-face-refusals-keep-the-placeholder-name
kind: issue
title: the pending-face refusals keep the word placeholder, which topo::null::is_placeholder also spells for a null edge
status: open
opened: 2026-11-23
priority: P3
cost: E
---


## What

`walks/surface-slot` made "this face has no surface yet" a pending surface slot (`topo::SurfaceSlot::Pending`, refused as `SurfaceMiss::Pending`), but the public topo refusals a pending face reaches keep the old vocabulary:

- `merge_faces::MergeKind::Placeholder` and `MergeCoplanarOutcome::placeholders`;
- `pcurves::PcurveMintError::PlaceholderChart { face }` and `geom_brep::PLACEHOLDER_RECOURSE`;
- `face_normal::SplineNormalError::PlaceholderChart`;
- `pcurves_rings` / `spline_trim`'s `SplineTrimGap::Placeholder`;
- `replace_face::ReplaceFaceError::PlaceholderSurface { face }`;
- `splitting::finish`'s `MergePlaceholders`.

Meanwhile `topo::null::is_placeholder` (`null.rs`) means something unrelated: a null edge whose two ends are one point. A reader meeting "placeholder" in topo now has two meanings to tell apart. Renaming the face-side names to the pending vocabulary would leave the null-edge reading the only one. The names are public (pncad-py tag strings and asserted texts read some of them), so the rename is its own small unit, not a drive-by of the slot change.

`TransformError::NurbsPlaceholder` is the curve placeholder's and leaves with `walks/curve-placeholder`.
