# DECIDE-10 fix-pass delta — PR #4929 at `79695dc646` (dual review was on `7937b969a8`)

**Verdict: MERGEABLE-WITH-NITS.** Dispositions A–F all check out, by execution where they make a claim. None of the measured pins moved. The fix pass adds no new instance of a defect it fixes. The two CI reds are main's own, reproduced locally on `origin/main` `b24f526a`. The nits are four NOTEs and two style items, none blocking.

## Dispositions
- **A. Confirmed, sure, by execution.** I ran R1's probe at the head:
  - (a) `w=1e-15`, read on: `Zero`, `proven=false`, numeric 1.
  - (b) `atan`, `w=1e-15`: `Zero`, `proven=false`.
  - (c) `w=1e-3`: `Positive`, numeric 1, disputed 0.
  - Every case gives the same answer with the read shut. `sym11_the_decision_read_declines_at_sym_{f64,probe}` pass.
  - No bypass. `signed::decision` and `signed::order` are the only readers of `params`, and their only callers are `sym.rs:3519` and `:3561`, both behind `read` (`sym.rs:3280`). `poly_sqrt` (rule G, `manifest`) reads no box.
  - Witness: `note_witness(T::WITNESS)` runs at the top of `decide_sign` (`sym.rs:5204`), before any walk. A walk before the first decision sees `None`, and the read is then off (the safe side).
  - Two witnesses in one session: my probe `delta_a_one_session_at_two_witness_kinds_is_asserted` panics "two witness kinds". Release keeps debug assertions (`Cargo.toml:299`), so it panics in release too. See NOTE-1.
  - Plant: with the witness clause deleted, 4 rows go red: `sym11_…declines_at_sym_f64`, `…_at_sym_probe`, `sym11_a_zero_that_rests_on_no_registration…` and `m10_7_r1…r1_min_and_max_of_one_form_are_that_form`.
  - At an exact witness the read still answers. My probe `delta_a_the_read_answers_at_an_exact_witness` gives `proven` with the read on and `numeric` with it shut.
  - **The P2 sibling is correctly a separate defect.** It is the same root (an exact fold against a rounded comparison) in a different rule, A0, and it predates this PR. The read's fix cannot reach it. Its write-up omits one place it can reach: see NOTE-2.
- **B. Confirmed, sure, by execution.**
  - R1's `r1_a_read_zero_counts_as_a_contradicted_registration` now reads contradicted 0, disputed 0, symbolic 0 (before: 1 and 1).
  - Planting the old `door_zero` turns `sym11_a_zero_that_rests_on_no_registration…` red.
  - `sym11_a_contradicted_registration_is_not_a_disputed_theorem` still reads `(Recorded, Positive)` with contradicted 1.
  - `door_zero` (`sym.rs:4483`) and `contradicts` both ask the same `ladder(.., Path::Contradiction)`, so the two columns are disjoint by construction. Every definite-sign zero is charged to one of them, or to the exact arm's `debug_assert`.
  - Could a genuinely contradicted registration now go uncounted? I found no case. A zero the registry-free rungs also prove does not rest on the registration, and it is charged as a dispute. The one gap I see predates this PR: NOTE-4.
  - My overlapping-box lie probe confirms Ev's accepted risk: `b = x + 0.5` registered equal to `x` over [1, 2] is `proven`, because `b − x` is not definite under interval dependency.
- **C. Confirmed, sure, by inspection.** I ran `rule C\b|[Cc]lause[- ]3|sign read|stronger claim|weaker|re-?label|buys theorems|\bgated\b|gate[ds]?\b|sign_gated|symbolic_zero` (case-sensitive "rule C", because `-i` matches "rule cannot") over `crates/*/src` and `tools/k-lint/src`. What is left:
  - the dated records at `sym.rs:239`, `:475-564` (marked as records);
  - `drive.rs:340` (explicitly historical);
  - `sym.rs:352`, which says rule C is retired;
  - unrelated CI and validator "gate" prose.
  - Nothing stale is described as live.
- **D. Confirmed, sure, by execution.** I built a scratch crate that depends on geom-core without the feature:
  - `geom_core::sym::Rung` gives E0603 "enum `Rung` is private";
  - `sym::rung::Rung` gives E0603 "module `rung` is private";
  - with `sym-profile-testing` it compiles.
  - `profile` is `cfg`-gated by the same feature (`sym.rs:1055`).
