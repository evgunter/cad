# Correspondence coding — PR 7488, reviews A and B (blind)

Source: `A.md`, `B.md` only. Evidence codes: EXEC (execution), INSP (inspection/argued), unstated.
Classes: `code`, `test-gap`, `contract-API`, `doc-or-claim-only`.

## 1. Verdicts as written

| | Verdict | MAJOR | MINOR | NOTE | Style | Rubric (idiom / test / doc) |
|---|---|---|---|---|---|---|
| A | APPROVE-WITH-FIXES ("All fixes are to text and to one test, none to the mint") | 0 ("None.") | 5 | 6 bullets (Claim 7, Claim 6, Claim 13, Claim 3, brief correction, to-file) | 3 | 4 / 3 / 3 |
| B | APPROVE-WITH-FIXES ("No MAJOR") | 0 | 5 (MINOR-1..5) | 6 (NOTE-6..11; NOTE-6 has parts a, b, c) | 9 (S1..S9) | 4 / 3 / 3 |

## 2. Bilateral headline

1. **History dependence of the mint's branch for nodes minted before the session** (item X1). Both MINOR, both EXEC, both show one document storing the quotient (~2.8e-17) or the exact axis by history. Mechanisms differ: A = a node the session never recorded is an unknown (claim-wording fix; A holds claim 3); B = an earlier ask memoizes the frozen form and survives the node being recorded later (claim 3 "broken").
2. **`decision_read` / `registered` switches in `question::RULES` are inert and pinned by no row** (X3). A MINOR (R1), B NOTE-6c + S5 (U12); both EXEC; both say the protection is the empty state, not the switches, and the doc credits the switches.
3. **`nothing_that_reads_a_value_is_proved`: the `max`/`min` half pins only T3** (X2), and **the 64-term `BUDGET` has no guard** (X6: A NOTE, B MINOR-4). Both EXEC.

## 3. Coding table

### 3a. BILATERAL (13)

