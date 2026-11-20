REVIEW of PR 7146 (oneround/section-area) at FROZEN HEAD f0fc46e1. Merge base d77f55bf (git merge-base with origin/main). The branch did not move.

VERDICT: APPROVE-WITH-FIXES, conditional on hosted CI run 44591339043. I resolved that run: conclusion success, head_sha f0fc46e1, PR 7146. It ran fmt, clippy --all-features, rustdoc, the gates, test (kernel) and test (app) at the default eps, and the 1e-6/1e-12 eps step. It skipped the interval backend, tools, python, demos, step-import, viewer, and the slow set apart from new or edited slow rows. The site fix is right and I measured it. What it still owes: 1 MAJOR (a broken test instrument; no shipped verdict changes), two doc errors, and a false headline in one of the rows it filed.

Dispatcher premise corrections:
- "The interval lane": there is no such lane any more. RING-4 deleted the `interval` feature, and what it ran is now the default test job, which did run. ci.yml's `interval` job tests the interval-transcendentals backend crate, which this PR does not reach. So CI skipped no "interval-lane rows".
- Claim 12: `VolumeSignUnresolved.volume` (topo/src/validate.rs:1959) is a (lo, hi) bracket, not a centre ± half-width. (−7.67e-13, 1.80e-14) holds both 0 and the true volume of about 1e-23. See m3.

MAJOR (1)
M1. geom-core/src/sym.rs, `Sym::less_sin`. The node is `Sub(x, Sin x)`, but its value channel is T's series. revalue evaluates a DAG op by op (sym/revalue.rs:168,182). topo/src/revalue_pin.rs:92 asserts that a DAG re-valued at p0 gives the p0 build's own bits. So any revalue pin over a site that reaches chord_bulge or less_sin now PANICS ("revalue no longer computes an op as Sym builds it"). DEMONSTRATED by `join_review_probes::review_revalue_pin_over_chord_bulge`: committed, red at head, green on the merge base. No shipped verdict moves. But the FORK-S4F laundering guard can no longer be used on any curved-area site (profile loop_orientation, section areas, windings). CI did not see it because no existing pin reaches an arc's area at Sym. Fixing it is a design choice: a LessSin op that revalue evaluates through the f64 door, or Sym valuing the node naively. The trait doc's "a recording scalar keeps the difference as its structure" is the premise that broke. sure.

MINOR (5)
m1. topo join.rs `curved_section_rows::a_far_centred_arcs_sliver_is_read_at_its_own_size` stays GREEN at both ε with chord_bulge mutated back to `span - span.sin()`, because it only checks the verdict class at 0.25ε and 40ε. Under that mutant my lens probe reads margins up to 60% wrong (far 1e6 at 1e-9; 51% at far 3e3 at 1e-12). DEMONSTRATED (the mutant survives). A row that pins it is committed: `join_review_probes::a_far_centred_arcs_sliver_reads_its_own_width_to_a_part_in_a_million`. It is green at head at both ε, red on the merge base, and red on the mutant. sure.
m2. geom-core/src/real.rs:2035-2046. `LESS_SIN_SERIES_BELOW` was inserted under `impl Real for f64`'s doc block. Its rustdoc now opens "`f64` is the kernel's default scalar … libm …", and the f64 impl lost its doc. sure.
m3. work/boxfinish/a-corner-cut-…-wrong-sign.md says the bracket "is not an enclosure". It is one, and VolumeSignUnresolved is the right refusal for it. I reproduced the cell exactly (R=1e4, n=(1,1,1), d=20ε). The row quotes head's bits (…286e-13) while citing the merge base (…341e-13). The NegativeVolume cells are real: I reproduced them at R=1e3 and 1e4, d=50ε. P2 fits the NegativeVolume half; the bracket half should be struck. sure.
m4. The `less_sin_series` doc says "within 4¹¹·6/25! < 2⁻⁵⁹ of the sum". That bound is relative to x³/6, not to the sum. Measured against the true value at x=2, the dropped tail is 1.97e-18, which is above 2⁻⁵⁹ = 1.73e-18. Harmless (0.009 ulp). sure.
m5. profile `loop_orientation` verdicts move, every one I checked for the better, and no row pins it. I ran two-vertex arc+chord profiles (crates/profile/examples/review_lens_profile.rs), 200 poses per radius. At 1e-12 the merge base gives 7 definite SliverLoop on loops whose true margin is 10.1–11.9ε (r=1e6). It also gives 4 escalations where the truth is definite (r=1e5), one of them reading −9.2e-12 for a +10.7ε loop. Head: 0 wrong. sure.

