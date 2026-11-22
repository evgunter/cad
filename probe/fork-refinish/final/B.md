**Recommendation** (`likely`). Build no new cache. A drive reuses a node's result through the evaluator's existing memo, held in one store per drive, and a box is split only along variables its undecided node reads.

Terms. A *replay* is one evaluation of the document over one box of the bisection. A *node* is a recipe node (one sweep, one Boolean). Its *content key* hashes its op, its evaluated inputs and its inputs' keys. Its *read set* is the variables it reads, directly or through its inputs. The *symbolic scalar* carries an expression beside each interval. A *session* is the per-replay state that today gives those expressions their meaning.

**Premise check.** The item asks what a finish may be keyed by. The answer is ratified one level up: reuse by content key is correct (`[determinism]`, "the key is the correctness proof") and the box lanes use "the same memoization ... no parallel path" (`[scalars]`). The drive does not, because a symbolic value means nothing outside the replay that made it. The repeated finish and the repeated build (two thirds of leaf time) are both that one flaw.

**The design.**
1. **The unit is a node's result**: its build, its finish or its refusal, with what it recorded. It is keyed by the node's own id, its content key and its naming key (today's memo rule). (`sure`)
2. **A node's result depends on its inputs alone.** At the symbolic scalar it does not today. Four changes (`sure` each is required; `likely` the list is complete):
   - the key hashes the expression as well as the number;
   - a number with no expression is named by its minting node and ordinal, not a per-replay counter (today two replays can give one name to two numbers: a false zero once values cross replays);
   - the expression table belongs to the drive;
   - there is no per-replay session. Session state is either a cache whose misses never change an answer, or it belongs to a node and travels with its value: registered identities, counts, deferred decisions. The retry ladder's per-replay cap becomes per node.
3. **One store per drive.** A replay takes a result from it wherever any earlier replay computed it; each entry is computed once. After point 2 nothing with a replay's lifetime is left for a node to read, so the order the leaves ran in cannot show (`likely`). The tour's plate is built once per drive. A measurement under the new split rule, on a document with two features that both need refining, could show that looking only in the parent box's replay suffices; nothing else in the design changes with it.
4. **The split follows the undecided node**: the widest variable it reads. Splitting any other reproduces the same undecided node in both halves. A node that reads no variable of the box refuses the box at once, with one new reason that names it. A read set that has run out of depth is the existing budget refusal. (`sure` it is sound; `likely` the larger saving)
5. **Receipts.** A leaf's receipt counts what its result rests on, served or built. The drive's total counts each distinct node result once: the sum over leaves can be had from the leaves, and this cannot. Verdicts, leaf receipts and the drive's total are the same with the store on or off. Pinned totals of multi-leaf drives are re-baselined once. (`sure`)

**Two corrections to the measurements.** The measured tour certifies nothing: every replay refuses at the holes, over 254 distinct hole bodies of one kind (a missing lane in the sweep doors), so its 22 s / 50 s split is of an all-refusing drive. The slow `m10_8` row has one leaf per drive, so this does not help it.

**Why it is sound.** A node's result is over the box of the variables it reads, which is identical wherever it is served. Its symbolic zeros hold wherever the build takes the decisions that replay took (E12), and a served node took the same ones. Nothing comes from the `f64` witness (`[scalars]`). (`sure`)

**Ratified text.** DESIGN is not changed. ERROR-DESIGN E6's K paragraph is not changed: the K telemetry is taken by a separate replay of certified leaves, which this does not touch. E6 changes in two places. Replacing "Split rule named, deterministic (D9): max relative width, ties to lowest index":
> Split rule named, deterministic (D9): of the variables the leaf's deciding node reads (the first undecided node in evaluation order) that have depth left, the one of greatest relative width, ties to lowest index. A leaf whose deciding node reads no variable of the box is refused, naming the node: no split changes it.

A new leaf-protocol bullet:
> A node whose inputs a leaf leaves unchanged is served from an earlier replay of the drive: result or refusal, and record, whole (DESIGN [determinism]). A leaf's receipt counts what its result rests on, served or built.

Who wrote the split rule could not be traced: the checkout's history is shallow.

**Rejected.**
- A finish cache keyed by body: a second mechanism, a third of the cost, and a key that is no proof.
- Building unmoved nodes once, up front: the memo's special case, with a dependency analysis added.
- The parent replay as the only prior: with two independent features that both need refining, the second is rebuilt once per leaf of the first.
- The split alone, or the memo alone: each leaves the other's waste.

**Not established.** How much of the build outside the finishes repeats. How often today's splits fall outside the deciding node's read set. The store's memory on wide documents. Whether the list in point 2 is complete: no thread-local or global may leak into a node's evaluation, and the store-on/off and one-thread/many comparisons are what check it.
