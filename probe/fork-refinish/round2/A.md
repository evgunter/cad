RECONCILIATION ROUND 2, A. Your reading of the agreements is right. On every open point I now land with B except one: the key also includes the node's own id.

**1. Store or parent: the store.**
- **B's product argument is right, and it survives the support-aware split.** Take a leaf whose first undecided node is F1. It splits along S1, the variables F1 reads, until F1 decides. Then F2 speaks, and its splits along S2 happen inside every S1 leaf. Those S1 leaves are cousins of each other, not parents. So under parent-only, F2 at an S2 box Y is built once per S1 leaf.
- **It is the ordinary case:** the tour itself has two holes with independent radii. I withdraw the threshold and parent-only.
- **Determinism under B's final state:** with no per-replay session, a node evaluation can read only three things: its inputs; the drive's append-only content-addressed tables, whose entries are functions of their ids (the plain form already is: a function of the id, the budget and the dials); and a scratch scope of its own. Add one once-cell per key, so no result is ever chosen between two writers. The schedule is then unobservable by construction, for everything that goes through those doors.
- **What is still taken on belief:** that no thread-local escapes the scope. The ones I know of: the `k_stats` frame, the shape report, `OPAQUE_SEQ` until it is keyed by the minting node, and the deferral numbering (see 5). The differential rows (one thread against many; store on against off) pin this, as they pin `plain_memo` today.
- **One implementation hazard:** a leaf that waits on a once-cell must not block a rayon worker the computing leaf needs. A wait cannot be circular, because a node's computation depends only on its upstream nodes, but starving the pool can still stall the drive. Unchecked.
- **The key: node id + content key (expression included) + naming key.** The naming key hashes the inputs' ids and naming keys, but not the node's own id ("names embed minting node ids", `eval/memo.rs`). Today's lookup supplies the own id by looking up `prior.nodes.get(&id)` first. A store keyed by content key and naming key alone would serve one node's value under another node's id, so the served value's names would carry the wrong minting node. Two nodes in one document with identical content, inputs and names are rare enough that serving across ids is worth nothing. With the own id in the key, putting the expression in the content key is belt and braces for the store, but it is still required so that "same key, same output" is true as a statement.

**2. The drive's total: B's.**
- **Leaf receipts:** the sum over a leaf's nodes' results, served or built, unchanged by the memo.
- **The drive's decision totals:** each distinct node result counted once. That is defined by the tree, so it does not change with the store or the schedule. It is `frozen`'s rule applied to every column.
- **Today's pinned drive totals** are re-baselined once, in the PR that lands the store. They fall, and the size of the fall is the measured saving. Pinned leaf receipts do not move.
- **No "saved" field:** it is the sum of the leaf receipts minus the drive's total, so it can be derived, and a stored copy would be a column kept for a check (`[simple.checks]`).

**3. `k_stats`: B's population, with the text to say it.** I withdraw "splice a served node's samples at every serve": that is 1,023 copies of one sample. K should see each driver-path decision once per distinct node result, which is the same population as point 2's total. "Store off" should not be a shipped mode. The differential is a test instrument that compares verdicts and receipts, not the funnel, so the two populations never have to agree.
E6's K paragraph, first sentence becomes:
> every driver-path decision lands in the k_stats funnel once per distinct node result the drive computes

**4. Which refusal is new: B's.**
- **Every axis the deciding node reads at its depth budget:** the existing `Budget(Depth)`, because a dial changes it.
- **At the f64 grid:** the existing `Budget(Resolution)`.
- **The new reason** is for one case only: the deciding node reads no variable that varies in the box. No dial changes that.

**5. A deferring session (read in `sym/deferred.rs` and its callers).**
- **The drive never defers.** Its leaves run `with_session_memo_retry`, an eager session.
- **Who defers:** `with_deferred_session` is used by the coincidence replay (`coincide/replay.rs`, the `unproven-coincidence` lint) and by tests. That is one evaluation, with no memo across replays.
- **How deferral works:** a pending Zero is a margin carrying an entry number that is unique in the process (`NEXT`). A margin carried out of its session "reads as no Zero in any other one": sound, but it would report the coincidence as unproven.
- **So in the final state** a node's pending entries are part of its result. The margins that name them are in its value, and its scope holds the entries, so they travel together. Discharge is then a form walk over content-addressed nodes in the drive's table, wherever the reader asks.
- **Until that is built, the bound is:** a node evaluated in a deferring session is never served and never stored. That is free today, since no deferring evaluation has a prior.

