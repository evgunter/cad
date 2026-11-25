---
id: a-compared-pair-of-two-dimensions-panics-at-telling
kind: issue
title: A compared pair or position of two dimensions panics when the sentence is told, not when its payload is built
status: open
opened: 2026-11-22
priority: P2
cost: M
---


`geom_core::Told::beside` / `beside_by_magnitude` (`crates/geom-core/src/told.rs`, `Compared::new`) and `Told::position` panic on two quantities of two dimensions. Every production `beside` sits inside a `Tell` body that pairs two `Told` fields stored separately on a payload (`crates/profile/src/path.rs`, the `LegFit`, corner and fillet sentences near lines 925, 2274, 2432, 2461, 2494), so a payload built with mismatched fields would panic when its refusal is rendered, far from the bug. No production path builds one today: every field there is built through `length(..)`. Pinned by `told::tests::a_pair_of_two_dimensions_panics` and `a_position_of_two_dimensions_panics`.

The type that would make it unrepresentable: `Told<D>` with the dimension as a type parameter, so `beside` and `position` take two of one `D`. Not built in `a-number-in-a-unit-is-rounded-once` (the owner's direction to skip polish): `Told`'s constructors take a runtime `Dimension` at about fifty call sites in eleven crates, and the viewer's readout picks the dimension at run time from a unit (`crates/viewer/src/readout.rs`, `dimension(unit)`), so it would need a dimension-erased form beside the typed one. The alternative is a payload that holds a built `Compared` instead of two fields.
