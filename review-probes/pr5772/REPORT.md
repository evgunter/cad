# Review of PR 5772 (FLUSH, n-ary union/intersect door), frozen head ec3a48d4

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The fixes are doc and PR-body corrections plus one disclosure; none blocks.

Merge base bd36ab7b. Probes: `flush/nary-door-review-1fcbt9`. Time: about 3 h 15 m (11:46 to 15:00 UTC).

## Claims

**1. No behaviour change: holds, measured.**
- I hooked `wire_combine` the same way on base and head (`hook.py`). The hook hashes each evaluation's body payload, name tables, contacts, rows, cited inputs and carried findings, or its error.
- I ran editor-core's whole suite, default profile with the slow set, debug, on both sides. Each side ran 3013 tests: 2985 pass and the same 28 fail.
- The capture saw 462 tests and 11,527 evaluations per side (78 errors), counted in order per test. It includes `docm5_subject`'s 4 tests (69 evaluations), so the author's exclusion is closed.
- Everything is identical except the rows of `coincidence_door::an_unproven_fold_row_…`. Full-Debug dumps show only `ZeroWord` indices move there.
- 4 interval tests past 400 s were killed on both sides (e6_drive ×2, m10_3_r2 containment, m10_6 margin_thin; base also m10_3_driver planted_flip). They are not union tests.
- NOTE (sure, measured): the PR body lists 4 moved indices, but there are **8**: two rows per evaluation (17753→17747, 39723→39717, 152376→152372 and 174245→174241 are missing).
- On readers: `ZeroWord` is private and resolves only in-session (`MarginDiag::pending` → `sym::deferred::entry`, `crates/geom-core/src/predicate.rs:1786`). No golden, digest or other capture moved. The test reading it passes on both sides. Harmless (likely).

**2. The given channel is unreachable from outside: holds, compile-fail shown.**
- `forge_compile_fail.rs` tries seven routes from `verbs/tests`: `topo::GluedPair`, `topo::boolean::GluedPair`, `given_at`, `BooleanInputs{given}`, `topo::boolean::boolean_op_given`, a hand-built `GlueVerdict`, and `restated_by`.
- It gives 7 errors (E0425 ×2, E0603 ×2, E0624 ×2, E0560); the `GlueVerdict` literal is blocked by its private fields.
- `Glue::of(.., Some)` has one caller, `boolean_op_given` (`ops.rs:723`), and that has one caller, `nary.rs`. `pncad-py` and `editor-core` reach only `nary_boolean`, which judges for itself.

**3. The pass is the old pass: holds (sure, by reading and the capture).**
- `member_ranks` is strictly monotone in `VarId`, so `ranks[q] < ranks[p]` is the old `reads[q] < reads[p]`. Ties go to the listed-first as before, and the box filter and loop order are the same.
- `given_pairs` and `restated_rows` are line-for-line the old ones: first verdict wins, refusal on a relation clash.
- Judgements still get `BooleanInputs::default()`. With two members there is still one list-order step and no `given`.
- I found no member set that differs, and the capture of every order the suite runs is identical.

**4. Lineage moved: mostly.**
- `MemberLineage::step` equals the old `Fold::step` lineage loop.
- `restates` dedups by `JudgedRow`, and editor-core then dedups by published index (wire.rs ~3425). That is the old dedup, and records are identical in the capture.
- Two residual copies remain: see S1 and S2.

**5. Error precedence: MINOR (likely, by reading; not executed, since there is no naming-error injector).**
- `NaryRefusal::Step` still becomes `UnionFoldStep{member, read}` (wire.rs:3309-3320), Hull and Judgement become `Boolean`, and `VerdictsDisagree` keeps the same emission error.
- The precedence change is wider than the PR body says. On base, an intersect's judgement j's rows were named (`name_rows`, `links.judged`) before judgement j+1 ran (base wire.rs:3291-3330). So a naming error at j beat an **empty** j+1.
- On head, every boolean runs first, and an empty judgement or step returns `NaryResult::Empty`, so `wire_combine` returns `Ok(empty())` (wire.rs:3309-3325). The naming defect is never announced: a typed error became a valid-looking empty result.
- The same holds for a naming error at step k and an empty step k+1. A judgement's naming error now also loses to any step refusal, not only to a later step's.
- That is a silent discard under `[failure.taxonomy]` ("silent discard is never an answer"), unlike the refusal case, where the user at least sees a refusal.
- No corpus evaluation hits it: all 78 errors are identical.
- Fix: say so in the PR, or name the judgements before deciding Empty.

