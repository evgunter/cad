---
id: a-face-cut-and-merged-in-one-step-publishes-a-piece-under-the-name-its-merge-retires
kind: issue
title: A face cut and merged in one boolean step publishes its unmerged piece under the bare name its merge lists as a retired constituent
status: closed
priority: P1
cost: D
opened: 2026-09-25
refs: [declared-flush-union-edge-and-vertex-names-follow-member-order]
branch: emit/cut-and-merged-pair
pr: 3526
closed: 2026-09-30
---


## Closed — fixed by PR 3526

Still live after #3241 in the pair boolean's own table, and so in every
fold step's table. #3241's union end pass renames the published union
(`emit_union_flush_names::a_face_cut_and_merged_in_one_step_publishes_no_constituent`),
but a standalone pair boolean published the bare constituent.

`name_boolean` now gives a group's bare base name to its one face only
when no merge holds the rest of the parent. A group with merged faces
goes through `Obstacles::split` with those faces as `merged`, so the
unmerged piece is `base` + `Borders` over the walls it shares a divider
with. `split` refuses if a piece borders none, counting merged faces
among the parent's faces. The row
`emit_pair_cut_and_merged::a_face_cut_and_merged_in_one_pair_step_publishes_no_constituent`
pins it. It is red on `12807d8b19`.

The merged row's constituent still names the whole parent. N3 defines
the set as the merged faces' operand names. The union now names by
parent in the finished body, so the order-free question that half served
closed with `declared-flush-union-edge-and-vertex-names-follow-member-order`.

The fold's declaration door (`look_through_fold`) had relied on the bare
piece. A name that is both a constituent of a bare merged row and the
parent of its own fragment now refuses `ConsumedByFold { by: Split }`.
That costs correct tables:
- `r2endsg`'s orders `[1,3,0,2]`, `[2,3,0,1]`, `[3,1,0,2]` and
  `[3,2,0,1]` bound the bare piece, and it was the geometrically correct
  face: the `ALONG`~`CEND` contact at x 0..1 lies on the x 0..1.4 piece.
  They published correct tables and are now lost to totality.
- `r4trig`'s orders `[1,3,0,2]` and `[3,1,0,2]` bound the bare piece,
  and it was the wrong face: their contact at x 0.8..1 lies in the
  merged face. They were silently wrong and now refuse.
- `r4trig`'s `[2,3,0,1]` and `[3,2,0,1]` now refuse there, before they
  reach `RayExhausted`.

Which face is right depends on where the partner's contact lies. No rule
that reads names alone recovers both cases, so refusing is the fail-loud
choice for now. The lost orders are owned by
`a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names`.
`union-refuses-in-some-member-orders-and-publishes-in-others` carries
the new counts.

## The finding

Found by EMIT on `emit/declared-flush-order`, measured with a scratch
probe over PR 3112's review corpus.

In `emit_topo::name_boolean` (`crates/editor-core/src/names/emit_topo.rs`),
the face pass names merges first and then each descent group. Suppose
one operand face is cut into pieces and some of them are absorbed into
a merge in the same step. The group's live members are then only the
unmerged pieces, and the merge lists the parent as a constituent. When
one piece is left, the `[one]` arm puts it under the group's bare BASE
name. That is the very name the merge lists.

Example, `fam012`:
- `a` = x∈(0,1), `b` = x∈(0.5,1.5), declared flush;
- `g` = x∈(0.3,0.4), y∈(−1,0.5), z∈(−0.5,2), through `a`'s y = 0
  wall and both caps;
- member order `[b, g, a]`.

At the step `(b ∪ g) ∪ a`, `g` cuts `a`'s y = 0 wall into x∈(0,0.3)
and x∈(0.4,1), and the second piece merges with `b`'s wall. The step
publishes two rows:
- `Merged([FromMember(a, Lateral(0)), FromMember(b, Lateral(0))])` for
  the merged face;
- `FromMember(a, Lateral(0))` for the x∈(0,0.3) piece.

So a name the merge says has retired resolves to a face. N3 says it
should fail with the merged name offered
(`crates/editor-core/src/names/README.md`, N3). The merge's own
constituent claims the whole of `a`'s wall, although only one piece of
it merged.

## What it costs

- Referencing `a`'s wall in this document binds to a piece, and it
  does so in some member orders and not in others. In `[a, b, g]` the
  wall merges before `g` cuts it, and both pieces are fragments of the
  merge.
- A seam beside the bare piece has to cite the bare constituent. The
  union's `retire_into_merges` rewrites a seam side into its merge only
  where the merge is the face beside the seam, and here the bare piece
  is. An earlier draft rewrote every listed constituent. In `abg`
  `[b, g, a]`, that published `Seam { g.Lateral(3), Merged([a.Cap(End),
  b.Cap(End)]) }` along the bare `a.Cap(End)`: 46 seam edges across the
  corpus cited a face they do not border. So the seam names around this
  piece differ between member orders too.
  `emit_union_flush_names::a_union_cites_only_what_the_finished_body_holds`
  pins that no seam cites a face it does not border.
- The row itself is published, so no rewrite can retire it.

## Fix direction

The pieces of a group that loses some members to a merge are fragments
of their parent, not the parent. Name the survivors with their
`Fragment` qualifiers, and name the merged piece's constituent as that
fragment too. Then N3 holds within one step.

This does not by itself make the union order-free. Merged-then-cut
(`Merged(set)#SideOf(..)`) and cut-then-merged
(`Merged([piece#SideOf(..), ..])`) are still two forms. Which one a
union publishes is the open question on
`declared-flush-union-edge-and-vertex-names-follow-member-order`.
