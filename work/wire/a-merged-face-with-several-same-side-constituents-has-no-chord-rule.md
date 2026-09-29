---
id: a-merged-face-with-several-same-side-constituents-has-no-chord-rule
kind: issue
title: A seam chord bordering a merged face that holds several faces of one operand has no naming rule
status: open
opened: 2026-09-28
---


## What

`emit_topo`'s `chord_descent` (in `name_boolean_edges`,
`crates/editor-core/src/names/emit_topo.rs`) reads a seam chord that
borders a MERGED face through to that face's one constituent on the
side it asks for. Since CONTACT-8 the merge glues planar groups it used
to skip, and a declared union can now glue faces of TWO members of one
assembly into one merged face. The chord then has several same-side
constituents and nothing picks the one it lies on. CONTACT-8 made that
a typed missing-rule refusal, `NamingError::MergedChordConstituents`
(it was an `Emission`, which reads as a kernel bug).

Reached by, measured on CONTACT-8's head:
- `docm8_flat_merged`'s split fixture, orders `[c, s, a]` and
  `[s, c, a]`;
- its fragmented-merge and three-neighbour-star fixtures, the four
  orders each where the capped member joins after the cutter and ONE
  of its two partners (`cap_outcome`);
- `wire_legal_union_refusals::a_merged_face_with_several_constituents_is_a_missing_rule_not_a_kernel_bug`.

The bodies are sound: the orders of the same documents that fuse are
tier-3 green at the analytic volume.

## The rule this wants

A geometric pick, the way `chord_on_rim` / `rim_holding` pick a rim:
the constituent whose operand face holds the chord. When it lands,
the rows above flip to `Fused`.

## A witness lost

`NamingError::SeamVertexParentage` had its only end-to-end witnesses
on these same orders: the merge used to skip the y-wall group (its seam
bends where the cutter leaves the capped block), and the vertex pass met
the unglued walls first. With the group glued, no document in the
corpus reaches that arm (`emit_topo`'s `([_], [], _, _)` in the seam
vertex pass); only `display_contract` and the concision rows pin its
sentence. A search over seven variants of the split fixture found none.
Whether the arm is still reachable is part of this row.
