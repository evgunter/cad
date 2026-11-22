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

**Evidence from the node-key unit** (its sweep and its audit of what a
node reads against what its key hashes; by reading unless said):

- **The parameter brackets are an input no key hashes.**
  `Sym::param_over` records each axis's `[lo, hi]` in the session
  (`Session::params`), and the decision read (`sym::signed`) reads them
  for every parameter a form mentions. A node's key hashes each fed
  value's number and expression; the expression names the parameter
  and not its bracket, and the number is the whole expression's
  enclosure, which two boxes can share (`a·0 + 5`). So two replays can
  hold a node under one key and two brackets.
- **A mint scope's count is kept by the session** while the scope is
  closed (`Session::mints`, `sym::mint`), so that a scope opened twice
  in one session mints no name twice. In a replay each node's scope
  opens once, so nothing reads it; it is per-replay state all the same.
- **The session's root mint scope** counts from zero per session: a
  value minted under no node's scope (a read after the evaluation, the
  clearance query's) is named by the replay's order. Measured: 53,065
  such mints inside a session over `editor-core`'s suite, 216 over the
  tour's, none in any node's result.
- **The deferred list's entry numbers** (`sym::deferred`'s `NEXT`) are
  a process-wide count: unique, and not the same in two runs.
- **`SymOp::Free`** has one name for every mint (`INDET_FREE`), by
  design: the identity over it holds for every value.
