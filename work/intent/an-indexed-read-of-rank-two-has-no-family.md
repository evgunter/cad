---
id: an-indexed-read-of-rank-two-has-no-family
kind: issue
title: DM3's xs[i, j] is refused: every family a node defines is one flat list, so no read has rank two
status: open
opened: 2026-10-10
---


REFERENCES DM3 (`crates/editor-core/REFERENCES.md`) ratifies "`xs[i]`, or
`xs[i, j]` for a family keyed by two indices", and `docs/DESIGN.md` says
"`xs[i, j]` reads one member". FORK-DM4 unit 1 (PR 4527) builds the indexed
read at every body seat with rank one only: `Node::input_fault` refuses any
`BodyRead` with more than one index (`InputFault::IndexRank`,
`crates/editor-core/src/node.rs`), and evaluation reads `SlotId::Index { seat,
k: 0 }` alone (`eval/wire.rs`, `instance_of`).

The reason is the families, not the read: every family a node defines today is
one flat list. A pattern of patterns lists its copies `j·M + i`
(`wire_pattern`), so `xs[k]` on a nested pattern reads the flat index and there
is no family whose rank is two. Building `xs[i, j]` needs a family value that
carries its shape (`M` per level), so the read can check each index against its
own extent and say `Instance { i, .. }` per level, and then lifting
`IndexRank`'s fixed rank of one to the family's rank.

Raised by both reviews of PR 4527 (r1 m2, r2 N1).
