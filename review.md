# Delta review of PR #3844 — `ddfc2e628a` + `ae4b1dbec5` (after the merge of main)

Lane `reach-delta/3844`, checkout `ae4b1dbe`. I read the PR body with `get` only, no comments and no reviews. CI on this head (run 37037217024) is green, and so are all 25 `scripts/gates`. Probes and logs are in `probes/` (`README-delta3844.md`). The oracle is box/closed-form arithmetic plus a head-vs-`origin/main` differential (7ec41898), never the kernel.
**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 6. The MAJOR is gone.

## Claims, by execution
1. **The MAJOR is gone (sure).** `volume_backstop` / `gate_volume_backstop` no longer take declarations, so no path lets a declaration reach a margin (structural; grep finds no other reader). Every plant refuses on head, exactly as on main, at ε 1e-9, 1e-6 and 1e-12 (`logs/head_*` = `logs/main_*`):
   - MAJ-1's cube at scales 1e-3, 1 and 1e3, under ∩, ∖ and ∪ (thinned);
   - ΔV at 0.5, 0.99, 1.01, 2, 100 and 10⁶ × the old allowance edge;
   - the rounded stack thickened by δ = 1.4e-8·s, 4e-9·s and 1e-12·s (3 scales × 2 rotations × 2 poses).
   Duplicates through the real door (5 flush pairs ×1, ×100, ×10⁴) build the same ∩, 0.20000000000000004 m³.
2. **The re-derivation is sound and pinned (sure, with m2).** Mutant `skip` (refuse on the f64 sign) turns red `volume_backstop_passes_a_closed_form_rounding_tie`, `reach_continuation::declared_rounded…` and CONTACT's `a_vertex_pair_reads_a_dipping_chord…`, at 1e-9 and 1e-12. The enclosure is not too wide: the smallest refused δ on the rounded stack is 1.9–4.5e-15·s (main: 5e-17–1.7e-16·s). So only ΔV ≲ 1e-13·s³, a few ulps, passes now that main refuses, which is the unit's intent. `widen1e-10` and `widen1e-7` both go red.
3. **+V follows DESIGN (sure).** On the two-shell body under ∩ and ∖, V/A at −0.5·zero, −0.99·zero, −1.01·zero and −0.99·escalate passes, and −1.01·escalate, −1.1, −2 and −1e3 refuse `≥ 0`, at all three ε. Main has no such arm and passes them all. `plusv_exact` turns `…reads_the_sign_against_the_band` red. The predicate is margin V/A on the band, not a `target_len` mint, so it does not belong in k-lint's `EPS_COUPLED_PREDICATES`. It is registered where invariant-lane names go: `edit_refusal_recourse` WORDLESS, `rim_dim_review_probes` FIXED, and the audit table.
4. **Residue (the verdicts sure; the pin fails, m1).** The verdicts reproduce the PR table at every ε (D4). At 1e-12 the standing +1.2ε ∪ builds at −1.78e-15 against A+B. The closed form predicts a +1.74e-15 crossing, so the build is 2 ulps off and inside the 4.56e-15 gap. The sunk −1.2ε A∖B builds at −3.55e-15 (22 % headroom to the gap). The P2 item is honest, and its ranges match D4: standing +0.5ε ∪ and sunk −0.5ε ∩/∖ also refuse at 1e-9 and 1e-6.
5. **TANG row (likely, by inspection plus a green suite):** standing at −1.2ε still asserts the offer, the declared union building, and the other class contradicted. Its subject survives.
6. **Meter.** With `door-tier3-meter` on, topo `ci` runs 2007/2007 and the script reproduces 693 / 55 / 55 / 3. Result changes: none. For the env and fail-loud questions, see m3.
7. **Suites** (`--profile ci`, topo+sweep+editor-core): 6416/6416 at each of 1e-9, 1e-6 and 1e-12. `rigid_map_near_eps_plane_nurbs` did not fail here. Topo under `--features probe`: 2021/2022, only the known `rim_dim_boolean_twins` red.

