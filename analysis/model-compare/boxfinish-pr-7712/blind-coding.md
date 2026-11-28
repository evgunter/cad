# Correspondence coding: PR 7712, frozen head `dcaa04d4`, reviews A and B

Source: `A.md` and `B.md` only. Evidence marks are as each review writes them:
EXEC = the review says DEMONSTRATED / ran a probe, mutant, gate or measurement;
INSP = BY-INSPECTION or argued; unstated = no mark (a bare "sure/likely/unsure"
confidence word is not an evidence mark and is given in brackets).

## 1. Verdicts as written

| | Verdict | MAJOR | MINOR | NOTE | Other |
|---|---|---|---|---|---|
| A | APPROVE-WITH-FIXES | 0 | 8 (numbered 1-8, "ranked") | 3 (unnumbered) | gate red reported in `## Gate` and as verdict condition (a), not numbered; 5 style lines; 3 SILENT deviations; rubric 4 / 3 / 3 |
| B | APPROVE-WITH-FIXES | 0 | 5 (1-5) | 8 (6-13) | gate red reported in the gate paragraph, S10 and the SILENT list; 10 style lines (S1-S10); 2 SILENT deviations; rubric 4 / 3 / 3 |

A: "I found no soundness break ... None of them is a code defect in the refuter itself."
B: "No correctness defect found ... The fixes are test gaps (one unsound mutant survives every in-tree row), sentences the change made false, an understated cost on main, and one unscheduled deviation."

Neither review raises a MAJOR.

## 2. Bilateral headline

1. **The change slows what main runs today, and the body says the overhead was not separated from noise.** `demo-tour certified`, switch on, alternating runs, disjoint ranges, output identical line for line: A +8.5% (90.2 -> 97.9 s), B about +10% (110.2 -> 120.9 s). The cost is the point pass on identity asks that still climb. A MINOR 1; B MINOR 3. Both EXEC.
2. **"A bracket recorded or MOVED clears the memo": the moved half has no row.** A mutant that clears only on first record passes every in-tree row. A inside MINOR 3 (M10b); B MINOR 2. Both EXEC.
3. **The `gate ok` check is red on the head** at one step ("the slow set names only tests that exist", two duplicated `sweep::all join_pierce_runs_sweep` entries in `.config/nextest.toml`), inherited and not the unit's; the tour never ran in the hosted gate. Both EXEC (CI read at step level). A makes it a condition of approval; B adds that the body's list of inherited reds omits it.

Close behind: the "no verdict moves" argument rests on more than soundness (A MINOR 2, demonstrated; B NOTE 6, argued); the `frozen` receipt column moves 1,490 -> 634 while the body frames receipts as unchanged (A MINOR 5; B NOTE 7); `deferred::discharge` is a disclosed deviation held by no `work/` row (A MINOR 6; B MINOR 5).

## 3. Coding table

### 3a. BILATERAL (18)

