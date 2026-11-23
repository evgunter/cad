# Review: session-state unit (PR 7492), frozen head `9ed55e47`

**Verdict: NOT-MERGEABLE-AS-IS.** Three MAJORs, all narrow. No answer moved: the differential is clean at three ε and the main-document path does what the unit says.

## MAJOR

**M1. The doc gate is red on the frozen head** (DEMONSTRATED, sure). `crates/geom-core/src/sym.rs`, `MemoSlot`'s doc links ``[`Session::retries`]``, a field this unit moved to `NodeState`. `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit` exits 1 (`unresolved link`, `could not document geom-core`); with that one link repaired in scratch it exits 0, so it is the only one. This is a step of the hosted lint job, which never ran on this head. The branch has since moved; its commit `97aecb2f` names this link (not reviewed).

**M2. A node's record does not ride a family's value, a part's nodes or the product, so claims 3, 10 and 11 are false on three shipped document shapes** (DEMONSTRATED at the record and the receipt, sure; no answer was made to change, see below).
- `editor-core/src/eval/members.rs`, `family_value`: the family's value gathers its members' verdicts, escalations and marks and sets `sym: Arc::default()`. Plate with a row of three copies: each member decides 1, the family's record holds 0, the leaf's receipt reads numeric 1050 against 1047 summed over its nodes. A node reading a family whole reads no member's registration (by inspection).
- `editor-core/src/eval/mod.rs`, `evaluate_at_descent`: part rows are entered before the node loop (`PartStore::enter`/`finish`), so a part's nodes open under the ROOT scope, not inside the instance node. **The PR body states the opposite.** A cylinder instanced: the leaf decides 629 symbolic and 3,020 numeric and its nodes' records sum to 0; the 37 identities the same cylinder registers as its own document ride no record of the instancing document. The filleted bracket instanced: 2,285 and 3,817 against 0; 94 registrations against 0.
- `editor-core/src/product.rs`, `gather_sources` → `record_at_rest` (under `product_recorded`): 3,122 of the instanced cylinder's decisions are taken at the root scope, in no node's scope at all. The root reads nothing a node registered.
- `MintScope::node` has one caller (`eval_node`) and nothing nests it, so `NodeGuard::close`'s "a scope inside a node's scope is part of that node" arm is reached by no shipped path and no row (by inspection, likely).
- **What I could not show:** a changed answer. With the root absorbing every node's registrations (a whole-replay registry, as on the base) the three probe documents decide identically. The base read all of these registrations everywhere; whether any shipped document loses a proof is unmeasured.

