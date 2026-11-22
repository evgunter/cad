**Recommendation (likely).** Build no new cache. A drive keeps one store of node results, filled by the evaluator's existing memo rule. A box is split only along the variables its undecided node reads.

**Terms.**
- **Replay:** one evaluation of the document over one box of the bisection.
- **Node:** a recipe node, such as a sweep or a Boolean.
- **Content key:** the evaluator's hash of a node's op, its evaluated inputs and its inputs' keys.
- **Naming key:** the same hash, with the inputs' node ids included.
- **Read set:** the variables a node reads, directly or through its inputs.
- **Symbolic scalar:** a number that carries an expression beside each interval.
- **Session:** the per-replay state that gives those expressions their meaning today.

**The design.**
1. **The unit is a node's result:** its build, its finish or its refusal, with everything it recorded. It is keyed by the node's own id, the content key and the naming key. A replay is served the result wherever the node's inputs are unchanged. The plate is built once per drive, and each hole once per distinct box of its own variables. (sure)
2. **A node's result depends on its key alone.** At the symbolic scalar it does not today, so four changes are needed. (sure each is required; likely the list is complete)
   - The content key hashes the expression as well as the number.
   - A number with no expression is named by the node that minted it, not by a per-replay counter. Today two replays can give one name to two different numbers, which becomes a false zero once values cross replays.
   - The expression table belongs to the drive.
   - Every other piece of session state is either a cache whose misses never change an answer, or belongs to a node and travels with its value. That covers the identities a node registered, its counts, its telemetry and its pending decisions. The retry ladder's per-replay cap becomes per node.
3. **One store per drive, each key computed once.** Nothing with a replay's lifetime is left for a node to read, so the order in which leaves run cannot change a result. (likely)
   - The alternative is serving only from the parent box's replay. With two independent features that both need refining, that rebuilds the second feature once per leaf of the first, where the store builds it once.
   - A measurement under the new split rule, on a document with two refined features, may show the parent-only version is enough. Nothing else in the design depends on that choice.
4. **The split follows the undecided node:** the widest variable it reads that still has depth left. Splitting a variable the node does not read reproduces the same undecided node in both halves. (sure it is sound; likely the larger saving)
   - A node that reads no variable of the box refuses the box at once, with the one new reason, which names the node.
   - A read set at its depth budget, or at the floating-point grid, stays the existing `Budget` refusal, because a setting changes it.
5. **Receipts.** (sure)
   - A leaf's receipt counts what its result rests on, served or built.
   - The drive's total counts each distinct node result once, as its `frozen` column already counts each frozen expression once.
   - Both are unchanged by the memo, which becomes an on/off switch like the drive's existing form memo. The tests compare runs with it on and off.
   - Pinned drive totals over several leaves re-baseline once.

**Premise check.** DESIGN already holds the answer: `[determinism]` ("the key is the correctness proof") and `[scalars]` ("the same memoization … no parallel path"). The drive does not use the memo because a symbolic value means nothing outside its replay. That is the flaw; the repeated finish and the repeated build (two thirds of leaf time) are its symptoms. Two corrections to the measurements: (sure)
- **The measured tour certifies nothing.** Every replay refuses at the holes, over 254 distinct hole bodies of one kind (a missing lane in the sweep doors), so its 22 s / 50 s split is of an all-refusing drive.
- **The slow `m10_8` row has one leaf per drive,** so this design does not help it.

**Why it is sound.** A node's result holds over the box of the variables it reads, and that box is identical wherever the result is served. Nothing is taken from the `f64` witness.

**Ratified text.** DESIGN: not changed. E6's K paragraph: not changed. ERROR-DESIGN E6:
- **Split rule**, replacing "Split rule named, deterministic (D9): max relative width, ties to lowest index":
  > Split rule named, deterministic (D9): of the variables the leaf's deciding node reads (the first undecided node in evaluation order) that have depth left, the one of greatest relative width, ties to lowest index. A leaf whose deciding node reads no variable of the box is refused, naming the node: no split changes it.
- **Leaf protocol**, a new bullet:
  > A node whose inputs a leaf leaves unchanged is served from an earlier replay of the drive: result or refusal, and record, whole (DESIGN [determinism]). A leaf's receipt counts what its result rests on, served or built.

**Rejected.**
- A finish cache keyed by body: a second mechanism, it covers a third of the cost, and its key is no proof.
- Building unmoved nodes once, up front: a special case of the memo.
- The split rule alone, or the memo alone: each leaves the other's waste.

**Not established.**
- How much of the build outside the finishes repeats.
- The store's memory use on wide documents.
- Whether a leaf waiting on another's computation can stall the thread pool.