| # | Underlying defect | A: where / severity | B: where / severity | Class | Evidence A | Evidence B | Divergence |
|---|---|---|---|---|---|---|---|
| X1 | `gate ok` red on the head: step "the slow set names only tests that exist" fails in `test (kernel)` and `test (app)` on two duplicated slow-set entries in `.config/nextest.toml`; inherited | `## Gate`, verdict condition (a) / unnumbered | gate paragraph; S10 [sure]; SILENT list / unnumbered + style | doc-or-claim-only (gate-breaking, inherited config) | EXEC | EXEC | A: fixed on main at `f3eef46091`, merge main and see green. B: the body's "Inherited reds" omits it (B-only half, folded here). Origin worded differently (see 5.5) |
| X2 | Unconditional cost on what main runs: the tour with the switch on is 8-10% slower; the body says "overhead not separated from noise" | MINOR 1; End-to-end; SILENT list | MINOR 3; claim 17; S2 (the number is not at the site); rubric | code (cost; claim half) | EXEC | EXEC (tour); single runs for the editor rows [likely] | numbers (see 5.4); A lists the unseparated cost as SILENT, B as Reported |
| X3 | "No verdict moves" rests on more than claim 1: a refuted ask leaves less session state than a failed climb (retry forms the growth guard counts), so a later decision of the leaf can be answered differently, towards more `Zero`s | MINOR 2; claim 7 "broken in its argument" | NOTE 6; claim 7 | code (order dependence; claim half) | EXEC (row `a_refutation_moves_a_later_identity_through_the_growth_guard`, cap 1); that it moves at shipped dials [unsure] | INSP (mechanism sure, reachability unsure) | severity MINOR vs NOTE; evidence EXEC vs INSP. A adds memo and budget freezes; B adds atoms minted into `Session::atoms` |
| X4 | Memo clear on a MOVED bracket is unpinned (mutant: clear only on first record / `insert(..).is_none()`) | inside MINOR 3 (M10b) | MINOR 2 | test-gap | EXEC | EXEC (and a reviewer row red under it) | none |
| X5 | `Dual` reading nothing is unpinned (mutant: `Dual` reads; no Dual row) | inside MINOR 3 (MDUAL) | NOTE 11 | test-gap | EXEC | EXEC (1,259 rows green; not run against `editor-core`) | severity: A inside MINOR, B NOTE |
| X6 | The structural question never taking the reading has no row | inside MINOR 3 (MQ survives; "nothing to pin beyond the code") | claim 13 remark ("by inspection only, no row") | test-gap | EXEC (mutant) | INSP | A runs the mutant and judges it no defect; B reports the gap without a mutant |
| X7 | Proposed E12 sentence "at an exact witness" is broader than the code: `Interval` only, `Sym<Dual<Interval>>` reads nothing | MINOR 4(d) | NOTE 9 | doc-or-claim-only | INSP [sure] | unstated [sure] | severity MINOR vs NOTE. B adds a second place: a deferring session's discharge reads nothing (see X11) |
| X8 | Proposed E12 sentence "the decision stays undecided" is inexact: a refuted in-band numeric margin returns an unproven `Ok(Zero)` | MINOR 4(c); SILENT list ("also refutes in-band numeric zeros") | NOTE 9 | doc-or-claim-only | INSP [sure] | unstated [sure] | severity MINOR vs NOTE |
| X9 | The reading runs before the registered-identity door, so a registration false at the point is no longer believed; within E12, but E12's text does not say it | MINOR 4(b) | NOTE 8 (last sentence) | contract-API | INSP [sure] | INSP [likely] | severity MINOR vs NOTE. A: the sentence should say the reading precedes the door. B: E12's registration paragraph still reads as if `x² ≡ x` over `[0.9, 1.1]` discharges |
| X10 | The `frozen` receipt column moves (1,490 -> 634) while the body frames what a refuted ask records as "the same as a failed climb" | MINOR 5; SILENT list; claim 17 | NOTE 7; claim 9 | contract-API | INSP plus measurement (reproduced) [likely] | unstated (634 reproduced in claim 14) | severity MINOR vs NOTE; whether PR 7492 keys on it (see 5.3) |
| X11 | `deferred::discharge` also climbs: a disclosed deviation ("not measured, outside the fence") with no `work/` row | MINOR 6; claim 16; style Q6 | MINOR 5; claim 16 | doc-or-claim-only (tracker; code side effect in B) | unstated [sure] | unstated [sure] | B adds: its reason is an expectation, not a measurement; a registration false at the point is proven in a deferring session and refuted in an eager one |
| X12 | `point::read` reads `sess.nodes.get`; once PR 7492 lands it must resolve through `Session::node_of`, or every node another leaf recorded declines silently | MINOR 7; claim 6 "partly" | claim 6 remark ("HELD today") | code (forward compatibility) | INSP [likely] | unstated | severity: A MINOR, B unlabelled remark. B adds: a second writer of brackets must clear `points`; `point::check` saves and restores `sess.counts` |
| X13 | `point_of` at the edges: a bracket wider than `f64::MAX` overflows `hi − lo` and gives the `hi` face for every symbol; a one-ulp bracket lands on a face. Sound; "off the faces, per symbol" false there | NOTE (1st); claim 11 | NOTE 12; claim 11 | code (claim half) | EXEC (`review_probes::the_point_rule_at_the_edges`) | unstated | evidence |
| X14 | `Decide for Sym`'s doc does not mention the refuter | NOTE (2nd) | inside MINOR 4 | doc-or-claim-only | unstated [likely] | INSP [sure] | severity NOTE vs MINOR |
| X15 | The registration evidence is a narrow sample (the tour and two corpus rows): other documents, drives, revolves, fillet tangency, tilted placements uncovered | claim 3 remark | NOTE 8 | test-gap (evidence coverage) | unstated | INSP [likely] | severity: A unlabelled, B NOTE |
| X16 | `PointReading` as a supertrait of `Decide` puts a hidden `pub fn refutes(SymId)` on every decision scalar | Style Q7 [likely] | S5 [unsure]; claim 10 ("defensible") | contract-API | unstated | unstated | A: any `T: Decide` body can call it. B: constant `false` for four of five, reaches a thread-local |
| X17 | `values_with_no_expression_decline` asserts `forms > 0`: cannot tell "declined" from "read and straddled" | Style Q3 [likely] | S6 [likely] | test-gap | unstated | unstated (cites its mutants) | none |
| X18 | Module doc "nothing outlives the leaf": the memo lives in the session; true only while a session is per leaf | Style Q5 [unsure] | S3 [likely] | doc-or-claim-only | unstated | unstated | none |