| # | Underlying defect | A: where / severity | B: where / severity | Class | Evidence A / B | Divergence |
|---|---|---|---|---|---|---|
| X1 | Mint's branch for one document depends on history when a node is minted before the session; "one branch at every valuation" / "function of the recorded nodes and nothing else" overstated | MINOR 1 (+ claim 1 "Broken as worded", Spec deviations SILENT) | MINOR-1 (+ claim 1 "except MINOR-1's corner", claim 3 "broken") | A: `doc-or-claim-only` ("honesty of the claim, not a live bug"); B: `code` (memo in `form_in` under content id) | EXEC (P1) / EXEC (2 probes) | Same severity. Mechanism and class differ; A holds claim 3, B breaks it. Production reach: A "found no production path"; B `unsure`, did not search. See Notes N1 |
| X2 | `question::tests::nothing_that_reads_a_value_is_proved` `max`/`min` half is blind to T1 (pins only T3) | MINOR 3 (+ claim 14, Rubric) | claim 14 remark, unranked (+ Rubric none) | `test-gap` | EXEC (P5, T1) / EXEC (T1, T2, T3) | A MINOR vs B unranked. A adds: the body's stated reason is wrong (gate `sess.witness == Some(Exact)`, `sym.rs:3523`); B: "passes under T1 and T2 as the author says" |
| X3 | `question::RULES` switches `decision_read`/`registered` inert; no row sees them turned on; doc credits them; later additions would go unseen | MINOR 4 (+ claim 14, End-to-end) | NOTE-6(c), S5, claim 2 remark | `test-gap` | EXEC (R1) / EXEC (U12) | A MINOR vs B NOTE |
| X4 | Hop proved zero in both components falls to quotient `0/0`; over a box refused as `Escalated`/`MarginDiag(Invalid)` by `path_junction_turn`, not `vertex_separation` as the body names | NOTE (Claim 7; claim 7 "Held, loosely") | NOTE-7 (claim 7 "held in outcome") | `doc-or-claim-only` | EXEC (P3), base INSP, `likely` / EXEC (17 lanes), base INSP, `likely` | None on severity. B adds "never stored and read" not literally true |
| X5 | `ProvedZero` is `Copy`, bound to no expression | NOTE (Claim 6) | S9 (`unsure`), claim 6 remark, S3 (dropped at `.is_some()`) | `contract-API` | unstated (`sure`) / unstated | A NOTE vs B style |
| X6 | 64-term `BUDGET` rests on a measurement no row guards | NOTE (Claim 13) | MINOR-4, S7 | `test-gap` | EXEC (R2, budget 3) / EXEC (U1, 4096) | A NOTE vs B MINOR. A: R2 red on `the_answer_does_not_read_the_sessions_dials` only; B: U1 red nowhere |
| X7 | Retry growth cap (`RETRY_FORMS` / `max_forms` 200,000) couples an ask's answer to earlier asks | NOTE (Claim 3) | NOTE-8 | `code` | unstated (`likely`) / INSP (`likely`) | None; both "not reachable" on measured corpus |
| X8 | Dispatch names `scripts/gates/run-gates.sh`, which does not exist | NOTE (Brief correction) | NOTE-10 (first half) | `doc-or-claim-only` | unstated / unstated | None |
| X9 | Inherited red `a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar` (sweep, 1e-12) has no open `work/` row | NOTE (To file) | NOTE-11 | `doc-or-claim-only` (outside the PR's ground) | EXEC (differential) + read / EXEC + read | A lists more unfiled rows (topo `face_normal::tests::no_source_file_folds_the_bit_by_hand`, 5 `band_*`, 2 `section_ellipse_seam_poses`); B names only this one. See Notes N2 |
| X10 | topo `square_search` reseeds per run, so its payload is not comparable base vs head | claim 15 remark, unranked | NOTE-11 | `test-gap` (outside the PR's ground) | EXEC / EXEC (seeds quoted) | A unranked vs B NOTE |
| X11 | EX1 "not yet of that shape" is status wording in a present-tense page | claim 17 + Style | S1 (`crates/geom-core/README.md:242-243`) | `doc-or-claim-only` | unstated (`sure`) / INSP (`likely`) | Both style. B adds: no `work/` row schedules the move. A adds: matches the unit, accepted on 6993 |
| X12 | `AxisUnsigned.zero` kept under `#[expect(dead_code)]` only to make the error uninhabited | Style | S3 | `code` | unstated (`sure`) / unstated (`sure`) | None |
| X13 | `Drop for Ask` allocates a fresh `Session` on every ask for `mem::replace` | Style (`likely`) | S4 (`sure`) | `code` | unstated / unstated | None |

### 3b. UNILATERAL, A only (4)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| A1 | `docs/predicate-dimension-audit.md` F43 names `k_stats::check_unlogged(.., "F43")`; code runs `k_stats::unlogged("F43", ..)` (`seg.rs:802`) | MINOR 2 | `doc-or-claim-only` | INSP |
| A2 | PR body's differential stale and incomplete: names merge base `e1830a1c` (head's is `6c5ea3ec`), base `sweep`/`topo`/ε rows "still running", inherited `sweep`/`topo` reds unlisted | MINOR 5 | `doc-or-claim-only` | unstated (missing runs executed by A) |
| A3 | Both-zero arm unpinned: mutant R3 (an axis in the both-zero arm) seen by no row | NOTE (Claim 7), claim 14, Rubric | `test-gap` | EXEC (R3) |
| A4 | Mutant R4 (fresh state per ask, "a cost-only change") seen by no row | claim 14 remark, unranked | `test-gap` | EXEC (R4) |

### 3c. UNILATERAL, B only (12)

| # | Underlying defect | Where / severity | Class | Evidence |
|---|---|---|---|---|
| B1 | Driver row `an_undecided_axis_sign_reaches_the_driver_as_a_cue_to_bisect` does not pin its second sentence, and the sentence ("refuses the far side as a flip") is false: leaves below zero refused **Budget**; the `FlipCrossing` found is a `segment_gap` flip; drive half identical under M3 | MINOR-2 (+ claims 8, 14, End-to-end, Rubric) | `test-gap` | EXEC (probe; M3) |
| B2 | `Interval` compile-fail doctest on `Expression::proves_zero` (`real.rs:2160-2163`) fails E0599 on `point`, not on `proves_zero` | MINOR-3 (+ claim 6) | `test-gap` | EXEC (compile attempts) |
| B3 | PR body cites two inherited reds to `status: closed` rows; an open row holds them | MINOR-5 (+ claim 15) | `doc-or-claim-only` | INSP (marked "DEMONSTRATED (read)") |
| B4 | Sign reading's least band unpinned: working band (U11) passes every row | NOTE-6(a) (+ claim 8) | `test-gap` | EXEC (U11) |
| B5 | Retry ladder inside the question (U2) seen only by 2 `m10_sym_profile_interval` walk ledgers; no behavioural row | NOTE-6(b) | `test-gap` | EXEC (U2) |
| B6 | `Real::Expression`'s sentence for `Sym` false of a `free_identity` result (`real.rs:352`); `proves_zero` would answer for the wrong expression | NOTE-9 | `contract-API` | INSP (`likely`) |
| B7 | Dispatch correction: the three geom-core census reds run under plain `cargo nextest` default profile (slow-set filter only in `--profile ci`) | NOTE-10 (second half) | `doc-or-claim-only` | unstated |
| B8 | T3 also reds the `Sym<Dual>` row's no-session half, which the body lists under T2 only | claim 14 remark, unranked | `doc-or-claim-only` | EXEC (T3) |
| B9 | `chord_tangents` (`path.rs:5551`) and arc-lowered line legs still spell the tip ray by `chord_director`'s quotient ("direction spelled twice" left in `line_to`'s siblings) | S2 (`likely`), End-to-end | `code` | INSP |
| B10 | `direct_leaving` (`path.rs:3678`) silently ignores an out-of-range index, against fail-loud | S6 (`likely`) | `code` | INSP |
| B11 | Pin ledgers (`m10_9_pins_interval.rs`, `m10_8_pins_interval.rs`) accrete "Then …" paragraphs | S8 (`unsure`) | `doc-or-claim-only` | INSP |
| B12 | SILENT spec deviation: the unit's "check in the symbolic lanes that the two spellings agree on the value channel" not built, not mentioned | Spec deviations (`unsure` it was owed) | `test-gap` (class uncertain, see N7) | INSP |