## MINOR
- **m1. The residue row cannot go red when a crossing stops refusing.** `crates/topo/tests/door_backstop_settled_residue.rs:199,207`: `Crosses` accepts a build within the gap at every ε, and the row only asks that at least one refusal happened. Mutants `nocap` (∪ ≤ A+B dropped) and `nofloor` (∖ ≥ A−B dropped) leave it green at 1e-9; only the planted unit row `volume_backstop_joint_and_sign_arms` catches them. So "pinned on their actual verdicts at each ε" holds for the sunk ∩ alone. (sure; by execution)
- **m2. The per-face interval lift has no unit row.** Mutant `nolift` collapses each lifted closed-form face to a point at its midpoint (`props/quad_lane.rs` `closed_form`). Every unit row stays green at 1e-9 and 1e-12; only CONTACT's contact9 row reds, and only at 1e-12. The fold's outward rounding alone builds both ties. (sure; by execution)
- **m3. The meter fails silent.** `crates/topo/src/boolean/door_meter.rs:81-86`: an open or write failure drops the line, so the table undercounts with no signal. The instrument breaks fail-loud to protect a result it cannot change anyway. `:80` `std::env::temp_dir()` is a runtime TMPDIR read in `crates/*/src`. It is feature-gated and only picks the sink, but `no-ambient-env.sh`'s pattern (`env::var(s)`) cannot see it. Python's `tempfile.gettempdir()` also reads TMP/TEMP, which Rust does not, and the one fixed path collides across concurrent runs. (likely; by inspection)

## NOTE
- n1. Under `skip`, the rounded-stack row reds first at **sunk B ∪ A** (23.785398163397442 vs 23.785398163397446), before the flush-top ∩ that the PR body and the doc (`reach_continuation.rs:709-713`) credit. The flush-top ∩ does refuse under `skip` too (DE probe). There are two ties, and the doc names one. (sure)
- n2. The meter covers `boolean_op_recut` only. REST-zip unions (`rest.rs:160`) and fallback results are unmetered, so "results" undercounts the door. (likely; inspection)
- n3. `noarm1` turns 6 rows red, so the MAJ-1 row's "delete arm 1 and … every other row stays green" (`ops.rs:3549`) is false. (sure)
- n4. The committed +V row probes −4·escalate, not one past the band. D3 shows the edge sits exactly at escalate. (sure)
- n5. `volume_backstop_positive` is absent from `rim_dim_boolean_twins`' fixed list, as `_violation` is. That row is red under `probe`, so the scaling claim is unchecked. (unsure)
- n6. Max tier-3/op time reads 141 % here against the PR's 79 %; it is a timing column, not a guarded claim. (sure)

## Style (exercised Q1–Q7; Q8: `volume_backstop`/`bound_holds`, `door_meter.rs` and the residue file end to end, not all of `ops.rs`)
- **Q1, likely.** The +V rule has two homes: `validate.rs:4715` `plus_v_read` (`decide`, `positive_volume[_enclosure]`) and `ops.rs:1713` `Posture::PlusV` (`decide_invariant`, its own name and its own interval confirm). One rule, two spellings.
- **Q1, sure.** The fix closes the duplications it was asked to: `resolve_face`/`face_loops`/`closed_form_of` have one home, and `Posture`/`Side` replace `encloses_material` and `±1.0`. I found no fresh copy.
- **Q4, likely.** `docs/predicate-dimension-audit.md:420-421` name `check` arm 1/2, while the row added beside them says `bound_holds`. That is two names for one function.
- **Q5, sure.** `reach_continuation.rs:695`'s headline "refuses its union typed" is now true only for A ∪ B, since B ∪ A builds. `:711` has a stray "The" line break.
- **Q7, unsure.** `ops.rs:1817` re-runs the whole-body interval re-derivation on every refine iteration while the f64 sign stays negative.
- **Q6, sure.** The removed allowance and the 1e-12 build-within-gap reading are scheduled, in the P2 item.