### 3b. UNILATERAL, A only (9)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| A1 | A refuted decision that renders the blocking form with the shape report installed passes every row: "the render stays behind the refutation" is unpinned (mutant MR) | inside MINOR 3 | test-gap | EXEC |
| A2 | The memo never kept passes every row: "a sub-graph several margins share is read once" is unpinned (mutant MNOMEMO) | inside MINOR 3 | test-gap | EXEC |
| A3 | A point tie read as a straddle passes every row (mutant MEQ; conservative direction) | inside MINOR 3 | test-gap | EXEC |
| A4 | The proposed E12 sentence never says the reading can decline (opaque, hull, free, unrecorded, unbracketed, lost domain) | MINOR 4(a) | doc-or-claim-only | INSP [sure] |
| A5 | The filed row's causal sentence ("because the forms they share were built by asks the reading now refutes") is stated as fact and checked by no measurement | MINOR 8 | doc-or-claim-only | unstated [likely] |
| A6 | Module docs `sym.rs:747-751` ("its DECISION path asks a form only where the numeric channel cannot answer") stale by omission | NOTE (2nd, first site) | doc-or-claim-only | unstated [likely] |
| A7 | Acceptance oddity: 8 `curve_carrier_clear` and 28 `curve_carrier_gap` decisions return `Zero` at `Sym<Interval>`; M11 says none of the four ever decide `Zero`. Unresolved | NOTE (3rd) | doc-or-claim-only (unresolved claim discrepancy) | EXEC (counted) [unsure] |
| A8 | The tie rule is written twice: `select_arms` and `Interval::select_le_zero` (`hi ≤ 0`) | Style Q1 [likely] | code | unstated |
| A9 | `PointReading for Dual`'s doc gives a scope reason ("measured on"), not a soundness reason, and reads as a claim | Style Q2 [unsure] | doc-or-claim-only | unstated |

A4 and A5 are mentioned by B and denied, not ignored (see 5.6, 5.1 and judgement 1).

### 3c. UNILATERAL, B only (13)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| B1 | The op table of the reading is unpinned: `Powi` read with `|exponent|` (an unsound reading: B's differential loses 624 `Zero`s the base proves) and `Floor` read as the identity both pass `geom-core`'s whole fast set (1,259 rows); `Atan2` and `Copysign` swaps are caught only by unrelated rows | MINOR 1 | test-gap | EXEC |
| B2 | Sentences the change made false: `Session::params` ("read by the decision read alone"), `Sym::param_over` ("the ONLY value the symbolic tier ever reads"), `sym/signed.rs` header ("the whole of how a value is read") | inside MINOR 4 | doc-or-claim-only | INSP [sure] |
| B3 | `DecisionShape::form`'s contract ("`None` otherwise") is false: a refuted undecided decision now has `None`, and `m10_10_evidence_interval.rs` lists predicates by `form.is_some()` | inside MINOR 4; SILENT list | contract-API | INSP [sure] |
| B4 | A zero-budget session still pays the point pass: `point::refutes` has no `max_terms == 0` test (`discharge_in` has one) | NOTE 10 | code (cost) | INSP [sure] |
| B5 | Rigidity registration is true over the reals only for an exactly orthonormal placement; a frame of `f64` literals unit only to rounding puts the point value a few 1e-17 off zero, and nothing guarantees the reading holds 0 | inside NOTE 8; claim 3 | code | INSP [likely], "not demonstrated" |
| B6 | Claim 2's premise is weaker than framed: for normal-form zeros no decision need have been certified; only two registrant families lean on it | NOTE 13 | doc-or-claim-only | unstated |
| B7 | The body's "refuted" column is not reproducible: the author's instrument was never pushed | claim 14 remark | doc-or-claim-only | unstated |
| B8 | No count of refutations in a receipt or the shape report | claim 17 remark; listed as a reported deviation | contract-API | unstated |
| B9 | `sym/point.rs:147-185` is `sym/revalue.rs:168-199` again, arm for arm, with no sentence at either site; `signed::enclose_deep` is a third enclosure over the same brackets | S1 [sure] | code | unstated |
| B10 | `PointReading for f64` (and `Probe`) give a reason that is not the reason | S4 [likely]; claim 10 | doc-or-claim-only | unstated |
| B11 | `a_select_takes_the_arm…`'s third case passes whether or not the unread arm is walked | S7 [sure] | test-gap | unstated |
| B12 | `T::refutes` beside `T::WITNESS`: two spellings of "this session decides at an exact witness" | S8 [unsure] | code | unstated |
| B13 | The filed hull row has no `priority` or `cost`; the row beside it has both | S9 [likely] | doc-or-claim-only (tracker) | unstated |

