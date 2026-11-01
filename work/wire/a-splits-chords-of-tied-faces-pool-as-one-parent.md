---
id: a-splits-chords-of-tied-faces-pool-as-one-parent
kind: issue
title: A split groups its edges by parent NAME, so the chords of two tied faces are named against each other as pieces of one parent
status: closed
opened: 2026-10-26
priority: P2
cost: M
closed: 2026-10-27
---



Filed by WIRE's `wire-paircorner` lane (PR 5732), the sibling of
`work/wire/a-pair-booleans-tied-corner-is-split-as-one-corner.md` at
the split.

## What

`crates/editor-core/src/names/emit_topo.rs` `name_split_edges_vertices`
keys `edge_groups: BTreeMap<(usize, StableName), …>` (around line
1047) by the half and the parent's NAME. A tied parent's candidates
share that name, so their edges pool into one group, and
`name_edge_pieces` names them against each other.

- `SplitFragment` groups (`Lone::Piece`) are unaffected: every piece
  is qualified by its `Ends` whatever the group holds, and
  `EdgePieces` mints by name across groups anyway.
- `SectionEdge`, `Seam` and `Merged` groups (`Lone::Whole`) are
  affected. A group of one is the bare name; a group of several is
  `Ends`-qualified. So whether one candidate's chord is bare depends on
  whether the plane also crosses the other candidate, which is the same
  non-locality the pair corner pass had.

## Measured (branch `wire/pair-boolean-walls-and-tied-corners`)

Probed with `lib_g14_split_walls::u_cutter_tie`, the subtract whose
table ties B's two prong fragments on each cap (z = 1 and z = 3),
split by the plane x = 3, which crosses both candidates of both ties.

- Each half holds 4 `SectionEdge` chords of the tied caps, 2 per tied
  name.
- Each is `SectionEdge` + `Fragment(Ends)` and `Unique`.
- Keyed per candidate, each candidate's one chord would be the whole
  section line across its face: the bare `SectionEdge`, tied 2
  (N4: "the undivided candidates stay under the bare name").

The probe was not committed. This fixture builds its cutter with
`slid`, which is D10-held ground; a non-placed spelling (the profile
on a frame at z = 1) should reach the same table.

## What it needs

- A ruling that a split's chord of a tied face is per candidate, as the
  corner is. This moves the measured names from unique `Ends` names to
  one tie, so it is not only a key change.
- Then key `edge_groups` by the parent's candidate (or by the operand
  face set for a merged parent), with a witness on the fixture above.

Closed by PR 5857 (2026-10-27). A split's chord groups are keyed by their parents' operand-entity sets, so an undivided tied candidate's chord is the bare name and ties (N4). A dump over 217 split tests shows them identical apart from the witness.
