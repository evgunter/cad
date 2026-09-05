---
id: live-guard-proves-ordering-not-identity
kind: issue
title: the Live guard proves a lookup precedes the construction, not that it resolved the key that gets wrapped
status: open
opened: 2026-09-05
refs: [D50]
track: P
---

## What

`live::tests::every_door_that_hands_out_a_live_looks_up_first`
(`crates/topo/src/live.rs:289`) reads each door's body as text and
compares two byte offsets: the first spelling from its lookup
vocabulary, and the first `Live::new(` / `Self(` construction. A door
passes when the lookup comes first.

That is an ORDERING fact, and the claim in the module header
(`crates/topo/src/live.rs:8`) is an IDENTITY one — *this key resolved*.
A door that looks up key `a` and then wraps key `b`

```rust
pub(crate) fn wrong(&self, a: HalfEdgeKey, b: HalfEdgeKey) -> Option<Live> {
    self.half_edges.get(a).map(|_| Live::new(b))
}
```

is green under the guard today, because the vocabulary matched before
the construction did. The compiler still bounds the damage — the
constructor is private, so such a door could only be written inside
`live.rs` — and the four doors that exist each wrap the key they
resolved, which is what makes this a residue and not a defect.

## Why it was not closed with D50

Closing it means reading the ARGUMENT of the lookup and the argument of
the construction and comparing them, which is a data-flow question over
one function body, not a text one: `Live::of(self, member)` inside a
`.map(|member| …)` closure resolves the closure's binding, and
`self.half_edges.get(he)` hands its `Some` arm to a pattern rather than
to the construction. A textual walk can carve both arguments, but it
cannot say the two names denote the same value without following the
binding — which is the call-graph read `source_walk.rs`'s module header
already rules out as a much larger decision than a guard row.

The cheap partial is available and was not taken because it would red
on the two doors that are correct: requiring the construction's
argument to be *spelled* the same as the lookup's holds for
`Live::of` (`contains_key(he)` / `Self::new(he)`) and
`resolve_half_edge_live` (`get(he)` / `Live::new(he)`), but says
nothing about `loop_cycle_live`, which constructs nothing itself, and
would have to special-case delegation. A row that wants this closed
should decide whether the argument comparison is worth that
special-casing, or whether the private constructor plus review of one
file is where the line belongs.

## Fence

Track P — `crates/topo/src/live.rs`, and `source_walk.rs` if the item
scan has to grow an argument carve.