## 4. Tally candidates

**None.** Criterion (a) fails for every finding: neither review raises a MAJOR, so no unilateral finding is "raised as MAJOR".

Near-misses, with the criterion each fails:
- **B1** (op table unpinned; unsound `Powi` mutant survives): unilateral, test-gap, distinct, EXEC. Fails (a) only: MINOR. The strongest near-miss: it is B's first-ranked finding and the one B's verdict line names ("one unsound mutant survives every in-tree row").
- **A1, A2, A3** (MR, MNOMEMO, MEQ survive): unilateral, test-gap, distinct, EXEC. Fail (a) only: inside MINOR 3. A3 is marked by A as the conservative direction.
- **B3** (`DecisionShape::form` contract), **B4** (zero-budget point pass), **B5** (rigidity rounding), **B8**, **B9**, **B11**, **B12**, **A8**: unilateral and class code / test-gap / contract-API. Fail (a) (MINOR, NOTE or style) and (d) (INSP or unstated).
- **A7** (carrier decisions returning `Zero`): unilateral and EXEC. Fails (a) (NOTE) and (b) (an unresolved claim discrepancy, not a located defect).
- **X3** ("no verdict moves"): A demonstrates it, B argues it. Bilateral, so fails (a) on both halves.

## 5. Notes

Factual contradictions:
1. **The filed row's cause.** A MINOR 8: its causal sentence "is stated as fact, and no measurement in the body checks it". B claim 14: "The filed row is accurate on the cause and on designer B's remedy". (They agree the remedy is designer B's, not new design.)
2. **What a user sees.** A claim 17: "Refused leaves now print a smaller 'form(s) frozen'" (refusal text, `drive.rs:1065`). B claim 17: "nothing a user sees differs from main". B's own NOTE 7 has `frozen` moving in the serialized receipt, so B's sentence may be scoped to the three cases it lists; as written the two differ.
3. **PR 7492 and `frozen`.** A MINOR 5: the body should say "that the store PR 7492 designs keys or compares receipts". B NOTE 7: "Not a key: no node key or memo key reads it; PR 7492 keeps it a leaf 'need'." Contradictory only on the "keys" half; both say receipts are compared.
4. **Same measurements, different numbers** (same direction and conclusion; hardware differs, B states 4 cores):
   - Tour, switch on: A base 91.3 / 89.3 / 90.0 s, head 97.8 / 95.9 / 100.0 s, +8.5%. B base 112.2 / 112.7 / 105.8 s, head 123.9 / 120.6 / 118.2 s, "about 10%".
   - `the_plain_memo_moves_no_decision`: A 28.5 vs 30.2 s (+6%, three runs, overlapping at one). B 37.5 -> 42.3 s (single run).
   - Claim 14 trace against the body's 3,048 / 3.0 s and 3,208 / 43.9 s: A `curve_carrier_clear` 3,481 at 4.9 s, `curve_carrier_gap` 2,489 at 55.1 s. B 3,344 at 4.0 s and 3,504 at 55.7 s. The units differ (A: "undecided decisions ≥1 ms", M11 tracing on; B: "climbs", run stopped after the first drive), so the `gap` counts (2,489 against 3,504) may not be the same quantity; the times agree.
