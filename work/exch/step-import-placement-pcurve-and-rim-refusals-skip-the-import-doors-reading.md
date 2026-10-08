---
id: step-import-placement-pcurve-and-rim-refusals-skip-the-import-doors-reading
kind: issue
title: step-import: the Placement, Pcurves, RimOffWallBoundary and TierInvalid refusals do not read at the import door, so a file's miss or size within its ε_in ends as a kernel defect, with no ending, or with an offer to tighten alone (D4 ¶1)
status: open
opened: 2026-10-08
priority: P3
cost: M
---


(Filed by the ENCL implementer on `encl/adoption-at-rest-eps-in`, from the sweep for certification refusals the import door reports.)

## What

That branch reads the `Adoption` and `Assembly` refusals at rest with the file's ε_in words. It renders them through `topo::EulerOpError::render_in_file` and `geom_brep::CertifyError::render_in_file`, and the comparisons live in `geom_core::MarginDiag::sized_recourse_in_file` and `geom_core::FileCoincidence::miss_recourse_in_file`. Four other `StepImportError` variants also report a certification of the file's own geometry at import, and none of them reads there:

- **`Placement`** (`crates/step-import/src/error.rs`, `Display` arm `Self::Placement`) prints `topo::TransformError` through its `Display`. For `TransformError::Certify`, that renders the re-certification at `Reading::Build` (`crates/topo/src/transform.rs`, `impl Display for TransformError`). A placed copy of a file's edge that misses at this run's ε therefore ends "There is no way through: this is a kernel defect; report it". That blames the kernel alone for the file's geometry, and the stopgap is never named where the miss lies within ε_in.
- **`Pcurves`** (same file, `Self::Pcurves`) prints `topo`'s pcurve mint refusal with no ending at all. `tier_gate`'s dm1 cells show it: "pcurve certification: MapResidual at sample 1 definitely exceeds the tolerance band — … (D4 ¶2; …)", and no `Recourse:` follows. That breaks D4 ¶1 (i)'s one-ending rule.
- **`RimOffWallBoundary`** (same file, `Self::RimOffWallBoundary`) states a residual that exceeds the ambient tolerance, with its value, and names no recourse. It is a miss at ε. Where `residual` ≤ ε_in, D4 ¶1's stopgap applies.
- **`TierInvalid`** (same file, `Self::TierInvalid`) prints each `topo::ValidationError` through its `Display`, and topo's validator composes its sized endings at `Reading::AtRest` with no ε_in in hand. A band-decided size at or below the file's ε_in is therefore offered "tighten the tolerance below …" alone, which is what the import door withholds for `Adoption` and `Assembly`, and a residual's miss within ε_in never names the stopgap. The sites, in `crates/topo/src/validate.rs`:
  - `WedgeCheck::ending` (validate.rs:2361–2364): `DIHEDRAL_ARM`, `WEDGE` and `SEPARATION` at rest on an undecided cause.
  - `classify_certify` (validate.rs:2678): `CertifyError::ending(Reading::AtRest)`, the tier-2 re-certification of the file's own edges. This is the door's reading's nearest sibling, and `CertifyError::ending_in_file` already exists for it.
  - `impl Display for ValidationError`, the ring-torus arms (validate.rs:3347 and 3353): `RING_TORUS` at rest on a `Refused` verdict and on an escalation.
  - Also, from the same sweep: `classify_offset_fit` (validate.rs:2830), `classify_mass_props` (validate.rs:2917, 2962), `classify_pcurve` (validate.rs:3068), and the planar-residual arms of `impl Display for ValidationError` (validate.rs:3422–3439).

## Repair shape

Route each through the door's file reading: `TransformError` gains a `render_in_file`, as `EulerOpError` did. The pcurve and rim refusals compose the `Unsized::residual_in_file` ending, or their own miss sentence through `FileCoincidence::miss_recourse_in_file` once a reporting margin is carried. `ValidationError` gains a rendering with the file's ε_in that step-import's `TierInvalid` uses, its sized arms ending through `SizedDecision::recourse_in_file`. A new call site there joins `scripts/gates/reporting-margin-door.sh`'s sentence list.
