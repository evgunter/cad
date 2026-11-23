# Correspondence coding — PR 7394, reviews A and B (frozen head 539a76e190f)

Source: `A.md` and `B.md` only. No code, branch or other file was opened.

## 1. Verdicts as written

- **A: `NOT-MERGEABLE-AS-IS`** — "One MAJOR, and it is text plus a tracker row, not code."
- **B: `APPROVE-WITH-FIXES`** — "No MAJOR."

## 2. Bilateral headline

1. **The drive-memo fixture's causal story is false, and the PR silently greens a row that was already red on main and is held by SCALAR's open row, uncited** (`m10_sym_drive_memo_interval.rs`, `RACING` doc; PR body). A leads with it as its only MAJOR (M1); B leads with it as MINOR 1 and fix (1). Both measured the same thing: certification is 1 / 1 / 2 / 8 at base, at base-without-the-walk and at head; only `frozen` moves. Both agree the 8/4 to 8/3 budget move is itself justified and is not a fixture changed to hide a decision. **Severity diverges: MAJOR vs MINOR.**
2. **The enclosure check is not at "the one site"**: the deferring session's proven Zeros and `Sym<Dual<Interval>>` are returned proven, unchecked (A m1; B MINOR 2 + NOTE 6).

## 3. Coding table

Severity key: MAJOR / MINOR / NOTE / claim (a per-claim remark) / style. "none stated" means the reviewer mentioned the fact but stated no defect.
Evidence key: EXEC = the report marks it demonstrated by execution (probe, mutant, measured value, CI row); INSP = by inspection or argued; unstated = the report does not say.

### 3a. BILATERAL (15)

