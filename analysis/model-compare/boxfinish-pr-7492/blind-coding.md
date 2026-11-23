# Correspondence coding: PR 7492, frozen head `9ed55e47`, reviews A and B

Source: `A.md` and `B.md` only. Evidence marks are as each review writes them:
EXEC = the review says DEMONSTRATED / ran a probe, mutant, gate or measurement;
INSP = BY-INSPECTION or argued; unstated = no mark (a bare "sure/likely/unsure"
confidence word is not an evidence mark and is given in brackets).

## 1. Verdicts as written

| | Verdict | MAJOR | MINOR | NOTE | Other |
|---|---|---|---|---|---|
| A | NOT-MERGEABLE-AS-IS | 3 (numbered 1-3) | 3 (4-6) | 2 (7-8) | doc-gate failure reported in `## Gate` and as the verdict's first reason, not numbered; 8 style lines; rubric 3 / 2 / 3 |
| B | NOT-MERGEABLE-AS-IS | 3 (M1-M3, "Three MAJORs, all narrow") | 6 | 4 | 10 style lines; rubric 4 / 2 / 2 |

A: "Nothing I found is unsound: every loss goes the conservative way." B: "No answer moved."

## 2. Bilateral headline

1. **Composite shapes carry no record.** A family's value is built with `sym: Arc::default()` and a part's nodes open under the root scope, so a leaf's receipt is not the sum of its nodes' records and registrations do not reach a whole-family reader or the instancing document (claims 3 and 11 broken; the PR body says the opposite about parts). A MAJOR 1 + 2; B M2. Both EXEC on counts.
2. **The doc gate is red on the head**: unresolved link `Session::retries` in `crates/geom-core/src/sym.rs`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit` rc=1. Both EXEC. B ranks it MAJOR (M1); A reports it under Gate and in the verdict, unnumbered.
3. **Rows green under their own named mutants**: `one_key_one_result_in_a_drive_that_registers` and `a_value_built_in_one_leaf_means_the_same_in_another`. A MAJOR 3; B M3. Both EXEC.

## 3. Coding table

### 3a. BILATERAL (16)

| # | Underlying defect | A: where / severity | B: where / severity | Class | Evidence A | Evidence B | Divergence |
|---|---|---|---|---|---|---|---|
| X1 | Doc gate red: unresolved link `Session::retries` (`sym.rs:4188`, `MemoSlot`'s doc) | `## Gate`, verdict, SILENT list / unnumbered | M1 / MAJOR | doc-or-claim-only (gate-breaking) | EXEC | EXEC | severity: B MAJOR, A no label |
| X2 | `members::family_value` (and `eval_family` fail arm) set `sym: Arc::default()`: family's record empty, receipt != sum of records, whole-family reader reads no member registration | MAJOR 1 | inside M2 | code | EXEC (counts 32482/42270 vs 26988/33959); INSP (registrations) | EXEC (1050 vs 1047); INSP (registrations) | none |
| X3 | Part rows entered before the node loop (`store.enter` / `PartStore::enter`): part nodes open under the ROOT scope, not inside the instance node; top's records sum to 0; part registrations reach no record of the top. PR body states the opposite (deduped here) | MAJOR 2 | inside M2 | code | EXEC (counts 979/1047 vs 0/0; 74 vs 0 registrations, with the `EmptyProduct` caveat); INSP (registrations, "likely") | EXEC (629/3,020 vs 0; 37 vs 0; 2,285/3,817 vs 0; 94 vs 0) | B demonstrates the registration half; A argues it |
| X4 | Decisions of the product gather (A: "mate solve, product gather"; B: `gather_sources` -> `record_at_rest`) are taken at the root scope, in no node's scope | inside MAJOR 2 (one clause, "by design") | inside M2 (own bullet; also claim 5, claim 18) | code | unstated (asserted) | EXEC (3,122 decisions) | depth: A a clause, B a measured bullet |
| X5 | `one_key_one_result_in_a_drive_that_registers` green under its named mutant (whole-replay / root-absorbing registry) | inside MAJOR 3; Style Q3 | inside M3 | test-gap | EXEC | EXEC | none |
| X6 | `sym_drive_memo::a_value_built_in_one_leaf_means_the_same_in_another` green under its named mutant | inside MAJOR 3; Style Q3 | inside M3 | test-gap | EXEC | EXEC | stated cause differs (see 5.1) |
| X7 | The drive table's node naming has no production reader (`Session::node_of` fallback "dead code"; `DriveMemo::node`, `record_node`, the `nodes` map "serves no reader today") | inside MAJOR 3 | inside M3 ("pinned by nothing"); Style Q7 [unsure] | code | unstated (argued) | EXEC for unpinned (mutant); unstated for no-reader | A calls it dead code; B calls it the next unit's, paid for now |
| X8 | `a_leafs_receipt_is_the_sum_of_its_nodes_records` runs no family or part (B adds: no product, names no mutant), so it cannot see X2/X3 | inside MAJOR 1; Style Q3 [sure] | MINOR 5 [sure] | test-gap | unstated on the row (rests on the EXEC probes of X2/X3) | unstated (rests on M2) | severity: A inside MAJOR, B MINOR |
| X9 | `debug_assert!` in `form_in` reachable through public doors (`with_session_memo` + a value minted outside); with assertions off the guard is gone and the answer depends on leaf order; release reading unstated in the PR | MINOR 5; claim 9; SILENT list | MINOR 1; claims 6, 8, 9; SILENT list | code | INSP [likely] | EXEC (panic reproduced via `editor_core::evaluate` + `with_session_memo_retry`); INSP for assertions-off | evidence: A INSP, B EXEC |
| X10 | `sym.rs` module header stale ("hash-consing table is per-leaf-replay … so are … the registry"; retry forms' home) | MINOR 4 (`sym.rs:664-668`, `:673-676`) | MINOR 4 | doc-or-claim-only | INSP [sure] | unstated [sure] | none |
| X11 | `docs/ERROR-DESIGN.md` E12 "session-level record" disclosed as not edited, scheduled nowhere | MINOR 4; Style Q6 | MINOR 4; Style Q6 | doc-or-claim-only | INSP [sure] | unstated [sure] | none |
| X12 | Warm caches are outside `MemoSize::bytes_estimate` and uncapped, so the "25 MB, does not grow" figure is not resident memory / is unguarded | MINOR 6 | claim 15 remark ("warm caches uncounted, no cap"); Style Q6 | code (claim figure) | INSP [likely], "not measured" | EXEC for the estimate (19.4 -> 25.3 MB); unstated for the gap | severity: A MINOR, B remark + style; B scores claim 15 "Held where measured" |
| X13 | Upstream identities replay in node-id order (`node_scope` -> `record_identity`): cycle drop and alias representative follow that order | NOTE 7 | NOTE (2nd) | code | INSP [unsure] | INSP [likely] | none |
| X14 | `NodeState::absorb` and `Session::counts_so_far` both add `frozen` by hand beside `SymCounts::absorb` | Style Q1 [likely] | Style Q1 [sure] | code | unstated | unstated | none |
| X15 | `NodeGuard` + `mem::forget` + `close()` answering `None` on a depth mismatch yields an empty record silently | Style Q7 [unsure] | Style Q7 [likely] | code | unstated | unstated | none |
| X16 | The brackets are on neither the record nor the key | claim 18 remark ("the body names this") | claim 1 exception, claim 18; Style Q2/Q5 (`NodeRecord`'s doc false for it) [sure] | contract-API | unstated | unstated | A lists a disclosed risk; B finds a false doc sentence |

### 3b. UNILATERAL, A only (6)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| A1 | `sym.rs:815` stale: the early walk "consults the leaf's registry and stays per leaf" | MINOR 4 (third site) | doc-or-claim-only | INSP [sure] |
| A2 | `sess.node.registry.clone()` is O(registry) per node | NOTE 8 | code | unstated [unsure], unmeasured |
| A3 | `record_node` takes a write lock per node new to a cold leaf | claim 15 remark | code | unstated, "Not measured" |
| A4 | `memo.rs` header "No kernel path carries a symbolic value across a session's boundary": the unit's own row carries `c` across two sessions | Style Q4/Q5 [likely] | doc-or-claim-only | unstated |
| A5 | `eval_node` comment "A reused value keeps the record of the run that built it": no row serves a prior at Sym | Style Q2 [unsure] | test-gap | unstated |
| A6 | The determinism row's (claim 12) documents have no family or part | claim 12 remark | test-gap | unstated |

### 3c. UNILATERAL, B only (16)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| B1 | `one_key_one_result_in_a_drive_that_retries` green with every node sharing the retry memos (its named mutant) | inside M3 (MAJOR) | test-gap | EXEC |
| B2 | `a_drive_is_the_same_on_one_thread_or_many_and_with_the_table_or_without` green when a leaf is handed an earlier leaf's EARLY forms, as are the 22 other rows run; the mutant is unsound (B's row proves `(max(x,0) − x)·t` zero over `x < 0` under it) | inside M3 (MAJOR) | test-gap | EXEC |
| B3 | `MintScope::node` has one caller; `NodeGuard::close`'s nested-scope arm is reached by no shipped path and no row | inside M2 (MAJOR) | code | INSP [likely] |
| B4 | Rule D `closed_forms` (`trig_closed`) keyed by the argument's digest alone, built under whichever attempt's rules are in; pre-existing | MINOR 2 | code | INSP [likely]; "did not construct a changed answer" |
| B5 | Ownership table incomplete: `Session::mints`, `mint::OPEN` in no row; `form::DIV_STEPS`, `form::REFUSED` mis-described | MINOR 3 | doc-or-claim-only | unstated [sure] |
| B6 | Further stale doc sites: `LeafPrior`, `sym::session_installed`, `geom/src/memo.rs` `session_memo`, `drive.rs` `ParamBoxVerdict::plain_memo` ("freezes") | MINOR 4; Style Q4 | doc-or-claim-only | unstated [sure] |
| B7 | Inherited reds without an open row (three `geom-core` source censuses; `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off…`) | MINOR 6 | doc-or-claim-only (tracker; poor fit, see 5.4) | EXEC ("sure on the runs") |
| B8 | A panicking leaf leaves its session installed on the thread (`with_session_in`); pre-existing | NOTE | code | EXEC |
| B9 | A `Poisoned` result keeps no record | NOTE | code | unstated [unsure] |
| B10 | `DriveMemo::publish_form` takes the write lock for every form with the table off | NOTE | code | unstated [sure]; cost measured as nothing visible |
| B11 | `Session::stated` is a whole-replay registry again (test-only); "no contradiction" under the union is not monotone | claim 13 remark; Style Q1 [likely] | code (test-only) | unstated |
| B12 | `Session::record_identity` and the tail of `Sym::register_equal` spell one rule twice; the comment says one clears the door memos, then clears inline | Style Q1 [sure] | code | unstated |
| B13 | `memo.rs` header and `Warm`'s doc "a cache holds nothing the table does not": the warm atom cache holds every early and retry atom | Style Q5 [sure] | doc-or-claim-only | unstated |
| B14 | `docm9`'s table and the switch's box arm disclosed and scheduled nowhere | Style Q6 [sure] | doc-or-claim-only | unstated |
| B15 | The record's public face (`counts`, `registrations`) is not enough for the store, which needs the roots (`frozen`) | end-to-end remark; claim 18 | contract-API | unstated |
| B16 | `upstream_records` re-walks the transitive upstream for every node and every member | end-to-end remark | code | unstated |

## 4. Tally candidates

Two, both B only, both test-gap, both inside B's M3.

1. **B1** `one_key_one_result_in_a_drive_that_retries` green under its named mutant. (a) A never names the row as blind: A's claim 5 is "Held" on the geom-core row and A's one editor-row mutant was the registry one; raised inside MAJOR M3. (b) test-gap. (c) a distinct row and mutant from X5/X6. (d) "DEMONSTRATED by mutant".
2. **B2** the determinism row green under the EARLY-forms hand-off mutant. (a) A tested the row only against a different mutant (leaf `frozen` = work), found it red and wrote "The row holds against that mutant"; A never raises early-form hand-off; inside MAJOR M3. (b) test-gap. (c) distinct from B1, X5, X6. (d) "DEMONSTRATED by mutant", plus a soundness witness row.

No A-only candidate: every defect inside A's MAJORs 1-3 is also in B (X2-X8).

Not candidates, and why:
- **X4** (product gather at the root scope): coded BILATERAL on A's one clause in MAJOR 2. This is the call the tally is most sensitive to: read as A not raising it, it would be a third B-only candidate (code, inside M2, EXEC).
- **B3**: unilateral and inside a MAJOR, but INSP, so fails (d).
- **X1**: MAJOR in B only, but A raises it; doc-or-claim-only besides.

## 5. Notes

Factual contradictions:
1. **Why `a_value_built…` is blind (X6).** A: leaf 2 starts from leaf 1's warm cache, which already holds `c` ("the warm cache satisfies it without the table"). B: the row is "green again with cold caches as well"; the root is the same id in both leaves, so the form table serves the root and `c` is never looked up. B's run contradicts A's cause.
2. **Warm caches against the table.** A claim 7: "warm caches are a subset of the table" (held). B Style Q5: "the warm atom cache holds every early and retry atom, which the table does not" (sure). A's own claim 6 says the cos twin "travels in the warm atoms" unpublished.
3. **Ownership table and thread-locals.** A claims 2 and 17: the table is accurate except for counts, and the dispositions of `form::REFUSED`, `form::DIV_STEPS` and the rest "are right". B MINOR 3: the table is incomplete and mis-describes `DIV_STEPS` and `REFUSED`. Both report the same underlying facts (`DIV_STEPS` is `cfg(test)`; `REFUSED` taken/cleared before each read) and judge the table's wording oppositely.

Differences that are not contradictions:
- Claim 12: A "held against my frozen-column mutant"; B finds the row blind to another mutant (B2).
- Rows blind to their mutant: A two, B four (B1, B2 are the difference).
- Claim 1: A "broken in spirit"; B "not broken by demonstration". Claim 10: A "partially checked", no break; B "broken for a family". Claim 15: A not measured; B measured, held.
- Claim 13 gating: A unverified; B "sure on the gating".
- Rubric: idiom A 3 / B 4; doc honesty A 3 / B 2; test quality 2 both.
- Probe documents and numbers differ throughout and do not conflict (the plate's numeric 1047 appears in both).

Coding judgements:
4. B7 (inherited reds without a tracker row) fits none of the four classes; coded doc-or-claim-only. It is MINOR, so the tally is unaffected.
5. X12 has a code half (uncounted, uncapped warm caches) and a claim half (the 25 MB figure); coded code. Bilateral, so the tally is unaffected.
6. A3 (`record_node` write lock) and B10 (`publish_form` write lock) are both drive-table lock costs at different sites; coded as two unilaterals, not one bilateral.
7. A5 is borne on by B's served-leaf probe ("receipt plus reused records equals the fresh receipt"), which B reports as held, not as a defect.
8. A1 may lie inside what B calls the module header; B does not quote it, so it is coded A only.

Not coded:
- Both reviews note that a failed upstream's record is not read and its readers are poisoned anyway; both treat it as benign.
- Each review's lists of what it did not run (A: the ε suites, the tour, memory and lock measurements; B: `demos/wild`, an assertions-off build, the rows' "red on the base").
- Author-reported deviations each review relays.
