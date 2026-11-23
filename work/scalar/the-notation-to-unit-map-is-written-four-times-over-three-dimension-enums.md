---
id: the-notation-to-unit-map-is-written-four-times-over-three-dimension-enums
kind: issue
title: The notation-to-unit map is written four times over three dimension enums
status: open
opened: 2026-11-20
priority: P2
cost: M
parent: patherror-display-renders-float-noise
---


"Which unit does a value of this dimension read in under this
notation" is written at four sites, over three enums that name the
same three kinds:

- `geom_core::told::Dimension::unit` (`crates/geom-core/src/told.rs`),
  over `geom_core::Dimension` (`Length`, `Angle`, `Dimensionless`);
- `viewer::props::WorkingNotation::unit`
  (`crates/viewer/src/props.rs`), over `editor-core`'s `Dimension`
  (`Length`, `Angle`, `Scalar`, `Count`);
- `viewer::readout::dimension` and `dimension_of`
  (`crates/viewer/src/readout.rs`), from `quantity::UnitQuantity`
  (`Length`, `Angle`, `Scalar`) to `geom_core::Dimension`;
- `quantity::Notation::with` (`crates/quantity/src/notation.rs`), the
  inverse, from a unit to the notation's slot.

`quantity::UnitQuantity` is the one every crate can name
(`quantity` is the leaf). The fold is `Notation::unit(UnitQuantity)
-> UnitDef` in `quantity`, with `geom_core::Dimension` either
replaced by `UnitQuantity` or mapped to it once, and `editor-core`'s
`Dimension` mapped once (its `Count` has no unit and stays its own
arm). Not folded in PR 7229: it renames a public `geom-core` enum
that `profile` and the sweeps' payloads name.
`three-dimensions-do-not-cover-the-sentences-already-in-the-tree`
changes the same enum and should land first or with it.
