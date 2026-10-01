---
id: a-split-that-de-ties-tied-faces-swaps-their-names-under-an-edit-that-moves-the-split
kind: issue
title: A split that de-ties two tied faces names each piece against the split's walls, so an edit that moves the split to the other face swaps the names
status: closed
opened: 2026-09-26
priority: P2
cost: E
closed: 2026-10-01
pr: 3523
---


## What

Tied member faces are separate parents spelled alike (N2, keyed by
entity). When a member splits one of them, that parent's pieces are
named `Fragment(Borders)` over the splitting member's walls, and each
set is unique; the other parent stays whole under the bare tied name,
now its only candidate, so the row narrows to `Unique`. The wall sets
say which walls a piece borders, not which of the two tied faces it came
from. An edit that moves the split onto the other tied face therefore
hands the same names to the other face.

## Evidence (re-measured under `Borders`, PR 3241 at 84c0ab87cb)

The fixture is `tied_prongs` in
`crates/editor-core/tests/emit_union_borders.rs` (a 4×4×4 block with two
slot ceilings that are one tied face of the fork), united with a bar at
x 2.9..3.1, y 0.8..1.7, z 2.5..3.5, behind a transform. The bar crosses
the lower ceiling and splits it. A `SetParam` on the transform's y
translation (+1.5) moves the bar across the upper ceiling instead. Both
member orders behave the same. Face centroids, by published name:

| name (after the tied parent) | before the edit | after the edit |
|---|---|---|
| `Borders{bar x=3.1}` | lower ceiling, x > 3.1 (y 1.25) | upper ceiling, x > 3.1 (y 2.75) |
| `Borders{bar x=2.9}` | lower ceiling, x < 2.9 (y 1.25) | upper ceiling, x < 2.9 (y 2.75) |
| the bare tied name, narrowed to `Unique` | upper ceiling (y 2.75) | lower ceiling (y 1.25) |

The pieces are named in `emit_union::name_by_parents` through
`names::borders::Obstacles::split`; the whole ceiling keeps the tied
spelling in the same function's one-face arm.

## Why it matters

Two names that denoted the lower ceiling's pieces now denote the upper
ceiling's, and the whole-face name moves the other way. Nothing reports
it, and a reference follows the name to the other face.

## Fix direction

The tie was the honest answer: nothing covariant tells the two ceilings
apart. A split that de-ties them has to either keep its pieces tied, as
N2's tie propagation would, or name them against something that follows
the tie's candidate rather than the split's walls. Which one is a design
call on N2's tie rule, so it may need Ev.

## The fork (2026-09-30)

A designer pair weighed this.

- **First round:** they split.
  - One proposed carrying each face's tie candidate as lineage and letting a reference pin it with `Chosen(k)`.
  - The other called the behaviour ratified: names denote roles, and the bar is a covariant discriminator, as in N4's narrowing clause (fork-log row 6).
- **Reconciliation:** both now recommend the second reading.
  - `k` is arena storage order, and an upstream edit can permute it silently, so pinning it is a worse swap than following the bar.
  - Both agree the "recorded user choice" N2 promised was never built.
  - Both would replace it with: repair by a discriminator in the recipe.

The `[ev]` PR (3523) carries the rewording.

## Closed

By design. Ev accepted the rewording on PR 3523 (2026-10-01): "this
sounds good! i'm not sure if i'm visualizing the situation correctly
and hence am not sure if there is actually a unique way to be able to
carry this forward, but the behavior you suggest seems safe and
principled". N2 now repairs a tie by a discriminator in the recipe or a
`Rebind`, with no per-candidate choice, and N4 names a divider as a
discriminator among a tie's candidates
(`crates/editor-core/src/names/README.md`).

`emit_union_borders::a_divider_moved_onto_the_other_tied_face_moves_the_names_with_it`
(`crates/editor-core/tests/emit_union_borders.rs`) pins the Evidence
table above by centroid, in both member orders: the two `Borders`
pieces lie on the lower ceiling (y 1.25) before the edit and on the
upper (y 2.75) after, and the bare tied name, narrowed to `Unique`,
moves upper to lower. A change to that motion is a change to the
ruling and re-baselines the test on purpose.

The optional datum-side discriminator is not filed; Ev did not ask
for it.