**M3. Four rows do not see their own subject** (DEMONSTRATED by mutant, sure; a blocker under `review-and-dependency-policy` where the untested axis is the row's subject).
- `geom-core/tests/sym_drive_memo.rs`, `a_value_built_in_one_leaf_means_the_same_in_another`: green with the table naming no node, and green again with cold caches as well. Leaf 2 is handed leaf 1's warm cache, and its root is the same id as leaf 1's, so the form table serves the root and `c` is never looked up. `DriveMemo::node` and `record_node` are pinned by nothing.
- `editor-core/tests/a_nodes_result_..._is_its_keys.rs`, `one_key_one_result_in_a_drive_that_retries`: green with every node sharing the retry memos (its named mutant; the default guard never binds).
- Same file, `one_key_one_result_in_a_drive_that_registers`: green with a whole-replay registry (its named mutant). Both rows compare leaves that run nodes in one order, so an order dependence inside a leaf is invisible to them by construction.
- Same file, `a_drive_is_the_same_on_one_thread_or_many_and_with_the_table_or_without` (claim 12's mutant): green when a leaf is handed an earlier leaf's EARLY forms. So are the other 22 rows I ran with it (`sym_drive_memo`, `the_plain_memo_moves_no_decision`, the unit's own). That mutant is unsound: my row proves `(max(x,0) − x)·t` zero over `x < 0` under it.

## MINOR

1. **The `debug_assert!` is reachable through public doors** (DEMONSTRATED, sure; severity likely). `editor_core::evaluate` with a prior built in another session, inside `geom_core::sym::with_session_memo_retry`, panics at `form_in` ("minted outside every session on the drive's table") on the plate's second axis. The base froze there. No kernel door passes a prior at `Sym`, so `drive()` does not reach it; the store will make it the main path. With assertions off (by inspection, likely): the id freezes, the frozen form is kept in the leaf's plain cache, goes back to the drive as a warm cache and is handed to another leaf, and the parents built on it are published, so the answer depends on which leaf ran first and "a cache holds nothing the table does not" is false.
2. **Rule D's closed forms are not a function of their key and the dials** (BY-INSPECTION, likely; pre-existing; claim 2). `sym/trig.rs`, `closed_forms`: keyed by the argument's digest alone, built through `sqrt_atom` → `root::mint` and `common_factor` under whichever attempt's rules are swapped in, and a refusal is memoised. The shipped ladder's first retry shuts `canonical_root`. `Session::reductions` keys on rules and ring bound for this reason and its doc says `trig_closed` does not. I did not construct a changed answer.
3. **The ownership table is not complete** (sure). `Session::mints` and `mint::OPEN` are in no row (node-key's ground). `form::DIV_STEPS` is a `cfg(test)` counter, not "scoped to one walk and restored". `form::REFUSED` is cleared before each read, not restored.
4. **Sentences false in a fact this change moved, not listed in the body** (sure): `sym.rs` module header, "Node ids are CONTENT HASHES" ("the hash-consing table is per-leaf-replay … so are … the registry", "a retry's forms live in the session's own retry tables"); `editor-core/src/eval/mod.rs`, `LeafPrior`'s doc ("its hash-consing table is per session … the forms would freeze … sound"); `sym::session_installed`'s doc and `geom/src/memo.rs` `session_memo`'s ("freezes"); `drive.rs`, `ParamBoxVerdict::plain_memo` ("the forms and atoms it held"). E12's "session-level record" is disclosed and scheduled nowhere; CLAUDE.md lands such a re-wording with the change.
5. **`a_leafs_receipt_is_the_sum_of_its_nodes_records` names no mutant and reads no family, part or product** (sure): M2 is what it does not see.
6. **Inherited reds without an open row** (sure on the runs): the three `geom-core` source censuses (red on base and head; `certified_endpoint_census` differs between trees only at `topo/src/props.rs`, not this diff's) are held by class rows only; `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off…` (1e-12, both trees) by closed rows only. Nine `editor-core` rows are on the open `green/main-red-eleven-…`; `split_pinch_keeps` on the dispatched `green/main-red-rows-two-lanes-…`. GREEN's ground.

## NOTE

- **A leaf that panics leaves its session installed on the thread** (DEMONSTRATED, sure; pre-existing, SYM's ground, to file): `with_session_in` takes the session only after `f` returns. After MINOR 1's panic every later session on that thread fails "symbolic sessions do not nest"; the stale session also keeps the drive's table alive. `NodeGuard` and the table's lock recovery are written for unwinding; the door is not.
- Upstream records merge in node-id order, not topological order. Which of two identities a cycle drops, and which representative an alias takes, follow that order (by inspection, likely); it is the document's, so the same in every leaf.
- A `Poisoned` result keeps no record (unsure whether anything decides before the poison is read).
- On the frozen head `DriveMemo::publish_form` takes the write lock for every form with the table off (sure); `f210de0c` on the branch addresses it (not reviewed). Measured, it costs nothing visible on the plate.

## Claims
1. **Held on the main-document path**, by inspection, the author's rows and my served-leaf probe; two exceptions by inspection (the brackets, known; MINOR 2). Not broken by demonstration.
2. **Broken** in completeness (MINOR 3) and for `trig_closed` (MINOR 2). The reduction cap holds by inspection: a miss recomputes and no count reads it.
3. **Closure right inside one document** (the schedule orders by the same `upstream_of`; with upstream records ignored the fed row and the bracket row go red). **Broken** for a family read whole and a part's instance (M2). A failed upstream's record is not read, and its readers are poisoned anyway.
4. **Held in drives**: over `editor-core`'s whole suite, slow set included, nothing registers at the root scope under a drive's table (traced). Outside a drive: not checked, my trace keys on the table.
5. **Held** by inspection and the tier row (red under shared memos). The document row does not pin it (M3). At the root scope one guard spans every decision of the gather.
6. **Held** by inspection: the plain walk is first-attempt only and reads no bracket or registry; first writer wins with a `debug_assert_eq!`; rule H's read is the same with cold caches. Exception: MINOR 1 with assertions off.
7. **Held**: every row I ran is green with every leaf starting cold.
8. **Broken**: reachable (MINOR 1).
9. **Held** (likely): each removed row's mechanism is the unrecorded or taint branch (one read in full, six by header); closing the SYM row follows. What they guarded is unguarded with assertions off.
10. **Broken for a family** (M2). Otherwise held: `NodeValue` and `NodeError` derive `Debug` (the record prints) and neither equality, hash nor serialization; nothing pinned moved.
11. **Broken** (M2).
12. **Held on what ran**; the mutant the row does not see is in M3, and a row that sees it is on my branch.
13. **Held** (sure on the gating, likely on the meaning): `stated` and its push are behind `sym-revalue-testing`; the instrument asks under the union, as the base's one registry did. A superset registry can freeze where a subset did not, so "no contradiction" is not monotone.
14. Fed twin: red under its mutant. `a_value_built…`: green under its mutant. The two document rows: green under theirs. "Red on the base" for the first three rows: **not checked**.
15. **Held where measured.** Plate: 74,486 nodes at 2, 8 and 32 leaves; 32 leaves take 8.5 s on one worker and 2.8 s on four, on base and head alike; estimate 19.4 → 25.3 MB (bracket 44.5 → 53.9 MB), warm caches uncounted, no cap. A document whose leaves build different expressions: **not constructed**. The table is dropped with its drive except after a panicking leaf (NOTE).
16. **Taken, clean.** `editor-core` whole with the slow set, base against head: default ε 10 reds each, 1e-6 10 each, 1e-12 11 each, the same rows with the same panic text. `geom-core` whole at default ε: the same 3. `demos/tour` under `ci-tour`, every row: 107 of 107 on both trees (978 s, 987 s). Rows: MINOR 6.
17. Thread-locals spot-checked: `COEFF_BOUND`, `REFUSED`, `DIV_STEPS`, `turn::KEY`, `deferred::NEXT`, `mint::OPEN` (MINOR 3). The second pass covers four crates; `geom`, `geom-brep`, `profile`, `verbs` are outside it. I swept them: the NURBS per-object `OnceLock`s all go through `session_memo` or its test. Docs: MINOR 4.
18. **What a store could still get wrong after this PR:** a served family value carries no record (M2); a part's nodes and the product sit outside every node's record, and the product's root-scope retry guard and mints are the replay's (M2); a served value in a leaf whose table serves nothing, or from outside the drive, is an assert and not a refusal (MINOR 1); `trig_closed` can carry an earlier node's retry into a first attempt (MINOR 2); the brackets and the `frozen` roots, both already on the store's row. What held: the plate's half boxes served from the root leaf through the evaluator's own memo match a fresh leaf node by node, and receipt plus reused records equals the fresh receipt; only `frozen` differs (30 against 36), as that row says.

## What the end-to-end exercise showed
The unit solves the right problem for a document of plain nodes: serving a leaf from an earlier one already works through `evaluate`'s prior on one table. What is missing in practice is every seam the evaluator has besides a node: a family, a part, the product. The record's public face (`counts`, `registrations`) is not enough for the store, which needs the roots. `upstream_records` re-walks the transitive upstream for every node and every member.

## Style
Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 in part (`sym/memo.rs` end to end; `sym.rs` is the largest file touched and I read its header by scan, the session, `form_in`, the ladder and the registration door, not the whole).
- Q1, sure: `sym.rs`, `Session::record_identity` and the tail of `Sym::register_equal` are one rule spelled twice (alias, cycle test, insert, clear the door memos). The comment in `register_equal` says `record_identity` clears them, and then clears them inline.
- Q1, sure: `NodeState::absorb` and `Session::counts_so_far` each add `frozen` by hand beside `SymCounts::absorb`.
- Q1 (the fresh-instance trap), likely: `Session::stated` is a whole-replay registry again, test-only, beside the per-node one this unit makes.
- Q2/Q5, sure: `NodeRecord`'s doc says two replays under one content key hold equal records; the key hashes no bracket.
- Q5, sure: `memo.rs` header and `Warm`'s doc ("a cache holds nothing the table does not"): the warm atom cache holds every early and retry atom, which the table does not.
- Q3: M3 and MINOR 5.
- Q4, sure: three doc sites still cite "freezes" for a value from outside the session (MINOR 4): the doc rotted, the code is as meant.
- Q6, sure: E12's re-wording, `docm9`'s table and the switch's box arm are disclosed and scheduled nowhere; "25.4 MB, does not grow" rests on one document and nothing guards it.
- Q7, likely: `NodeGuard::close` answers `None` on a depth mismatch and `node_scope` turns that into an empty record: a kernel bug read as "the node decided nothing". The explicit `close` then `mem::forget` beside a `Drop` that also closes is awkward.
- Q7, unsure: the table's `nodes` map (74,486 entries on the plate) serves no reader today; it is the next unit's and is paid for now.

## Rubric
- Idiom and structure: **4**. The owner of each piece is where it lives, and the taint machinery is gone; one rule is spelled twice.
- Test quality: **2**. The tier rows pin their mutants; four of the five new document and table rows do not see their subject.
- Doc and comment honesty: **2**. The body's account of parts is the opposite of the code, the module header was left on the old state, and a broken link reds the gate.

## Spec deviations
- Reported by the author: node-key merged before its PR; four mutants not run; the teardown did not move (filed); E12 not edited; the ε rows and the tour's base not run; `MemoSize.nodes`; the brackets left off the record.
- SILENT: a family's value carries no record; part rows are built outside every node (the body says inside); the product decides at the root; the orchestrator's condition that the assert stands "only if no public door can reach it" is answered with "no leaf of any drive meets it"; `Session::mints` left out of the table; the module header left stale.

## Gates
Hosted: nothing has run on the frozen head. Its run had only the change filter queued; the PR head has moved twice and the present run (45111186465) has the change filter green and every lint and test job queued. Mine, on the frozen head in a clean worktree: `cargo fmt --all --check` (and `benches`) clean; `cargo clippy --workspace --exclude viewer --all-targets --all-features -- -D warnings` clean; `scripts/run-gates.sh` and `payload-rung-sweep.py --check` 0 against the PR's base; `work.py lint` ok; the tour's fmt and clippy clean; **the doc gate red (M1)**.

## What I executed and what I only read
- Branch `boxfinish/review-session-state-X` at `ba75d510`: `crates/editor-core/tests/review_session_state_probes.rs` (family, part, bracket-as-part, served leaf, foreign prior, cost), `crates/geom-core/tests/review_session_state_probes.rs` (two rows, each red under its mutant), `probe/review-session-state/mutants.patch` (six mutants and a root-scope trace switched by environment variables; not applied on the branch). The two receipt probes are red on the head on purpose.
- Ran: the mutants above against the unit's rows; the root-scope trace over `editor-core` whole; the differentials in claim 16; the cost probe on both trees; the gates. My "head" build carries the mutant patch switched off and my probes.
- Only read: claims 6, 9 and 13, MINOR 2, the assertion-off behaviour, the family's lost registrations. Not run: `demos/wild`, a build with debug assertions off, the rows' "red on the base".
