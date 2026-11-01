---
id: the-reference-path-reads-every-node-from-mint-to-read
kind: unit
title: A reference's history path reads every node from the minting node to the read, so a flip between them is the diagnosed cause
status: open
opened: 2026-10-17
priority: P2
cost: M
---

Ev's ruling on PR 4691 (Q2, 2026-10-17): "i think the wider path makes
sense, but feel free to stage via the narrower path". PR 4674 lands the
narrower path: a reference's path in N5's "Two scopes" is the nodes its name
mentions plus the node defining its read
(`crates/editor-core/src/names/README.md`, Resolution).

**Owed.** Widen it to every node on a DAG path from the name's minting node
to the read, inclusive, and re-word "Two scopes" to say so. The case it
fixes (fork-log row 114, designer B, not executed): extrude `e` mints face
`F`; split `s` cuts it and `F` goes to `s.above`; `u` reads `s.above`, and
the reference is `(u's read, F)`. Flipping `s`'s predicate moves `F` to
`s.below`. `s` is neither a node `F`'s name mentions nor the read's node, so
the narrower path blames `e`. The wider path reads `s` and reports
`PredicateFlip` at `s`. No new diagnosis arm; only the node set grows.

Pin that case (blamed on `s`, not `e`) and one where the two paths agree.
The walk runs only when a reference fails and a cause is asked for.

Waits on PR 4674 (the narrower path and the `ReadGone` arm).

