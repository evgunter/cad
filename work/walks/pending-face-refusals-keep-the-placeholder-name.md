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
- `splitting::finish`'s `MergePlaceholders`;
- the pcurves alias of the recourse, `topo::pcurves::PLACEHOLDER_RECOURSE` (beside geom-brep's);
- editor-core's mirror of the replace-face refusal, `R::PlaceholderSurface` (`crates/editor-core/src/verbs/shell.rs`, the shell verb's map) and its concision rows;
- attach's "chartless" names for the same fact (`Body::slot_chartless`, `Landing::chartless`, `wears_no_chart`), which read "no chart" where the slot says pending;
- mesh's `TessellateError::UnsupportedSurface { face }` for a pending face (`crates/mesh/src/tessellate.rs`, `face_surface`; pncad tag `unsupported_surface`), which reads as a surface kind the tessellator lacks;
- step-export's `UnsupportedSurface { kind: "pending" }` (`crates/step-export/src/writer.rs`), where "pending" is not a `SurfaceKind`.

Meanwhile `topo::null::is_placeholder` (`null.rs`) means something unrelated: a null edge whose two ends are one point. A reader meeting "placeholder" in topo now has two meanings to tell apart. Renaming the face-side names to the pending vocabulary would leave the null-edge reading the only one. The names are public (pncad-py tag strings and asserted texts read some of them), so the rename is its own small unit, not a drive-by of the slot change.

`TransformError::NurbsPlaceholder` lost its last producer when both placeholders were gone and retired in `walks/surface-slot`.
