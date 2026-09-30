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

## The arm is reachable (EMIT, 2026-09-29, on origin/main `f207b7e118`)

`NamingError::SeamVertexParentage` has live end-to-end witnesses again:
the three `wire_legal_union_refusals::no_order_of_…_refuses_a_fold_contact`
rows reach it. Each row puts a block covering `a`'s top cap over the
split fixture's area-overlap declaration (`a` against `s` on the two
y-walls), and pins every member order:
- `{a, s, big}`: 2 of 6 orders, `[a,big,s]` and `[big,a,s]`;
- `{a, s, big, p}`: 8 of 24, every order that folds `big` into `a`
  before `s` joins;
- the split fixture plus `big`: 18 of 24.

Before CONTACT-8, 25 of those 28 orders already refused
`SeamVertexParentage`, so the merge's glue did not reach them. The
other three (`[s,a,big,c]`, `[s,big,a,c]` and `[big,s,a,c]` of the
split fixture) refused at an earlier step, a fold step's contact
verdict (`work/emit/a-legal-union-refuses-a-fold-minted-contact-verdict-in-some-member-orders`).
The rows name this row as the owner, and a fix flips their entries to
`Fused`.
