---
id: a-per-member-indexed-read-at-a-node-seat-names-members-twice
kind: issue
title: A per-k reader's node-seat read xs[k] names its members Member › Member › e, the index said twice
status: open
opened: 2026-11-10
priority: P2
cost: M
---


Found by a designer's probe on fork row 258 (PR 6555) and not pinned by
a test. REFERENCES DM3 says a member read answers the member's own
table, with the index said once: in the read, and in `Member` only where
the family is read whole or a member's entity is carried.

At a node seat it is said twice. A per-`k` reader reading `xs[k]` at a
body seat is projected by `eval/wire.rs` `reads_projected` /
`member_projected`, which reads the family's table projected onto the
member (`NameTable::project`), so the member's names keep their
`Member { (k, i), e }` qualifier. The reader is itself a family under
`k`, whose table wraps each member's names in `Member` again
(`eval/members.rs`, `names::name_family`): its members are named
`Member › Member › e`. A per-`k` reader of plain `xs` reads the member's
own value (`eval/members.rs` `overlay`) and names `Member › e`, so the
two spellings of one read name its result apart.

PR 6555 stores a selection's `xs[k]` on the family's own index chain as
plain `xs` (`edit.rs` `mint_selection`), which a node seat's read does
not do. Done when a per-`k` node-seat `xs[k]` names its members as the
plain read does (normalised at the door as a selection's is, or
projected onto the member's own table), pinned by a test comparing the
two readers' tables.
