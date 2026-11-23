REVIEW — PR 7492 (session-state unit), frozen head 9ed55e47. Reviewer branch: boxfinish/review-session-state-X @ bb48acc4 (two probe rows, both red on the head).

**Verdict: NOT-MERGEABLE-AS-IS.** The doc gate is red on the head. Two of the unit's central claims (11, and the part half of 3) are false on documents with a family or a part, and one is false in the PR body's own words. Two of its rows stay green under the mutants they name. Nothing I found is unsound: every loss goes the conservative way.

## Gate
- Hosted CI on 9ed55e47 when I looked: "change filter" queued, "prime the build cache" skipped. No job had run, so there is no green to lean on.
- My local runs on the head:
  - pass: `work.py lint`, `cargo fmt --check` (workspace and benches), `scripts/run-gates.sh` (GATE_BASE = merge-base), `payload-rung-sweep --check`, clippy `--workspace --exclude viewer --all-targets --all-features -D warnings`.
  - **fail: `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`, rc=1**: `unresolved link to Session::retries` at crates/geom-core/src/sym.rs:4188. This unit moved `retries` into `NodeState` and left the link. The lint job would go red. DEMONSTRATED, sure.

## MAJOR
1. **A leaf's receipt is not the sum of its nodes' records on a family** (claim 11 broken). DEMONSTRATED, sure. My row `review_session_state_probes::probe_a_family_leafs_receipt_against_its_nodes_records` uses g_family_of_booleans' cut_row with a block hole and a union of the family. Session counts are symbolic 32482 / numeric 42270; the top-level records sum to 26988 / 33959.
   - Cause: `members::family_value` and `eval_family`'s fail arm write `sym: Arc::default()` (eval/members.rs:317, :511). Each member's record lives only on the member value.
   - Consequences: a downstream node that reads the family whole gets an empty record, so it reads none of the members' registrations (BY-INSPECTION). On the base it read them through the session's one registry, so this is a silent loss of door proofs on patterned documents. The next unit's "receipt = sum of served records" under-counts families.
   - The row `a_leafs_receipt_is_the_sum_of_its_nodes_records` runs only the bracket and the plate (no family, no part), so its premise excludes this (style Q3).
