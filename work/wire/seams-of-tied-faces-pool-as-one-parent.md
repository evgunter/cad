---
id: seams-of-tied-faces-pool-as-one-parent
kind: issue
title: A pair boolean's and a union's seams group by their sides' NAMES, so seams of two tied faces are named against each other as pieces of one seam
status: closed
opened: 2026-10-27
priority: P2
cost: M
closed: 2026-10-28
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


## Measured (PR 5857's review, 2026-10-27)

The pair-boolean arm misnames today. Probe: `u_cutter_tie` minus a block x∈[3,3.5], y∈[0.5,3.5], z∈[-0.25,1.25]. Each of C's two walls gets two `Unique` `Seam{tied Cap(Start) floor, C's wall}` + `Fragment(Ends)` rows, one per candidate. Keyed per candidate, each would be a bare `Seam` tied 2 (N4).

The union arm was checked by inspection only; it is likely reachable.

When this row is built, reconcile the keys: the pair corner pass keys by `(name, Upstream::candidate)` (emit_topo ~2935), and the split now keys by operand-entity sets (`type Parents`). Pick one spelling for "per candidate" and use it at all three sites. Rename the split's `Parents`, which clashes with `emit_union`'s `struct Parents`. Add tie witnesses for the `Seam` and `SetPiece` arms the split re-keyed.

## Closed

2026-10-28 (WIRE orchestrator): PR 5876 merged. Seams now key per candidate (`ParentEntities`) at the split, the pair seam, the union seam and the pair corner. Pair and union witnesses are red on base. Untied rows are unchanged by dump.