NOTE (6)
n1. The verdict that moves end to end is (R=1e4, n=(0.05,1,1), d=12ε): Ok → Escalated. It is right under the predicate's definition, whose perimeter is the conservative reach·|Δt|: my mp oracle gives 9.664769e-9 and head reads 9.664765e-9. But the geometric mean width is 1.159e-8, above 10ε. So the base's Ok was the geometrically true answer, reached on a reading 4 orders off. sure.
n2. On a wider grid (R 1e3/1e4, arc either way, 4 normals, d 2–50ε), 12 outcomes change. All of them become `Join(Escalated split_section_area)` at d=12ε: 2 from Ok, 6 from SectionWindingUndecided, 2 from NegativeVolume. The volume-bracket bits also change. Every new escalation's margin matches my oracle to 6 digits and lies in (ε, 10ε). sure.
n3. f64 less_sin is odd and monotone on every sample. Worst error 3.07 ulp, over 4e5 points down to 2⁻³⁴⁰. ±inf gives NaN, as the old form did. likely.
n4. Surviving geom-core mutants: 506→500; dropping the x²³ term altogether (≤0.8 ulp); `<=`→`<` at the cutoff; the Interval tail [0,1] replaced by the point 1. The last one cannot be detected: the truncation is 2e-18 relative, far inside the outward rounding. Killed: 20→21, cutoff 2→1 and 2→4, f64 naive, Interval naive, Dual slope ×½, Dual slope naive 1−cos, Dual value naive. sure.
n5. Claim 11: the error it can add to 2A is bounded by δ·L, with no far amplification. A wrong definite verdict would need δ ≳ 9ε·P/(2L), past the vertex-on-curve tolerance. I could not construct one. likely.
n6. Outside the fence, I filed work/boxfinish/a-cylinder-faces-flux-sums-two-terms-of-its-far-axis-origins-size.md. `props/curved.rs:1554` `cylinder_face` computes flux = R²·A_chart + origin·A⃗ about the axis origin: two terms of size R·area that cancel to |p|·area, the same shape this PR fixes. It may account for the boxfinish bracket's width. By inspection, likely.

