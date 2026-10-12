---
id: an-instance-defines-one-body-per-part-placement
kind: issue
title: An instance defines one Body port holding its part's whole world; FORK-1's signature (one Body per world placement, fixed by the pin) is unbuilt
status: open
opened: 2026-10-08
refs: [a-placement-is-the-bundle-of-mates]
---


Found by INTENT stage 2 unit C (`the-product-is-an-explicit-list`, branch
`intent/s2-c-world`), which built the world without it and put the
question to the orchestrator.

FORK-1 (PR 4222) and spec §4 ("Instances") say an `InstantiatePart`
defines one `Body` per world placement of its part, each at its world
coordinates, the list fixed by the pin. C keeps one `body` port
(`Node::outputs`, the `InstantiatePart` arm in
`crates/editor-core/src/node.rs`) whose value is the part's whole world
grafted into one multi-solid body (`eval/parts.rs`, `PartValue`), its
names the part's product names under `InPart`.

What the per-placement signature needs that the spec does not settle:

1. **Value shape.** Port k = copy k needs per-copy bodies and name
   tables out of the part; `PartValue` carries the aggregate, and the
   carried mate declarations (ASM-R2b D-1 rows, keyed in the aggregate
   arena) have no per-copy home.
2. **Migration.** Spec test 6 writes one `PlaceInWorld` per body root;
   a root instance of a two-placement part would need one placement per
   port to keep its product.
3. **Pin moves.** `UpdateReference` does not resolve the pin, so it
   cannot re-mint outputs when the part's placement count changes; the
   evaluation would refuse a count that disagrees with the pin.
4. **Readers.** B keeps `WrongOperand` for a non-`Part` reader of a
   multi-output node, so only placements could read a port past 0.
   What a non-`Part` reader of a split's port evaluates to is B's open
   Q2, and the same question settles a reader of an instance's port.

**Where it is decided: stage 3** (`a-placement-is-the-bundle-of-mates`),
not stage 2. FORK-S3M, stage 3's placement fork, is choosing the final
shape. One option is FORK-1's per-placement `Body` ports with a
`Place` reading a list of shapes of one space. The other is one
`bodies: Bodies` port whose members are named by the part's placements,
with `Place` reading one shape. The open `[ev]` PRs #4324 and #4326
reshape instances further (an instance enters a document only as a
placement, with no `frame` port). Nothing in stage 2 assumes either.

Split's consumer-ward re-point (`refactor.rs`, the crossing reads
re-pointed with `SetParam` to the instance's body) and inline's heir
rule (`Uncarried::Bodies`) are stated over the one port and change with
this row.