- **E. Confirmed, sure, by execution.** The pins are restored as sums (`[96,12]→[100,8]` and `[152,28]→[156,24]`, both sides), and `decide_3` passes.
  - Gate plant: I re-added `Form::gated`, set it on the read's two outputs, carried it through `add`, `mul`, `neg`, `recip` and `quotient`, and put it into `digest` bit 1.
  - Under that plant, `m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row` goes red (`:626`), and so does `sym_root_rows::what_only_the_read_answers` (C6/S8, `:827`).
  - Nothing else in geom-core goes red (453 + 518 lib), apart from main's `bounds_census`. `decide_3` stays green.
  - So the guard is real, with the caveat in NOTE-3.
- **F. Confirmed, sure, by inspection.** "symbolic" is now the one word across:
  - the column, `SymCounts::symbolic`;
  - `ShapeOutcome::Symbolic` and `SampleOutcome::Symbolic` → `"symbolic"` (`k_stats.rs:1438`);
  - the drive line `symbolic=` (`drive.rs:958`);
  - k-lint's summary `(N symbolic, …)` (`main.rs:187`);
  - k-lint's refusal of `symbolic_zero`, `sign_gated` and `registered` (`outcome_vocabulary.rs:116`).
  - Outside the dated records, the old spellings survive only in k-lint's history prose (Style S1).

## Measured documents (`numeric` and verdicts since `7937b969a8`)
- The fix pass changed no pin value in `m10_10_pins`, `m10_9_pins` (the bracket at `certifies_at`), the tilted family, `sym_9_retry` or `m10_8`. Their diffs are prose plus one label rename (`A0+C_early`→`shipped`).
- At the head, 33 of 33 pass in 502 s, `--include-ignored` (evidence rows skipped): `m10_10_pins` 5, `m10_9_pins` 10, tilted 20, `sym_9_retry` 5, `decide_3` 1.
- So no `numeric` count and no verdict moved. A changes nothing at `Interval` by construction.

## New findings
- **NOTE-1 (A), sure, inspected plus demonstrated in dev.** `Session::witness` constancy is a `debug_assert_eq!` (`sym.rs:4550`).
  - The workspace's release profile keeps it on. `benches/Cargo.toml:144` turns debug assertions off, and there a mixed session would silently keep the first witness, so the read could run at a point.
  - No known caller mixes witnesses in one session.
- **NOTE-2 (P2 sibling's reach), likely, inspected.** The filed item names only the K replay at `Sym<Probe>`. But the coincidence replay runs `Sym<f64>` at `SymRules::shipped()`, with A0 on (`editor-core/src/coincide/replay.rs:67,86-89`). Its proofs become `topo::Discharge::Proven`, which is a verdict path.
  - I have not shown it reachable: it needs catastrophically cancelling literals in a decision.
  - The item's reach paragraph, and maybe its priority, should say so.
- **NOTE-3 (E), sure, demonstrated.** The ledger guard hashes every built form's digest. So it reds on any change to `Form::digest` or to the forms built, not on an atom merge as such. The only semantic guard for the atom key is still C6/S8. That is adequate, but "the guard" is a detector rather than a statement of the invariant.
- **NOTE-4 (B), unsure, inspected, predates this PR.** `door_zero` asks only the first attempt's door form. The definite-sign path never runs the retries, so a registration contradicted only at a retry's ring or mask would go uncounted. The same was true on main.

## Style
- **S1 (Q4), unsure.** `tools/k-lint/src/lib.rs:628` and `tests/outcome_vocabulary.rs:7-9` narrate the `SymbolicZero`/`symbolic_zero` variant as the vocabulary's history. The variant is now `Symbolic`, so a reader has to infer the rename.
- **S2 (Q4), unsure, predates this PR.** Two references name `retag_symbolic_zero`, which no longer exists (main already has `retag_at`):
  - `geom-core/tests/m10_7_r1_retag_probe.rs:2`;
  - `m10_7_r2_sym_probes.rs:534`.
- **Questions exercised:** Q1 (the vocabulary grep), Q2, Q3 (five plants: A, B, gate, and two R1 probes), Q4, Q6 (the P2 schedule), Q7. Q8 not done: `sym.rs` was read around the delta only, not end to end.

## Gate verification
- Run 38355859327 on `79695dc646`. The only red job is `test`:
  - `bounds_census::every_sole_bracket_bound_door_is_in_the_roster` at the default, 1e-6 and 1e-12 rows (13668 of 13669);
  - `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven` at 1e-12 only;
  - the slow set: 423 of 423.
- Main's push CI skips `test`, so I reproduced locally on `origin/main` `b24f526a`:
  - `bounds_census` is red (`bounds_census.rs:803`);
  - the ring row is red at `CAD_TOLERANCE_EPS=1e-12` (`coincidence_door.rs:44`) and green at 1e-9.
  - Both are main's.

## Probe commit
`decide/10-review-delta`: R1's probes plus the delta probes in `crates/geom-core/tests/review_r1_probes.rs` (aggregated in `tests/all.rs`), and this report. Not for merge.