2. **A part's nodes are not "inside the instance node"**, contrary to the PR body ("A scope opened inside a node's scope (a part's nodes inside the instance node) is part of that node"). DEMONSTRATED for the counts, sure; BY-INSPECTION for the registrations, likely.
   - eval/mod.rs:4624-4633 enters every part (`store.enter`) before the top's node loop, at the root scope. Each part node opens its own scope directly under the root, and closing it does not hand its identities upward (`!sess.outer.is_empty()` is false, sym.rs NodeGuard::close).
   - Probe `probe_an_assemblys_leaf_receipt_and_its_parts_registrations` (plate as part, one instance): the session counts the plate's 979/1047 decisions, and the top's records sum to 0/0. The plate alone has 74 registrations on its records; the top has 0.
   - Caveat: my instance refused EmptyProduct (the plate is not placed in its world), so the registration half is shown only through the empty record.
   - By construction, top-level nodes of an assembly no longer read any registration its parts made. On the base they did. Verdict effect on a real assembly: not measured.
   - The receipt identity fails on every assembly, and decisions made while a part is entered (mate solve, product gather) are outside every node's scope by design.
   - The body's ownership table and its "Decided in the lane" bullet ("in a drive's leaf nothing registers there") hold for the root registry, but not for "every decision is in a node's scope".
3. **The rows' named mutants (claim 14).**
   - `one_key_one_result_in_a_drive_that_registers`: **GREEN under its own named mutant.** The mutant was a node's scope opened over every earlier node's registrations, planted as sym.rs:2617 `if true` so that the root takes every node's identities. All four editor rows stayed green; geom-core's `a_registration_is_not_read_by_a_node_it_does_not_feed` went red under the same build, so the mutant was live. DEMONSTRATED, sure. The cause: across the leaves, a node with one key runs after the same predecessors, which register the same things. So the row cannot see order leakage.
   - `sym_drive_memo::a_value_built_in_one_leaf_means_the_same_in_another`: **GREEN under its named mutant** (node_of resolving in the leaf's own cache alone, sym.rs:2805). Leaf 2 starts from leaf 1's warm cache, which already holds `c`, so the drive-table fallback is never reached. DEMONSTRATED, sure. The fallback in `Session::node_of` is then reachable only by carrying a value across sessions, which the memo header says no kernel path does: in production it is dead code, guarding only the `debug_assert!`'s premise.
   - The fed-node twin (empty registry whatever upstream holds, sym.rs:2661): red, as claimed.
   - Determinism row (claim 12), under my mutant (leaf `frozen` = work, not NEED, sym.rs:3073): red; `m10_sym_drive_memo_interval::every_leaf_reports_one_column...` is also red. The row holds against that mutant.

## MINOR
4. Stale prose this change made false:
   - sym.rs:664-668: "The hash-consing table is per-leaf-replay … holds nothing across leaves … so are … the registry". Now the table is the drive's and the registry is a node's.
   - sym.rs:673-676: retry forms are "dropped with the leaf"; they are now dropped with the node's scope.
   - sym.rs:815: the early walk "consults the leaf's registry and stays per leaf".
   - docs/ERROR-DESIGN.md:705: "a session-level record". The body discloses this one as "not edited here", with no schedule (style Q6). It is a reword forced by the code change, not a design change, so per CLAUDE.md it lands with this PR.
   - BY-INSPECTION, sure. The first three are not in the body's list.
5. The release-build reading of the new `debug_assert!` in `form_in` is unstated:
   - `[profile.release]` keeps debug assertions, so a release build panics there.
   - With assertions off, the taint guard is gone, so the parent's plain form carrying the foreign indeterminate IS published (`note` → `publish_form`). A later leaf that recorded the child is then served it, and the answer depends on leaf order. That is the very R1-M6 shape the removed branch existed for.
   - The state is reachable through public doors (`with_session_memo` plus a value minted outside it, which is exactly what `a_node_minted_outside_..._refused_loudly` does). No editor drive path reaches it that I found: drive passes no prior, parts are entered inside the session, geom::memo/nurbs serve only with no session.
   - BY-INSPECTION, likely.
6. Warm caches are excluded from `MemoSize::bytes_estimate` ("not counted at all") and only ever grow. The table keeps one cache per concurrently finished leaf, and each tends toward the whole table. So the "25 MB, does not grow with leaf count" figure covers the shared table, not resident memory: about workers × table more. BY-INSPECTION, likely. Not measured (claim 15).

## NOTE
7. `node_scope` replays upstream identities via `record_identity` in node-id order. The cycle drop and the alias representative depend on that order, so two upstream sets in another order could pick a different representative. It is deterministic per document, so there is no claim-1 break. BY-INSPECTION, unsure whether it moves any decision.
8. The registry clone per node (`sess.node.registry.clone()`) is O(registry) per node. Cost is unmeasured on registering documents. unsure.

## Per claim
1. Held on the two documents and by inspection. Broken in spirit for family readers and assemblies (MAJOR 1, 2), which hand a node a different record than its inputs' registrations would.
2. Session fields enumerated; the table is accurate except that counts are not all in nodes' scopes (MAJOR 2). Reduction cap: by inspection, a miss recomputes the same answer, and counts are unaffected (reductions count nothing). Warm atoms: lookups are guarded by the form carrying the id, so presence does not matter (likely).
3. Broken for parts (MAJOR 2) and for a family read whole (MAJOR 1). A FAILED upstream's record is dropped by `usable_in(..).ok()`; its downstream is poisoned anyway.
4. Root registry: held in a drive by inspection (register_equal callers: arc.rs, sweep/swept.rs, all inside ops). Root decisions: broken (part entry).
5. Held: geom-core row red under its mutant; the guard is per node.
6. Held by inspection. publish_form keeps the first copy and debug_asserts equality; the cos twin is recorded early, not published, and travels in the warm atoms.
7. Held by inspection (warm caches are a subset of the table), except MemoSize/memory (MINOR 6).
8. MINOR 5.
9. The seven rows pinned the taint/foreign branches, which are deleted, so "cannot occur" holds only with assertions on. Closing the row is right given MINOR 5 is stated somewhere.
10. NodeRecord derives Debug/PartialEq/Eq and is printed in the one-key row's text. I found no content-key or serializer use. Reused values keep their record. Partially checked.
11. Broken (MAJOR 1, 2).
12. Held against my frozen-column mutant. Its documents have no family or part.
13. By inspection, the merge line replays `sess.stated` under a taken node state and restores it. `stated`/`instrumented` are test-only per the body; I did not verify the cfg gating on every field (unsure).
14. MAJOR 3.
15. Not measured (time). MINOR 6 by inspection. Lock: `record_node` takes a write lock per node new to a cold leaf. Not measured.
16. Not run: the 1e-6/1e-12 suites, the tour on either tree, inherited-red bisection. The unit's own editor rows pass at the default ε.
17. MINOR 4. Spot-checked thread-locals form::REFUSED (taken before each combine), rational::COEFF_BOUND, form::DIV_STEPS (cfg(test)), product::GATHERS (debug counter), turn::KEY (constant): the dispositions are right. The named blind spot ("through OpEnv") is the real one, and its part-rows member is exactly MAJOR 2.
18. What a cross-leaf store could still get wrong: families' and parts' counts/registrations missing from records (MAJOR 1, 2); brackets not on the record (the body names this); the order dependence of NOTE 7 if the store merges upstream records differently; records of reused/prior values (kept as built: fine).

## End-to-end
The change solves the right problem for flat documents. It does not cover the two composite shapes the evaluator has, member families and part instances, and those are where the next unit's "receipt counts what its result rests on" and "a registration travels to what it feeds" have to hold. The rows built to prove the property cannot fail on those shapes, and the registers row cannot fail at all under its own mutant.

## Style
Exercised Q1, Q2, Q3, Q4, Q5, Q6. Not exercised: Q7 beyond the line below, Q8 (sym.rs is too large to read whole in this budget).
- Q3 `a_leafs_receipt_is_the_sum_of_its_nodes_records`: its documents exclude families and parts, the failing mode. sure.
- Q3 `one_key_one_result_in_a_drive_that_registers`: cannot fail under order leakage, because the same predecessors run in every leaf. sure.
- Q3 `a_value_built_in_one_leaf_means_the_same_in_another`: the warm cache satisfies it without the table. sure.
- Q4/Q5 `memo.rs` header "No kernel path carries a symbolic value across a session's boundary": that row itself carries `c` across two sessions through a public door. likely.
- Q1 `NodeState::absorb` and `Session::counts_so_far` both spell "absorb, then add frozen by hand" (sym.rs, both new). That is two copies of one rule that `SymCounts::absorb` deliberately excludes. likely.
- Q2 the `eval_node` comment "A reused value keeps the record of the run that built it" asserts an invariant nothing checks: no row serves a prior at Sym. unsure.
- Q6 ERROR-DESIGN E12 is "not edited here" with no schedule. sure.
- Q7 `NodeGuard` plus `mem::forget` plus `close()` returning None on a depth mismatch is a fragile shape: a mismatch silently yields an empty record. I'd have returned the record from the guard's drop path only. unsure.

## Rubric
- Idiom/structure 3: the ownership split is clean, but part and family scoping are not modelled.
- Test quality 2: two rows are blind to their named mutants, and the receipt row's premise excludes the failing shapes.
- Doc/comment honesty 3: the moved facts are mostly updated; four stale sentences remain, and the body's part claim is false.

## Spec deviations
- Reported by the author: E12 not edited (unscheduled); four mutants not run.
- SILENT: part and family records are empty (registrations no longer cross into the top or into whole-family readers); the doc gate is red; the release reading of the debug_assert.

## Executed
- Gates as above.
- Probes (bb48acc4): family receipt, red; assembly receipt/registrations, red.
- Mutants:
  - fed twin: red.
  - value_built: GREEN.
  - root absorbs registrations: geom row red, all four editor rows GREEN.
  - frozen := work: determinism row red, plus the m10 column row red.
- Only read: the 1e-6/1e-12 suites, the tour, the inherited reds, the memory and lock measurements.
- Defects outside this unit's fence: none to file. The stale `Session::retries` link is this unit's.
