# Review of PR 6806 (`spiricarm/horn-cusp`): R2

**Frozen head reviewed:** `0c3420d4cb983431637bb3dfbe16586c2a30ee40`. **Probes:** branch `spiricarm-rev6806-r2/probes` (`062e5b490e`).

## Verdict: APPROVE-WITH-FIXES

I found no unsound reading. The height reading holds against an independent 60-digit oracle. MG is equivalent over every geom-brep and sweep row. The fixes are about the tests: one test can go red only coarsely, and the fuzz's input is not an enclosure beside the cusp. There are also some stale docs.

## Findings

**MINOR-1. `closed_form_limb2.rs:limb_2_against_a_horn_reads_the_sheet_up_to_its_cusp` lets a 16× weakening of the height reading through** (claim 6). These mutants were applied to `Meters::at_height`'s return, with `probes/r2_mut.py` (debug build):
- MC (no reading) and MWadd (+1e-12): red at the limb-2 horn row.
- MW16 (×16): green at the limb-2 row and at both fuzzes. Red only at the two static rows.
- MW0 (ρ_s(z_near) taken as 0): green at the limb-2 row, the fuzzes and `…own_height`. Red only at `…least_that_reads`.
- MU (reads 0, unsound): red at limb-2 (the 1e-6-off arm), at both fuzzes and at the static rows.

So the limb-2 row does reach its gate, and it guards soundness. Its only power against a looser reading is the band: a loss of about four bits beside the cusp is invisible to it, and only the static rows pin tightness. Confidence: sure (executed).

**MINOR-2. The fuzz's `f_sup` is not an enclosure of `|f|` beside the horn's cusp, and the absolute slack is at the scale of the cusp's readings** (`envelope_lemma_fuzz/projected.rs`: `a_spindle_bound_is_this_sheets_distance_at_interval`, `sheet_form`; claim 7).
- `f_sup = 2·max f64 sheet_form` cancels beside the cusp. At R = r = 1.1471618723357644, (ρ, z) = (3.0136e-8, −2.6295e-4), the exact |f| is 6.93e-23 (Decimal, 60 digits) against 2× f64 = 6.14e-23.
- `over`, fed that `f_sup`, gives `certified` = 1.94e-16 where the exact distance is 2.18e-16, 11 % short. A second horn point is 3.4 % short.
- The assertion `far ≤ hi·(1+1e-9) + 1e-15` absorbs this. But 1e-15 is the cusp's own scale: the static row's bound is 1e-15 and the box is 4.4e-16 wide. So the horn draws this PR adds cannot see an under-read below 1e-15 beside the cusp, which is exactly where the new reading works.

The new `m − r` / side oracle is algebraically right: off = (m² − r²)/(m + r), and the foot's ρ ≥ 0 ⇔ rρ + R(m − r) ≥ 0. It is not exact, though: it is f64, and it shares `geom::sheet_apex` with the code under test. Confidence: sure (measured), likely on the consequence.

**NOTE-1. Height reading upheld** (claim 1). `probes/r2_oracle.py` is a Decimal oracle at 60 digits: the exact distance to the arc |v| ≤ v*, sampled on a 7×7 grid over each box. The probe test `r2_probe_dump_at_height` drew 60 000 boxes:
- surfaces: the horn, near-horns (gap 1e-14 to 1e-6), apples and lemons;
- boxes: beside the cusp or apex, straddling h, near |z| → r, on the other sheet, and random; points and boxes alike.

Interval `at_height` had **0 violations**. The f64 instantiation under-reads 1 212 point boxes by rounding (it reads 0 where the truth is 1e-19 to 1e-16). That one is uncertified and only the f64 fuzz uses it. Confidence: sure.

**NOTE-2. MG upheld** (claim 2). The proof holds: ρ_hi − ρ_s ≥ ρ_hi + |R| ≥ √(ρ_hi² + (r − h)²) ≥ apex. It uses r ≤ h + |R| and far < r, `apex_root` is always `Some` on `Spindle`, and the fold seeds from apex. At Interval, a lemon divisor straddling 0 gives an entire interval, and the min keeps apex's upper end.

Executed: MG over `cargo nextest run --release -p geom-brep -p sweep` (6 099 rows, 141 `#[ignore]`d skipped). Five rows failed:
- `recourse_roster::every_decided_name…`
- `a_needle_cone_cut_off_its_apex…`
- `the_seam_split_census…`
- `the_steeper_tilts_union…`
- `the_walk_matches_the_serial_golden_at_four_threads`

**All five fail identically at the unmutated head in release**, so they are release-profile artefacts, not MG. Not run: topo, mesh and the other crates, or any ε other than the default. Confidence: sure.

**NOTE-3. `over` is byte-identical, and the other kinds are unchanged** (claim 3). The diff touches only `certified` and adds `at_height`. `over` and `over_at` are untouched. Non-sheet kinds give `over.or(apex)`, which is main's `.ok().or_else(apex)`, except that `apex_root` is now evaluated eagerly (a `√f` on the cone, harmless). Confidence: sure.

