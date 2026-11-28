REVIEW — PR 7712 (boxfinish/refute-first) at frozen head dcaa04d4e42ec14f7cd086b9cc46e9ac80ee3f26. Probes and logs: boxfinish/review-refute-first-X @ d653157677.

## Verdict: APPROVE-WITH-FIXES
I found no soundness break (claim 1 held under a 27-margin domain/branch battery, by argument and by mutants). Conditions: (a) the PR's `gate ok` is RED on this head. The cause is inherited: two duplicated slow-set entries in sweep (`.config/nextest.toml`), already fixed on main at f3eef46091. So main has to be merged and a green gate seen. (b) It still waits on PR 7698. (c) The MINORs below are fixes to the body, the proposed sentence and the rows. None of them is a code defect in the refuter itself.

## Gate, at step level (CI run 45394882010, head dcaa04d4, pull_request event)
- `test (kernel)` and `test (app)`: build ✓, default-eps test ✓ (76 s / 61 s), eps 1e-6/1e-12 ✓ (about 1 s each, so I can't tell what it selected), slow-set tests this diff edits ✓, doc-tests ✓. The step "the slow set names only tests that exist" ✗: `slow-set entry listed twice: sweep::all: join_pierce_runs_sweep::a_pinchs_cones_share_one_point_key` and `…a_shared_vertex_fan_reads_its_cuts_from_its_own_first_entry`. These come from main via the merge ref; the unit doesn't touch the file. HEAD has 3 occurrences of the name, origin/main has 2.
- lint ✓ (gates, fmt, clippy, rustdoc). The sym instrument job ✓ (20 min). demos, tools and interval were skipped: the gate never runs the tour.
- `gate ok` ✗ (from the step above).

## MINOR (ranked)
1. **Cost on main, measured** (claim 12; `sym.rs` `Sym::decide_sign`). With the switch ON, `demo-tour certified`, 3 alternating runs per tree, same build: base 71c627cc gives 91.3 / 89.3 / 90.0 s (mean 90.2), head gives 97.8 / 95.9 / 100.0 s (mean 97.9). That is +7.7 s (+8.5%) and the ranges do not overlap. The output is identical line for line after normalising timings (211/301/511, 972,873 of 2,037,816 proven, 36 frozen). editor-core slow Sym rows, same protocol: `m10_3_driver_interval::the_two_parameter_drive…` 2.61 vs 2.61 s; `m10_4_stackup_interval::the_two_hole_plate_stackup` 1.15 vs 1.17 s; `m10_sym_drive_memo_interval::the_plain_memo_moves_no_decision` 28.5 vs 30.2 s (+6%, runs overlap at one of three). Where main ships today, nearly every undecided ask is an identity: it pays the point pass and then still climbs. So the change slows what main runs by about 8% on the tour, and the body's "overhead not separated from noise" is now separated. DEMONSTRATED, sure. No fast-set row crossed 0.25 s.
2. **"No verdict moves" rests on more than claim 1** (claim 7; growth guard in `sym.rs` `ladder`, `SymRetry::max_forms`). A refuted ask no longer builds retry forms, so a later identity whose retry main would withhold at the cap is now attempted and proven. Row `sym::point::review_probes::a_refutation_moves_a_later_identity_through_the_growth_guard`, at Sym<Interval>, cap 1, 512-bit retry: on head the identity is `Zero` PROVEN (symbolic 1, retried 1); with the refuted margin climbed first as main does, it is the in-band numeric `Zero` (symbolic 0, retried 0). The sign is the same both ways. The proof, the symbolic column and the K retag move. It goes in the "more proof" direction and did not move the tour's counts at the shipped cap. Still, the body's argument that the outcome depends only on soundness is incomplete. The memo and budget freezes are the same kind of order channel. DEMONSTRATED, sure (that it moves at shipped dials: unsure).
3. **Rows that cannot see a claimed property** (claim 13; mutants run on `geom-core --lib sym::`, 106 rows):
   - M9 (certification check dropped): red, `a_reading_with_a_lost_domain_declines`.
   - M10 (no clear in `param_over`): red, `the_reading_is_the_same_in_any_order…`.
   - M2 (certified select takes when_gt): red. M3 (every select hulls): red. MC (point at the centre): 4 red.
   - SURVIVE: **M10b**, which clears only when a bracket is first recorded, not when it MOVES. The body says "recorded or moved", and no row moves a bracket. **MR**, where a refuted decision renders the blocking form with the shape report installed. The brief asked for the render to stay behind the refutation, and no row pins it. **MNOMEMO**, where the memo is never kept. "A sub-graph several margins share is read once" is unpinned. **MQ**, the structural question taking the reading: it survives, and it is behaviourally equivalent while the refuter is sound, so there is nothing to pin beyond the code. MEQ (a point tie reads as straddle) survives; it is the conservative direction. MDUAL (reading at Dual) survives; there is no Dual row.
   DEMONSTRATED, sure.
4. **The proposed E12 sentence (PR 7698) against the code** (claim 15). The code does more than the sentence says:
   - (a) It declines (opaque, hull, free, unrecorded, unbracketed, lost domain), and the sentence never says the reading can decline.
   - (b) It runs BEFORE the registered-identity door, so a registration false at the point is no longer believed. That is within ratified E12, which says "a fact that holds only past a decision is not registered… an identity of the expressions it built". But it changes what the door is trusted for, and the re-cut pin `sym_root_rows::the_doors_zero_beside_the_read_is_proven` shows it. The sentence should say the reading precedes the door.
   - (c) "the decision stays undecided" is inexact. An in-band numeric `Ok(Zero)` is refuted too and is returned as an unproven `Zero`, the same as a failed climb, but not "undecided".
   - (d) "at an exact witness" is broader than the code, which admits `Interval` only. `Dual<Interval>` is an exact witness per `discharge_checked` and reads nothing.
   BY-INSPECTION, sure.
5. **What a refuted ask records** (claim 9). `count_decision(false)`, no `retag_at`, a numeric K sample, and `report::record(.., None, ..)` all match the code. But the `frozen` column is not "the same as a failed climb". `SymCounts::frozen` is documented as "a claim about the subject" (a leaf's NEED). It lives in `RefusedLeaf.decisions` and the drive `Receipt` (both `PartialEq`), and it is printed in refusal text ("N form(s) frozen", `drive.rs:1065`). It moves 1,490 → 634 on the box-arm-off tour, which I reproduced. The body gives the number but frames the outcome as unchanged. It should say a receipt column moves, and that the store PR 7492 designs keys or compares receipts. BY-INSPECTION plus measurement, likely.
6. **Deviation with no schedule** (claim 16, style Q6): `deferred::discharge` also climbs. It was left "not measured, outside the fence", and no `work/` row was filed. sure.
7. **After PR 7492** (claim 6; `point.rs` `read`): it reads `sess.nodes.get`. On 7492 `nodes` is the leaf's cache of the drive table, and `Session::node_of` resolves a node another leaf recorded. As written, every such node declines, so the refuter goes quietly weaker on shared-table drives. When 7492 lands, `read` must go through `node_of`, decline on `Session::unknown`, keep `points` per leaf (the brackets are the leaf's input there), and keep the clear in the one `param_over`. BY-INSPECTION, likely.
8. **Filed row** `work/boxfinish/a-margin-through-a-hull-declines-the-point-reading-and-climbs.md`. Its facts are accurate, the slate is right, and the remedy is designer B's, not new design. Its causal sentence "because the forms they share were built by asks the reading now refutes" is stated as fact, and no measurement in the body checks it. likely.

## NOTE
- `point_of` on a bracket wider than f64::MAX (for example ±1e308): `hi−lo` overflows, so the point is the `hi` face and the same for every symbol. One-ulp brackets land on a face. Both are sound but break "off the faces, per symbol". DEMONSTRATED (`review_probes::the_point_rule_at_the_edges`), sure. The point bracket gives the point.
- Module docs `sym.rs:747-751` ("its DECISION path asks a form only where the numeric channel cannot answer") and `Decide for Sym`'s docs (5425ff) don't mention the refuter. Stale by omission. likely.
- Acceptance oddity I did not resolve: at Sym<Interval> I count 8 `curve_carrier_clear` and 28 `curve_carrier_gap` decisions returning `Zero`. M11 says none of the four ever decide Zero. The symbolic totals equal M11's, so these are probably in-band numeric zeros or zeros already present on main. unsure.

## Claims
1 held: 27-margin battery (sqrt of squares on both brackets, sqrt²/x/x/x⁻²x² over brackets touching 0, atan2 on the cut with −0, copysign, floor, asin∘sin past π/2, tan∘atan, acos, select/min/max ties, less_sin, a true registration), no margin both refuted and proven. Argument checked op by op against the `combine` rules.
2 held: the refuter sits after `deferred::defer` (a deferring session never reaches it); Sym<f64>/Probe/Dual read nothing; the structural question asks `ladder` directly.
3 held within E12 (see 4b). The shipped registrants (arc endpoints, tangency, radials, rigidity) are identities of the built expressions, with signs as literals and certified door decisions. The sample doesn't cover documents outside the tour and the two corpus rows, or other drives' leaves.
4 held (each kind declines; decoration propagates through select and copysign caps).
5 held.
6 partly: M10b is unpinned; 7492 needs `node_of` (MINOR 7).
7 broken in its argument (MINOR 2): the symbol is the VarId digest (^ version scope), so it is stable.
8 partly: the tour and 4 editor-core rows are identical; the m10_9 registry row is red on both. topo/sweep/profile slow sets, and editor-core at 1e-6/1e-12, NOT run (build budget).
9 partly (MINOR 5). 10 held: the sentences per scalar are fine. Style note below.
11: one finding (NOTE). Both `from_f64` sites enter the concrete Interval at chosen values, which is fine.
12 measured (MINOR 1). 13 (MINOR 3).
14 reproduced: 211/301/511, symbolic 1,832,193, numeric 2,541,916, frozen 634. `curve_carrier_clear`: 3,481 undecided decisions ≥1 ms, 4.9 s (body: 3,048 / 3.0 s). `curve_carrier_gap`: 2,489, 55.1 s (body: 3,208 / 43.9 s; M11: 33.6 s), so slower, as the body says. One run with M11 tracing on.
15 (MINOR 4). 16 (MINOR 6, NOTE).
17: a margin genuinely zero at the point, or with no bracket, climbs exactly as on main, so the user sees main's output. Refused leaves now print a smaller "form(s) frozen".

## End-to-end
It solves the measured problem: −71% leaf time with the box arm off, verdicts unchanged. On main as shipped (switch on) it buys nothing and costs about 8%, because the asks there are identities. Whether that trade is wanted is the orchestrator's call; the body should state it.

## Style
Questions exercised: Q1, Q3, Q5, Q6, Q7, Q8 (point.rs end to end).
- Q1: `select_arms` is one rule used for both the walk and the value, which is good. But `Interval::select_le_zero` re-decides the same comparison inside, so the tie rule is written twice (`hi ≤ 0`). likely.
- Q7: `PointReading` as a supertrait of `Decide` puts a `#[doc(hidden)] pub fn refutes(SymId)` on every decision scalar. Any `T: Decide` body anywhere can call it; sealing stops impls, not calls. likely.
- Q3: `values_with_no_expression_decline` asserts `forms > 0`, so it cannot tell "declined" from "read and straddled". likely.
- Q5: the module doc says "nothing outlives the leaf". The memo lives in the session, and that holds only while a session is per leaf (7492). unsure.
- Q2: the doc on `PointReading for Dual` gives a scope reason ("measured on"), not a soundness reason. That is fine, but it reads as a claim. unsure.

## Rubric
idiom and structure 4 (small, one-home point rule, iterative walk); test quality 3 (rows are real and mutant-red, but three claimed properties are unpinned); doc and comment honesty 3 (the module docs are good; the sentence, the receipt framing and the cost framing are inexact).

## Spec deviations
Reported: the Dual exclusion, `deferred` left alone, the identity row's box `[0.25, 1]` and `max(x,−x)`, the free mark minted directly. SILENT: the reading also refutes in-band numeric zeros; the frozen receipt column is framed as unchanged; the main-path cost is left unseparated.

## Executed
geom-core point rows; 11 mutants (two re-run cleanly after my own interference); the 4 probe rows on the branch; tour timing 3×2; editor-core 4 rows 3×2; box-arm-off acceptance on a local merge of probe-switch-off and this head (never pushed). Read only: the 7492 diff, the registrants, report.rs.
