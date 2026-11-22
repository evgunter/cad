---
id: a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state
kind: unit
title: A node's result at the symbolic scalar reads per-replay session state: the expression table, the registry and the retry cap belong to a replay, not to a node or the drive
status: open
opened: 2026-11-22
priority: P1
cost: H
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
---


The last two of the four changes in point 2 of the parent row's
weighed state (`[ev]` PR 7368): the expression table belongs to the
drive, and there is no per-replay session state a node can read. Each
piece is either a cache whose misses never change an answer, or
belongs to a node and travels with its value (the identities it
registered, its counts, its deferred entries); the registry is scoped
to a node's inputs, and the retry ladder's per-replay cap
(`SymRetry::max_forms`) becomes per node.

Measured (M9): 23.0 s of the tour's 177.6 s of leaf time is the
session's per-replay teardown (publishing the leaf's forms to the
drive's memo, the `frozen` column, dropping the tables), which a
drive-scoped table pays once. Untested by any measurement: a drive in
which the retry ladder runs or an identity is registered.

**Sequenced after the decide path's unit** (SYM's row, in flight on
`boxfinish/decide-path-check`), which removes four of the session's
count columns and two of its walks: this unit moves the same counts
onto a node's value, so it stacks on that branch once its PR is open.
After `a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression`.
