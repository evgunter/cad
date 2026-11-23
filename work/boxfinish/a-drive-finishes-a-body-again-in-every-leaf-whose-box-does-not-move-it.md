---
id: a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it
kind: issue
title: A drive builds and finishes a body again in every leaf, though the leaf's box moves none of its inputs: a third of the tour's leaf time
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression, a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state, a-drive-keeps-no-node-result-from-one-leaf-to-the-next]
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

## Corrected by the designers' first reports (2026-11-22)

Both designers, independently, corrected this row before designing.

- **The measured tour certifies nothing.** Every one of its 1,023
  replays refuses at the holes (M4, M7: "still 0 certified"). The
  22 s / 50 s split above is of an all-refusing drive, not of the tour
  once its holes finish. The structure (the plate rebuilt and
  re-finished per replay) stays.
- **The holes' refusals are not one refusal repeated.** There are 254
  distinct hole bodies, so the box does move them. What repeats is the
  KIND (an undecided length on a direction derived from an end
  vertex), which is the sweep doors' row and not the driver's. A hole
  finish that succeeds costs about three times a refusing one (71 ms
  against 24.5 ms, M7), so reuse matters more once they finish.
- **The a0 row gains nothing from this row:** it runs one leaf per
  drive and repeats no body. Its levers are finishes per build
  (ONEHOME's) and the cost of each finish.
- **"Same nodes or rebuilt" has an answer in the code:** expression
  ids are content hashes, so an unmoved body has the same ids in every
  replay with its table entries rebuilt per session; an opaque value's
  id is the exception (a per-replay counter).
- **The question is one level up from "what may a finish be keyed
  by":** a document node's result is already a function of the
  evaluator's content key (DESIGN `[determinism]`, `[scalars]` "no
  parallel path"), and the drive passes no prior.

**Latent, found on the way, and part of whatever is built here:**
`ContentBits for Sym` (`crates/editor-core/src/eval/memo.rs`) feeds the
value channel only, so at the symbolic scalar the content key is not
the correctness proof DESIGN says it is: two inputs with equal numbers
and different expressions share a key. It cannot bite today (checked:
every evaluation at a `Sym` scalar, shipped and test, passes `None` as
its prior; `LeafPrior::of` and `evaluate_nested` hard-code it), and
that call-site rule is the only thing holding it. Beside it: an opaque
value's id is a per-replay sequence, so two replays' "opaque #5" are
different reals under one id.

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

**Measured after the designers agreed (M8,
`probe/M8-REPORT.md` on `boxfinish/probe-switch-off` at `d89e28745d`;
the switch's box arm off, the check after a definite sign removed):**

- **The split rule is the larger saving.** On the tour's study the
  deciding node is the first hole's extrude in every bisected replay,
  and 292 of the 511 splits (57%) are on the one variable it does not
  read. Collapsing the traced tree to the splits it does read leaves
  127 replays of 1,023. On a two-variable drive, 3 of 15 splits and 23
  replays of 31; on a one-variable drive, nothing.
- **Most node time is on a result the drive already has.** Tour, time
  inside node ops: 71.1 of 81.4 s of
  build outside finishes (87%) and 48.0 of 52.6 s of finishes (91%) are
  on a content key already seen. The plate has one key in 1,023
  replays; each hole 127.
- **A served refusal is most of it here.** The holes refuse every time,
  so a memo that keeps only a built value saves 9.8 s of build and
  15.5 s of finish; keeping a refusal under its key adds 61.3 s and
  32.5 s. (Whether two refusals under one key carry the same payload
  was not compared.)
- **The parent's replay against the whole drive, under today's split
  rule:** a prior handed from parent to child catches 4,818 of 6,650
  repeats and misses 1,832, all of them the holes'. Under the new split
  rule this drive's tree is the first hole's alone, so it does not
  separate the two; a document with two features that both refine is
  still the measurement that would.
- **No live instance of the key's gap at the symbolic scalar:** no two
  replays hold a node with equal content keys and different symbolic
  ids in its result (5,115 results over 257 keys on the tour), and no
  result holds a value with no expression. The fix is owed by the
  argument, not by an observed wrong answer.

**One key, one refusal (M9, `probe/M9-REPORT.md` at `5dc671ed99`).**
On the tour each hole's extrude fails under 127 keys in 1,023 runs, and
every repeat of a key carries a byte-identical refusal (the error value
with its enclosures and bands, the escalation list) and the same
symbolic count delta over the node. The same holds of every node that
builds: one record and one count delta per key. Nothing per-replay
shows in what the drive receives from a node under one key. No retry
ran in these drives, so a refusal that depends on a retry cap reached
earlier in the replay is still untested.

**The time outside node ops is the session's teardown** (M9, one run):
of 177.6 s of leaf time, node ops are 153.7 s (86.5%) and 23.0 s is the
symbolic session's teardown per replay (publishing the leaf's forms to
the drive's memo, the leaf's `frozen` column, dropping its tables);
everything else is under 1 s. M8's "134 of 215 s" subtracted two
different runs and is withdrawn. The teardown is paid per replay, so
the split rule divides it with the replay count.

**Not established:** the certifying tour, since this one refuses at
the holes in every replay; a drive in which a retry runs; the store's
memory on wide documents; whether a leaf waiting on another's entry
can stall the thread pool.

**Ev's answer (PR 7368, 2026-11-22): "this sounds great!"** Built as
four units, this row parked on them: the split rule
(`a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read`),
the key (`a-nodes-content-key-at-the-symbolic-scalar-hashes-the-number-and-not-the-expression`),
the session's state (`a-nodes-result-at-the-symbolic-scalar-reads-per-replay-session-state`)
and the store (`a-drive-keeps-no-node-result-from-one-leaf-to-the-next`).
