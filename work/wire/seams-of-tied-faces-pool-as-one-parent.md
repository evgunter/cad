---
id: seams-of-tied-faces-pool-as-one-parent
kind: issue
title: A pair boolean's and a union's seams group by their sides' NAMES, so seams of two tied faces are named against each other as pieces of one seam
status: open
opened: 2026-10-27
priority: P2
cost: M
---

Filed by WIRE's `wire-splitchords` lane, the sibling of
`work/wire/a-splits-chords-of-tied-faces-pool-as-one-parent.md` at the
pair boolean and the union. Found by the sweep; not measured on a
fixture.

## What

Two seam passes key their groups by the seam's spelling, so two seams
whose sides are different candidates of one tied face name pool into
one group, and `name_edge_pieces` (`Lone::Whole`) qualifies each by its
`Ends` where it would otherwise be the bare `Seam`, tied with its
sibling. Whether one candidate's seam is bare then depends on whether
the other candidate has a seam too, which NS3 rules out and N4 answers
("the undivided candidates stay under the bare name").

- `crates/editor-core/src/names/emit_topo.rs` `name_boolean_edges`:
  `seam_groups: BTreeMap<(NameRef, NameRef), SeamGroup>`, keyed by the
  two sides' names. Its comment calls the pooling "the widening B1 asks
  for, not a collision"; B1 is that ties propagate, which a
  per-candidate key keeps.
- `crates/editor-core/src/names/emit_union.rs` `name_by_parents`:
  `seam_groups: BTreeMap<StableName, ...>`, keyed by the `Seam` base,
  where `Parents::face` gives each side's parent as an index that a
  key could carry.

## What it needs

Key each by its sides' entities (the operand faces or the union's
parent indices), as the split's chord, seam and fragment groups are
keyed by `Parents`, with a witness: a seam of each candidate of
`fixture::u_cutter_tie`'s tied cap against one untied face.

