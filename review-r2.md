# DECIDE-10 review — R2 (concurrent arm), PR #4929 at `7937b969a8`

**Verdict: APPROVE-WITH-FIXES.** None of the eight claims was falsified. Every receipt I re-took reconciles to the sum of the old columns, and no `numeric` count or verdict moved. The fixes are prose: the doc sweep missed rule C, the gate, and the "stronger claim" premise in a dozen places.

## Findings
- **MINOR-1 (Q4). Rule C and the gate are still described as live.** `sure`, by inspection.
  - The header says rule C "is sound, unit-pinned … and stays dial-off": `crates/geom-core/src/sym.rs:105-109`.
  - "`|x| − x` … is rule C's, and rule C is dial-off": `sym.rs:343-344`.
  - `signed`'s module doc, "Rule C: … the clause-3 fold": `sym.rs:1065-1067`.
  - `canonical_root`'s doc says "Its fourth source is a certified read and rides rule C's dial": `sym.rs:1735-1737`. This contradicts `root.rs`, which now says no source reads a value.
  - `params` is "read by rule C, by the decision read and by rule G's certified side-condition source": `sym.rs:2439-2441`.
  - Form equality covers "the poison flag and the gate": `sym.rs:3897`.
  - The early walk runs "rule C's fold at each `sqrt`/`abs`": `sym.rs:4002`.
  - `param` and `param_over` describe "rule C's sign read": `sym.rs:4561` and `:4578`.
  - The same phrase in `crates/editor-core/src/analysis.rs:520`.
  - The PR says "Docs say what the code does now". Its sweep patterns had `signed_root` and `.gated` but not the prose words "rule C", "clause-3" or "gate", so they could not match these lines.
  - **Class:** the code stayed right and the doc rotted. Sweep `rule C|clause[- ]3|gate[ds]?\b` over `crates/*/src`.
