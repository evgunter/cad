# Correspondence coding: PR 7723, frozen head `3a2910cb29`, reviews A and B

Source: `A.md` and `B.md` only. Evidence marks are as each review writes them:
EXEC = the review says DEMONSTRATED / ran a probe, mutant, gate or measurement;
INSP = BY-INSPECTION or argued; unstated = no mark (a bare "sure/likely/unsure"
confidence word is not an evidence mark and is given in brackets).

## 1. Verdicts as written

| | Verdict | MAJOR | MINOR | NOTE | Other |
|---|---|---|---|---|---|
| A | APPROVE-WITH-FIXES | 1 (M1) | 4 (m1-m4) | 3 (n1-n3) | gate red reported as inherited on both rows ("both already filed"); 12 style lines; rubric 3 / 3 / 2 |
| B | NOT-MERGEABLE-AS-IS | 1 (MAJOR-1) | 5 (MINOR-1..5) | 4 (NOTE-1..4) | verdict also rests on the red gate and on `mergeable_state: dirty`; 13 style lines; rubric 3 / 3 / 3 |

**The verdicts differ for the same MAJOR.** Both find exactly one MAJOR and it is the same defect (X1), and both call the fix small (A: "The fix is small"; B: "one commit ... the spelling before it was closed"). A approves with that fix; B refuses, adding two reasons A does not count against the PR: the red gate (A: inherited, so not the unit's) and the conflict with main (A never mentions it).

Both agree the unit otherwise does what it set out to do (A: "The rest of the change does what it claims"; B: "Everything the unit set out to do held on every probe").

## 2. Bilateral headline

1. **`FinishedBody::gathered` is a public trait method that returns a finished body for any body it is handed** (no tier 1/2, no session box, no check that the aggregate is the graft of the parts), and two `unreachable!` are reachable through it on caller-writable input. Breaks `[unrepresentable]` / `[failure.taxonomy]`; falsifies `AtRestBody`'s doc and a premise of the PR body; claim 3 "Broken" and claim 8 "Partly" in both. `gather_finished` itself is sound for its one caller in both. A M1; B MAJOR-1. Both EXEC by external probes.
2. **The between-members row `carrying_the_members_verdicts_does_not_certify_them_apart` cannot show an overlap was found**: it asserts only "not `Ok(empty)`" at `Interval`, where the census reads undecidable; any `Err` passes. A m1; B MINOR-1. Both EXEC (both measured the census reading; both wrote an f64 reading that does tell the cases apart).
3. **`KeptSign::gathered` is a twin of `assembled` without its checks** (band only by `debug_assert`, no tol / lane / partition check, order assumed), and **docs the change made false were left** (`framed.rs` header, DESIGN `[one-way.places]`' "and the `⊔` graft"; both read the DESIGN re-wording as owed with the change, not a new decision, and leave sign-off to the orchestrator). A m2 + m3; B inside MAJOR-1 + MINOR-4.

## 3. Coding table

### 3a. BILATERAL (14)

| # | Underlying defect | A: where / severity | B: where / severity | Class | Evidence A | Evidence B | Divergence |
|---|---|---|---|---|---|---|---|
| X1 | `FinishedBody::gathered` (public) finishes any body with any parts and keys, nothing checked; the two `unreachable!` (`KeptSign::gathered` on no part, `rekeyed` on foreign keys) are input-reachable. Deduped here: `AtRestBody`'s doc "its one constructor is ... finish" and the PR body's premise are false because of it | M1 / MAJOR; claims 3, 8; SILENT list | MAJOR-1 / MAJOR; claims 3, 8; SILENT list; Style "fresh instance"; the `AtRestBody` doc also listed under MINOR-4 | contract-API | EXEC (three probes: forged 2 m brick with a 1 cm brick's certificate; both panics reproduced) | EXEC (two probes: unfinishable far brick returned finished; unrelated body taken as a Boolean operand); INSP for the two `unreachable!` | panics: A EXEC, B INSP. B names the causing commit (`78985a46e2`) and the earlier closed signature; A does not |
| X2 | `KeptSign::gathered` near-copies `assembled` and drops its checks: tol and lane unchecked, no partition / coverage check, arena order assumed | m2 / MINOR; claim 9; Style Q1 | bullet inside MAJOR-1; claim 9; Style (`props.rs:2359` vs `:2078`) | code | INSP [sure]; plus EXEC observation (dropped-runs mutant still returns an `AtRestBody` with no runs for some faces) | INSP ("Also by inspection") [sure in Style] | severity: A MINOR of its own, B inside the MAJOR |
| X3 | The between-members row does not read an overlap (passes on undecidable and on any `Err`). Deduped here: the row's doc says the census "still finds them overlapping" (B SILENT list) | m1 / MINOR; claims 7, 10; Style Q3 | MINOR-1; claim 7; SILENT list; rubric | test-gap | EXEC (measured `CensusUndecidable{MayOverlap}`; f64 census mutant leaves the row green; own f64 row red under it) | EXEC (measured non-empty for one place, overlapping AND apart; f64 readings `SameSideInterference` / `[]`) | B adds: the row also passes for copies that are apart |
| X4 | `eval/framed.rs` module header stale, two sites ("through `graft_into`"; "its finish asks of the whole") | m3 / MINOR; Style Q5 | MINOR-4 | doc-or-claim-only | INSP [sure] | INSP [sure] | none |
| X5 | DESIGN `[one-way.places]`: "Those doors are the sweep doors ... and the `⊔` graft" no longer true of a `⊔` of finished members; not re-worded | m3 / MINOR [unsure on sign-off]; claim 14; SILENT list | MINOR-4 [likely on "re-wording, not a new decision"]; SILENT list | doc-or-claim-only | INSP | INSP | B adds "this narrows which doors a ratified exception names"; both leave sign-off to the orchestrator |
| X6 | The PR body's inherited-red list omits the gate's second red (`sweep a_cylinder_wall_through_a_prism_corner...` at ε 1e-12) | m4 / MINOR [sure]; Gate; SILENT list | NOTE-1; SILENT list; verdict | doc-or-claim-only | EXEC (re-ran on base `f21ca495ca` at 1e-12: red there too; names the filed row) | EXEC for the red (job logs); INSP for "the unit does not reach it"; "Not reproduced locally" | severity: A MINOR, B NOTE. A proves inherited by a run and finds the tracker row; B argues it and says no row is cited |
| X7 | `Framed::gathered` maps `GatherError::Aggregate` to `(0, GraftIn::Moved)`: a tier-2 refusal reads as "member 0 past the session box". Both also hold `Aggregate` unreachable by argument | n3 / NOTE; Style Q1 [likely]; claim 8 | MINOR-3; claim 8 | code | unstated | INSP [likely] | severity: A NOTE, B MINOR |
| X8 | A gather of no member is reported in-band as `GatherError::Graft { member: 0, JoinDesync(...) }` | n3 / NOTE; Style Q7 [likely] | MINOR-3 (last sentence); Style `combine.rs:405` [sure] | contract-API | unstated | INSP | severity: A NOTE, B MINOR. Reachability differs, see 5.6 |
| X9 | The row that stands for the dropped debug re-derivation (`at_f64_a_carried_gather_is_what_a_finish_derives`) is one fixture at f64 comparing a volume; a check would have run on every gather | n1 / NOTE; Style Q3, Q6 [likely] | MINOR-5 (4th bullet); Style Q6 [likely] | test-gap | EXEC for the comparison probes; the weakness argued | unstated (no mutant named for this bullet); own probes "widen it ... and held" | severity: A NOTE, B MINOR. A alone adds that the row's tolerance (1e-15 absolute on 2e-6) is loose |
| X10 | Carried and derived volumes are not bit-identical at f64 (a few ulp) | n1 / NOTE | NOTE-2 | code | EXEC (2.0000000000000008e-6 vs 2e-6) | EXEC (1.5999999999999999 vs 1.6000000000000003; cause given: carried run keeps the member's flux) | none; both treat it as benign |
| X11 | The carry's whole soundness premise (every tier-3 check is local to one entity or solid) lives in `gather_finished`'s doc comment and is guarded by nothing but one row | Style Q2/Q4 [likely] | Style `combine.rs:416-441` [likely] | test-gap | unstated | unstated | A spells the consequence (a future two-solid tier-3 check is skipped silently) |
| X12 | `Gathered` (a `Deref` enum) is matched at three call sites (wire, distribute, product), each finishing the `Grafted` arm itself | Style Q7 [likely] | Style (`wire.rs:5356`, `distribute.rs:303`, `product.rs:1380`) [likely]; `Deref for Gathered` [unsure]; end-to-end | code | unstated | unstated | A's end-to-end also praises the same collapse ("each collapse to one call") |
| X13 | The "leaf is finished" question has a second spelling: the sweeps row reads `<Interval>::TAKE_FINISHES` directly; the const and the leaf type agree only by construction / by hand | claim 5 remark [likely, style] | claim 5 ("Held, with reservations") | code | unstated | unstated | B lists two more spellings (`MOVE_LOGS_ITS_ASK`, `HeldKind::move_logs_its_ask`). See 5.3 for the contradiction inside this claim |
| X14 | `git merge-base origin/main HEAD` is not the unit's base; the unit is `f21ca495ca..3a2910cb29`, 23 files | preamble ("a correction to the brief") / no label | NOTE-3 | doc-or-claim-only (the brief; process) | EXEC | EXEC | the explanation differs, see 5.4 |

### 3b. UNILATERAL, A only (7)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| A1 | `editor-core/src/product.rs` module doc lines 37-47 stale ("the aggregate is finished at its end"; a refusal class that no longer happens) | m3 / MINOR; Style Q5 | doc-or-claim-only | INSP [sure] |
| A2 | `eval/wire.rs:5300-5302` comment stale (each body grafted through `graft_into`) | m3 / MINOR | doc-or-claim-only | INSP [sure] |
| A3 | Past the attributed stop (PR 7716 merged), `demo-tour certified` panics at `chaintol.rs:453`: a pin's export at `Sym<Interval>` refuses `SliverDihedral` because a leaf's move re-finishes; no row schedules it (`delete-the-box-scalar-sweep-leaf-switch.md` line 32 covers f64 only). "Outside the unit" | n2 / NOTE; Style Q6 [likely]; claim 11 | code (outside the unit) + tracker gap | EXEC |
| A4 | Three nearly parallel refusal enums (`PlacedGraftError`, `GatherError`, `GraftIn`); the translation between them is where X7 arises | Style Q1 [likely] | code | unstated |
| A5 | topo `a_gather_logs_the_same_verdicts_at_every_lane` compares f64 against a zero-width `Interval` only, not a boxed turn | Style Q3 [likely] | test-gap | unstated |
| A6 | `Held::gathers_finished` reads a crate-global const inside a method generic over `T` | Style Q7 [unsure] | code | unstated |
| A7 | PR 7492 textually conflicts with this PR's rewrite of the same `product.rs` lines | claim 12 remark | code (integration) | unstated (read only) |

### 3c. UNILATERAL, B only (21)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| B1 | The PR conflicts with main (`mergeable_state: dirty`) | verdict reason / no label | code (integration) | unstated |
| B2 | A poisoned image is carried as finished: `RigidMotion::map_scalar` is public, `identity().map_scalar(\|x\| -x)` is a point inversion typed as a rigid motion, the gather carries it (volume 2.0) where the derived finish refuses `PoisonedSurfaceDatum`; before the unit the aggregate's finish refused it. "Not this unit's ground: file" | MINOR-2; claim 3 second half; end-to-end | code / contract-API | EXEC [sure for the demonstration, unsure a legitimate document reaches it] |
| B3 | `parts.len() != keys.len()` is silently zip-truncated in the trait method | bullet inside MAJOR-1 | code | INSP |
| B4 | The dispatch doc says the gather "derives only tiers 1 and 2"; at `AtRestBody` it derives nothing | parenthesis inside MAJOR-1 | doc-or-claim-only | unstated |
| B5 | A leaf's `topo::take` refusal becomes `GraftIn::Moved` -> `NodeErrorKind::NotFinished`, where `Held::finished`'s doc says `UnfinishedLeaf` (reachable once the const is true) | MINOR-3 | code | INSP [likely] |
| B6 | `props.rs` `AtRestPolicy::leaf`'s doc and `validate.rs`' F34 paragraph still list "the `⊔` graft" among the leaf doors | MINOR-4 | doc-or-claim-only | INSP [sure] |
| B7 | `gather_finished`'s `# Errors` omits `Aggregate` | MINOR-4 | doc-or-claim-only | INSP [sure] |
| B8 | No editor-core row reads a carried certificate: all four editor rows stay green under the dropped-runs mutant | MINOR-5 | test-gap | EXEC (mutant) |
| B9 | No row drives `distribute::assembled` (the third caller) through the carry at a box scalar | MINOR-5 | test-gap | unstated [likely] |
| B10 | `a_gather_under_a_symbolic_turn_finishes_and_measures_its_members_volume` asserts containment of 2e-6 with no width bound | MINOR-5 | test-gap | unstated |
| B11 | `wire_assemble` now runs every graft before any per-operand carry, so a document with two refusals may report a different first one | NOTE-4 | code | INSP |
| B12 | Name collision: four functions `gathered`, enum `Gathered`, `gather_finished`, `gathers_finished`, `Gathered::Grafted` beside a `Grafted` alias and `gather_sources` | Style [sure] | code | unstated |
| B13 | `framed.rs:616-621` restates `Held::finished`, "the evaluator's one taker door", sixty lines above | Style [sure] | code | unstated |
| B14 | `LEAVES_ARE_FINISHED`: its doc says "read off the switch's own types" but it reads two consts; the name is always true at f64; one sentence tangled | Style [sure]; claim 5 | doc-or-claim-only | unstated |
| B15 | `product_finishes_once.rs:107` pins the source text `T::finish(*aggregate` (a row that follows spelling) | Style [sure] | test-gap | unstated |
| B16 | `a_gather_carries_its_finished_members_verdicts.rs:114-143`: a generic `at<T>` with a label, called once at f64 | Style [likely] | code (test code) | unstated |
| B17 | `Framed::gathered`'s error is a `(usize, GraftIn)` tuple; two of three callers drop the index | Style [likely] | contract-API | unstated |
| B18 | The census's undecided readings over a turned box at plain `Interval` are disclosed and have no row filed | claim 7; Style Q6 [likely] | doc-or-claim-only (tracker) | unstated |
| B19 | The author's differential was taken two commits before the head (`78985a46e2`) | Style Q6 [likely] | doc-or-claim-only | unstated |
| B20 | `..first.clone()` clones the first part's runs only to discard them | claim 9 remark | code | unstated |
| B21 | The filed n-ary row "quotes the call loosely" | claim 13 remark | doc-or-claim-only | unstated |

## 4. Tally candidates

**None.** Each review raises exactly one MAJOR and it is the same defect (X1), so criterion (a) is met by nothing at the level of a whole finding. The sub-items that sit inside one review's MAJOR and are absent from the other all fail a further criterion.

Near-misses, and the criterion each fails:
- **B2** (a point inversion through `RigidMotion::map_scalar` reaches the gather and is carried as finished): unilateral, code / contract-API, distinct, EXEC. Fails **(a)**: raised as MINOR. It is the strongest near-miss, and it contradicts a fact A states in its own MAJOR (5.1).
- **B8** (no editor-core row reads a carried certificate): unilateral, test-gap, distinct, EXEC by mutant. Fails **(a)**: MINOR.
- **B3** (zip truncation of `parts` against `keys`): unilateral, code, inside B's MAJOR-1. Fails **(d)**: by inspection; and **(c)** is doubtful, being one more symptom of X1's unchecked door.
- **B4** (dispatch doc "derives only tiers 1 and 2"): inside B's MAJOR-1. Fails **(b)** (doc-only) and **(d)**.
- **A3** (the `chaintol.rs:453` wall past the P0): unilateral, code, EXEC. Fails **(a)**: NOTE, and A places it outside the unit.
- **B1** (PR dirty against main): unilateral and a stated reason for B's verdict. Fails **(a)**: never labelled MAJOR; class is a poor fit for (b); evidence unstated.
- **X2** (the `assembled` twin): inside B's MAJOR but A raises it (m2), so not unilateral.

## 5. Notes

Factual contradictions:
1. **Can a non-rigid map reach the gather?** A (M1, claim 3): "`RigidMotion` has private fields, and its third axis is the cross product of the other two (`with_signed_axes` keeps that), so a mirror or a non-rigid map cannot reach the gather"; "The types do protect `Placed` and `RigidMotion`." B (MINOR-2, claim 3): "`RigidMotion::map_scalar` is public ... `RigidMotion::<f64>::identity().map_scalar(|x| -x)` is the point inversion typed as a rigid motion"; probe `a_point_inversion_typed_as_a_rigid_motion_reaches_the_gather` DEMONSTRATED. A argues by inspection; B's run contradicts it.
2. **PR 7492 against this head, `product.rs`.** A (claim 12): "it wraps `gather_sources` into `gather_into` under `sym::apart`, which conflicts with this PR's rewrite of the same lines." B (claim 12): "PR 7492's `product.rs` merges with this head with no conflict there."
3. **What `LEAVES_ARE_FINISHED` reads.** A (claim 5): "Held. The const is read off the switch's own types." B (claim 5, Style): "The const is read off `TAKE_FINISHES`, a sibling const of `type Leaf`, not off the type"; the doc "says 'read off the switch's own types'; it reads two consts."
4. **Why `git merge-base origin/main HEAD` is wrong.** A: main's history "split off after 38832098", `f21ca495ca` and the merges #7722, #7713, #7651, #7711, #7720 are "no longer an ancestor of origin/main", "those merges look like they have dropped off main". B (NOTE-3): "In my clone `origin/main` was stale." One observation, two incompatible explanations (a claim about main itself against a stale local ref). Both use the same base.
5. **The chain table on the probe tree with this head and without PR 7716 (apparent).** A (claim 11): "the 1- and 2-link rows certify at `Interval` and `Sym<Interval>`. With 7716 also merged, all four link counts certify." B (claim 11, PR 7716 "not looked at"): "the table certifies at `Interval` and `Sym<Interval>` at 1, 2, 3 and 4 links." Reconcilable only if A ran just two rows without 7716; A's wording reads as a contrast.
6. **Is `GatherError::Graft` reachable (apparent).** A (claim 8, n3): "`Graft` and `Aggregate` are not argued reachable"; the empty-list path "is never taken" by its only caller. B (claim 8): "`Moved` and `Graft` (none) reachable: my row `the_gathers_typed_refusals_are_reachable`." Reconcilable if B's row calls `gather_finished` directly.

Differences that are not contradictions:
- The verdict (section 1). Doc-honesty rubric: A 2, B 3.
- X6: A says the unlisted red is filed (names the row) and re-ran it on the base; B says "has no row cited" (of the PR) and did not reproduce it.
- The census-empty mutant against the overlap row: A green (mutant applied at f64 only, "where the author's row never reads"); B red (claims 7, 10). Different lanes.
- Mutants re-run: A five ("four claims re-run, all confirmed", the fifth at f64 only); B six, "each red where the author says".
- Claim 6: A did not run the whole-suite differential ("rests on the author's account"); B ran editor-core's whole suite with the slow set on base and head (3753 run / 17 failed against 3758 / 17, the same 17). Both re-took the corpus count (398 -> 375; per-document match).
- Claim 4 attribution of the `chaintol.rs:401` stop to the open P0: A confirms by merging PR 7716 (passes 1,1,0,0); B by the log signature (first difference at verdict 1313), 7716 not looked at.
- Claim 2 coverage: A probed a curved drum and a zero-width turn at `Interval` (carried strictly inside derived); B did not probe curved members or a boxed motion and found the unmoved `Interval` enclosures identical.
- Claim 12 semantics: A "neither PR reads what the other writes"; B "the ruling that the gather is handed its members' records stops being optional once both land".
- Claim 13: A "Accurate"; B "Accurate" with one loosely quoted call (B21).
- B's dropped-runs mutant reds "topo rows 2, 3"; A's reds "the f64 row and the Sym row". Derive-not-carry: A "both topo box rows"; B adds "editor row 1".

Coding judgements the counts or the tally are sensitive to:
7. **Folding into X1.** The `AtRestBody` doc falsity is inside A's M1 and under B's MINOR-4; coded as part of X1 (a claim falsified by the code defect). Coded apart it would be a 15th bilateral with a severity divergence. The two `unreachable!` are likewise folded (A EXEC, B INSP).
8. **B3 and B4** are B-only sub-items of the shared MAJOR. Read as mere symptoms of X1 they would not be findings at all (B-only falls to 19). Neither reading yields a candidate.
9. **B2 is the call the tally turns on.** B ranks it MINOR and places it outside the unit's ground. Had B ranked it with claim 3's break as MAJOR it would be the one candidate (unilateral, code, distinct, EXEC).
10. **Doc sites are coded one row per site**, as in the format reference: two shared sites (X4, X5), two A-only (A1, A2), two B-only rows (B6, B7). Coded as one finding "docs left false" it is one bilateral and bilateral falls to 13, A-only to 5, B-only to 19.
11. **X7 and X8 are split** (two different mappings) though each review writes them in one finding.
12. **B8 against A's m2.** A observes that under the dropped-runs mutant "only the rows that measure volume notice"; B turns the same mutant into a named gap (no editor-core row). Coded B only, since A asserts no row gap there.
13. **B18.** A mentions the `Interval` census escalation as "the census's own limit, not a hole in this unit"; B's finding is that it has no row. Coded B only.
14. **X14** is not a defect of the PR; coded because B gives it a NOTE.
15. **A's loose-tolerance remark** on the f64 row is folded into X9; coded apart it is an 8th A-only (test-gap, EXEC by measurement, NOTE).

Not coded:
- Each review's list of what it did not run or read.
- Author-reported deviations each review relays (option B, no `debug_assert`, the command not run to its end, the narrowed between-members row; B also relays no `compile_fail` row).
- B's diagnosis of the causing commit for X1 (a cause, not a further defect).
