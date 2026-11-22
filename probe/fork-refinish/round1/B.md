Reconciliation round 1 from B: I move to A's drive store, on a structural argument; totals reconcile if defined over distinct keys; per-replay state is a defect under either store. Your reading of the agreement is right.

**(i) What moved me, what did not.**
- Moved: the store. My claim "the support-aware split removes most cousin repeats" is wrong in a case that is structural, not rare. Two independent features F1 (reads S1) and F2 (reads S2), both needing refinement: the split refines S1 until F1 decides, then S2 inside every S1 leaf. F2's value at an S2 box Y is then built once per S1 leaf. Parent-only costs (S1 leaves) x (S2 boxes); a store costs (S1 leaves) + (S2 boxes). Independent toleranced features are the ordinary study.
- Moved: the key. I wrote that value-only keys "cannot bite inside one drive". True for a lookup by node id (today's `prior`). False for a store looked up by content key, which excludes node ids: A's two equal depth variables are two nodes with one key and two expressions. With the store, the expression in the key is required, not latent.
- Moved: A's registry point is stronger than mine: a registration by an unrelated earlier node can change another node's answer. That is per-replay state leaking into a node, the same class as the caps (iv).
- Accepted: m10_8's a0 row is one leaf per drive (A, from M4); I had it "not established".
- Not moved: "the drive's totals become the decisions it actually took" as a definition (iii). And A's trigger "every axis in the support at its depth budget" is the existing `Budget(Depth)` (a dial changes it); the new reason is for an empty read set only (no dial changes it).
- My argument is answered on cost. On determinism it is answered only if (ii) holds.

**(ii) Where the memo lives, as a final state: one store per drive.**
- What makes a leaf independent of who ran first: key -> result is a function, so a first writer and a second writer hold the same value. That is true exactly when a node's evaluation can read nothing with a replay's lifetime.
- Held by construction, not belief, if the final state has **no per-replay session at all**: a node evaluation gets the drive's content-addressed store (append-only, id -> expression, key -> result) and a scratch scope of its own (registrations visible = those riding its inputs' values; caps; counts; opaque ordinals). There is then no replay-lifetime object a node could read. The evaluator already opens a per-node bracket for the verdict log; the symbolic scope gets the same lifetime.
- Each key is computed once: a once-cell per key, later arrivals wait. Leaf evaluation is sequential inside, so a waiter cannot be what the computer needs. "First writer wins" then never chooses between two values.
- What a reviewer still has to believe: that no thread-local or global outside the scope leaks. The existing differential dials (sequential against parallel, store on against off) pin it, as they do `plain_memo`.
- Parent-only keeps one virtue: a leak would give a deterministic wrong-versus-unmemoized answer, not a schedule-dependent one. That is a worse failure to find, not a better one.
- Lost with parent-only if M8 shows cousins dominate: the cross product above. "Add the map later" then costs no new soundness work (the prerequisites are the same) but re-defines the drive's totals and re-argues determinism, so it is not a later addition; it is the design.
- Memory: the store holds every distinct result for the drive. Eviction is always sound (recompute), given tree-defined totals (iii).