- **MINOR-2 (Q4). Ranking claims by strength survives in the prose that justifies shipped choices.** `likely`, by inspection.
  - `Rung`'s doc says the ladder's order "keeps a stronger claim from being re-labelled as a weaker one: `discharge`'s docs argue it": `sym.rs:4176-4179`. Those docs no longer argue it.
  - `SymRetry` leaves the plain rung out because "re-asking it could only re-label it": `sym.rs:2143-2144`. See also `sym.rs:1833`.
  - "The rule-A attempt is the WEAKER half … six REGISTRATIONS and no theorem": `crates/editor-core/src/drive.rs:353-358`. The same claim is at `sym.rs:889`.
  - Rule G's attempt goes first because it is "the one that buys theorems": `drive.rs:364` and `sym.rs:2269`.
  - These are shipped decisions (the ladder's order, shipping the rule-A mask) still argued on the distinction Ev deleted. The decisions may stand, but the stated reasons are void.
- **NOTE-1 (claim 1). `Rung` is `pub` and still records, per decision, which rung answered.** `sure`, by inspection.
  - It travels through `sym::profile::DecisionRecord::answered` (`sym/profile.rs:279`).
  - `Rung::Door` is exactly the retired `registered` kind, so a reader built with `sym-profile-testing` can still branch on "registered or not". It cannot branch on whether the read was used: both land on `Early`.
  - The feature is dev-only: dev-dependencies in `editor-core` and `geom`, and nothing shipped. So the type guarantee holds for shipped builds only.
  - The brief calls `Rung` private, but it is `pub` (`sym.rs:4185`).
  - Nothing else can branch on how: `revalue::Decision::proven` is a bool, `ShapeOutcome::Symbolic` is one variant, the K vocabulary has one token, the receipt has one column, `topo::Discharge` is `{Numeric, Proven}`, and the replay's `Rung` variants carry nothing.
- **NOTE-2 (claim 7). The link's door-shut refusal is a measurement, and it predates this change.** `sure`, executed.
  - I ran the same replay on `main` (`8a6f17d8`) with my probe. It gives 831/0/0/712, frozen 1318, refusing at node 31 (the extrude's pcurve), identical to the head.
  - The door-shut replay decides 1543 decisions against shipped's 1525, so on the link the door changes the construction's control flow. It does not just move columns.
  - On the bracket and the pad the populations are equal (2452 and 2656).
- **NOTE-3 (claim 4). Rule C was off in every shipped configuration, and rule G is unchanged in all of them.** `sure`, by inspection.
  - At base, `signed_root: true` appeared only in `SymRules::all()` and in tests.
  - `shipped`, every `without_*` set, and the coincidence replay (`coincide/replay.rs:89`, `SymRules::shipped()`) had it off.
  - The retry masks are ANDed with the shipped rules.
  - Rule G's source 4 was gated on `sess.rules.signed_root`, so it never fired shipped. Deviation 3 changes `all()` only.
- **NOTE-4 (claim 5). The contradiction check without the door is equivalent, and a plant reds it.** `sure`, executed and argued.
  - At base, `Registered` could only answer after the early and top rungs had both declined, so `contradicts` was already plain ∨ early ∨ top.
  - `door_zero` still raises `registrations_contradicted` before the check, now on gated door zeros too (the safe direction).
  - Plant C (the door asked on the contradiction path) reds `sym11_a_contradicted_registration_is_not_a_disputed_theorem` and `m10_9_r1_sym_probes::r1_two_registrations_make_the_two_channels_contradict_and_it_is_counted`.
- **NOTE-5 (claim 3). Merging atoms across the gate is sound by construction.** `likely`, argued.
  - An atom's key is `(op, payload, digest of its argument's form)`, and the atom stands for `op(that polynomial)`: the same real at every point, not only within the box.
  - The only cross-leaf state is the plain memo (`sym/memo.rs`), and the plain walk never reads.
  - I could not build two forms that share a key and differ anywhere.
- **NOTE-6 (claim 2). No counterexample.** `unsure`.
  - The read takes an arm only on a strict certified sign. The tie declines (`signed.rs:384-397`).
  - The registry's zeros are only as good as its witness. The rows' own `x·x ≡ x` over [0.9, 1.1] is a recorded lie, which ERROR-DESIGN already documents. S12 now proves through it with the read on.
  - This is the ruling's premise ("checked at the witness"), not a new hole.
- **NOTE-7 (claim 8). The E12 TAKEN re-wording describes this change; it is not a change of design.** `likely`, by inspection.
  - The new sentences restate Ev's 2026-10-12 ruling (one column, the differential).
  - `b773363cc1` is absent from this shallow clone, so I could not check the PR's `git log -S` attribution.
- **NOTE-8 (Q3). `decide_3_no_predicate_loses_a_decision` lost its two-sided pins.** `likely`, by inspection.
  - The link's two predicates were pinned at exact numbers, so drift either way went red. The row is now monotone: a rise passes silently.
  - This is disclosed in the PR body.

## Style
- **Q1. "Proven" is spelled at least six ways**, so "proven" is not yet one concept with one name. `likely`.
  - `MarginDiag::proven`/`as_proven`, `Pending::Proven`, `topo::Discharge::Proven`, `ShapeOutcome::Symbolic`, `SymCounts::symbolic`, `SampleOutcome::SymbolicZero`/`"symbolic_zero"`, `theorems_disputed`, and `report::record(symbolic: bool)`.
  - The column and its own K token disagree (`symbolic` against `symbolic_zero`): `sym.rs:1384` and ERROR-DESIGN:613.
- **Q7. A boolean decides two behaviours at once.** `discharge_in(id, decision: bool)` (`sym.rs:4305`) means both "retry" and "ask the door", and `rungs(.., door)` threads half of it. A flag that carries two policies is the shape that drifts. `unsure`.
- **Q2. An overlong doc line** (109 columns), left unwrapped by the edit: `crates/geom-core/src/k_stats.rs:1495`. `sure`.
- **Q4 (dated prose).** The rule-F and SYM-9 header sections (`sym.rs:233`, `:477-558`, `:882`) still read in the retired columns ("theorem / gated / registered"; "FOUR out of `symbolic_zero` … the unit's finding"). They are dated records, but the "finding" they name is no longer one under the ruling. `unsure`.
- **Q6.** Every deviation is disclosed. S13 is filed as P3 with both candidates. No unscheduled deferral. `sure`.

## Code quality
- **Counts:** MAJOR 0, MINOR 2, NOTE 8. Spec deviations: 8 reported, 0 silent.
- **Idiom/structure: 4.** The type change is clean (`ZeroWord`, `jointly`, `deferred`). The `decision` bool and the `pub` `Rung` are the warts.
- **Test quality: 4.** Plants A, B and C each red the intended rows. Plant B reds only S8 and C6, so no measured-document row guards the atom key.
- **Doc/comment honesty: 3.** MINOR-1 and MINOR-2: about a dozen sentences describe a deleted rule or a deleted ranking.

## Questions exercised
- Q1, Q2 (by grep), Q3 (three plants, executed), Q4 (rule C, gate and ranking sweep), Q5, Q6, Q7.
- Q8 in part: I read `sym.rs`'s header and every touched region, not all 6,000+ lines.
- Claims 1–8.

## Gate verification
- Hosted run 38328595199: the job `test` fails only at ε = 1e-12, on `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`. The 1e-6 row ran 13594 tests and all passed, as did the default row, and the slow set passed 417 of 417. Every other job is green.
- The ring row reds locally at ε = 1e-12 on both the head and `main` `8a6f17d8`, with the same panic at `coincidence_door.rs:44`. It is a known red on main.

## What I ran (one target dir, dev unless noted), with the PR's numbers
| what | mine | PR |
| --- | --- | --- |
| geom-core suite, `sym-profile-testing` | 922 passed | 922 passed |
| editor-core measured rows (m10_9_pins, m10_10_pins, tilted, sym_9_retry, sym11_exact, decide_3), with the slow ones | 20 of 20 passed | pass |
| bracket at `certifies_at`, base → head | 1386/5/57/1004 → 1448/1004, frozen 1869 | same |
| bracket, read shut, on base | 1386/0/57/1009 (head row: sum 1443/1009) | same |
| bracket, door shut | 1386/5/0/1061 → 1391/1061 | same |
| link, shipped → door shut, head | 862/663 → 831/712, refuses at a pcurve; identical on base | same |
| pad at `certifies_at`, shipped and door shut | 1372/1284 and 1308/1348 | same |
| tilted family (end cap, start cap, FlipZ) | 572/400/0/514 → 972/514, frozen 1380 | same |
| tilted per-predicate splits | [32,16,0,0], [24,24,0,0] → [48,0], [48,0] | same |
| tilted on SYM-5's tier | 541/298 at both | — |
| bracket at ceiling + δ (`PAST_THE_CEILING`) | 623/3/0/464 → 626/464 (3 `sign_gated`, confirmed) | 626/464 |
| pad release leaf at 1e2·ε, best of 3, ladder (not interleaved) | base 37.299 s → head 33.235 s, [1376,1280] retried 4 | 22.473 → 22.487 s |
| pad release leaf, rules alone | base 29.293 s → head 26.604 s, [1372,1284] | — |

My box is slower than the PR's, and my takes were not interleaved, so the leaf numbers say only that the cost did not rise.

**Plants** (in `review-r2-probes/`; reverted after each run):
- **A**, a sign-of-the-box fold at `abs`: reds 9 rows, including `a_sign_of_the_box_proves_nothing`.
- **B**, the gate bit back in `Form::digest`: reds S8 and C6 only.
- **C**, the door on the contradiction path: reds the 2 rows in NOTE-4.
- **Re-introducing a kind:** at the type level nothing reds; the type is the guard. At the K level, k-lint's `outcome_vocabulary` rows red on any new token (by inspection).

**Probe commit:** branch `decide/10-review-r2`. The patches and the base probe are in `review-r2-probes/`, and this file is beside them.
