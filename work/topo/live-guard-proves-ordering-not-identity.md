---
id: live-guard-proves-ordering-not-identity
kind: issue
title: the Live guard compares the spelling of the key looked up, not the key
status: open
opened: 2026-09-05
refs: [D50]
track: P
---

## What

`live::tests::every_door_that_hands_out_a_live_looks_up_first`
(`crates/topo/src/live.rs`) checks two things about each door: that a
lookup precedes the construction, and — since the fix pass — that the
argument SPELLED into the first lookup is the argument spelled into the
first construction. `self.half_edges.get(a).map(|_| Live::new(b))` reds
now; `Live::of`'s `contains_key(he)` / `Self::new(he)` and
`resolve_half_edge_live`'s `get(he)` / `Live::new(he)` pass because the
spellings agree.

Spelling agreement is not key identity, and two gaps survive it.

## The two that survive

**1. The lookup names an arena, not a body.** The vocabulary is
anchored on `half_edges.` — `self.faces.get(f)` no longer stands as a
half-edge lookup — but any `half_edges` will do, so a door that resolves
`other.half_edges.get(he)` and wraps `he` reads as correct. That is the
module header's own live-but-wrong residue (`live.rs`, *What a `Live`
claims*) reaching the guard: a token proven against one body and
spliced into another is the validator's business, and a textual walk has
no more purchase on it than the type does. A rebinding is the same gap
in miniature — `let k = he; … Live::new(k)` compares `he` against `k`
and reds, which is loud rather than wrong, but
`let he = other_key; … get(he) … Live::new(he)` agrees with itself.

**2. An item declared inside a door's body is not scanned as a door.**
The item scan sets its cursor past each body it reads, so a nested
`fn forge(k: HalfEdgeKey) -> Live { Live::new(k) }` inside
`loop_cycle_live` never becomes a row of the door table. It does red —
through the construction census, and through the argument comparison
(*"`loop_cycle_live` looks up `he` and wraps `k`"*) — but under the
ENCLOSING door's name, so the message points a reader at the wrong item.

## Why neither was closed with D50

Closing (1) means knowing that two receiver expressions denote the same
`Body`, which is a data-flow question over one function body and not a
textual one; it is the call-graph read `source_walk.rs`'s module header
already rules out as a much larger decision than a guard row. Closing
(2) means recursing into item bodies, which changes what
`CodeOnly::fns` means for the two mutation-door guards built on it —
their populations would gain every nested helper — so it is a change to
the shared scan, not to this row.

Both are disclosed in the row's own doc comment. A row that wants either
closed should decide the scan question first.

## Fence

Track P — `crates/topo/src/live.rs`, and `source_walk.rs` if the item
scan has to learn to recurse.
