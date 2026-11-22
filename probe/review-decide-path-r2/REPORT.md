# Review: PR 7394 at frozen head 539a76e190f (merge base 0a62bae413b)

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The core change is right and is what I measured: a definite numeric sign asks the tier nothing, the merge base with only that removal (`R2_S1`) reproduces every re-cut pin I ran, and the instrument goes red on every mutant I planted, the corpus lane included. The fixes are about honesty and coverage. (1) The drive-memo fixture's causal story is false. (2) The enclosure check does not reach the deferring session's proven Zeros, or `Sym<Dual<Interval>>`. (3) The unit's "the number is its expression's, by construction" does not hold. (4) The cuts are unscheduled. The gate's one red is inherited from main but no row names it, so `gate ok` stays red until main is fixed.
Probes: branch `boxfinish/review-decide-path-r2` (commits d41334f59, a5de2fd52; `probe/review-decide-path-r2/`).

## MINOR (ranked)

1. **The fixture move's causal account is wrong, and the PR silently re-pins a main red that another program's row holds.** DEMONSTRATED, sure.
   - Where: `crates/editor-core/tests/m10_sym_drive_memo_interval.rs`, the `RACING` docs and `every_leaf_reports_one_column_under_every_schedule_and_both_dials`; the PR body's table.
   - What I ran: my probe `review_r2_the_race_drives_memo_on_and_off` at the merge base 0a62bae4, at the merge base with the check's walks skipped (`R2_S1=1`), and at the head. All three certify the same leaves: race 8/4 = 1, race 8/3 = 1, 12 leaves = 2, 64 leaves = 8, with the memo on and off alike. Only `frozen` moves (8/4: 36 → 0; 8/3: 331 → 204).
   - So the check's walks moved no verdict, and the drive memo is not the mechanism.
   - The row is already RED at the merge base: its race arm reads certified 1 against `certifies: false` (line 539).
   - That red is main's, recorded in `work/scalar/the-race-rows-leaves-need-two-amounts-where-they-needed-six.md`, section "The 8-leaf race now certifies, red on main (ONEHOME, 2026-11-19)". Its cause there is the signed extrude depth.
   - The "0 → 1, 1 → 2, 4 → 8" figures are against the stale merge base 7012fc40 that the body names, not against 0a62bae4.
   - The doc comment's "Inferred, not traced: the check's freezes, published to the drive's memo…" enshrines a causal story the measurement refutes ([[review-and-dependency-policy]]).
   - The budget move itself is justified: at 8/4 the head freezes nothing, so the row's non-vacuity guard could not hold. SCALAR's row is answered and not cited. Not a fixture changed to get green, but a green bought with a wrong explanation.
2. **The enclosure check misses the deferring session's proven Zeros.** DEMONSTRATED, sure.
   - Where: `sym/deferred.rs` `discharged` → `discharge_retried`. Production readers are `editor-core` `coincide/replay.rs` (4 sites) and `profile` `seg.rs:853`.
   - Why it matters: a numeric Zero at `Interval` can exclude 0 inside the zero band, which is exactly the window the check exists for.
   - Probe `a_deferred_discharge_proves_the_collision_with_no_check`: a false algebraic zero over `[-4e-12, -2e-12]` is handed to the reader as proven, with nothing firing. The same margin on the eager path panics (`an_opaque_minted_outside_the_session_collides_with_the_sessions_first`).
   - E12 says "where the tier answers `Zero` at an exact witness". The PR body's "every rung, a retry's and the door's … one site" is true of `decide_sign` only.
   - Disposition: SILENT deviation. The body's sweep calls it "the reader's answer" and does not say it is unchecked.
3. **"The number is its expression's, by construction" does not hold.** DEMONSTRATED, sure.
   - Where: `Sym::value`'s new field doc ("so the number is its expression's evaluation: what the enclosure check … stands on").
   - Three public routes mint one node with two values:
     - (a) `Sym::param(s, v1)` and `Sym::param(s, v2)`. The planted rows use this very device.
     - (b) `Sym::opaque` minted outside a session takes seq 0, and the session resets `OPAQUE_SEQ`, so its first opaque value has the same id.
     - (c) `param_over`'s bracket is not tied to its value.
   - (b) is a pre-existing soundness hole: a false PROVEN `Zero` for two different reals. On the merge base it is silent; at the head the check catches it on the eager path only (probe above). The instrument reads the collided node as present, not absent (`the_collision_is_present_to_the_instrument`).
   - Making the field private buys "no write outside sym.rs" (held), not the invariant. To file on SYM (`sym.rs`, `with_session_in` / `OPAQUE_SEQ`).