| # | Underlying defect | A (where, severity) | B (where, severity) | Class | A evidence | B evidence | Notes |
|---|---|---|---|---|---|---|---|
| BI-1 | Drive-memo fixture: false causal story ("check's freezes published to the drive memo"); row already red at merge base; SCALAR's row answered but not cited; body's "0→1, 1→2, 4→8" not against the real base | M1, **MAJOR** | MINOR 1, **MINOR** (+ style bullet 3, claim 12) | doc-or-claim-only | EXEC ("DEMONSTRATED, sure"; base built, walk re-added behind a switch) | EXEC ("DEMONSTRATED, sure"; probe at base, base+`RX_S1`, head) | Severity divergence. A itself says "text plus a tracker row, not code". |
| BI-2 | Deferred discharge (`sym/deferred.rs` `discharged` / `discharge_retried`) returns a proven Zero with no enclosure check; body's "one site" claim true of `decide_sign` only | m1 (first bullet), MINOR | MINOR 2, MINOR (+ style bullet 8, claim 3, claim 16, rubric "no row pins the deferred path") | code | EXEC (p1) | EXEC (probe `a_deferred_discharge_proves_the_collision_with_no_check`) | Reach disagreement: A says shipped deferring sessions are `Sym<f64>`, so no shipped lane reaches it; B names production readers (`coincide/replay.rs` 4 sites, `profile` `seg.rs:853`). |
| BI-3 | `Sym<Dual<Interval>>`: exact witness but `enclosure_probe` not forwarded, check skips silently, breach returned proven | m1 (second bullet), MINOR | NOTE 6, NOTE | code | EXEC (p12) | EXEC (probe `at_sym_dual_interval_the_check_reads_no_enclosure`) | Severity divergence MINOR vs NOTE. A folds it into the same finding as BI-2; B splits. |
| BI-4 | `OPAQUE_SEQ` restarts at session start: an opaque minted outside a session collides with the session's first; false proven Zero (pre-existing, SYM's) | N1, NOTE | MINOR 3(b), MINOR | code | EXEC (p6, p7, p8) | EXEC (probe `an_opaque_minted_outside_the_session_collides_with_the_sessions_first`) | Severity divergence NOTE vs MINOR. Both say "to file on SYM". |
| BI-5 | `Sym::value` field doc ("the number is its expression's evaluation") is not given by privacy: `Sym::param` binds one symbol at two values | claim 16 remark, claim | MINOR 3, MINOR (+ style bullet 2, claim 17) | doc-or-claim-only | INSP (claim 16 listed "read only") | EXEC ("DEMONSTRATED, sure") | B adds route (c), `param_over`'s bracket not tied to its value; A probed the same shape in passing (claim 3, p9: "did not trip it") without calling it a defect, so it is not coded unilateral. |
| BI-6 | The cuts are unscheduled: five documents only, no `Sym<f64>` corpus lane, drives never reached (and that is not disclosed where it should be), no `work/` file schedules any of it | m3 (fourth bullet), MINOR (+ claim 9, end-to-end) | MINOR 4, MINOR (+ claim 9, end-to-end) | test-gap | INSP ("DEMONSTRATED by reading") | INSP ("By inspection, sure") | Split: this is the shared part of A's broader m3; the remainder is UA-2..UA-5. |
| BI-7 | The 20k retry cap withholds attempts silently; `Report` carries no count of it | N3, NOTE (+ claim 7) | MINOR 4 sub-point, MINOR (+ style bullet 7, claim 7) | test-gap | unstated ("I did not measure") | INSP | Severity divergence NOTE vs MINOR. |
| BI-8 | `contradiction::check` restores the receipt/counts but not the memos; a later decision in the same session sees the instrument's walks | N2, NOTE | NOTE 8, NOTE | code | EXEC (p3: `frozen` 0 where 2) | INSP ("By inspection, likely") | Same severity; evidence differs. |
| BI-9 | PR body's baseline table cites merge base `7012fc40`, not `0a62bae4` | N4, NOTE | NOTE 9, NOTE | doc-or-claim-only | unstated | unstated ("Sure") | |
| BI-10 | The free exception is as wide as "the DAG reaches a `Free`", not "this is a `free_identity`" | claim 4 remark, claim — **none stated** ("Held, by inspection") | NOTE 7 (first direction), NOTE; claim 4 "Broken in both directions" | code | INSP | EXEC (probe `a_margin_that_contains_a_free_identity_escapes_the_check`) | Per-claim verdict diverges (Held vs Broken). Both say unreachable from the one caller. Split: the converse direction is UB-3. |
| BI-11 | The gate's one red, `geom-brep` `recourse_roster` (three `ssi/enclose.rs` cone sites), is inherited from main | claim 17 observation, claim — **none stated** ("Held") | MINOR 5, MINOR | doc-or-claim-only | EXEC (gate logs read; row green on the frozen head locally) | EXEC for the red (CI row); "Sure" | Shared part is the observation only. Split: "no row holds it" is UB-1. A says the sites "exist on main and not on the head"; B adds "main is red too". |
| BI-12 | The instrument's `absent` under-reports (weak match: same site, different trigger) | claim 7 gap ("absent is the root node only"), claim (+ style bullet 8) | claim 7 remark + MINOR 3(b) sub-point ("collided node read as present, not absent"), claim | test-gap | unstated | EXEC (probe `the_collision_is_present_to_the_instrument`) | Coded bilateral on the shared site; if read strictly as two triggers, it is one UNILATERAL-A claim and one UNILATERAL-B claim. Neither is MAJOR. |
| BI-13 | `decide_6`: `order > 0` is a weaker non-vacuity floor than `settled > 0` | claim 13 remark, claim | style bullet 6 (+ claim 13), style | test-gap | INSP | INSP ("likely") | |
| BI-14 | `sym.rs` `# Cost` keeps figures it then disclaims as overstated | style bullet 2 (part), style | style bullet 4, style (+ rubric) | doc-or-claim-only | INSP | INSP ("likely") | Split from A's broader "history in comments" bullet; remainder is UA-13. |
| BI-15 | `Path::Algebra` under `expect(dead_code)`: product type surface shaped for a test instrument | style bullet 4, style | style bullet 5, style | code | INSP ("unsure") | INSP ("unsure") | A also cites `discharge_in` returning the rung; B also cites the widened `Rung` re-export and the child module reaching privates. |

### 3b. UNILATERAL-A (17)