**6. Records: holds as stated.**
- Fold half: the kernel threads `records_a = body.contacts.carried()` itself. `carried()` drops citations and `recited` keeps row order, so this equals the old `acc_records.carried()`, and records are identical in the capture.
- m2 is not fixed (sure, by reading): `given_pairs` still keeps the first verdict (`nary.rs:622`), while `restated_rows` maps to every verdict's row.
- NOTE: the row's adopted `## Design` says m2 "close[s] with it" (`work/flush/a-glue-verdict-…md:89`). Its `## Fix` doesn't say it didn't. The deferral is recorded only on the CONTACTHOLD row.

**7. Verb seat and determinism: holds.**
- `verb_content_tag` is unchanged (8/9), and `VerbKind::ALL` is untouched. The only `run_members` caller is `wire_combine`, and I found no persisted verb payload.
- verbs' ci profile is green (37/37, `layer_guard` included).
- Memory, measured peak RSS (debug): `docm3_union::the_dies_union_is_the_chain_it_replaced` 143 MB base vs 144 MB head; `m4_pr2_eval::doc_param_edit…` 152 vs 153 MB.
- The door's loops are sequential. `parallel_node_map_interval::…one_and_four_threads` passed on head.

**8. Suites.**
- editor-core default profile with the slow set: covered under claim 1, same 28 reds on both sides.
- topo, sweep and verbs, ci profile at default ε, head: 5976 run, 5975 pass. The one red was my own probe directory tripping `every_suite_file_is_aggregated`; it is green after I removed it.
- At ε 1e-6 and 1e-12, ci with `--ignore-default-filter`, over a union-relevant filter across the four crates (1469 tests each):
  - 1e-6: 8 red;
  - 1e-12: 10 red;
  - the 7 `revalue_corpus` rows are known reds.
- NOTE (sure): four sweep reds the PR's ε filter never reached: `r1_lane0_e2e::the_f64_seam_answers_every_public_door` (1e-6), and at 1e-12 `bisector_nodes::slanted_bores_…pinched_to_itself`, `review_4781_r1_probes::review_4781_touch_pairs_every_op` and `review_4781_r2_probes::r2_every_same_side_touch_pose_answers_as_a_touch`. All four are **red on base** too (run there), so they are not this PR's.
- Batteries (release, ignored): all five are identical on base and head, 32,973 compared lines; only compile and timing lines differ. They go through the pair door (`boolean_op_given(None)`), so this is expected.

## Style
I exercised Q1, Q2, Q3, Q4, Q5, Q7 and Q8 (I read `nary.rs` end to end).
- S1 (sure, Q1): `topo::FaceDescent::result_face` (`nary.rs:164`) copies editor-core's `operand_key` (`emit_topo.rs:1552-1571`): the same `(a_keys, b_keys)` match and the same unreachable text. Before the PR, `FaceDescent` called `operand_key`, so the move minted a second home.
- S2 (likely, Q1): `Fold::step`'s obstacle closure (`emit_union.rs:2205-2215`) still re-derives A→lineage and B→(member, f), which is `MemberLineage::step`'s `from` (`nary.rs:119`). It is used for obstacles only, but it is the same rule twice.
- S3 (likely, Q2): `wire.rs:3366-3369`'s "The kernel carried them … in the same list order" holds a cross-crate invariant by prose. editor-core's `Carried{input:0, record}` lookup into its own `acc_records` is right only while the kernel's `records_a` is exactly that list.
- S4 (sure, Q1): `pairwise = members.len() > 2` is spelled in both `nary.rs:452` and `wire.rs:3308`, and the 2-member mapping (`Judgement{a:0,b:1}`, `nary.rs:516`) relies on editor-core knowing it.
- S5 (sure, Q5): `NaryRefusal::Judgement`'s doc ("the same in every member order", `nary.rs:315`), `nary_boolean`'s doc ("never on their order", `nary.rs:403`) and `MemberInputs::rank`'s doc are false for two members. There rank is ignored and operand A is the listed-first. The behaviour is pre-existing on main; the promise is new.
- S6 (sure, Q5): `NaryRefusal::error`'s doc says "`None` for an arity refusal", but it also returns `None` for `VerdictsDisagree` (`nary.rs:346`).
- S7 (likely, Q3): the census `no_public_door_states_a_given_pair` checks names only (`GluedPair`, `given_at`, the `given` field). Making `boolean_op_given`, `Glue::of` or `restated_by` `pub`, or adding a new mint under another name, passes it. Only the compiler's privacy rules hold the line, so the census is narrower than its doc says.
- S8 (unsure, Q7): `NaryStep::restated` is a `pub` field whose sort order `restates()`'s binary search relies on. A field and an accessor are two doors.
- S9 (unsure, Q7): the door returns every judgement's and every step's full `BooleanBody` (`nary.rs:530`). The cost is negligible on the die, but every caller pays it, including verbs callers that want only `result()`.

No glimpse of the other review lane.