**(iii) Totals: both statements hold, with one definition.**
- A leaf's receipt: the sum over its nodes' results, served or built. Same with the store on or off. (Both reports.)
- The drive's total: the sum over the drive's **distinct (node result) keys**, each once. Defined by the tree, not by what ran, so it is also the same with the store on or off and in any schedule. With the store on it equals what was computed, which is A's reading. This is `frozen`'s rule ("a set on both receipts... not summed") extended to every column.
- A separate number, sum of leaf receipts minus the drive's total, is the work the store saved.
- Readers: pinned rows read leaf receipts (invariant) and the drive total (invariant, smaller than today's sum; re-baselined once). The report lines say "decisions" of the drive: distinct is what they should mean. `k_stats`: a sample is taken where a decision is computed, so with the store each distinct decision lands once. That is the better population for K (a decision repeated in 1,023 leaves is one sample), but the funnel then differs with the store off. Say so in E6's K paragraph or dedupe by key there too.

**(iv) Per-replay state is a defect under either store.** Parent-only: the store-off replay can differ. Shared: the schedule can show. From `sym::Session`'s fields, replay-lifetime state a node's result can read:
- `registry` (and `forms_door`, `retries`' door memos, functions of it);
- `retries` with `SymRetry::max_forms`, and `reductions_held` (a held-count: check whether it caps);
- `OPAQUE_SEQ`;
- `deferred` (a deferring session's pending decisions: who discharges them if the node is served?);
- `budget_frozen`, `plain_tainted`, `foreign`, `plain_unpublished` (the taint machinery; gone with a drive-scoped table);
- `params` and `witness` are per-drive facts of the variables and scalar, safe.
`forms`, `forms_early`, `trig_closed`, `reductions` are caches of functions of ids: safe at any scope if not capped.

**(v) ONEHOME's row is a different axis, not a conflict.** It carries facts between different nodes inside one build (a Boolean's result re-deriving facts on cells its operand already certified). The node memo carries one node's whole result between replays. `[topology.tiers]` already has the first ("a finish carries the fact of a pair it finds unchanged in a finished operand"). One caution for that row: its title makes "bit-identical data" the criterion. The carry should be by lineage (the door carried the cell unchanged), since `[verdicts]` says bit-equality is never the test and at `Sym` equal bits are not equal expressions, the same defect as the key.

**(vi) Recommendation as it stands.** As my first report, with three changes: the memo is one store per drive, keyed by content key (expression included) and naming key, compute-once per key; the symbolic session's lifetime is the node evaluation, not the replay; the drive's total is over distinct keys. `For Ev` whole:

## For Ev

**Recommendation** (`likely`). Build no new cache. A drive keeps one store of node results, filled by the evaluator's existing memo rule, and a box is split only along variables the undecided node reads.

Terms. A *replay* is one evaluation of the document over one box of the bisection. A *node* is a recipe node; its *content key* hashes its op, its evaluated inputs and its inputs' keys. Its *read set* is the variables it reads, directly or through its inputs. The *symbolic scalar* carries an expression beside each interval; a *session* is the state that gives those expressions meaning.

1. **The unit is a node's result** (its build, its finish, or its refusal, with the decisions it took), identified by its content key. A replay takes it from the drive's store where any earlier replay computed it. The plate is built once per drive; a hole once per distinct box of its own variables. (`sure` of the unit.)
2. **A node's result depends on its key alone.** Today it does not, at the symbolic scalar. The fixes: the key hashes the expression as well as the number; a number with no expression is named by its minting node and ordinal, not by a per-replay counter; the expression table belongs to the drive; and there is no per-replay session. A node evaluation sees the identities its inputs registered and nothing else, and its caps and counts are its own. (`sure` each is required; `likely` the list is complete.)
3. **The store cannot make a result depend on the schedule**, because after point 2 nothing with a replay's lifetime exists for a node to read, and each key is computed once. (`likely`.)
4. **Bisection splits the widest axis in the first undecided node's read set.** Any other split reproduces the same undecided node in both halves. A node that reads no variable of the box refuses the box at once, with a reason of its own that names it. (`sure` it is sound; `likely` it is the larger saving.)
5. **Receipts.** A leaf's receipt counts what its certificate rests on, served or built, and is the same with the store on or off. The drive's total counts each distinct node result once. (`sure`.)

**Premise check.** The item asks what a finish may be keyed by. The ratified answer is one level up: `[determinism]` ("the key is the correctness proof") and `[scalars]` ("the same memoization ... no parallel path"). The drive does not use the memo because a symbolic value means nothing outside the replay that made it. That is the flaw; the repeated finish and the repeated build are both symptoms. The measured tour certifies nothing (every replay refuses at the holes, over 254 distinct hole bodies), so its 22 s / 50 s split is of an all-refusing drive.

**Why it is sound.** A node's result is over the box of the variables it reads, identical in every replay that shares its key. Nothing is taken from the `f64` witness (`[scalars]`).

**Ratified text.** DESIGN: none. E6, split rule: "Any `Indeterminate` → bisect and recurse. The split axis has the max relative width among the variables the leaf's first undecided node reads, ties to lowest index. A leaf whose first undecided node reads no variable of the box refuses, naming the node: no split changes it." E6, leaf protocol, one sentence: "A node's result is a function of its content key; a leaf takes it from the drive's store where an earlier replay computed it (DESIGN [determinism])." E6's "leaves are independent" becomes "a leaf's verdict is a function of its box alone".

**Rejected.** A finish cache keyed by body (a second mechanism, a third of the cost, a key that is no proof). The parent replay as the only prior (two independent features that both need refining cost their product, not their sum). Hoisting unmoved nodes (the key's special case).

**Not established.** How much of the build outside the finishes repeats; the store's memory on wide documents.

**What M8 confirms or overturns.**
- Distinct keys per node, and the non-finish share whose key repeats: sizes point 1. A small share leaves the finish third as the whole gain.
- Split axis against read set: point 4's payoff. Overturned as "the larger saving" if nearly every split is already in the read set.
- Parent against cousin repeats: taken under today's split rule, so it bounds but does not settle store against parent-only; the two-feature product is the argument. If cousins are near zero on a multi-feature document, parent-only would do.
- Equal keys with different expressions: any hit confirms the key fix is required for the store and overturns my first report's "cannot bite inside one drive".