4. **The cuts are disclosed but unscheduled** (Q6). By inspection, sure.
   - What is cut: the corpus lane runs five documents at `Sym<Interval>`; no `Sym<f64>` corpus; drives are never reached; a retry attempt the 20k cap refuses is a silent miss, and `Report` carries no field for it.
   - Disclosure: the `contradiction_corpus.rs` header covers the five documents. Nothing at `contradiction.rs` says that drives are unreachable.
   - No `work/` file schedules any of these (grepped head and main for `contradiction_corpus` / `sym::contradiction`).
   - Nightly coverage rests on a full-suite build the PR body says currently dies at `pncad-py`.
   - Not re-measured: corner_table at 212 s / 11 GB (it would not fit this 15 GB box beside the builds).
5. **The gate's red is inherited, and no row holds it.** Sure.
   - Run 45031853988 is on 539a76e1. Every job executed and no step was skipped that should not have been. The `sym instrument` job ran both rows (4.5 s, 6.3 s after a 14m15s build).
   - The one red is `test (kernel)`'s step "slow-set tests this diff adds or edits": `geom-brep` `recourse_roster`, with three unlisted `ssi/enclose.rs` cone sites.
   - Those sites come from main's sectcone commits after 0a62bae4 (ea59b6fd…024a8fe7), reached through the PR merge ref. The roster on main lacks them, so main is red too.
   - No row under `work/` on main names them. Inherited but unfiled (implementer-discipline §6).
   - The diff's own roster entry (CARRIER_CALL, beside line 340's sibling) is right by inspection.

## NOTE
6. **`Sym<Dual<Interval>>` is exact-witness but unchecked.** DEMONSTRATED, sure. `Dual` does not forward `enclosure_probe`, so the check reads `None` and skips; an in-band false zero is returned proven (`at_sym_dual_interval_the_check_reads_no_enclosure`). No production lane decides at that scalar today (rows only).
7. **The free exception is "the DAG reaches a `Free`", not "this is a `free_identity`".** DEMONSTRATED, sure.
   - A margin that only contains a free identity's result escapes: a planted false zero plus `f − f`, its enclosure excluding 0, is returned proven (`a_margin_that_contains_a_free_identity_escapes_the_check`).
   - Conversely, an identity closure that never calls `free` fires the check (`a_free_identity_that_never_mints_its_variable_is_not_recognised`).
   - Neither is reachable from the one caller (`free_residual` always mints).
8. **`check` restores `counts` and the retry cap only.** By inspection, likely. It does not restore the memos, `plain_roots` or `plain_built`, so a decision after `check` in the same session, or a drive leaf's publication and NEED, would see the instrument's walks. It is unreachable today because no caller runs it inside a drive.
9. **The PR body's baselines cite merge base 7012fc40, not 0a62bae4.** Sure. Only item 1's figures depend on it.

