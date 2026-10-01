---
id: a-merged-face-with-several-same-side-constituents-has-no-chord-rule
kind: issue
title: A seam chord bordering a merged face that holds several faces of one operand has no naming rule
status: open
opened: 2026-09-28
priority: P0
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

## A pair boolean reaches it with no declaration (EMIT, 2026-09-30)

A slot subtracted across a cylinder boss sunk into a plate reaches
this refusal on an everyday recipe, which is why this row is now P0:
- plate `[0,3]² × [0,1]`; boss of radius 0.3 about (1.5, 1.5), z 0.5
  to 1.5, drawn as `circle_split(.., 2, ..)`, so its wall is two
  semicircular pieces; the two joined by a pair union or a `Union` in
  either member order;
- slab x 1.4 to 1.6, y −1 to 4, z 0.8 to 2.0, subtracted.

The slab cuts each wall piece in two, and the subtract merges the two
cosurface pieces on each side of the slot. Each merged face holds
BOTH boss pieces, so the chord where a slab wall meets it has two
A-side constituents. It refused `Duplicate` at the face pass until
`a-slot-across-a-sunk-cylinder-boss-refuses-duplicate-merged-face-name`
closed; it now reaches `chord_descent`.
`emit_union_borders::a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls`
pins it, and the chord rule flips its expectation.

A UNION reaches it too: joining the same plate and boss to a bar at
x 1.4 to 1.6, y 0.5 to 2.5, z 0.8 to 2.0 with a plain pair union
refuses `MergedChordConstituents { several: 2 }` as well. The bar
passes through the boss wall on both sides, so each merged wall face
again holds both boss pieces. Measured on PR 3547's head.

What it adds to the rule: the two constituents lie on ONE curved
surface, so the pick cannot read carriers. It has to ask which
constituent's operand face holds the chord as a region, on a cylinder.
`topo::point_in_face` answers for planar faces only
(`KindUnsupported` otherwise), so the pick needs a curved in-face test
or a combinatorial record of which pre-merge face the chord bordered.

## Now on the document crosslap's path (REACH, 2026-10-01)

Since the continuation ruling (PR 3613, built on
`reach/cosurface-continuation`) an undeclared continuation refuses at
the reduction. The north-star crosslap's beams have four: their tops
and bottoms carry on into each other across the notch edges. Declaring
only the mate (the five `Rest` findings) now refuses
`undeclared_contact` on one of them, and following the menu to its end
(all nine findings declared) reaches this row's
`merged_chord_constituents`. So the document-layer crosslap, which used
to glue on the mate alone (to a body whose coplanar tops and bottoms
were left unmerged), waits on this rule. The kernel tour's crosslap
declares the whole inventory and glues; only the naming layer stops.
Pinned in `crates/pncad-py/tests/test_north_star.py`
(`TestCrosslapGlued`).
