---
id: step-import-placement-pcurve-and-rim-refusals-skip-the-import-doors-reading
kind: issue
title: step-import: the Placement, Pcurves and RimOffWallBoundary refusals do not read at the import door, so a file's miss within its ε_in ends as a kernel defect or with no ending (D4 ¶1)
status: open
opened: 2026-10-08
priority: P3
cost: M
---


(Filed by the ENCL implementer on `encl/adoption-at-rest-eps-in`, from the sweep for certification refusals the import door reports.)

## What

That branch reads the `Adoption` and `Assembly` refusals at rest with the file's ε_in words. It renders them through `topo::EulerOpError::render_in_file` and `geom_brep::CertifyError::render_in_file`, and the comparisons live in `geom_core::MarginDiag::sized_recourse_in_file`, `MarginDiag::miss_recourse_in_file` and `FileCoincidence::definite_miss_recourse`. Three other `StepImportError` variants also report a certification of the file's own geometry at import, and none of them reads there:

- **`Placement`** (`crates/step-import/src/error.rs`, `Display` arm `Self::Placement`) prints `topo::TransformError` through its `Display`. For `TransformError::Certify`, that renders the re-certification at `Reading::Build` (`crates/topo/src/transform.rs`, `impl Display for TransformError`). A placed copy of a file's edge that misses at this run's ε therefore ends "There is no way through: this is a kernel defect; report it". That blames the kernel alone for the file's geometry, and the stopgap is never named where the miss lies within ε_in.
- **`Pcurves`** (same file, `Self::Pcurves`) prints `topo`'s pcurve mint refusal with no ending at all. `tier_gate`'s dm1 cells show it: "pcurve certification: MapResidual at sample 1 definitely exceeds the tolerance band — … (D4 ¶2; …)", and no `Recourse:` follows. That breaks D4 ¶1 (i)'s one-ending rule.
- **`RimOffWallBoundary`** (same file, `Self::RimOffWallBoundary`) states a residual that exceeds the ambient tolerance, with its value, and names no recourse. It is a miss at ε. Where `residual` ≤ ε_in, D4 ¶1's stopgap applies.

## Repair shape

Route each through the door's file reading: `TransformError` gains a `render_in_file`, as `EulerOpError` did. The pcurve and rim refusals compose the `Unsized::residual_in_file` ending, or their own miss sentence through `MarginDiag::miss_recourse_in_file` once a reporting margin is carried. A new call site there joins `scripts/gates/reporting-margin-door.sh`'s sentence list.
