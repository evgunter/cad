---
id: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
kind: issue
title: A drive builds and finishes a body again in every leaf, though the leaf's box moves none of its inputs: a third of the tour's leaf time
status: open
opened: 2026-11-22
priority: P1
cost: H
---


Split from `tier-3-at-every-box-replay-leaf-multiplies-the-decisions`
(its second part) once M7 showed it is the largest cost left when the
check after a definite sign is gone. A design question, weighed and decided: the last section holds
(`docs/ERROR-DESIGN.md` E6 carries the ratified text).

**What is measured** (the box-scalar switch's box arm off, so every
sweep door finishes its leaf at `Sym<Interval>`; `boxfinish/probe-switch-off`,
`probe/M4-REPORT.md`, the M5 section of the cost row, `probe/M7-REPORT.md`).

- **A drive finishes one body many times.** In the tour's certified
  study, 3,069 box-lane finishes cover 255 distinct bodies (by vertex
  fingerprint): 92% of the finishes repeat a body already finished in
  the same drive.
- **The repeated body is one value.** The tour's plate is finished
  1,023 times, once per replay, with the same boxes and the same
  symbolic nodes every time: the leaf's box varies only in variables
  the plate does not read. Each hole's value is finished about eight
  times.
- **Refusals repeat too.** The holes' 2,046 finishes all refuse.
- **What it costs once the check is gone** (M7): the tour's drive is
  215 s of leaf time over 1,023 replays; box finishes are 72 s of it
  (33.6%, the same share as before the check went): the plate's 1,023
  finishes 22 s (21.6 ms each), the holes' 2,046 refusing finishes 50 s
  (24.5 ms each). The build outside the finishes is the other two
  thirds, and it too is repeated per replay.
- **Per leaf:** a plate with holes runs four finishes, one per sweep
  door; a leaf with a move runs three (the door, the moved copy's
  re-finish, the union's result). `m10_8`'s
  `the_a0_set_is_a0_alone` is 1.6 s against a 1.0 s budget on that.
- **Not measured:** how much of a replay's build (outside the finish)
  is of bodies the leaf's box does not move; whether the symbolic
  nodes of an unmoved body are the same node ids across replays or
  equal expressions rebuilt; what share of the 1,023 replays refuse at
  the same place for the same reason.

**Beside it:** ONEHOME has been cutting the number of finishes a
document's build runs by other means (`corpus_finish_count` 505 → 390:
readers move no body, an aggregate finishes once, the n-ary fold and
the at-rest intersection take placed operands; its fork 288). That is
finishes per build; this row is builds and finishes per drive.
(BOXFINISH orchestrator)

## Weighed (2026-11-22, fork-log row 302): no new cache; the evaluator's memo across a drive's leaves, and a split that follows the deciding node

Two designers, four rounds; they end on one final state.

**The problem, restated.** A document node's result is already a
function of the evaluator's content key (DESIGN `[determinism]`: "the
key is the correctness proof"; `[scalars]`: the box lanes use "the same
memoization ... no parallel path"). The drive does not use the memo
(`drive::classify_leaf` passes no prior) because a symbolic value means
nothing outside the replay that made it: its expression table,
registrations, counts and caps live in a per-replay session. The
repeated finish and the repeated build are both that.

**The final state.**

1. **The unit is a node's result**: its build, its finish or its
   refusal, with what it recorded (verdict rows, escalations, marks,
   registrations, counts, deferred entries). Telemetry samples are not
   in that list. It is keyed by the node's own id, its content key and
   its naming key (the naming key embeds the inputs' ids, not the
   node's own, so the id is in the lookup as it is today).
2. **A node's result depends on its inputs alone**, which at the
   symbolic scalar takes four changes:
   - the content key hashes the expression as well as the number
     (`ContentBits for Sym`);
   - a value with no expression is named by the node that minted it
     and its ordinal there, not by a per-replay counter;
   - the expression table belongs to the drive (expression ids are
     already content hashes; the plain-form memo already argues this
     per drive);
   - there is no per-replay session state a node can read: every piece
     is either a cache whose misses never change an answer, or it
     belongs to a node and travels with its value. The registry is
     scoped to a node's inputs; the retry ladder's per-replay cap
     (`SymRetry::max_forms`) becomes per node.
3. **One store per drive**, each entry computed once (a waiter waits),
   so that no result is ever chosen between two writers and the
   schedule is unobservable. Still taken on belief, and pinned by the
   one-thread-against-many and memo-on-against-off differentials: that
   no thread-local escapes a node's scope (the `k_stats` frame, the
   shape report, the deferral numbering). A measurement under the new
   split rule, on a document with two refined features, counting time
   in nodes whose entry repeats only at a cousin, may show that handing
   the parent leaf's replay to its children is enough; nothing else in
   the design depends on which.
4. **The split follows the deciding node**: of the variables the first
   undecided node reads that have depth left, the widest. Splitting any
   other variable reproduces that node in both halves. A deciding node
   that reads no variable of the box refuses the box at once, with a
   reason of its own naming it; a read set at its depth budget or at
   the grid is the existing `Budget`. The order rule that picks which
   node speaks first (`classify_replay`) becomes load-bearing and says
   so.
5. **Receipts.** A leaf's receipt counts what its result rests on,
   served or built. The drive's `decisions` total counts each distinct
   node result once, as its `frozen` column already counts each frozen
   expression once: the sum over leaves is derivable from the shipped
   leaves and this is not, the report's symbolic share then speaks of
   the document and not of the bisection's depth, and the row has one
   meaning. Multi-leaf pinned totals re-baseline once. The memo is a
   dial like `plain_memo`; the verdict, every leaf and its receipt, the
   drive's total and so `serialize()` and `content_key` are the same
   with it on or off.
6. **A deferring session** takes no prior and shares no store until a
   node's deferred entries travel with its value (the drive never
   defers).

**Weighed and rejected:** a finish cache keyed by a body (a second
mechanism; its key is no proof; a third of the cost); building unmoved
nodes once up front (the memo's special case); the parent replay as
the only prior (two independent features that both need refining cost
the product of their boxes, not the sum, in the second feature's own
build); the split alone or the memo alone.

**Order of work the designers suggest:** the key and opaque fixes;
then the session's state per node and the table per drive; then the
store; then the split rule with its refusal. Each step is
verdict-neutral against today's receipts except the drive's totals and
the replay counts, which are meant to fall.

**Not established:** how much of the build outside the finishes
repeats, and the read-set tree's size (M8, running); the store's
memory on wide documents; whether a leaf waiting on another's entry
can stall the thread pool.

