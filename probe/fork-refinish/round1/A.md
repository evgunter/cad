Your list of agreements is right, with one correction to my own first report.

**(i) What moved me in B, and what did not**
- **Moved: the key that hashes the number only.** The evaluator looks up a prior by node id first (`prior.nodes.get(&id)`) and only then compares keys. My "two depths, both 10 ± 0.1" example therefore needs two different nodes, so it cannot serve a wrong value inside one drive. B is right that this is latent: the fix makes the statement true, and it is not a soundness blocker. A drive store must keep that lookup, (node id, content key) and the naming key, never the key alone.
- **Moved: receipts unchanged by the memo** (see iii). I withdraw "the drive's totals become the decisions it took".
- **Moved: per-node caps** (see iv). I missed them.
- **Moved: the order rule becomes load-bearing.** Once the split follows the deciding node, the order that picks which node speaks first decides the split axis, so its comment must say so.
- **Not moved:** nothing in B contradicts mine. I still read the support-aware split as the larger saving and the memo as necessary for the plate. Both reports have that.

**(ii) Where the memo lives, compared as final states**
- **Both rest on one claim:** a served value equals the value that would have been rebuilt. That holds only if a node's result is a function of (node id, key), which is (iv). Neither store gets that claim by construction.
- **Store, first writer wins:** if the claim holds, which leaf wrote first changes nothing. If it fails, the store fails *depending on the schedule*. Parent-only fails *deterministically*, and the same way at 1 and at 64 threads. That is a real D9 difference in how a defect shows, not in what is sound.
- **What the code can hold by construction:** a debug assertion where two workers build the same entry (they must be equal), and the differential in (iii), memo on against memo off. Both are checks a reviewer can run, so neither option has to be taken on belief.
- **Parent-only, if M8 shows cousins dominate after the support-aware split:** each hole is rebuilt once per subtree in which its projection recurs, so the "about 8x" stays in part.
- **Adding the store later is cheap:** the lookup goes from "one earlier evaluation" to "a map from (node id, key) to result", and the hit path, records and receipts do not change. The session work in (iv) is identical for both, so nothing is thrown away.
- **I now lean parent-only as the final state** (no shared mutable state; memory of one evaluation per pending box), unless M8 says otherwise. Proposed threshold: take the store if, after the support-aware split, repeats that match only a cousin are more than about a fifth of all repeats.

**(iii) What the drive's totals report**
- **Who reads them:** the pinned rows and the report lines need "decisions this certificate rests on". `k_stats` needs the population of margins the drive's answer rests on.
- **Leaf receipts:** each is the sum over its nodes' results, served or built. They are the same with the memo on or off, as B says.
- **Drive totals:** the sum of the leaf receipts, as today.
- **New, separate count:** nodes served, and decisions computed.
- **`k_stats`:** carry a node's samples with its value, the way `k_stats::detached` already hands a recording back as a value for the caller to splice. Splice them at each serve, so `k_stats` is also unchanged by the memo.
- **Both statements hold:** every instrument is invariant under the memo, and the separate count says what was computed. That makes the memo a pure performance dial, checkable by differential.

**(iv) Per-replay caps**
- **Yes, a defect under either store.** A cap that changes an *answer* makes a node's result depend on what ran earlier in the same leaf. A served node consumes no cap, so served and rebuilt results drift. Fix: caps per node.
- **Session state I checked** (`struct Session` in `sym.rs`):
  - `retry` with `SymRetry::max_forms` changes answers: an attempt is no longer offered "for the rest of the leaf". Needs a per-node cap.
  - `registry`: registrations made earlier in the leaf, which also clear the door memos. Changes answers. Scope it to the node's upstream nodes, carried with their values.
  - `params`: the brackets the decision read uses. The ones a node uses are its read set's, which are equal across a shared projection. Safe, provided every leaf records them, served nodes included (B flagged this; still unchecked).
  - `witness`: noted at the first decision, and constant within a drive. Safe.
  - The opaque sequence: covered by keying an opaque value by its minting node and ordinal.
  - `forms`, `forms_early`, `forms_door`, `trig_closed`, `reductions` with its `reductions_held` cap, `atoms`, `free`: caches or bookkeeping. Safe only if a hit or a miss changes cost and never an answer. `reductions_held` is stated as a cache cap; I have not verified it.
  - `nodes` (the expression table): moves to the drive.
- **The rule to state once:** session state is either a cache whose misses do not change answers, or it belongs to a node and travels with its value. Nothing else.

**(v) ONEHOME's row**
- **No conflict; it is the same principle one level down.** That row carries facts *within one build*: into a later finish, from a finished input whose data is bit-identical (the product's aggregate, `distribute::fused`, a Boolean's untouched faces), by lineage, as `FinishedBody::moved` already does.
- **Our unit is the whole node across replays.** Once the memo serves the plate, the per-leaf union of the served plate with new holes is exactly where ONEHOME's carry pays in every leaf, so the two multiply.
- **Same gate:** ONEHOME's "a carrying finish agrees with one that re-derives, finding for finding" is our differential. Neither should key a finish by a body fingerprint.