5. **Origin of the gate red.** A: the duplicates "come from main via the merge ref ... HEAD has 3 occurrences of the name, origin/main has 2", "already fixed on main at f3eef46091". B: "present on the merge base: inherited". Both say inherited and not the unit's; whether the duplication is on the merge base or arises through the merge ref is stated differently, and only A says main already carries the fix.
6. **Per-scalar reading sentences (claim 10).** A: "the sentences per scalar are fine" (one style doubt, on `Dual`'s). B: "`f64`'s and `Probe`'s give a reason that is not the reason". An opposite assessment of the same sentences.

Differences that are not contradictions:
- E12 and the declines: A wants the sentence to say the reading can decline (A4); B says "the declines, the per-symbol point and the memo need no sentence". A judgement, not a fact.
- E12 direction: A "the code does more than the sentence says" (declines, precedes the door) and also less (Dual); B "the code does less than the sentence in two places".
- The unseparated tour cost: A lists it under SILENT deviations, B under Reported. Both quote the same body sentence ("not separated from noise").
- Claim 1 method: A a 27-margin battery plus mutants and an op-by-op argument; B a 134,784-margin base-vs-head differential (46,814 proven on both, 0 rows differ). Both held.
- Mutants: A 11, B ten. Shared survivors: the moved bracket (X4), Dual (X5). A alone: MR, MNOMEMO, MEQ, MQ. B alone: `Powi`, `Floor`. Caught in both: M9, M10, tie takes `when_gt`, hull read.
- Claim scoring: claim 6 A "partly" / B "HELD today"; claim 7 A "broken in its argument" / B determinism held, outcome a NOTE; claim 8 A "partly" / B "HELD on what I ran"; claim 9 A "partly" / B "code as claimed".
- Coverage: B ran the kernel fast set (10,559), 72 boxed slow rows, `editor-core`'s fast set at 1e-6 and 1e-12 and 18 slow symbolic rows on both trees; A ran the tour, 4 editor rows and the `geom-core` point rows, and did not run the topo / sweep / profile slow sets or the two ε rows.
- `m10_9_what_the_registry_buys` is red on both trees in both reviews; only B names the open row that holds it.
- Rubric identical: 4 / 3 / 3.

Coding judgements:
1. **Mentioned and denied.** The criteria give two codes, and A4 and A5 fit neither exactly: B mentions each and says there is no defect (the declines "need no sentence"; the row is "accurate on the cause"). Coded A only, since B did not raise them. X6 is the mirror case (A ran the mutant, listed it as a survivor, then said nothing needs pinning) and is coded BILATERAL because A lists it among its survivors. Moving all three the other way gives 17 / 10 / 13 or 20 / 7 / 13; the tally is unaffected.
2. **The E12 sentence is split** into its mismatches (X7, X8, X9, A4) rather than counted once. Counted once it is one bilateral, and the totals are 16 / 8 / 13.
3. **Stale-doc sites are split** by site: the shared one (X14), A's own (A6), B's three (B2) and the `form` contract (B3, separate because a test reads it and B lists it as SILENT). B read `sym.rs` 740-760 and may have seen A6's sentence; B does not cite it.
4. **A1 and B3 touch the same call** (`report::record(.., None, ..)` for a refuted ask): A1 is the missing row that would pin it, B3 the doc sentence it falsifies. Coded as two unilaterals. Merged they would be one bilateral with a test-gap half (EXEC) and a contract half (INSP).
5. **A9 and B10** are both about a per-scalar `PointReading` doc sentence, on different scalars (`Dual`; `f64` and `Probe`). Coded as two unilaterals, as different sites.
6. **Folded by dedup**: B's S2 (the cost's number is not at the site) into X2; B's S10 and SILENT entry (the body omits the gate red) into X1; B's "a deferring session's discharge reads nothing" into X7 / X11; A's three SILENT entries into X8, X10 and X2.
7. **Classes that fit poorly**: X1 (inherited gate red) and X11, B13 (tracker) are coded doc-or-claim-only; X3 and X13 have a code half and a claim half and are coded code; X9 is coded contract-API though its remedy is a sentence. None changes the tally, which is empty on criterion (a) alone.

Not coded:
- Each review's lists of what it did not run or probe (A: slow sets, ε rows; B: `editor-core`'s slow set and the kernel crates at the two ε rows, `Lit(NaN/∞)`, consumers that catch an `Indeterminate`).
- B's dispatcher correction that the merge base is `71c627cc7` (A used the same base).
- Author-reported deviations each review relays; A's condition (b) that the PR waits on PR 7698.
- An internal figure in B: the verdict says a 134,784-margin differential, claim 1 gives 76 templates × 24 × 24 × 3 boxes (131,328).
