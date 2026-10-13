# SYM-17 Phase 2 — delta review of the fix pass (PR #4197, `a89f7a54eb..1355348da2`)

**Verdict: MERGEABLE-WITH-NITS.** All seven dispositions are done as the PR body says. I re-ran every mutant the reviews listed, and each one now reds a committed row. One sibling of C was not swept: the decided door still admits an unbounded `Interval` translation (MINOR-1). The rest are notes.

## Dispositions
- **A — confirmed.** `cargo test --doc -p pncad`: 36 passed, 0 failed (local). Hosted run 38358229103's `doc-tests` step is green. A sweep for `OrthoFrame|ortho_frame` outside logs and closed items finds only SYM-17's own spec, unit and item, and `work/frame`'s new section, which names the rename. One stale hit was missed, and its file type was outside the PR's sweep: NOTE-1. *sure; demonstrated.*
- **B — confirmed.** The row is honest, and it fails in both directions:
  - Mutant "compose applies the outer motion twice" reds the bound, `defect <= k·zero` (`rigid_motion.rs:1515`).
  - Mutant "compose ignores the outer motion on `u`" reds the refusal end (`:1529`, no refusal ever comes).
  - My probe at a factor of 0.4999·zero gives a worst defect of 0.9998·k·zero, and a shear factor gives 28.8 zeros at k = 64. So "about k bands" holds to first order, for a margin taken one at a time.
  - zero = 1e-9 and escalate = 1e-8. The door already returns `Undecided` for the 2-fold product. `NotRigid` starts at k ≈ 13. The doc's "enough of them" is true (it takes 2). *sure; demonstrated.*
- **C — confirmed at the mints.** On `1355348da2`:
  - R1's `r1_interval_translation_finiteness` with assertions: `translation(+∞)` gives `NonFinite(Translation)`, and `MAX∘MAX` gives `NonFinite(Product)`.
  - R2's `r2_inverse_overflow_is_reachable` passes.
  - `is_bounded` per scalar:
    - `f64` and `Probe`: the default (`x − x` not poison). Right.
    - `Interval`: `!poison && lo, hi finite`. Right. A `Trv` enclosure with finite ends, `sqrt([-1,4]) = [0,2]`, reads bounded, which is consistent with `is_poison`.
    - `Dual` and `Sym`: the value channel. `Sym<Interval>` with value `[MAX,∞]` reads unbounded. `Dual(1, ∞)` reads bounded: the tangent is not asked, as the doc says.
  - Q1: `is_bounded` is a distinct predicate. It has a stated difference from `is_finite_length` and a cross-reference, and its home beside `is_poison` is right. But the decided door's translation probe (`0·tᵢ` through `decide`) is still a second spelling of the same question in the same file, and the two disagree at `Interval`: MINOR-1. *sure; demonstrated.*
- **D — confirmed.** Each mutant was planted on `1355348da2` and run against `geom-core` `rigid_motion` plus `topo` `a_widened_rotation`/`review_m4_pr2_transform`:
  - R1 A2 and R2 `inv` (`inverse` stops deciding finiteness) red `each_mint_names_its_own_non_finite_part`.
  - R1 A3 (the translation goes unchecked) reds the same row.
  - R1 E (`c + 10(c−c)`) reds the transpose row, bound for bound (`:1265`).
  - "The mint skews its angle by 1e-6" reds `the_rotation_mints_are_the_affine_constructions_on_an_exact_axis` and **both** `a_widened_rotation_moves_a_body` rows, at the containment assert (`a_widened_rotation_moves_a_body.rs:72`).
  - "`Interval::is_bounded` waves an enclosure through" reds `an_interval_overflow_refuses_at_the_mint`.

  *sure; demonstrated.*
- **E — confirmed.** The three directions match the pins:
  - `asm2a`/`kstats` are back to `main`'s 466 / [466, 466] / 922. I diffed them against `origin/main`: only line numbers move.
  - `OWN_FRAMES` goes 2 → 22.
  - `symbolic_zero` is −30 and −200, multiples of ten.
  - One overclaim: NOTE-3. *sure for the counts.*
- **F/G — confirmed, with notes.**
  - **One Rodrigues body.** `Mat3::rotation_about`/`_unit` share `rodrigues`. `Affine3::rotation_about_axis` and `RigidMotion::rotation_about_axis` share `anchored_rotation`, and so do the two `identity_minus_*`. `swept.rs` calls `rotate_point_about_unit_axis`.
  - **No hand re-spelling remains** in `crates/*/src`: I swept `rotation_about(` and `sin_cos`+Rodrigues. Two new spellings are NOTE-4.
  - **The helper.** 217 call sites. The only inline door calls left are the refusal rows, which is right. It sits behind `test-support`, whose description is now stale: NOTE-5.
  - **`PlacedAxis`.** It is typed and tagged (`placed_axis`). No row reaches it: NOTE-6.
  - **The single frame read.** It is not a behaviour change. `eval_node`'s `env` is the `op_env.lane.params` that `run_op` read before (`eval/mod.rs:3645`). The fault and `None` arms are the same, and the read is deterministic. The 466→476 withdrawal is said in the body, the code comment (`eval/mod.rs:4119`) and K-REPORT. *sure.*
  - **Present-tense docs.** `turned_between`/`translated`/`embed`/`pin` appear only under "Not built yet" and in `UnitVec3`'s pointer. *sure.*

