---
id: injective-folds-a-faces-in-chart-normal-through-a-sign-four-ways
kind: issue
title: injective.rs spells 'the face lies left of the traversal about the outward normal' four ways, two folding a ±1 into an in-face normal
status: open
opened: 2026-11-14
priority: P3
---


## Finding

Found by the review of TESSLANE's sense-door unit (`tesslane/sense-door`),
2026-11-14. Filed beside
`main-red-injective-rs-folds-the-face-sense-by-hand`.

`crates/topo/src/injective.rs` spells "the face lies left of the traversal
about the outward normal" four ways:

- `present_plane`: `let s = if b.sense { 1.0 } else { -1.0 };`, the
  quarter-turn sign for a line edge's chart direction.
- `present_plane_corner`: `let s = at(if b.sense { 1.0 } else { -1.0 });`
  then `bv = [-a[1] * s, a[0] * s]`, the edge's in-face normal.
- `present_plane_vertex`: `let s = if b.sense { 1.0 } else { -1.0 };` then
  `n = [-tau[1] * at(s), tau[0] * at(s)]`, commented "the traversal turned
  a quarter, by sense, about the outward normal".
- `face_side`: `Some(if sense { side } else { -side })`.

Two related reads come at the same rule another way:
`left_is_above(..) == piece.boundary.sense` (at two sites) here, and
`pcurves::singular_turn`'s `if above == sides.sense { 1 } else { -1 }`.

The middle two fold a `±1` from the bit and multiply it into a 2-D chart
vector, the edge's in-face normal. That is a normal folded through a sign,
which is the shape the fold census's doc lists as invisible to it. They sit
in a file that already reads `crate::face_normal::chart_outward_normal` for
its 3-D normal (`Presented::shared_normal`). topo's fold census
(`face_normal::tests::no_source_file_folds_the_bit_by_hand`) sanctions all
three `1.0` sites as chart scalars, on the precedent of `present_plane`'s
entry.

## What closing it takes

The owner decides whether "the face's side of a traversal, in a plane's
chart" wants a door: one function taking the bit and a chart direction and
answering the in-face normal or the side. If it does, the four sites read
it, and the census's three `injective.rs` `"1.0"` entries leave `sanctioned`
(and `"side"` if `face_side` moves too). If not, say why the sign is a chart
scalar at each point of use.