| # | Underlying defect | Where, severity | Class | Evidence | Notes |
|---|---|---|---|---|---|
| UA-1 | No row pins the check at the registry `Door` rung, nor the instrument's door past the first attempt | m2, MINOR | test-gap | EXEC (two mutants survive all PR rows) | B's claim 3 and 8 report door and retry mutants red; B ran different mutants and raised no gap. |
| UA-2 | `sweep` `mass_props_are_thread_count_invariant`: `dispute_coverage`, its floor and guard rows deleted; nothing runs the instrument over `sym_arc_loft` / `sym_thin_strip`; two closed `work/nurbs` rows lose their fix | m3 (first bullet), MINOR | test-gap | INSP ("DEMONSTRATED by reading") | Remainder of m3. |
| UA-3 | `sym11_far_placement_rows` asserts neither `checked > 0` nor `absent == 0` | m3 (second bullet), MINOR | test-gap | INSP (by reading) | Remainder of m3. |
| UA-4 | `contradiction_corpus`: `checked > 0` asserted on the sum over five documents, not per document | m3 (third bullet), MINOR | test-gap | INSP (by reading) | Remainder of m3. |
| UA-5 | `sym11_witness_kind_rows` frozen-check, opaque/hull, poisoned and early-unfreeze rows deleted with no successor; frozen-walk-reads-as-agreement pinned nowhere | m3 (fourth bullet, tail), MINOR (+ style bullet 8) | test-gap | INSP (by reading) | Remainder of m3. |
| UA-6 | The §5 sweep missed three debug-only deciding sites (`topo/src/held.rs`, `boolean/shell_witness.rs`, `boolean/insert.rs`); body says one | m4, MINOR (+ claim 2, claim 5) | code (inherited, outside the diff; the PR-attributable part is the body's count) | INSP ("BY-INSPECTION") | B claim 5: "Held". |
| UA-7 | `predicate::tests::a_proven_zero_is_offered_no_tolerance` can no longer fail on the proof at the import door; its new comment is false for `free_identity` zeros | m5, MINOR | test-gap | EXEC (probe prints both sentences) | B claim 13: "still pins the recourse text". |
| UA-8 | Seven doc passages describe the removed check as live (`ladder`, `register_equal`, `rational.rs`, `profile.rs`, `real.rs` `Witness`, `with_replay_session`, rule-E comment) | m6, MINOR | doc-or-claim-only | INSP | B claim 17: docs hold except the `value` and race docs. |
| UA-9 | The two `should_panic` rows are red in a build without debug assertions; no gate on them | N5, NOTE (+ style bullet 6) | test-gap | EXEC (run: 3 reds) | |
| UA-10 | `sym instrument` CI job keys on `crates/geom-core/src/sym*` only | N6, NOTE | test-gap | unstated | A: "As ratified". |
| UA-11 | Decisions an `aside` discarded are not in the instrument's record | claim 7 gap, claim | test-gap | unstated | |
| UA-12 | `a-retrys-zero…` row closure is partial: only bit-width retries are planted | claim 16 remark, claim | test-gap | unstated | |
| UA-13 | History in comments on the re-cut pins ("Moved when…", "stopped asking") across five test files | style bullet 2, style | doc-or-claim-only | INSP ("sure") | |
| UA-14 | `contradiction.rs` `RETRY_FORMS = 20_000` shadows `sym.rs` `RETRY_FORMS = 200_000` | style bullet 1, style | code | INSP ("sure") | |
| UA-15 | `contradiction_corpus.rs` keeps a `lane` label, `Build` fn type and `opts_of` closure for one caller | style bullet 3, style | code | INSP ("likely") | |
| UA-16 | `Session::forms_door`'s doc justifies the third memo by a distinction only the instrument reads | style bullet 5, style | doc-or-claim-only | INSP ("likely") | |
| UA-17 | `mass_props…` `expected` still lists two lines under "Pinned, unexplained" while the body says they are explained | style bullet 7, style | doc-or-claim-only | INSP ("likely") | |

### 3c. UNILATERAL-B (4)

| # | Underlying defect | Where, severity | Class | Evidence | Notes |
|---|---|---|---|---|---|
| UB-1 | The inherited gate red is held by no `work/` row; `gate ok` stays red until main is fixed | MINOR 5 (remainder), MINOR | doc-or-claim-only (tracker) | INSP for the missing row; the red itself is a CI row; report says only "Sure" | Remainder of BI-11. |
| UB-2 | Nightly coverage of the cut lanes rests on a full-suite build the PR body says dies at `pncad-py` | MINOR 4 sub-point, MINOR | test-gap | INSP (from the PR body) | Remainder of BI-6. A's N6 leans on the nightly without this caveat. |
| UB-3 | Converse of the free exception: an identity closure that never calls `free` fires the check | NOTE 7 (second direction), NOTE | code | EXEC (probe `a_free_identity_that_never_mints_its_variable_is_not_recognised`) | Remainder of BI-10. Unreachable from the one caller. |
| UB-4 | `sym.rs` `reaches_free` duplicates `reaches_opaque`'s closure: a fourth DAG walker, `std::HashSet` where siblings use `IdSet` | style bullet 1, style | code | INSP ("sure") | |

## 4. Counts

Split as coded above (each distinct site is a row):

| | MAJOR | MINOR | NOTE / claim / style | Total |
|---|---|---|---|---|
| BILATERAL, by A's severity | 1 | 3 | 11 | 15 |
| BILATERAL, by B's severity | 0 | 7 | 8 | 15 |
| UNILATERAL-A | 0 | 8 | 9 | 17 |
| UNILATERAL-B | 0 | 2 | 2 | 4 |

- Severity divergences among the bilateral rows: BI-1 (MAJOR / MINOR); BI-3 (MINOR / NOTE); BI-4, BI-7 (NOTE / MINOR); BI-5, BI-11 (claim remark / MINOR); BI-10 (claim "Held" / NOTE "Broken").
- If the four remainders of A's m3 (UA-2..UA-5) are collapsed into one "coverage removed" finding, UNILATERAL-A is 5 MINOR + 9 lower = 14.
- If BI-12 is read strictly as two different triggers, BILATERAL is 14 and each side gains one claim-level unilateral.

## 5. Tally candidates

**None.**

There is exactly one MAJOR in either report, A's M1 (BI-1). It is not a candidate:

- (a) fails: it is BILATERAL, not unilateral. B raised the same defect, with the same measurements, as MINOR 1.
- (b) fails: defect class is doc-or-claim-only. A's own verdict line says "text plus a tracker row, not code".
- (c) holds (counted once), (d) holds (both demonstrated by execution).

Near-misses, each failing only on severity (condition (a), not raised as MAJOR):

| Finding | Unilateral | Class | Evidence | Failing condition |
|---|---|---|---|---|
| UA-1 (door rung and retry door unpinned) | A | test-gap | EXEC (surviving mutants) | (a): MINOR |
| UA-7 (predicate row cannot fail on the proof) | A | test-gap | EXEC | (a): MINOR |
| UA-9 (`should_panic` rows red without debug assertions) | A | test-gap | EXEC | (a): NOTE |
| UB-3 (free exception, converse direction) | B | code | EXEC | (a): NOTE |

Other unilateral MINORs fail more than one condition: UA-2..UA-5 and UA-6 fail (a) and (d) (by reading or inspection); UA-8 fails (a), (b) and (d); UB-1 fails (a), (b) and (d); UB-2 fails (a) and (d).

## 6. Observations recorded but not coded as findings

Neither reviewer states a defect in the PR for these.

- **Per-claim verdict divergences**: claim 4 (A "Held" / B "Broken in both directions", BI-10); claim 15 (A reproduced the direction with four timings / B "Partly", two timings). Claims 3 and 12 agree (partly broken; inference refuted).
- **Claim 1**: A notes the walk-count profile row needs `sym-profile-testing`, so under `-p geom-core` alone nothing of the PR's pins the removal. B: held by inspection.
- **Claim 10**: A notes a consumer pinning `serialize()` loses the token with no replacement when a count was non-zero; B notes Rust consumers naming the four fields break at compile time and "the serialized text does not move". Both "Held"; the two remarks about the serialized text do not agree.
- **Claim 11**: both find the same two `revalue_corpus` `apart_to_its_parting` rows red on both sides. A additionally reports `e2e_channel_slider_over_an_epsilon_box` and three `sweep` reds as inherited and filed; B did not run `sweep` or `topo`, and reports `the_forms_the_plates_walks_build_are_pinned` going red to green.
- **Claim 14**: A held "by arithmetic, not by a trace"; B demonstrated bit-for-bit with `RX_S1`.
- **Instrument job cost**: both report about 14 to 15 minutes of build for about 6 s of tests.
- **Rubric**: A 4 / 3 / 2; B 4 / 4 / 3 (idiom, test quality, doc honesty).
