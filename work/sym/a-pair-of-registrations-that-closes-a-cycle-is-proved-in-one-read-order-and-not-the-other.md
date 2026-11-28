---
id: a-pair-of-registrations-that-closes-a-cycle-is-proved-in-one-read-order-and-not-the-other
kind: issue
title: A pair of registrations that closes a cycle is proved in one read order and not the other: which identity a reader keeps follows the order it reads its inputs' records in
status: open
opened: 2026-11-24
design: true
priority: P2
cost: M
---


Found in the delta review of the session-state unit (PR 7492,
`work/boxfinish/a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`).

**The fact.** A node's registry is built from the records of what it
reads, one record after another (`geom_core::sym::reads`,
`crates/geom-core/src/sym.rs`), and a registration that would close a
cycle through the ones already held is dropped
(`Session::record_identity`, same file). So of two true identities that
close a cycle through each other, a reader keeps the one it reads
first. Node `A` states `u ≡ sin(v)`, node `B` states
`v ≡ atan2(u, c)`: a node that reads `A` then `B` proves `u − sin(v)`
through the door, and one that reads `B` then `A` holds
`v ≡ atan2(u, c)` alone and decides numerically.

**Pinned as it stands**:
`crates/geom-core/tests/a_nodes_answer_at_the_symbolic_scalar_is_its_own.rs`,
`the_order_two_records_are_read_in_decides_an_answer`. The order is
the document's: a node's reads in field order
(`eval::read_records`, `crates/editor-core/src/eval/mod.rs`, over
`wire::seen_reads`), each record after the records it was built over.
It is the same in every leaf, so the dependence is not on a schedule.
It is on how the document is written.

**By inspection, not run** (the second reviewer): the node an alias
chain ends at follows the same order (`Session::alias`), so reordering
a union's members could move a door proof.

**The question.** Both answers are sound: a dropped identity is a
missed cancellation. What is open is whether a proof should depend on
the order a node's inputs are written in. Ways out a designer would
weigh: keep both identities and make the registry a relation the door
closes under (a congruence, with the cost of the closure); choose the
kept identity by a rule on the identities themselves and not by
arrival (an order on ids); or leave it and say so where a document's
author would read it. No document in `geom-core`'s or `editor-core`'s
suites has two registrations that close a cycle across nodes: reversing
the order in every scope moved no row before the pin above was added.
