---
id: a-merged-face-with-several-same-side-constituents-has-no-chord-rule
kind: issue
title: A seam chord bordering a merged face that holds several faces of one operand has no naming rule
status: closed
opened: 2026-09-28
priority: P0
cost: H
closed: 2026-10-18
pr: 5263
branch: wire/merged-chord-rule
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

A `PlacedUnion` operand reaches it with planar faces only (SHOW,
2026-10-02, measured against the heat-sink recipe of
`demos/tour/src/heatsink.rs`): a `3 × 1 × 0.25` base and five
`0.1875 × 1` fins standing flush on it and running its full depth, so
each fin's two end faces are coplanar with the base's long side faces.
`find_flush_candidates(base, group)` reports 15 pairs (five feet
`Rest`, ten end walls `SameOriented`); declaring all 15 and unioning
refuses `Naming(MergedChordConstituents { several: 5 })`. Each merged
side face holds the five fins' end faces from the one group operand.
This is the shape a real extruded heat sink has; the tour's scene
stops its fins 1/8 short of the long sides.

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
Pinned in `crates/pncad-py/tests/test_north_star.py` at the time
(`TestCrosslapAtTheNamingWall`); see below.

## More orders once declarations stop refusing by order (EMIT, 2026-10-06)

Once `DeclareResolve` stops refusing by order (INTENT's stage 4, or a
fan-out), `NamingError::SeamVertexParentage` gains `near` (the ZIP
document, `emit_union_flush_names.rs`) `[0,2,1]` and `[2,0,1]`, which
refuse `DeclareResolve` today. Measured in
`work/emit/union-refuses-in-some-member-orders-and-publishes-in-others.md`,
"Re-measured on main (2026-10-06)".

## The arm loses its witnesses again (FUSE, 2026-10-06, `fuse/set-names`)

With maximal edges at every boolean output stage (PR 4140) and joined
edges named for the set of the edges they cover, every
`SeamVertexParentage` order in the three
`wire_legal_union_refusals::no_order_of_…_refuses_a_fold_contact` rows
fuses at the volume the geometry says: 2 of 6, 8 of 24 and 18 of 24
orders flip to `Fused`. The vertex the arm refused was a cut vertex
the join now removes, or a vertex on a joined edge that the set's
constituents now name. No document in the editor-core suite reaches
`emit_topo`'s `([_], [], _, _)` arm on that branch; only
`display_contract` and the concision rows pin its sentence. The
`MergedChordConstituents` orders of the split fixture (`[c,s,a,big]`)
remain, and are this row's subject.

## Two witnesses were split pieces, not chords (EMIT, 2026-10-12, PR 4794)

Two witnesses stop refusing under the killed-lineage chase
(`emit_topo`'s `KilledSplits`, PR 4794): R1's split fixture folded with
`a` last, and the same fixture under `big` (`[c,s,a,big]` and
`[s,c,a,big]`). Both now fuse at the volume the geometry says.

The edge that refused was no join-minted chord. It was a piece of a
member edge, split off around a stretch the fold killed. The kill
dropped that stretch's birth record, so the lineage stopped short of
the operand table and the edge went to `chord_descent`. It now resolves
through its lineage. Over the whole editor-core suite, instrumented on
PR 4794's tree, 8084 edge reads resolve through a killed stretch where
they did not before. Each lies within the operand edge it is named for
(`chord_within`), and each operand edge is named in its table.

The rows changed with it:
- `wire_legal_union_refusals::a_split_fixture_with_a_merged_wall_fuses_in_every_order`
  pins all six orders fused, with every member edge named lying on its
  source.
- `no_order_of_the_split_fixture_under_a_covering_block_refuses_a_fold_contact`
  pins all 24 orders `Fused`.
- `a_merged_face_with_several_constituents_is_a_missing_rule_not_a_kernel_bug`
  now witnesses the refusal's sentence on the sunk-boss slot, which
  still refuses `MergedChordConstituents { several: 2 }` (`emit_union_borders`).

The rule is still missing for the witnesses whose chord is a join's
own:
- the sunk-boss slot (still red in `emit_union_borders`);
- the boss-and-bar union;
- the heat-sink fins;
- the crosslap.

Only the sunk boss was re-measured.

## The document crosslap leaves the row (EMIT, 2026-10-15)

The boolean now keeps the split records of the edges its carve of
discarded material kills (`BooleanNaming::carved_lineage`, closing
`work/emit/a-booleans-carve-kills-a-split-record-its-discard-rows-do-not-copy.md`).
The crosslap's chords on the merged bottom read through to the
operands' own edges, so they are named as those pieces and never
reach `chord_descent`: the union evaluates, and
`TestCrosslapGlued` pins its exact oracle, 1.875. Measured by
toggling the read of the copy in `emit_topo::KilledSplits`: without
it the scene still refuses `merged_chord_constituents`. The rule this
row asks for is unchanged:
`wire_legal_union_refusals::a_merged_face_with_several_constituents_is_a_missing_rule_not_a_kernel_bug`
still reaches it.

## Closed (2026-10-18)

PR 5263 (merged `6bf23859b7`): a chord along a merged face names the constituent that holds it whole, read as "the chord was never joined (no `EdgeJoin` keeps it) and exactly one constituent holds its middle strictly inside", with both ends checked against the pick; anything else keeps the typed `MergedChordConstituents`. Full review found a MAJOR on the first build (a middle-point pick on a chord spanning both constituents) and the fix pass closed it; the delta review found it mergeable. The test this row cited as still red (`a_merged_face_with_several_constituents_is_a_missing_rule_not_a_kernel_bug`) is gone: the sunk-boss slot names. What a chord spanning two constituents is named stays open on `a-chord-spanning-two-merged-constituents-has-no-name`; the heat-sink fin witness stands on `PlacedUnion` (D10-held) and was not measured. Residue: REFS's `a-union-judgement-that-merges-two-faces-of-one-member-links-neither`, `work/issues/a-face-holds-a-point-has-no-public-door`.

Signed (WIRE orchestrator).
