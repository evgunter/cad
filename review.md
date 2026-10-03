# Review of PR #3977, frozen head 1d4f235512

Lane `reach-dual3977-r2`. Wall clock 17:09–17:36 UTC, 2026-10-03. **Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 2 · MINOR 3 · NOTE 2.
Glimpses: none. I read only my own brief on the briefs branch and the PR body (`get`); I read no comments, no reviews and no other `analysis/reach-dual/*` branch.
Method: I ran a main-vs-head differential (merge-base 623004b99 in a worktree) and logged every certify site on head (`PROBE_LOG`). I ran mutants on `rederive` (`probes/mutant_rederive.patch`). My oracles are closed forms: a slab is ±t·w², and the corner sliver is V∩ = (2x−d)·d²/6, which does not depend on the edge scale s. I derived that one myself, and it agrees with the PR's `det·dip²/(6 b_z c_z)`.

## MAJOR
**M1. The interval check lets genuinely inside-out bodies through that main refused (claim 1 falsified).** `crates/topo/src/validate.rs:4426`, reading `crates/topo/src/props.rs:753` (`rederive`). DEMONSTRATED BY EXECUTION (`probes/probe_r2_inverted.rs`, main vs head, ε 1e-9).
- The probe uses 30 inside-out slabs: t×w×w bricks whose six carrier planes are re-anchored `far` along themselves, the PR's own `far_anchored_slab` construction mirrored.
- Main refuses 28 of them `NegativeVolume`. Head passes 8 of those 28. The `f64` sum had the sign right in every one:
  - 100 nm×1 mm² at 1 km: exact −1e-13, f64 −1.109e-13, interval [−3.83e-13, +1.61e-13];
  - 10 µm×1 mm² at 20 km: exact −1e-11, f64 −3.28e-12;
  - 100 µm×1 cm² at 100 km: exact −1e-8, f64 −6.5e-9. Here V/A = 5e-5 m, which is 5·10⁴ ε.
- The enclosure is sound: every interval I printed contains the exact value. It is also loose. Its half-width (2.7e-13 at 1 km) is ~25× the `f64` error (1.1e-14). That is the interval dependency effect in the cross products about a far anchor.
- So the PR's premise, "a volume below its own rounding", does not describe these bodies. Their volume is far above the rounding and below only the interval's overestimate. A straddle that passes therefore admits inversions that main caught.
- Anchors ≤100 m produced no newly passed body in my grid.

