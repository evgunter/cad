---
id: an-indexed-read-of-rank-two-has-no-family
kind: issue
title: DM3's xs[i, j] is refused: every family a node defines is one flat list, so no read has rank two
status: open
opened: 2026-10-10
priority: P2
cost: H
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

**Status after PR 6555 (2026-11-10).** A family keyed by two indices exists today: an index declared `within` another (`fixture::index_of` with `Some(i)`) and a family over it reach both, so a member read carries two indices. PR 6555's selections read such members at rank two and evaluate: `crates/editor-core/tests/msolve1_transform_aware.rs` `a10_a_nested_family_head_is_read_at_its_member` and `a11_a_placement_between_two_patterns_composes_outer_t_inner` seat a top on `outer[1, 1]` (`eval/wire.rs` `member_at` matches the read's values against the family's members). `InputFault::IndexRank { rank: 2 }` is reachable at the door: `mate1_r1_probes.rs` and `fix_pattern_mate_crossing.rs` pin a one-index side over that family refusing `{ found: 1, rank: 2 }`. What this row says is refused — a node seat's `xs[i, j]` — needs re-checking against that: the flat `j·M + i` listing of a pattern of patterns (`wire_pattern`) is one family shape, and the index-within shape another.
