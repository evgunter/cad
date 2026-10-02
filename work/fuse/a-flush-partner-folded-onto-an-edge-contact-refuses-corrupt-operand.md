---
id: a-flush-partner-folded-onto-an-edge-contact-refuses-corrupt-operand
kind: issue
title: A union whose accumulator already holds a non-manifold edge contact refuses CorruptOperand { operand: B } when a flush partner folds onto it
status: open
opened: 2026-10-02
priority: P0
cost: M
---


## Measured (FUSE measurement lane, 2026-10-02, main at bdfdda30c)

A scratch probe in `crates/topo/tests` folded three bricks with
`union_with(acc, next, flush_declarations(..) + carried)`, carrying the
previous step's contact records as `carried_a` with class `Rest`:
`a` = x∈(0,1), `b` = x∈(0.5,1.5), both y∈(0,1), z∈(0,1), declared
flush; `c` = x∈(0.5,1.5), y∈(−1,0), z∈(−1,0), which touches the
`a ∪ b` rim along an edge only (a non-manifold edge contact). In the
orders that fold `c` first against `b` and then bring `a` in (`b,c,a`,
and `a,c,b` for the x∈(0.5,1.5) variant), step 2 refuses
`CorruptOperand { operand: B }` on today's kernel, with no
canonical-form change in play. Variants with `c` at x∈(0.25,0.75) and
x∈(−0.5,0.5) refuse the same way in `b,c,a`.

Not yet reproduced through the public door (`Node::Union` in
editor-core), and the probe's own carriage of records may contribute,
so the first step is that reproduction. If it holds, a boolean output
is not a legal operand of the next boolean, which DESIGN requires of
every output; priority rises to P0 then. The probe was scratch and was
removed.

Same probe: some orders of these documents already fail tier 3′ today
(`ERR(VV, EEOverlap)` in `c,b,a` of the x∈(0.5,1.5) variant, and in
`a,c,b` and `c,a,b` of the x∈(0.25,0.75) one). Those were not examined
either; reproduce them in the same step.


## Reproduced through the public door, and fixed (branch `fuse/corrupt-operand-edge-contact`, 2026-10-02, main at e00d2442b)

**P0**: it reproduces through `Node::Union` and chained
`Node::Boolean(Union)`, with each step's flush pairs declared and no
records carried. Before the fix, `Node::Union` refused
`Boolean(CorruptOperand { operand: B, corruption: Vertex })`, naming
the member folded last:
- `b,c,a` in all three spans;
- `a,c,b` for x∈(0.5,1.5).

The kernel refuses the same orders with no `carried_a` at all, so the
probe's carriage was not the cause.

**Cause.** The accumulator holds two vertices at each end of `c`'s
edge contact. The next member's vertex there pairs with both
(`contacts.vv`). The reduction classified and inserted pair by pair, so
the crossing pair's null edges moved that vertex's orbit before the
touching pair's `build_sectors` walked it. That walk met a
`NullScaffold` curve and refused through `sectors::corrupt`. **Fix:**
every v-v pair is read before the first insertion. A touching pair
inserts nothing, so its reading stays good. Two crossing pairs at one
vertex refuse typed, as `SharedVertexCrossings`
(`work/fuse/a-vertex-crossing-both-sides-of-a-pinch-refuses-shared-vertex-crossings.md`).

**Tier 3′.** The orders that failed 3′ in the probe fail with the
records carried, and in more orders without them:
- Not carried: that is
  `work/wire/a-boolean-drops-its-operands-own-contact-records.md`.
- Carried: `remap_contacts` dropped a step's own v-v row whose end had
  fused into its partner. The other vertex at that point was left
  unrecorded, and 3′ refused it, plus the edge overlaps either side.
  Rows sharing an end are now remapped as one group. Carried, every
  order of every span passes 3′.

Pinned in `crates/topo/tests/union_flush_onto_edge_contact.rs` and
`crates/editor-core/tests/union_flush_onto_edge_contact.rs`.
