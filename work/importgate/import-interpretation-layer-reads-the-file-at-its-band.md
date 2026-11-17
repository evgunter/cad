---
id: import-interpretation-layer-reads-the-file-at-its-band
kind: unit
title: step-import: the interpretation layer — evidence and description as two types, every ε_in reading through the kernel's readings at the file band, the move ledger, build_axes for every placement
status: dispatched
opened: 2026-11-17
priority: P1
cost: H
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
branch: importgate/interp-layer
---


The first of the units that build D7's interpretation stage (`adoption-rebuilds-caches-at-eps-from-eps-in-interpretation`), as three designers converged on it (2026-11-17, `log.md`). The final state the stage is heading for: the file is parsed into *evidence*; interpretation, at the file band, turns evidence into *descriptions* that hold no file-derived carrier, parameter or vertex point; the build makes every cache by the kernel's own constructions and certifies at ε; the at-rest gate is unchanged. This unit is the interpretation side for surfaces and placements, and the vocabulary the rest stands on. Edges, vertices and parameters (the build) are the next unit; snaps beyond record-onto-record wait on Ev.

## Scope

- **Two types.** Split what `SolidSpec`/`EdgeSpec` conflate: the file's statements (points, carriers, records, parameters) as *evidence*, readable by interpretation readings and the move report only; and the *interpreted* model (topology as the file states it after the existing reported normalizations, the kernel's surfaces). Edges still carry their file carrier as evidence into adoption until the build unit; the types make that one visible seam rather than the default.
- **One file band.** Every ε_in reading in `crates/step-import` (frames and placement identity in `entities.rs`, `recognize.rs`, `recognize_curve.rs`, the vertex-loop check, `normalize`, `knot_split`, `chart.rs`'s `infer_outer`, `adopt.rs`'s `coincident_surfaces` and `carrier_on_surface`) reads through the kernel's own readings at the file band (`chart::file_band`: (ε_in, K·ε_in), or the run's band where ε_in < ε), as a named, levered margin in metres — no local `<= eps_in`. Zero / in band / definite read per `[verdicts]`.
- **Placements.** Every placement, surface placements included, is built by ISO 10303-42's `build_axes` from the file's ratio vectors (axis normalized, reference projected and normalized): the standard's meaning, never refused except a degenerate input (zero vector, reference parallel to axis), reported beyond rounding as `PlacementReMinted` already is. The "verbatim within ε_in" windows go.
- **Two records of one surface.** Two surface records on one solid that read Zero as one locus at the file band, decided at the faces (never at stored origins), become one record before anything is at rest; the earlier record in the file stays; one move entry. Faces of different solids stay two records with contact rows.
- **The move ledger.** One record per entity interpretation moved beyond rounding (file entity id, what moved, a bound in metres), on the import's result, never in the body; extends `NormalizationKind`, with `curve_promotions` folded in.
- **Interpretation refusals.** One refusal vocabulary that names the file entity, the reading, the miss in metres and ε_in: an evidence miss ("re-export, or import with ε_in ≥ a if the file is less exact than it declares"); an in-band ambiguity naming the ε_in that would decide it; an invalid root (a radius, half-angle or zero vector a constructor refuses). Absorb `work/encl/the-import-door-has-no-ending-for-a-margin-between-eps-in-and-k-eps-in.md` and `work/encl/a-definite-reading-within-eps-in-ends-two-ways-at-the-import-door.md` if they are still open (claim by `git mv`, per `work/README.md`).
- **Docs.** `lib.rs`'s "Two tolerances" section and its module docs ("no normalization that moves bits", "M7-2+", "caches re-minted by the kernel's own machinery") are stale against the tree; rewrite them to the present.

Closes: `step-import-adopts-frame-directions-on-eps-in-that-check-1-refuses-at-eps`, `step-adopt-reads-a-plane-origin-against-another-planes-normal`, `import-adoption-keeps-two-records-of-one-carrier`, and `step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification` if the invalid-root refusal covers it.

Not in scope: rebuilding edge carriers, vertices or parameters; snaps of tangency, coaxiality or axis-normality; retiring the set-ε-to-ε_in stopgap; D7's wording (an `[ev]` PR).

Letter H, tier dual: an architectural split whose shape the build unit and every later import row stand on.
