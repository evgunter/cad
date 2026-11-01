---
id: material-arm-split-ends-as-a-hand-minted-invalid
kind: issue
title: validate: material_arm_error's Split arm escalates through a hand-minted INVALID Indeterminate; decide its ending and give it one
status: open
opened: 2026-10-25
priority: P1
cost: M
design: true
---


(Split from `ring-contact-and-sliver-split-endings-want-their-decisions`
by the band rule, `work/README.md` Priority: this is the part of that
row CLEAVE's `the-indeterminate-seal` waits on.)

## What

`topo::validate::material_arm_error` (`crates/topo/src/validate.rs`,
near `:6312`) refuses `MaterialArmOutcome::Split { predicate }` as
`ValidationError::SliverDihedral { check: WedgeCheck::MaterialSide, .. }`
whose cause is a hand-built `Indeterminate { margin:
geom_core::MarginDiag::INVALID, .. }` (near `:6327`). It is the
RESTFRONT site in the re-scope of
`topo-mints-indeterminates-outside-the-funnel` (its "Other programs'
rows"), and the seal makes `MarginDiag::INVALID` crate-private in
`geom-core`, so this mint has to be gone before the seal lands.

The parent row's second bullet says why the ending is also wrong:
samples that disagree about which end of the wedge an edge makes
(`material_cusp_side`) are what a tangent edge whose curvature
difference changes sign between samples gives, and moving the geometry
reaches that, so the defect ending may not hold.

## Fix

Decide whether the split is a definite refusal with its own lever (the
parent's "ruling on the split's story"), and give it that ending
through a typed arm or the funnel's own reading, not a hand-minted
`Indeterminate`. `design: true` for that choice.

The parent keeps the rest: `RingContactEscalated`'s closed decision and
the two chart-region arms' margins, which mint nothing.
