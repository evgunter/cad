Reconciliation round 2 from B: I concede the lookup key, the drive's total and `k_stats` to A; I hold the store, but as a close call with a stated bound; the new refusal is for an empty read set only. Your list of agreements is right.

**1. Store or parent.**
- The two-feature product is real under parent-only with the read-set split: while F1 is undecided every split is in S1 and F2 is served down the tree; at each S1 leaf the splits turn to S2, and F2 at each new S2 box is built once per S1 leaf. No split order avoids it. It is ordinary: any study with two features that each need a split.
- Its bound, which I did not state: the replay count is the product either way (the leaves are a product partition), and every node that reads both features (the Boolean that joins them, the product) is built per replay under either home. The store saves F2's own build and finish per repeat, nothing downstream. So the store's gain is F2's share of a replay, not a change of order in the drive's cost.
- A's determinism point against "no per-replay session, once-cell per entry, a node sees the drive's append-only store and its own scratch": the schedule is then unobservable provided a node's evaluation reads nothing but its arguments. Left to believe: no thread-local or process-global leaks in (the deferred list's process counter, the `k_stats` bracket, the evaluation epoch), and the 128-bit collision assumption. The sequential/parallel and on/off differentials check it. That is the same standing the plain-form memo has today.
- Both are defensible. I lean store (`unsure`): the session work is identical, it is the general form and parent-only its special case. A's threshold is the right test, with one condition: it must be taken under the new split rule on a document with two refined features, as time in nodes whose entry repeats only at a cousin. M8 under today's rule cannot give that number.
- The key: A is right. (node id, content key, naming key), today's rule exactly. A value's names embed its minting node's id, and I cannot show the naming key includes the node's own id, so two input-less nodes with equal content could be served each other's names. Key-alone buys only sharing between distinct nodes with identical inputs, which is not the repeat at issue. The expression still goes in the key (the statement "same key, same output" should be true), but with the node id in the lookup it is latent inside a drive, as A says.

**2. The drive's total: A's.** The total stays the sum of the leaf receipts: a consumer can re-check it from the shipped leaves, and today's pinned drive totals do not move (`frozen` stays the distinct set it already is). Beside it, one new count: decisions over the drive's distinct entries, each once. My one condition: that count is defined by the tree (distinct lookup keys), not by what happened to run, so it too is the same with the memo on or off; with it on, it is what was computed.

**3. `k_stats`: A's.** One rule: everything a node records (verdict rows, escalations, marks, samples, registrations, counts) is part of its value and appears wherever the value is served. The funnel is then unchanged by the memo, and E6's "every driver-path predicate sample lands in the funnel" stands as written, per leaf. Whether K's statistics should weight a decision once or once per leaf is a reporting question, answerable at the reader by key, and no E6 text changes.

**4. Which refusal is new: the empty read set only.** The split takes the widest axis in the deciding node's read set that still has depth. If the read set has axes but none has depth left (or the grid floor), that is the existing `Budget(Depth)` or `Budget(Resolution)`: a dial changes it, and it prices under Budget. If the node reads no variable of the box, no dial changes it: the new reason, naming the node.

**5. A deferring session** (`sym::deferred`; used by the coincidence lint's replay, never by a drive). A deferred Zero's margin names an entry on the session's list by a process-unique number; the entry is the margin's expression id. A margin carried into another session "reads as no Zero in any other one", so a served node's deferred Zeros would discharge as unproven: sound, but the memo would show. By the agreed rule the entries a node pushed belong to it and travel with its value (they are content: expression ids). Until that is built, a deferring session takes no prior, as today. An eager and a deferring replay decide differently ("deferring changes which sign an escalation reads"), so their results never share a store.

**6. For Ev, whole and final.**

## For Ev

**Recommendation** (`likely`). Build no new cache. A drive reuses a node's result through the evaluator's existing memo, and a box is split only along variables the undecided node reads.

Terms. A *replay* is one evaluation of the document over one box of the bisection. A *node* is a recipe node (one sweep, one Boolean); its *content key* hashes its op, its evaluated inputs and its inputs' keys. Its *read set* is the variables it reads, directly or through its inputs. The *symbolic scalar* carries an expression beside each interval; a *session* is the per-replay state that gives those expressions meaning.

**What the problem is.** DESIGN already says reuse by content key is correct (`[determinism]`, "the key is the correctness proof") and that the box lanes use "the same memoization ... no parallel path" (`[scalars]`). The drive does not, because a symbolic value means nothing outside the replay that made it. The repeated finish and the repeated build (two thirds of leaf time) are both that.

1. **The unit is a node's result**: its build, its finish or its refusal, with everything it recorded. A replay is served it wherever the node's inputs are unchanged. The plate is built once per drive. (`sure`)
2. **A node's result depends on its inputs alone.** At the symbolic scalar it does not today. The expression table moves to the drive. A number with no expression is named by its minting node and ordinal, not a per-replay counter (today two replays can give one name to two numbers: a false zero once values cross replays). Every other piece of session state is either a cache whose misses never change an answer, or belongs to a node and travels with its value: registered identities, counts, telemetry; the retry ladder's per-replay cap becomes per node. The key hashes the expression as well as the number. (`sure` each is required; `likely` the list is complete)
3. **The split follows the undecided node.** Splitting a variable it does not read reproduces it in both halves. A node that reads no variable of the box refuses the box at once, with a new reason naming it. (`sure` sound; `likely` the larger saving)
4. **Receipts.** A leaf's receipt and the telemetry count what its result rests on, served or built, and are the same with the memo on or off. The drive's total stays the sum of its leaves; a new count says what was computed. (`sure`)

**One choice left, close.** Where a replay looks: only its parent box's replay, or one store per drive. Parent-only has no shared state. With two independent features that both need refining it rebuilds the second once per leaf of the first; a store builds it once. The gain is that feature's own build, since everything reading both is built per replay either way. I lean store (`unsure`); a measurement under the new split rule decides, and nothing else in the design changes with it.

**Sound because** a node's result is over the box of the variables it reads, identical wherever it is served; nothing comes from the `f64` witness (`[scalars]`).

**Ratified text.** DESIGN: none. ERROR-DESIGN E6, replacing "Split rule named, deterministic (D9): max relative width, ties to lowest index":
> Split rule named, deterministic (D9): of the variables the leaf's deciding node reads (the first undecided node in evaluation order) that have depth left, the one of greatest relative width, ties to lowest index. A leaf whose deciding node reads no variable of the box is refused, naming the node: no split changes it.

E6 leaf protocol, a new bullet:
> A node whose inputs a leaf leaves unchanged is served from an earlier replay of the drive: result or refusal, and record, whole (DESIGN [determinism]). A leaf's receipt counts what its result rests on, served or built.

**Corrections to the measurements.** The measured tour certifies nothing: every replay refuses at the holes, over 254 distinct hole bodies of one kind (a missing lane in the sweep doors). The slow `m10_8` row has one leaf per drive, so this does not help it.

**Rejected.** A finish cache keyed by body (a second mechanism, a third of the cost, a key that is no proof). Building unmoved nodes once up front (the memo's special case). The split alone or the memo alone (each leaves the other's waste).