**6. Recommendation:** B's round-1 For Ev, with my amendments folded in. It is given whole below.

## For Ev

**Recommendation (likely).** Build no new cache. A drive keeps one store of node results, filled by the evaluator's existing memo rule. A box is split only along variables that its undecided node reads.

Terms:
- **Replay:** one evaluation of the document over one box of the bisection.
- **Node:** a recipe node, such as a sweep or a Boolean.
- **Content key:** the evaluator's hash of a node's op, its evaluated inputs and its inputs' keys.
- **Read set:** the variables a node reads, directly or through its inputs.
- **Symbolic scalar:** carries an expression beside each interval.
- **Session:** the state that today gives those expressions their meaning, and lives as long as one replay.

**The design.**
1. **The unit is a node's result:** its build, its finish, or its refusal, with the decisions it took. It is keyed by the node's id, its content key and its naming key. A replay takes it from the drive's store if any earlier replay computed it, and each key is computed once. The tour's plate is built once per drive, and each hole once per distinct box of its own variables. (sure of the unit)
2. **A node's result depends on its key alone.** Today it does not, at the symbolic scalar, so four changes are needed (sure each is required; likely the list is complete):
   - The key hashes the expression as well as the number.
   - A number with no expression is named by the node that minted it, not by a per-replay counter.
   - The expression table belongs to the drive.
   - There is no per-replay session. A node sees only the identities its inputs registered, and its caps, counts and pending decisions are its own.
3. **So the store cannot make a result depend on the order the leaves ran in.** Nothing with a replay's lifetime is left for a node to read. (likely)
4. **The split follows the undecided node:** the widest variable it reads. Splitting any other variable reproduces the same undecided node in both halves. A node that reads no variable of the box refuses the box at once. (sure it is sound; likely the larger saving)
5. **Receipts:**
   - A leaf's receipt counts what its result rests on, served or built, and is the same with the store on or off.
   - The drive's totals, and the K telemetry, count each distinct node result once.
   - Today's pinned drive totals are re-baselined once. Leaf receipts do not move. (sure)

**Premise check.** DESIGN already holds the answer: `[determinism]` ("the key is the correctness proof") and `[scalars]` ("the same memoization … no parallel path"). The drive does not use the memo because a symbolic value means nothing outside its replay. That is the flaw; the repeated finish and the repeated build (two thirds of leaf time) are its symptoms. The measured tour certifies nothing: every replay refuses at the holes, over 254 distinct hole bodies. So its 22 s / 50 s split is of an all-refusing drive.

**Why it is sound.** A node's result holds over the box of the variables it reads, and that box is identical in every replay that shares the node's key. Nothing is taken from the `f64` witness.

**Ratified text.** DESIGN: none. ERROR-DESIGN E6:
- Split rule:
  > Any `Indeterminate` → bisect and recurse along the variable of greatest relative width among those the leaf's first undecided node reads, ties to lowest index. A leaf whose first undecided node reads no variable of the box refuses, naming it: no split changes it.
- Leaf protocol, one sentence:
  > A node's result is a function of its key; a leaf takes it from the drive's store where an earlier replay computed it (DESIGN [determinism]).
- "Leaves are independent" becomes:
  > a leaf's verdict is a function of its box alone.
- K paragraph:
  > every driver-path decision lands in the k_stats funnel once per distinct node result.

**Rejected.**
- A finish cache keyed by body: a second mechanism with a key that is no proof, and it covers a third of the cost.
- The parent replay as the only prior: two independent features that both need refining cost the product of their boxes, not the sum.
- Hoisting unmoved nodes: a special case of the key.

**Not established.**
- How much of the build outside the finishes repeats (the M8 measurement now running).
- The store's memory on wide documents.
- Whether a leaf waiting on another's computation can stall the thread pool.
