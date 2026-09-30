---
id: a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names
kind: issue
title: A declaration on a face one fold step cut and partly merged cannot be routed by names, so its correct orders refuse beside orders that would bind the wrong face
status: open
priority: P2
cost: M
design: true
opened: 2026-09-30
refs: [union-refuses-in-some-member-orders-and-publishes-in-others, a-face-cut-and-merged-in-one-step-publishes-a-piece-under-the-name-its-merge-retires, 3526]
---


## The finding

Found by EMIT on `emit/cut-and-merged-pair` (PR 3526).

A union's declaration door (`look_through_fold`,
`crates/editor-core/src/eval/wire.rs`) routes a member-space name through
the fold's compositions. It reads the accumulation's rows by name only.
DM4 (`crates/editor-core/REFERENCES.md`) says the composition is read
off the rows that descend from the face, "never by measuring it again",
and that "which fragment the pair meant is the geometric question the
routing step does not ask".

One fold step can cut a member face and merge part of it. The
accumulation then holds two faces for that name:
- a bare `Merged` row listing the name as a constituent;
- a `Borders` fragment of the name.

The right face for a declared pair depends on where the partner's
contact lies. The two corpus cases need opposite answers:
- `r2endsg`: `ALONG` is x 0..3, `CEND` is x −1..1 and the slab `smid` is
  x 1.4..1.6. In orders `[1,3,0,2]`, `[2,3,0,1]`, `[3,1,0,2]` and
  `[3,2,0,1]`, `smid` cuts `ALONG`'s wall and the x 1.6..3 part merges
  with `BEND`'s. The `ALONG`~`CEND` contact at x 0..1 lies on the
  x 0..1.4 fragment.
- `r4trig`: in orders `[1,3,0,2]` and `[3,1,0,2]`, the contact at
  x 0.8..1 lies in the merged face.

Before PR 3526 the pair step published the fragment under the bare
constituent's name, and the door bound it. For `r2endsg` that was the
correct face. For `r4trig` it was the wrong face, silently. Looking
through to the merged row flips both. No rule that reads names alone
routes both cases correctly. So since PR 3526 the door refuses
`ConsumedByFold { by: Split }` for every such order. The 4 `r2endsg`
orders published correct tables before and are now lost to totality.

## The question

Can the door route such a pair without re-measuring the face? Options
the designers should weigh:
- read the partner's contact region at the step (DM4 rules this out today);
- have the fold keep, per step, which piece each declared contact lies on;
- keep refusing, and offer both the fragment and the merged row.

The refusal is pinned by the `KNOWN_MIXED` counts in
`emit_union_rim_piece_ranks` (`r2endsg` DeclareResolve:12; `r4trig`
Boolean:2/DeclareResolve:12) and by the "split and partly merged in one
step" case in `wire.rs`'s
`a_member_face_consumed_other_than_by_a_merge_refuses_naming_the_composition`.