**NOTE-4. The sheet `v` read at `Interval` changes more than the PR body says** (`chart_point.rs:ChartInverse::point`; claim 4). `window()` has one caller, and `SheetTube` has one other reader, `projected::raw`, which is `unreachable!`. Beyond "the angle window's read renamed", `Azimuth::near` differs from `principal_in` in three ways:
- an `Interval` direction whose length enclosure reaches 0 now reads the whole window [−π, π], not 0;
- the cut-side decision is taken at `Band::least` (F31) rather than at the caller's band, and no longer logged as `pcurve_chart_cut_side`;
- `band` goes unused for a sheet.

None of these meets a sheet point near the surface: a zero length is the tube centre, a distance r off. A straddle of ±π reads past π, as the latitude window did before. Not executed. Confidence: unsure.

**NOTE-5. The walk's cusp copysign is reached only by a horn** (`mesh/src/walk.rs:loop_polygon`; claim 5). `sheet_apex.v = atan2(h, −R)` equals π exactly iff h = 0 and R > 0. On the other kinds, a sphere's poles are ±π/2 and a cone's apex is 0, and a torus wraps. The copysign is correct for the needle, the lower needle and the four-arc horn: MH is red per the PR, and all three pass here. Confidence: likely.

**NOTE-6. Residue and done-whens** (claim 8).
- `limb-2s-floor-on-a-sheet-reads-the-whole-torus` exists on the spiricarm slate.
- `implicit.rs` (227) is dismissed in the PR body as read by nothing at a cusp, and has no row. Acceptable if that is true; I did not verify it.
- Both done-whens are met. The wide apples mesh (they pass here). `SHEETS` gains (0.999, 1). The lune rows build, mesh, pass tier 3 and measure. MC is the red row the cusp row asked for.

Confidence: likely.

## Style

Exercised: Q1, Q2, Q3, Q4, Q5, Q6 and Q7. For Q8 I read `Meters`' impl end to end (about 1688–1965), not all 3 292 lines of `projected.rs`.

- **S1 (Q5).** `projected.rs:Meters::certified`'s first doc sentence still says "`over` … else `apex_root`". On a sheet it is now the least of three. Confidence: sure.
- **S2 (Q4).** `projected.rs:Meters::apex_root` is documented as "a bound … wherever the lever does not decide positive". On a sheet it is now always in the min. Confidence: likely.
- **S3 (Q1).** `at_height` spells h² two ways in one function: `sheet_apex(..).height` for the guard, and `(minor − major)(minor + major)` in `level`. Confidence: sure.
- **S4 (Q1).** The |z| range of a z-box straddling 0 is computed twice in one impl: `over_at`'s `z_min` and `at_height`'s `near` (`z.0 <= 0.0 && 0.0 <= z.1`). Look for a third in `pcurve_cache.rs`. Confidence: likely.
- **S5 (Q1/Q7).** "Is this a horn" is spelled `sheet_apex(..).height == 0.0` in `topo/chart.rs:Chart::poles`, but `vp == PI && !v_domain_wraps()` in `walk.rs`. The second identifies the horn by a float coincidence. Confidence: likely.
- **S6 (Q7).** `certified` dispatches on `Spindle`, then `at_height` re-matches it and returns `None` otherwise. The fold seeds from `apex: Option` and uses `least.map`, which reads as if apex could be `None` and silently drop the others. On a sheet it never is, but nothing says so. Confidence: likely.
- **S7 (Q2).** `at_height`'s 14-line doc carries rationale about the history of the alternatives ("where `|f|` over the lever reads its rounding over z²"). That is PR-body material. Confidence: unsure.
- **S8 (Q3).** `the_football_revolves.rs:holds_by` accepts `|got − exact| ≤ 1e-10·exact + got.pad`. The pad is the kernel's own, so the check gets easier as the enclosure degrades. This predates the PR. Confidence: unsure.
- **S9 (Q6).** `limb_2_against_a_horn_reads_the_sheet_up_to_its_cusp` took 1.7–2.0 s in debug here, against 0.07 s in release. Its sibling `…up_to_its_apexes` is in the slow set, and this row is not. Confidence: likely.
- **S10 (Q6).** The PR's claim that "no `Interval` row reads a horn's cusp `v`" rests on a sweep, with no guard. Confidence: unsure.

## Claims

1. Upheld (MINOR-2 covers the oracle around it).
2. Upheld: proof checked; executed over geom-brep and sweep.
3. Upheld.
4. Upheld, with undisclosed differences (NOTE-4).
5. Upheld.
6. Lunes and wide apples upheld; the limb-2 row is coarse (MINOR-1).
7. Algebra upheld; not exact and not independent of the harness's `f_sup` (MINOR-2).
8. Upheld.

## Disclosures

I read no `spiricarm-rev6806-*` branch but my own, and none of the PR's comments or reviews. I posted nothing to GitHub. No accidental glimpse.

The probes branch makes `at_height` `pub(crate)` for the dump. My first MG run in debug was killed for time (it was mine, by PID) and rerun in release. A worker restart also killed one baseline build, which I reran.
