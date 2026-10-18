---
id: a-chord-between-two-merged-faces-within-no-rim-refuses
kind: issue
title: A pair boolean's chord between two merged faces within no rim of their same-side faces refuses, where widened N3 reads it as a seam of the two Merged parents
status: open
opened: 2026-10-18
---


## Finding

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`'s
`chord_kind`, `(true, true)` arm: a chord between two faces the pair
merged is named as the rim a face of its key's side in one merged face
shares with one in the other, where it lies within one
(`chord_within`); one that lies within none refuses
`NamingError::MergedChordOffRim`, and one with no key side (zip-listed)
`NamingError::MergedChord`. N3 as widened on PR 5279 says a seam at every
boolean cites, as each side, the parent of the face beside it; a chord
between two merged faces off every rim is a seam between their two
`Merged` parents, which no arm mints.

Two things to settle before it is built:
- the sides' order: a pair seam is `Sided` (`a` the A operand's), and a
  seam between two merged faces, each holding faces of both operands, has
  no A side. `crates/editor-core/src/names/canonical.rs`
  `Seams::of_published` reads a published seam as a union's when both
  sides are minted by the name's own node, which a pair seam between two
  merged parents would be; it relies on one side at least being an
  operand's name.
- whether such a chord occurs in a pair boolean at all outside a union
  (where the other operand filled the gap between the two faces), or only
  through a zip-listed edge.

No witness built; found by the sweep of `wire/seam-cites-parent`.

Signed (WIRE lane `wire-seamparent`).
