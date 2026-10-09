---
id: a-union-member-is-keyed-by-its-read
kind: issue
title: A union keys a member's names by the operation it reads, so two members read out of one operation (a split's two halves) cannot be named apart
status: open
opened: 2026-10-09
priority: P2
cost: M
needs_ev: true
refs: [part-split-half-retires, operands-are-reads]
---

DM4 (`crates/editor-core/REFERENCES.md`, "Naming keys by member, not by
depth") wraps a member's names in `FromMember { member: RecipeNodeId, of }`
and argues for the key from the edge: "The key is the edge, never the
inner name's minting node … The member id can; it is data the node
already carries, and DM5 makes it unique within one union."

Unit B (`operands-are-reads`, PR 4342) made DM5 distinctness over the
variables read, not the operations (REFERENCES DM5: "two outputs of one
operation are two variables, so a union of a split's two halves … is
admitted"). A union's member is now a read, and two members can be read
out of one operation: `Union[split.0, split.1]`. Both carry the member id
of the split, so `names::member_view` and `names::name_union` (both keyed
by the member's node id) cannot tell the two apart.

Until this is settled the union refuses such a list typed, before any
fold: `NodeErrorKind::MembersShareAnOperation { operation, members }`
(`eval/wire.rs`, `wire_union`), Python tag `members_share_an_operation`,
pinned by `intent_s2_b_reads::dm5_is_over_the_variables_read` and
`a_pair_declared_across_one_splits_halves_is_sided_by_table`. The way to
rejoin two halves meanwhile is a pair `Boolean`, whose `FromA`/`FromB`
naming keys by seat.

The question for Ev: should `FromMember` key by the member's read (the
variable: an output's operation and port), which is DM4's own argument
("the key is the edge") carried to the edge B made a read? Doing so
changes a ratified clause and moves every union name: every name digest
over a document holding a `Union` (the corpus name digests, `perf2`,
`name_words_corpus`, the die's tables and their goldens) is re-baselined,
and `MembersShareAnOperation` retires with it.