## New findings
- **MINOR-1 — the decided door admits an unbounded `Interval` translation, so C's invariant holds only at the mints.** `rigid_motion.rs:705-715`: `0·[MAX,∞]` is `[0,0]`, so `check_rigid` admits `[MAX,∞]`, `[0,∞]` and `[−∞,∞]`. Each of those refuses at `translation`. `transform_rigid` then refuses `Certify{Escalated carrier_endpoint_start [0, inf]}`, which is the "kernel defect" ending R1's MINOR-2 named, reached through the door now. The module doc's "every number finite" (`:2`) and "the origin and every entry are finite numbers" (`:40`) are false for door-admitted enclosures. This is the class rule's sibling sweep: C fixed the mints, and the door is its other minting path. *sure; demonstrated* (`delta_door_translation_unbounded_at_interval`, `delta_door_admitted_unbounded_translation_moves_a_body`). It is reachable only by a caller who hands an `Interval` map to the door: literal frames arrive as decided `f64`.
- **NOTE-1** `scripts/doc-gate.sh:24` still cites `OrthoFrame::from_aim`. The fix's sweep covered `*.md,*.py,*.toml,*.yml` and not `*.sh`, the same blindness as R2's S10, one extension over. *sure.*
- **NOTE-2** The symbolic chain row's `frozen` counts moved after the PR body's table: head gives 2628/5658/8522/11386 against the body's 2626/5654/8516/11378 (+2 a link). `symbolic_zero` (1978/2934/3929/4924), `numeric` and the verdicts are unchanged. The pre-fix merge `2a9e4c47e4` already gives the head's numbers, so `main`'s merge moved them, not the fix commit. Only the body's table is stale. *sure; demonstrated.*
- **NOTE-3** `docs/K-REPORT.md:3060-3063`: "at rounding-sized margins (its entries were admitted at A6 against the same band)". The second clause gives margins up to the band's zero, not rounding. The conclusion holds because rule 2's zero arm is a non-gating tally, but the stated reason is wrong for a literal near the band's edge. *likely.*
- **NOTE-4** G mints a fresh normalizing/witness pair, `Affine3::rotate_point_about_axis`/`rotate_point_about_unit_axis` (`affine.rs:201,213`), each a one-line body over the shared Rodrigues. `swept.rs`'s `Turn` still holds one rotation twice (axis+angle for points, `anchored` for directions and the placement), and `Placing`'s doc (`swept.rs:67`) still names `rotate_point_about_axis`. *likely.*
- **NOTE-5** `crates/geom-core/Cargo.toml:12` says three things ride `test-support`, and describes the module as "the suites' one reading of a reporting margin's upper end". It now also carries `rigid`. *sure.*
- **NOTE-6** `RevolveError::PlacedAxis` (`revolve/mod.rs:459`) has no row that reaches it, only the Python tag census. *unsure* whether a placement can collapse an axis that passed the sketch pair.
- **NOTE-7** The PR body's "The change" still lists the inner tag `non_finite_translation`, which G removed. *sure.*

## Style
- **Q3.** Mutant "the mint widens the angle 21× (`a + 10(a−a)`)" stays green on every `rigid_motion` and `a_widened_rotation` row. The new containment check is monotone the easy way: it catches a skew, not a widening. *sure; demonstrated.*
- **Q7.** `Real::is_bounded` defaults to `is_finite_length`, the waving answer, so a future enclosure scalar that does not override it inherits the unsound reading silently (`real.rs:380`). *likely.*
- **Q2.** `finite()`'s doc (`rigid_motion.rs:362`) still says "the value channel reads every component … as a finite number". At `Interval` it reads both endpoints now. *sure.*
- **Q1** was exercised (C; the helper sweep; the rotation families). So were Q2, Q3 (nine mutants), Q4 (the door as C's sibling; the `OrthoFrame` sweep by extension) and Q5 (the module doc, `Cargo.toml`). Q8 was partial: `rigid_motion.rs` 1–140 and 340–720, plus the fix diff, not the whole file.

## Gates
- Run 38358229103 is red only in `test`:
  - `bounds_census::every_sole_bracket_bound_door_is_in_the_roster` at every ε (`blend/reach.rs` `displaces`). It is byte-identical to `origin/main` `b24f526a39`, and FAILs locally on `main` and on the head with the same roster diff.
  - The ε = 1e-12 `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`. It FAILs locally on `main` and on the head.
  - Every other job is green, `doc-tests` included. Both reds are `main`'s.
- The committed `#[ignore]` bisection row (release) re-takes the body's numbers exactly. At one link it certifies at 6.4938e-9 and refuses at 6.5378e-9 (`pcurve_envelope` `FidelityU`). At four it certifies at 6.7317e-10 and refuses at 6.7773e-10. At σ = 0.01 the first refusal is `carrier_endpoint_start` `[0, 8.9988e-5]`, with 8/15/24/35 refused.

## Probe commit
`404fb52c18` on `sym/17-review-delta`. Not for merge:
- `rigid_motion.rs`'s `delta_probes`;
- `topo/tests/review_sym17_delta_probes.rs`;
- `chaintol.rs`'s `delta_chain_rows_probe`.

The mutants were run by a script in scratch and are not committed.