**M2. Tier 3 now certifies the contact9 sliver, and wrong boolean bodies built from it newly pass tier 3.** `crates/topo/tests/contact9_side_codes.rs:159` (the unit's own fixture, 1 m edges, ε 1e-12). DEMONSTRATED BY EXECUTION (`probes/probe_r2_e2e.rs`, `probes/probe_r2_reuse.rs`, main vs head).
- That ∩ body's volume reads −2.96e-16 against an exact +5.83e-19. `point_in_solid` reads its side at infinity off that sign (`at_infinity_side`, filed lane-free at P3). The result: a point 60 m away reads `In`, and so do points 150 ε outside the sliver. Points inside it read `In` correctly.
- Reused as an operand at ε 1e-12, against C (a disjoint 2 m cube) and a 100 m box `big`:
  - `sliver ∪ C` and `C ∪ sliver` return the sliver alone. The volume reads −3e-16; the answer is 8 m³. **Main's tier 3 refused both `NegativeVolume`. Head's passes both.**
  - `sliver ∩ C` returns C (8 m³ where the answer is empty), `big ∩ sliver` returns `big`, and `big − sliver` returns ~the sliver. These pass tier 3 on main too.
- The same misreading shows on 8 widened poses per ε (rotated 30°/63°, shifted 4 m and 37 m, k = 1e3 at ε 1e-9). Main's tier 3 refused every one of them; head certifies them all.
- The boolean ships the same bodies on main. What changes is that check 7 was the only signal on them, and it is gone. That makes P3 `at_infinity_side` a blocker for this unit, not a residue.

## MINOR
- **m1. The new rows cannot go red when the enclosure gets wider (claim 5, half falsified).** `crates/topo/src/tier3_tests.rs:2684`, `contact9_side_codes.rs:159`. EXECUTION.
  - Width zeroed: the tier3 row goes red at :2710 (ε 1e-9 and 1e-12). Contact9 goes red too, but through the backstop's `vol(A ∪ B) ≤` bound at :79, not check 7.
  - Width doubled: every row is green. Each assertion only gets easier to pass as the width grows (Q3). M1's probe is the missing row.
- **m2. A stale citation.** `contact9_side_codes.rs:132` cites `work/geom/a-planar-face-sums-its-area-about-a-far-carrier-origin`, but the file is `work/flux/…`. Inspection plus `ls`; sure.
- **m3. Contact9's membership check samples only points inside the sliver.** `contact9_side_codes.rs:159`. That is why M2's `In`-everywhere body passes the row. EXECUTION: the probe adds outside points and a far point.

## NOTE
- **n1. Differential (claim 3).** EXECUTION. Over 6857 tests (topo, sweep, editor-core at ε 1e-9) and topo at ε 1e-12, the logged changes are only the PR's own rows. 174 refusals were proposed off the f64 sums (164 at ε 1e-9; 13 + 2 at 1e-12): 162 certified `Void` and 12 straddled (8 of them in my probe). Nothing is newly refused, so claim 4 holds on the suites.
  - Contact9 and the tier3 row are green at ε 1e-6, 1e-9 and 1e-12. Topo at ε 1e-12 has 2101 of 2103 passing. The two reds are `rigid_map_near_eps_plane_nurbs` (known, main's) and `every_suite_file_is_aggregated`, which my mounted probe caused.
  - I did not check this head's CI; my verdict does not rest on it.
- **n2. Claim 2 holds as stated.** Exact volumes at ε 1e-12 / 1e-9 / 1e-6 are 5.8333e-19 / 5.8333e-13 / 5.8331e-7, all matching the oracle, and every sliver passes tier 3. But the e2e grid at ε 1e-12 reads the ∩ volume up to 3.5e3× wrong, and negative in 4 poses (M2).

## Style (exercised: Q1 Q2 Q3 Q4 Q5 Q6 Q7; Q8 not: validate.rs is >10k lines, and I read only the touched regions)
- `props.rs:707`: `Round`'s doc says "a sign stands only where the second excludes zero too". Check 7's pass side is never re-derived (`validate.rs:4367`), so the doc promises more than the code does. The same overclaim is in the PR's "What now holds". Q5. likely.
- `props.rs:728` and `props.rs:2468` both build `VolumeEnclosure{lo=hi=volume}` from `rederive`; that is two copies. `shell_role` (`validate.rs:4772`) and `plus_v_by_sign` map `plus_v_certify` differently. Q1. sure.
- `validate.rs:4777`: `Err(_) => Some(None)` folds a re-derivation refusal silently into "no role". That is not fail-loud: check 7 reports it as `VolumeUncomputable`, and check 10 drops it. Q7. likely.
- `validate.rs:4367`: the paragraph defending the uncertified pass is longer than the code. Its figure of "45 km" is derived, and no guard re-measures it. It is filed in the lane-free item, so it is scheduled. Q2/Q6. unsure.
- The check-7/backstop duplicate (disclosed, left for the door unit) is a fresh instance of the duplication the PR's own `rederive` lift removes. Q1. sure.
- `work/reach/lane-free-volume-sign-reads-decide-on-a-rounded-sum.md` gives `at_infinity_side` P3 and the phrase "a sliver solid read Void". M2 shows it live on a boolean output, which this PR now certifies. Q4. likely.

## Probes (`probes/`, mounted temporarily, not shipped as code)
`probe_r2_inverted.rs` (M1), `probe_r2_e2e.rs` (widened poses, all ops, both orders, membership), `probe_r2_reuse.rs` (results reused as operands), `mutant_rederive.patch` (zero/double mutants and the interval print).