**(vi) Recommendation now, and the E6 text**
The node memo as in the revised For Ev below, with the parent replay as the prior unless M8's cousin share says to use a store.

E6 split rule, replacing "Split rule named, deterministic (D9): max relative width, ties to lowest index":
> Split rule named, deterministic (D9): of the variables the leaf's deciding node reads (the first undecided node in evaluation order), the one of greatest relative width, ties to lowest index. A leaf whose deciding node reads no variable that can still be split is refused, naming that node: no split changes it.

E6 leaf protocol, a new bullet:
> A node whose inputs a leaf leaves unchanged (no variable it reads was split) is served from the parent's replay, result, refusal and record whole (DESIGN [determinism], content-keyed transfer). A leaf's receipt counts what its result rests on, served or built.

**What M8 decides**
- **Cousin-only share of repeats after the split:** decides (ii).
- **Distinct keys per node, and the share of the non-finish build whose key repeats:** confirm or shrink the payoff both of us inferred.
- **Split axis against the deciding node's read set, and the size of the read-set tree:** confirm or overturn "the support-aware split is the larger saving" (B's 127 against 1,023).
- **Equal keys with different expressions** for the same node id: would overturn the "latent" concession above and make the key fix P0. Across different node ids it is expected and harmless under lookup by node id.

## For Ev (revised, whole)

**Recommendation (likely).** Build no new cache. The drive uses the evaluator's existing per-node memo: each leaf takes its parent box's replay as its prior, so a node the split did not touch is served whole. Separately, a box is split only along variables the undecided node reads. If a measurement now running shows that, even after that split, many repeats come from two different branches of the tree, the prior widens to one store per drive. That changes nothing else.

Terms:
- **Replay:** one evaluation of the document over one box of the bisection.
- **Node:** a recipe node, such as one sweep or one Boolean.
- **Content key:** the evaluator's hash of a node's op, its input values and its inputs' keys.
- **Read set:** the variables a node reads, directly or through its inputs.
- **Session:** the symbolic tier's per-replay state: its expression table, registered identities, counters and caps.

**What the problem is evidence of (sure).** DESIGN already says reuse by content key is correct ("the key *is* the correctness proof", `[determinism]`), and that the box lanes use "the same memoization … no parallel path" (`[scalars]`). The drive passes no prior because a symbolic value's meaning lives in per-replay session state, so it means nothing in another replay. The repeated finish and the repeated build (two thirds of leaf time) are both that symptom.

**The final state.**
1. **The node is the unit.** A node whose read set was not split is served from the parent replay with its body, finish, verdict rows, escalations, and any refusal it hit. The tour's plate is built once. Each hole is built once per distinct box of its own variables (sure on the unit).
2. **A symbolic value means the same in every replay of a drive** (likely). Three changes:
   - The expression table belongs to the drive. Expression ids are already content hashes.
   - A value with no tracked expression is keyed by the node that minted it and its ordinal there, not by a per-replay counter. Today two replays can give one number to two different values, a false zero waiting to happen.
   - Session state is either a cache whose misses never change an answer, or it belongs to a node and travels with its value. Identities a node registers, its decision counts and its telemetry samples travel; caps that change answers (the retry ladder's per-leaf limit) become per node.
3. **The key at the symbolic scalar hashes the expression too** (sure of the logic). Today it hashes only the number. Lookup by node id keeps this from biting inside a drive, but "same key, same output" is false as stated.
4. **The split follows the undecided node** (sure it is sound; likely the larger saving). Splitting a variable that node does not read reproduces it in both halves. If it reads no splittable variable, the box is refused at once with a new reason naming the node, instead of bisecting to the budget.

**Records and receipts.** A leaf's receipt and the telemetry count what its result rests on, served or built. Both are the same with the memo on or off, so the memo is a pure performance dial, checked by comparing runs with it on and off. A separate count says what was served and what was computed.

**Ratified text.** DESIGN: none. ERROR-DESIGN E6: the split rule ("of the variables the leaf's deciding node reads, the widest") and the immediate refusal; one leaf-protocol sentence: "a node whose inputs a leaf leaves unchanged is served from the parent's replay; a receipt counts what a result rests on". The split rule's author could not be traced (shallow history).

**Corrections.**
- The measured tour certifies nothing: every replay refuses at the holes. Those refusals are 254 distinct bodies of one kind, a missing lane in the sweep doors, not one refusal repeated.
- `m10_8`'s slow row has one leaf per drive, so this does not help it.
- Once the holes do finish, each costs about 3x a refusal (inferred), so reuse matters more.

**Alternatives.**
- A finish cache keyed by body: a second mechanism; its key is no proof, and it covers a third of the cost.
- Building unmoved nodes once, up front: a special case of the memo.
- The split rule alone, or the memo alone: each leaves the other's waste.
- Each of the last two (my first report and the other designer's) can be reversed by adding the other.
