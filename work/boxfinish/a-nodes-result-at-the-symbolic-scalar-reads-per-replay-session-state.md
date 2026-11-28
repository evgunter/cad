---
id: a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state
kind: unit
title: A node's result at the symbolic scalar reads per-replay session state: the expression table, the registry and the retry cap belong to a replay, not to a node or the drive
status: closed
opened: 2026-11-22
priority: P1
cost: H
parent: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
branch: boxfinish/session-state
pr: 7492
closed: 2026-11-28
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
drive's memo, the `frozen` column, dropping the tables). **A
drive-scoped table does not pay it once**, as this row first said:
measured by the unit, the teardown is the leaf's early memo, which
stays per leaf, and moving the table, the plain forms and the atom
records to the drive changed neither the wall time nor the teardown
(the section below, and
`a-leaf-rebuilds-and-drops-its-early-forms-though-most-read-no-bracket`).
The unit makes a node's answer its own; it does not make the drive
faster. Untested by any measurement: a drive in
which the retry ladder runs or an identity is registered.

**Sequenced after the decide path's unit** (SYM's row, in flight on
`boxfinish/decide-path-check`), which removes four of the session's
count columns and two of its walks: this unit moves the same counts
onto a node's value, so it stacks on that branch once its PR is open.
After `a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression`.

**Letter H; dual tier, a concurrent pair** (the symbolic session is
read by every decision at a symbolic scalar; a piece of state moved to
the wrong owner changes an answer without a red row, and the two
units after this rest on a node's result depending on its key alone).

**Dispatched 2026-11-22, stacked on `boxfinish/decide-path-check`**
(PR 7394, in review), which removes four of the session's count
columns, its `free` table and the walks after a definite sign: this
unit moves what is left. The lane merges `boxfinish/node-key` when
that PR opens (it gives a value with no expression a name scoped to
its minting node, the scope this unit extends), and reports a census
of the session's state and its plan before it builds.

**The split rule rests on this unit** (split-reads lane, 2026-11-22).
`drive::bisect` splits only what the deciding node reads
(`Doc::free_reads`), and a deciding node that reads nothing of the box
refuses `ReadsNoVariable`: "no split changes it". That is exact when a
node's result is a function of its inputs. While a node can read
per-replay session state (the retry ladder's per-replay cap, the
registry), an earlier node that reads another variable can move that
state, and a split of that variable could change what the deciding node
says. Not observed: no retry runs in the drives measured, and the
corpus differential compares content keys, which do not see the session.

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
  such mints inside a session over `editor-core`'s suite and 216 over
  the tour's. No built result holds a value with no expression
  (45,430 results over `editor-core`'s suite, read at the top of each
  value), and over the tour's tests none holds an expression that
  reaches one (24,628 results, each value's expression walked).
- **The deferred list's entry numbers** (`sym::deferred`'s `NEXT`) are
  a process-wide count: unique, and not the same in two runs.
- **`SymOp::Free`** has one name for every mint (`INDET_FREE`), by
  design: the identity over it holds for every value.

## Built and measured (the unit's lane, 2026-11-23)

**The census, and each piece's owner** (`geom_core::sym::Session`'s
doc states it in the code):

- **the drive's** (`sym::memo`, `DriveMemo`): the expression table,
  the plain forms, the atom records, the frozen set; the dials and the
  witness kind are the same for every leaf of a drive;
- **the leaf's input**: the parameter brackets; **caches valid for the
  leaf**: the early memo, the reduction memo, rule D's closed forms,
  the budget-frozen set;
- **a node's** (`sym::NodeState`, opened by `MintScope::node`, left as
  a `NodeRecord` on `NodeValue::sym`): the registry, the counts, the
  retry memos and so `SymRetry::max_forms`, the door memo, the plain
  roots;
- **left as it was**: the deferring session's list (`sym::deferred`),
  which is the lint's replay and never a drive's; its entries are
  named process-wide and it takes no prior.

**Does a node's answer depend on its predecessors (item 2).** Two
mechanisms constructed at the tier's door, red before and green after,
each with its mutant run
(`geom-core/tests/a_nodes_answer_at_the_symbolic_scalar_is_its_own.rs`):
an identity registered by a node that does not feed the asker, and the
retry guard spent by an earlier node's forms. A third, the atom
table's one unrecorded id (rule B's `cos` twin), was not made to
change an answer: under the shipped rules rule H takes `sin²` before
rule B sees it. The twin is recorded with the form that carries it
all the same, because a drive-wide atom table would otherwise make
rule H's reading of it depend on other leaves, and that is a row
(`sym::tests::rule_bs_cos_twin_is_recorded_with_the_form_that_carries_it`).
No document in `geom-core`'s or `editor-core`'s suites held either
mechanism: head against the base at the default ε, no pinned receipt,
frozen count, walk-ledger digest or `serialize()` line moved.

**One key, one result (item 3)**, where the ladder runs (R2's bracket
at ten ε: 52 keys, 38 repeats, 54 retried decisions) and where
identities are registered (the plate), and a leaf's receipt equal to
the sum of its nodes' records
(`editor-core/tests/a_nodes_result_at_the_symbolic_scalar_is_its_keys.rs`).

**The teardown (item 4)** did not move: the finding is
`a-leaf-rebuilds-and-drops-its-early-forms-though-most-read-no-bracket`.
The drive's table on the tour's 512-leaf study: 75,915 nodes, 34,546
plain forms, 1,036 atoms, 36 frozen, 25.4 MB by the table's own
estimate (19.3 MB for the forms and atoms before); one leaf of the
same document holds 72,691 nodes, so the table does not grow with the
leaf count there. Peak resident memory of the run 705 MB before and
after.

**The taint and unrecorded branches (item 6)** are removed; the row
they were filed under is closed with the list of the rows that pinned
them (`work/sym/a-taint-induced-freeze-under-a-hit-still-reads-by-order`).

## Closed

Landed by PR 7492 (2026-11-28), after a concurrent pair's two rounds
and two fix passes; the record of both is
`docs/model-compare-log/rows/PR-7492.md`.

What is true now: every piece of the symbolic session is the drive's,
a node's or a cache valid for the leaf. A node's value carries its
record (`geom_core::sym::NodeRecord`: its registrations, brackets,
counts and roots, and the records it was built over), and a scope
opens over the records of exactly what it reads and over nothing of
the scope it is inside. A family's record is its members' gathered; a
part's run and the product's gather are scopes of their own
(`sym::apart`). A leaf's receipt is the sum of its nodes', parts' and
product's records. An absent node is an unknown that is neither
published nor pooled, and `evaluate` refuses a prior built under
another drive's table.

What it leaves, each with its own row:

- The product's gather reads no registration of the instances it
  gathers, and drops its record where it refuses:
  `a-documents-registrations-do-not-reach-its-products-gather`.
- The order a reader takes its inputs' records in decides which of
  two cycle-closing identities it keeps:
  `work/sym/a-pair-of-registrations-that-closes-a-cycle-is-proved-in-one-read-order-and-not-the-other`.
- The per-leaf teardown did not move:
  `a-leaf-rebuilds-and-drops-its-early-forms-though-most-read-no-bracket`.
- What the store has to hold to, as notes on its row:
  `a-drive-keeps-no-node-result-from-one-leaf-to-the-next`.
