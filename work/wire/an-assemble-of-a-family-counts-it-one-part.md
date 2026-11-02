---
id: an-assemble-of-a-family-counts-it-one-part
kind: issue
title: A ⊔ reading a family counts the whole family as its master's parts, not one per member
status: closed
opened: 2026-10-28
closed: 2026-11-01
priority: P1
cost: E
refs: [5903, 5954]
---

Found by the carry (`a-placement-carries-its-sources-at-rest-result`).

`Carry::parts` is how many parts a value's body is; a family's
(`Instances`) is each member's. `wire_assemble`
(`crates/editor-core/src/eval/wire.rs`) adds `value.carry.parts` once
per member read, so a `⊔` over a two-member pattern of one block
counts 1 part, and its product says `parts == 1`
(`Product::parts`, the sum of its pieces' `carry.parts`), where the
same two copies spelled as two reads count 2. A family read whole is
as many parts as its members: `parts × members`.

Patterns become index variables in G (`patterns-are-index-variables`),
where each member is its own arrival and read; this is fixed there or
beside it.



## Closed as already fixed (2026-11-01, WIRE orchestrator)

G's fix pass a0c6555c6a made a family's carry `Carry::of_family`
(`crates/editor-core/src/eval/carry.rs`), whose `parts` is the sum of its
members' parts; `members.rs` builds every `Instances` family through it, and
`wire_place` carries `parts` through `Carry::copied`. So `wire_assemble`'s
per-read `value.carry.parts` already counts each member. Probe on main
(wire-carries lane, not committed): a block, `fixture::row` with k = 2,
`Assemble { members: Bodies::Family(row) }` as the product, read family
`carry.parts` = 2, assemble `carry.parts` = 2, `Product::parts` = 2. The
other `Carry::parts` readers (product, distribute, parts, stackup) read the
same summed count.
