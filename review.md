# Review of PR #3847, frozen head 069293bfd8

Lane `reach-dual3847-r1`. Wall clock 2026-10-02 15:52 → 16:58 UTC. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 3.
CI run 37018148276 is `success` on head `069293bf`, so the gate exists. I read the PR body with `get` only, and no comments or reviews. **Glimpse: none.**
I fetched no `analysis/reach-dual/*` branch except my own, which already sat at the frozen head.

## Claims, by execution (oracle: mpmath at 50 digits on the exact f64 inputs, never the kernel)
1. **The slack is sound. HOLDS, sure.** Door fuzz `probes/door_fuzz_r1.rs` + `probes/oracle_r1.py`: 27,000 (pose, ε) cases.
   The poses mix tilted and flat frames and off-plane centres, at scales ×1e-3/×1/×1e3. They are near-tangent at the near extreme or at the far one with δ from 1e-4 down to 1e-16·scale, or generic crossings, each at ε 1e-12/1e-9/1e-6.
   Results: **0** certified roots off the band, **0** roots farther than their own slack (worst err/slack 0.62), **0** `Miss` on a crossing, **0** certified on a non-crossing.
   A second frame model (v = n×u rounded the way the kernel rounds it) also gives 0 off the band. A switch sweep (`c₀ = 0 ± 20u`, 6k cases) gives worst err/slack 0.22.
   By inspection the first-order drop is harmless here. Every `mul` has one operand that is either exact or large (`D + r`), so no δa·δb term is unbounded. `hypot` is Lipschitz-exact. `div_exact` divides by `2r`, which is exact. `sub`/`add` charge `u·|result|` plus their operands' bounds.
   The linearisation `δψ ≈ δR/|R′|` holds only while `δlo ≪ |lo|`, and the definite-extreme gate (`|lo| > Kε`) enforces that.
2. **Root placement is correct on both sides of the switch and at it. HOLDS, sure** (the sweep above). The selection is load-bearing; see MINOR-1.
3. **Plain fields bit-identical. HOLDS, sure.** `probes/diff_main_r1.rs`, head against merge-base `cd49025f`, 18,000 cases: `c0`, `a1`, `e_u`, `e_v`, `terms` show 0 bit diffs. Root bits do move in both doors; see NOTE-1.
4. **The table holds, and widens. HOLDS, sure.** `probes/e2e_r1.rs`, public API, head vs main, ε 1e-12/1e-9/1e-6. Cases:
   - radii (1, 0.8), (0.5, 2), (1.3, 1.3); scales ×1e-3/×1/×1e3; δ = 1e-4..1e-6·s;
   - identity and rotated+displaced poses, both orders, ∪ ∩ ∖;
   - reuse: (A∪B)∖C and (A∖B)∪B;
   - 400 `point_in_solid` samples per union against the two-ball oracle.

   No regression, no invalid body, no volume off, and **0 wrong `point_in_solid` answers**. 54 cases newly build at 1e-12 and 54 at 1e-9; the 1e-9 gains are the ×1e3 poses, which the PR does not mention (NOTE-3).
   Refusals that are not this PR's: `transform_rigid` refuses ×1e3 at 1e-12 (`carrier_matches_mapped_source`), and (A∖B)∪B refuses at the pierce door.
5. **Re-posed rows can fail, but only for the bound. PARTLY, sure.** `probes/mutants_r1.sh` ran each mutant against 36 touched rows at three ε:
   - running bound ×0 → reds both re-posed rows at every ε, plus `the_near_tangent_family_stops_at_1e_7…` at 1e-12. Good.
   - phase term dropped → **green**.
   - angle term dropped → **green**.
   - near/far selection inverted → **green**.
   - far-extreme share dropped → **green**.
6. **The design stop is honest. HOLDS, sure.** With bound ×0 forcing certification (ε 1e-12, unit scale, generic tilted poses), the f64 roots land >1e-12 off in 30/200 near-side and 86/200 far-side poses at δ 1e-7, and in 93 and 145 of 200 at δ 1e-8.
   So f64 itself, and not only the bound, misplaces these crossings. The two sibling items read as real. `line-roots-carry-no-root-slack-meter.md` is honestly marked "estimated, not executed".
7. **k-lint: no new crowding. HOLDS, sure.** I ran `scripts/k_probe_sweep.sh`'s dump on head and on main, skipping only the plain probe suites: `review_ring_clearance_r1_probes::recorded` is **red on main too** at 1e-9. Then `tools/k-lint` over the three ε.
   The predicate/outcome columns are identical to main's at every ε, and so is the flag set (rule-1 35, rule-2 20, rule-3 26 on both). No margin is newly in the zero band. Margin VALUES do differ; see NOTE-2.
- The touched rows, unmutated: 74/74 green at each of 1e-9, 1e-6 and 1e-12.