## 4. Tally candidates

**None.** Criterion (a) fails for every item: neither review raises any MAJOR (A: "MAJOR None."; B: "No MAJOR"). The highest unilateral severity is MINOR (A1, A2; B1, B2, B3).

## 5. Notes

Contradictions on a fact:

- **N1 (claim 3).** A: "Held: P2 (an identity asked cold vs after large non-identities that share its subgraph) gave the same answers". B: "3 **broken**", by a probe where an earlier ask fixes the answer of a node recorded afterwards. Different scenarios, opposite claim verdicts. X1 is coded BILATERAL at the shared consequence (claim 1 false by history for pre-session nodes). Alternative coding, if mechanism is the unit: A MINOR 1 A-only (`doc-or-claim-only`), B MINOR-1 B-only (`code`), giving 12 / 5 / 13. No effect on the tally either way (both MINOR).
- **N2 (open rows).** A: topo `face_normal::tests::no_source_file_folds_the_bit_by_hand` is red on base and head and "only closed rows hold it". B: "each default-ε red has an open row". A ran topo at default ε only, so the two conflict if that row is among the 6 topo reds.
- **N3 (hosted CI).** A: "'change filter' queued, 'prime the build cache' skipped. Nothing ran". B: at last look `lint`, `test (kernel)`, `test (app)`, `mesh budget meter` **success**, `gate ok` queued. Likely time of look (B: "At my first look only `change filter` had run"), not reconcilable from the text.
- **N4 (claim 6 doctests).** A: "3 compile_fail E0599 plus 1 that compiles", claim held without caveat. B: the `Interval` one errors E0599 on `point`, not for the stated reason (B2). Same error code, opposite reading of what it proves.
- **N5 (claim 10, tip mutant).** Both reproduce link door-shut (695, 843), frozen 2600. A: "refusing at the extrude (Euler) op". B: "refusing at limb 2's stall". Refusal site differs as worded.
- **N6 (claim 11 pad).** A: "sym_9 pad −12/−7 (11+1, 6+1)", by arithmetic. B: "pad 1467→1483" under a recount mutant (a 16 difference). Cannot tell whether these are the same pin; slab (4) and bulge agree.

Could not code cleanly:

- **N7.** B12's class: an unbuilt check from the unit; `test-gap` chosen, `code` equally arguable. Not MAJOR, no tally effect.
- X8, B7 are corrections to the dispatch, and X9, X10 are outside the PR's ground; recorded, classed by nearest fit.
- Not coded as findings (statements of what was not run or of coverage): A "driver bisect row … I did not run it", A/B "new rows on the base not run", B's list of jobs the change filter skipped, B's `demos/tour` 98/98.