THE TWELVE CLAIMS
1 HELD. My own lens poses: seams at any angle, arcs at five t0, far 1e2–1e6, 600 readings per cell, mp oracle. The base reads a definite Positive with truth ≤ε from far 1e3 at 1e-9 (20/600; 36–66 at 1e4–1e6) and from far 1e2 at 1e-12. It reads 4 Zero with truth ≥10ε at far 3e3. End to end, n=(0.05,1,1) d=12ε returns Ok on the base at R=1e3 and 1e4 (with n1's nuance).
2 HELD, with m4. Every ratio equals (2k+2)(2k+3) exactly. Worst 3.07 ulp; continuous across the cutoff on both sides; odd and monotone.
3 HELD. 4028 boxes (points, wide, straddling 0 and ±2, wider than the series' reach, infinite, 1e300), plus their Dual<Interval> slopes, against mpmath: 0 escapes. Point enclosures are at most 7 ulp wide. [0,1] is a true bound on the tail.
4 HELD for the values: the Dual value is bit-identical to f64; the slope 2sin²(x/2) is right and conditioned; the Dual<Interval> slope encloses; the Probe forwards. FALSIFIED in effect for Sym's DAG: M1.
5 HELD. Algebraically equal to the old term for both traversals and either sign of Δ. Arcs with Δ 3.5, 6 and 5.8 and arc-triangles with the apex on either side of the chord (also at r=1e4) match closed forms. Ellipses with semi-axis ratios 1.41 and 1.73, cut by planes off the cylinder axis, through real corner cuts: margins match the mp oracle to 6 digits. I found no way to build an ellipse lens directly.
6 HELD. Head is 0 wrong in every cell at both ε; read margins are within 2e-9 relative of the truth.
7 FALSIFIED as worded (m5, n2). Every move goes toward the right answer.
8 HELD. benches, demos/tour, demos/wild, tools ×3 and interval-transcendentals all pass `cargo check --all-targets`; nothing outside geom-core implements Real. 140 slow rows that reach areas and windings: 140/140 at 1e-9. At 1e-12, 139/140; I killed geom-brep::all marched_windings_agree_with_a_projected_oracle after 13 min (77 s at 1e-9). It is a seeded SSI fuzz, and pair_section does not reach chord_bulge, but I did not run it on the merge base: could not tell.
9 HELD for the topo row: red with the base formula at both ε, and the author's quoted failures reproduce. But see m1. geom-core rows: n4.
10 HELD. No 1−cos, _m1 or versine door exists. My sweep found only profile (fixed), loop_area (the home), extent.rs:548 (filed) and interval.rs (the door). Reading props/curved.rs found n6.
11 COULD NOT TELL by execution; by the bound, no (n5).
12 PARTLY FALSIFIED: m3. The extent row is accurate (u·r = 1.1e-12 at r=1e4; P3 fits). I did not re-run the NEARPOINT evidence. The from-f64 gate passes, and with the new row removed it goes red, naming from_f64(ratio).

SPEC DEVIATIONS
Reported: the required Real method (accepted, but its Sym arm is M1); the 1e-12 row at far 1e3–4e3 (fine: at 3e3 and 1e-12 I already lose 102 of 600 lenses to arc certification); profile changed (an improvement, unpinned: m5); the end-to-end pose not committed as a test (disclosed with no schedule).
Silent: M1; m1.

RUBRIC
Idiom and structure 4: a small, well-placed site change. Dual spells /2.0 and 2.0* where `scale_pow2` exists.
Test quality 3: m1, n4, nothing pins profile.
Doc honesty 3: m2, m4, m3.

## Style (exercised Q1–Q8; Q8 on real.rs's Real trait section and join.rs from certify_section_area to the end of the file)
- profile/validate.rs:2650 restates the segment term (`r²·θ.less_sin()`), while chord_bulge (loop_area.rs:119) calls itself "the one statement of it". This parallel existed before; the PR touched both and kept two spellings. likely.
- sym.rs:5358 `self.sin()` goes through sin_cos, minting an unused Cos node and computing a cosine it throws away. unsure.
- dual.rs:680 divides by from_f64(2.0) where `scale_pow2` is the door, and is tighter at Interval. likely.
- real.rs:587-599: "Declared, never defaulted…" is a long justification asserting that the Sym form is safe; M1 shows it is not (Q2). likely.
- Q3: less_sin_rows' enclosure row asserts overlap with the bracket, not containment of the truth. sure.
- Q6: the end-to-end test is owed and unscheduled. sure.
- Q1 class, far-centre cancellations: the site (fixed), extent.rs (filed), curved.rs cylinder_face (filed). Look next at the cone, sphere and torus lanes in props/curved.rs and props/quad/rotational. likely.

EXECUTED: the CI run, at the step level; a merge-base worktree with its own target; the lens probe on base, head and the naive mutant at both ε; big-arc and arc-triangle probes; 96 corner cuts on base and on head; 1000 profile lenses on base and head at both ε; a less_sin dump against mpmath at every scalar; 13 geom-core mutants; the topo row against the base formula and the naive bulge; the revalue pin; 7 excluded roots; 140 slow rows at both ε; the from-f64 gate with and without its row.
READ ONLY: the NEARPOINT evidence, k_stats Probe, props/quad.
PUSHED: oneround/section-area-review @ 359ecaea. It holds join_review_probes.rs with 2 rows (one red by design) and ignored printers, two examples, the geom-core dump, and the boxfinish row. The fix pass keeps the rows and may drop the printers. The `mod` line in join.rs and the all.rs include exist only for the probes.
INTERRUPTION COST: small. The container, the targets and the logs survived, so there was no cold build. I re-ran the slow-row pass at both ε (~17 min) and committed first thing on resume.