## Claims
1. Held, by inspection. The definite arm runs `count_decision` and `report::record` and returns. `deferred::defer` acts before it and only re-tags a Zero. Under the feature, `revalue::note` and `settle` record and nothing else. `aside` only marks. No other caller of `ladder` exists outside `contradiction.rs`.
2. Held, by inspection. `[profile.release]` has `debug-assertions = true`, so `--release` equals debug here. The condition (`lo/hi`, then `reaches_free`) and `current_predicate()` are reads. `enclosure_probe` runs outside the assert in every build and is pure. Not compared at `debug-assertions = false` (cost).
3. Partly broken: see 2, 3 and 6. Held at every rung, every retry and the door on the eager path (M4 and M10 red).
4. Broken in both directions (7), at constructed margins only.
5. Held. The plain `assert!`s the diff adds are in `#[cfg(test)]` modules. The 21 `debug_assert`s in geom-core write nothing. The topo `site_rows` site is filed by the PR.
6. Held. Every `sym-revalue-testing` edge is a `[dev-dependencies]` one. `cargo tree -e features --edges normal -i geom-core` shows `default` only for the workspace, tour, wild, pncad and viewer.
7. Held for the three events. The cap and "absent" hide what 4 and 3(b) say.
8. Held. Every named mutant was run and went red: exact→false, algebra first-attempt-only, no receipt restore, no enclosure check, no `Free` read (geom-core and geom-brep), keep free, no door, walk absent, algebra `None`; plus mine, M10 (check at attempt 0 only), which the retry row catches. Corpus lane: `R2_MUT=sub` (a constant non-zero difference's form reads zero) and `R2_MUT=retry` (the first retry answers zero) both turn `contradiction_corpus` red on all five documents.
9. The cuts are real as disclosed, but unscheduled (4).
10. Held. No reader is left in code or docs on head (`git grep`); `m10_bulge_renders.txt` is a historical record that no code reads. Rust consumers that name the four fields or build `SymCounts` by literal break at compile time; the serialized text does not move.
11. Held for editor-core's 178 symbolic rows (`sym*`, `m10_9*`, `m10_sym*`, `m10_10*`, `m10_8*`, `m10_derived_frame*`, `m10_bulge*`, `revalue_corpus`, `contradiction_corpus`, `coincidence_door`, `decide_*`), head against base, at 1e-12 and 1e-6.
    - Two rows go red → green: the drive-memo row (1) and `the_forms_the_plates_walks_build_are_pinned`, which at the base mismatches at 1e-6 and 1e-12 because the check's walks were ε-dependent.
    - No row goes green → red.
    - `revalue_corpus::*_apart_to_its_parting` (two rows) are red on both sides.
    - sweep and topo were not run (not built). The tour suite is green in the gate's demos job.
12. The author's inference is refuted (1).
13. Held. `order > 0` can still go red, though it now rests on a single declining read. The predicate row still pins the recourse text.
14. Held, DEMONSTRATED for the editor-core ones: the base with `R2_S1=1` reproduces the head bit for bit.
    - Slab ledger: 7924 / 94, digests equal.
    - Plate ledger: 34792 / 24376, digests a134…, 2a05….
    - `PAST_THE_CEILING`: 2175 / 53 / 1307.
    - The pad: (1483, 1709, 1307).
    - Sweep digests (14 / 12) were read only.
15. Partly. Not at switch-off (it needs its own build). Switch on, as shipped: `m10_8_the_a0_set_is_a0_alone` 1.31 → 1.07 s, the slab finish row 0.03 → 0.02 s. The sym instrument job costs about 15 min of runner per tier PR, nearly all of it the editor-core build (6 s of tests).
16. Held, except where a closure claim leans on the check covering every Zero: "a retry's zero has no cross-check" is closed for the eager path only (2). `value` has no outside writer: the compile proves it, and CI's demos, tools, python and viewer jobs are green.
17. Held, with 5. The docs say what the code does, except the `value` doc (3) and the race doc (1).

## End-to-end
The change solves the right problem: the decision path pays nothing for definite signs, and the instrument finds blunt algebra and retry defects through the CI lane. What matters in practice:
- Drives, where registrations actually live, are seen by nothing in a shipped build or in CI.
- The deferred readers (the coincidence door) get proofs no check guards.
- The device the planted rows use (one symbol, two values) is itself a way to break the soundness argument from public API.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only partly: `sym.rs` is too large to read end to end, so I read `contradiction.rs` whole and the `decide_sign` / walk region.
- `sym.rs` `reaches_free`: a fresh copy of `keeps_an_opaque_atom`'s `reaches_opaque` closure (the same stack/seen/op test). It sits beside `Session::reaches` and `Session::closure`, making four DAG walkers, and uses `std::HashSet` where its siblings use `IdSet`. The fix mints a duplicate (Q1). sure
- `Sym::value`'s doc asserts an invariant nothing enforces (Q2/Q5; see MINOR 3). sure
- `m10_sym_drive_memo_interval.rs` `RACING` doc carries an inferred causal story that is false (Q2). sure
- `sym.rs` "# Cost" keeps figures taken with the check on, under a caveat that they overstate. Stale numbers kept with an apology rather than re-taken or cut (Q2/Q6). likely
- `Path::Algebra` with `expect(dead_code)`, the `Rung` re-export widened to a second feature, and the instrument as a child module reaching the tier's privates: the product type surface is shaped for a test instrument. I would not have done it this way, though Ev accepted the shape (Q7). unsure
- `decide_6_the_profile_decides_nothing` now has a one-read floor: weaker non-vacuity, monotone the wrong way if reads vanish but one (Q3). likely
- `contradiction::Report` has no cancelled-attempt count, so a clean report cannot say what the cap hid (Q3/Q6). sure
- The class behind MINOR 2 and NOTE 6: "every Zero the tier returns" was argued at one site. Other places a proven Zero leaves the tier (`deferred`, any future reader of `discharge_*`) need the same sweep (Q4). likely

## Rubric
- Idiom and structure: 4. A clean deletion; one duplicated walker.
- Test quality: 4. Every mutant reds, but no row pins the deferred path or the nested scalar.
- Doc and comment honesty: 3. The race doc's false cause, the `value` invariant, the stale Cost figures.

## Spec deviations
- Reported by the author: the corpus lane is five documents; no drives; no `Sym<f64>` corpus; the 20k retry cap; the 8/3 fixture; geom-core's self dev-dependency.
- SILENT: the deferred discharge is unchecked (2); `Sym<Dual<Interval>>` is unchecked (6); the drive-memo row was red at the merge base and SCALAR's row was answered uncited (1).

## Executed vs read
- Executed:
  - geom-core probes (7 rows).
  - The editor-core race probe at the head, the base and the base with `R2_S1`.
  - The base with `R2_S1` against the head's pins (ledgers, ceiling, pad).
  - 11 geom-core and geom-brep mutants, and 2 corpus-lane mutants.
  - The 178-row differential at 1e-12 and 1e-6.
  - Timings; `cargo tree`.
  - The gate's steps and logs.
- Read only: the sweep and topo digest moves, claim 2 at `debug-assertions = false`, and the switch-off cost figures.
- Filed by me: nothing. To file:
  - SYM: the opaque-seq collision (3b).
  - SYM: the deferred and nested-scalar check gaps.
  - GREEN or GERM: the `geom-brep` roster's `ssi/enclose.rs` main red.