## Findings
- **MINOR-1: a load-bearing line no row pins.** `crates/topo/src/boolean/circle_roots.rs:568`. DEMONSTRATED BY EXECUTION (`probes/mutant_fuzz_r1.sh`). Inverting `select_le_zero(past(hi), π − past(lo))` keeps every committed row green at all three ε.
  Yet under the same fuzz it certifies **3 roots off the band**: at ε 1e-9, ×1e3 near tangency (k=15, ρ 913, lo −8.6e-5), the error is 1.28e-9 against a charged slack of 7.7e-10, and there are 42 slack violations. Reverting to main's `acos(−c₀/A₁)` behaves the same way (3 off the band, 29 violations).
  The new door row sits at ρ = 1, δ ≥ 2⁻²⁰, where the amplification (≈ u/√δ) stays under 1e-12, so it cannot see the defect it was written to fix. Fix: a door row at a pose where the far-extreme form errs by more than the band, for example ρ ~ 1e3 at ε 1e-9. Confidence: sure.
- **MINOR-2: the slack's new terms are unguarded.** `circle_roots.rs:558`. DEMONSTRATED BY EXECUTION. Dropping the angle term or the phase term reds nothing.
  The angle term is needed for the bound to hold: in 1 of 3,000 generic poses at each ε, the measured error (2.24e-14) exceeds the slack without it (2.3e-14, minus 1.1e-14·ρ), though it stays far under ε. Dropping the far-extreme share is also unpinned. Confidence: sure.
- **NOTE-1: root bits move, undisclosed.** `circle_roots.rs:568` reaches the circle×cylinder square arm too. That arm's roots changed bits in 4,158 of 7,396 cases certified on both, by ≤ 2.1e-12 rad, and at 1e-9 one ×1e3 far-side pose went `C2 → Uncertain` (the angle term).
  The body says that arm's slack is "as before, apart from the new angle term". D9 asks to say what moved. By execution, sure.
- **NOTE-2: "no geometry moved" is not accurate at bit level.** On `demo/lily_walls` 609 K-samples per ε across 29 predicates change value, with outcomes identical; for example `bool_circle_curved_clearance` goes 2.535160107049e-7 → 2.535160113012e-7. The demo's vertices moved by ~1e-15. By execution, sure.
- **NOTE-3: the 1e-9 gains go unreported.** At ε 1e-9 the PR also builds ×1e3 near-tangent pairs that main refused (9 poses × 6 ops). The body says 1e-9 is "unchanged". By execution, sure.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 not done: I read `first_harmonic_roots` and its module docs, and `implicit.rs`'s new code and its test, but not either whole file)
- Q1 · `circle_roots.rs:463-476`. `FirstHarmonic` now carries the extremes twice: `c0`/`a1`, still read by the coaxial arm and the old noise, and `lo`/`hi`.
  It also carries two noise vocabularies (`noise` vs `lo_noise`/`hi_noise`/`phase_noise`). Consistency is the caller's job by hand; the cylinder passes `c0 ∓ a1` at `circle_cylinder.rs:145`. Likely.
- Q1 · `implicit.rs:847`. `Rounded` is private to geom-brep, but the cylinder item and the GERM torus evidence both propose reusing it for topo-side meters. That is the shape that becomes a second copy.
  The unit roundoff is also spelled three times: `implicit.rs:853`, `circle_roots.rs:265`, and the test at `implicit.rs:1149`. Likely.
- Q3 · `circle_sphere.rs:351`. The placement row is posed where the old acos and the inverted selection both pass (MINOR-1). Sure.
- Q3 · `implicit.rs` test. Its ceiling is 16u·L plus slack; it is fine against degradation, but it pins the bound only against the Interval enclosure of the same chain, not against a truth with a different frame. Unsure.
- Q2 · `circle_roots.rs:36-51`. The module docs now explain the slack in about 15 lines of algebra that no row checks term by term. Likely.
- Q4 · `snowman.rs:282`. "the f64 evaluation … cannot place the pierce point" is borne out (claim 6), so it is fine. `predicate-dimension-audit` rows were updated, and are accurate. Sure.
- Q6 · the remaining two decades are scheduled (`f64-cannot-place…`, P3). The unit item is set `status: review` and is "closed partially"; its residue lives in a new item. Fine. Sure.
- Q7 · I would have kept the half-chord computation in one helper shared with `conic_crossing_roots` (`splitting/classify.rs`), which has the same acos-near-±1 shape. It is filed but not shared. Unsure.

## Probes (`probes/`)
- `door_fuzz_r1.rs`: door fuzz, spliced as a `#[cfg(test)]` module.
- `oracle_r1.py`: mpmath oracle.
- `diff_main_r1.rs`: main-vs-head differential.
- `e2e_r1.rs`: public-API end-to-end probe.
- `mutants_r1.sh`, `mutant_fuzz_r1.sh`: the mutant runs.
