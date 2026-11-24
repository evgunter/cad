# Review: PR 7652, `oneround/spline-control-rounding`, frozen head `3f6705a42`

Reviewer lane, ONEROUND. Merge base `64aeba04f`. Brief: `analysis/oneround/briefs/spline-control-rounding-review-brief.md` at `3ed95540b`.
Probes and this report are on `oneround/spline-control-rounding-review`.

## Verdict: APPROVE-WITH-FIXES

`Rounded::step`'s bound holds against my own exact arithmetic, including chains of up to 360 composed steps. The power-of-two exemption is sound where the product is normal. Before merge, the unit owes:

- one gate row it turns red (MAJOR-1);
- one sibling site where the invariant its new row pins is still broken, pre-existing but restated as true in this diff (MINOR-1);
- a false claim in its new public `at_middle` door (MINOR-2);
- a hand formula it says is gone (MINOR-3);
- an unpinned guard it says it made explicit (MINOR-5).

Two PR-body claims about the differential are wrong: "no aligned read grows", and the account of why the walks diverged (MINOR-4). MINOR-6 is main's.

## MAJOR

**MAJOR-1. geom-core's recourse roster is red on the frozen head and green on the merge base.** DEMONSTRATED, sure.

- `geom-core::all recourse_roster::every_decided_name_is_read_and_every_site_is_listed` fails with `an indirect site not listed: running.rs: dehomogenized: name`.
- Cause: `running::dehomogenized` (`crates/geom-core/src/running.rs:312`) and `Tracked::dehomogenized` are new funnels. Both forward their caller's `name` to `Rounded::divisor`, and the roster requires every forwarder to be an entry (`crates/geom-core/tests/recourse_roster.rs:178`).
- The same row passes on `64aeba04f` (my run, a clean stash of the base worktree).
- The row is in the slow set, so the `ci` profile the PR ran never reaches it. It reds the nightly on merge.

Second half, BY-INSPECTION, likely:

- topo's roster cannot see the new name `spline_trim_weight` at all. The `decide` happens in geom-core under a name forwarded across the crate boundary.
- So a doorless topo name (no `bool_`/`split_`/… prefix, and no `EXCEPTIONS` entry beside `spline_trim_clear`/`spline_trim_arm`, `crates/topo/tests/recourse_roster.rs:194`) went in unregistered and unflagged.
- topo's roster is red on base and head alike for main's own reasons (`live.rs`, `census.rs: meeting_point`, `join.rs`, `section_cert.rs`, `sheet_apex_rows.rs`), so it could not have flagged this anyway.
- `pcurve_spline_chart_weight` sits under geom-brep's `pcurve_` door. Its roster passes on head.
- No K-stats table or other census names decision names (I grepped `docs`, `scripts`, `.github` for an existing name: only the roster files carry them).

## MINOR

**MINOR-1. `Patch::split` still drops `t`'s rounding for a ratio that rounds to the half.** DEMONSTRATED (red row), sure. Pre-existing, but this diff rewrites the code and comment and asserts the opposite.

- `crates/topo/src/spline_trim.rs:304-310 (the test at 307)` takes `Tracked::exact(st)` whenever `s == 0.5`. The new comment says "The exact half is a halving's; any other fraction is a ratio of the window's differences".
- But `Patch::restricted` (`spline_trim.rs:370,374`) passes `(w0 − r0)/(r1 − r0)`. That ratio rounds to exactly `0.5` for ordinary knots while the true ratio is off it, for example `(0.58 − 0.06)/(1.1 − 0.06)`. A random search found 120k such triples, the true ratio up to `0.99u` from the half.
- This is precisely the mutant the PR calls "pow2 also exempts t", committed at a caller. The PR's new row `a_step_by_a_ratio_rounded_to_a_power_of_two_holds_its_drift` pins the invariant at the door. The site's sibling was not swept (style Q4).
- Probe `spline_region::rows::review_probes::review_a_restriction_at_a_ratio_rounded_to_the_half_holds_its_drift`, on my branch: a bilinear chart on `u ∈ [−0.0289…, 4.0108…]`, restricted at `w0 = 1.99098…`.
- Result: the cut point drifts **1.19× the worst bound its channel carries over the whole cell** (`1.713e-17` against `1.440e-17`). It is judged in `Expansion` arithmetic against the restriction at the true ratio. An independent Python/`Fraction` mirror gives the same `1.18947785766508`.
- Why no test announces it: the site's drift row `the_cut_rounding_bound_holds_the_cut_drift` judges a split at an `f64` fraction against double-double at the same `f64` fraction (`spline_region_rows.rs:1418-1426`). Its premise therefore excludes the ratio's rounding (Q3).
- At the floor this excess is almost certainly swamped by `rounding_charge(2·reach)` (16 half-ulps of the reach, against an excess of about `0.3u·|P|`). I found no verdict it moves. It is a too-small bound inside a bound.

**MINOR-2. `Tracked::at_middle` is not off every member of its bracket by at most its bound.** DEMONSTRATED (red row), sure.

- `running.rs:446-458` returns `bracket_mid(lo, hi)` with error `rounding + (hi − lo)·0.5`. Both are rounded to nearest and neither is charged.
- For `[1, 1 + 2⁻⁵²]` the middle rounds (ties to even) onto `1`. The member `1 + 2⁻⁵²` then lies a whole ulp from it, against a bound of half an ulp.
- Probe: `geom-core::all review_spline_rounding_probes::review_at_middle_holds_every_member_of_its_bracket`.
- This is the old `half_width` plus `middle`, moved into a new **public** door whose doc ("off the exact value by its bound and by the half-width of its bracket") states the false claim.
- It feeds topo's Interval-lane pieces (`bezier_pieces`' `out_of_scalar`, `image_pieces`' `end`/`reach`), so a piece box can be under-widened by half an ulp at an odd-ulp-wide enclosure. Pre-existing in effect. Its scope and confidence are recorded under the false-claims class below.

**MINOR-3. "Every hand formula is gone" (PR body, correction 3) is not true of `image_pieces`' `reach`.** BY-INSPECTION, sure.

- `spline_trim.rs:1278-1282`: `off = |x.value − h_c/h_w| + x.error`, then `h[c].with_charge(|h_w|·off)`.
- That is a bound added and scaled by hand. The quotient, difference, sum and product it does in `f64` are uncharged. Only the final add goes through a door.
- Also by hand, pre-existing and not claimed:
  - `Piece::bbox`'s widening `r.u.0 − worst[0]` (`spline_trim.rs:684-687`): a rounded-to-nearest subtraction that can land inward;
  - both floors' `gap − read − cut` and root sums.
- `Rounded::lower_end`/`upper_end` exist for the widening.

**MINOR-4. The differential's account of what moved is partly wrong.** DEMONSTRATED, sure.

My instrument:

- the same logging patch on base and head, never committed;
- reads logged at geom-brep's `cut_rounding` call in `Net::floor`, topo's `Patch::floor` cut, and topo's `Piece::bbox` per-channel worst;
- then two more targeted passes: per-point box bounds, `chain_parity`'s clear decision, and `spline_region`'s carrier floor decision;
- run over the `ci` profile of geom-brep + topo (6711 / 6713 tests) at default ε and at `1e-12`, `CAD_FUZZ_SEED=12345` in both trees. No slow rows, so my counts are below the PR's.

Reads per site, base → head (default ε; at `1e-12` the classes are identical and the counts slightly higher):

| site | tests | reads | equal | smaller | larger |
|---|---|---|---|---|---|
| geom-brep cut (`f64` only) | 34 | 50,524 = 50,524 | 48,700 | 1,492 (least 0.954) | 332, all < 4e-16 relative |
| topo cell cut | 21 | 211,384 → 211,378 | 194,678 | 16,646 (least 0.873, median 0.998) | 52, all in the one walk whose length changed |
| topo piece box `u`/`v` | 28 | 52,481 = 52,481 each | 47,449 | 5,031 (least 0.857) | **1 aligned growth each, in a walk of unchanged length** |

The 332 cut growths are ulp-level reassociation; I agree with the PR there. The topo cut numbers reproduce the PR's own (16,646; 0.873; 52).

Two corrections:

- **The box growth is not "a one-read offset in a walk that diverged".**
  - In `spline_region::rows::a_query_on_a_far_trim_curve_along_v_is_given_no_side` both trees read 4,951 boxes and make 4,945 chain visits with 1,553 clears. Read 2737 grows by **1.26 % (u) and 1.42 % (v)**, and nine per-point bounds grow by up to 5.4 %.
  - The cause: `chain_parity::ray_parity_as`'s clear decision for one piece moved from not-clear to clear at visit 1634 (and again at 1708, …). The walk then visits a different piece order of the same length.
  - The sorted multisets show no growth, so no bound grew. But "no aligned read grows" is an artifact of aligning by index, and the decision that moved is unreported.
- **The 1910 → 1902 walk** (`a_point_on_a_far_rational_sheet_is_not_read_off_its_carrier`) changed because the carrier floor's logged decision (`spline_region.rs:258`, `SPLINE_REGION_CARRIER`) moved from **Zero to Indeterminate** at read 589, and once more later (base 2 Indeterminate, head 4).
  - "No verdict class moved" holds for the outcomes the PR logged (`read_capped`, `place`, `ring_decks`), not for these logged decisions.
  - Both moves go toward the truth (a smaller charge), and neither test's verdict changed.

At `1e-12`, with my pinned seed, both trees fail the same two rows: `envelope_lemma_fuzz::projected::a_line_near_a_ruling_…` and `…a_carrier_on_its_chart_…`. Neither is the red the PR names. They are seed-dependent and identical in both trees, so they are not this unit's.

Not done: the TRUE error of a sample of the shrinks. I checked the exemption's soundness at the door instead (C2, below).

**MINOR-5. `Piece::bbox`'s new explicit positive-weight test is pinned by no row.** DEMONSTRATED (surviving mutant), sure.

- The PR names `spline_trim.rs:668` as the place where positivity "is now explicit". Replacing that test with `if false` turns no `ci` row red among the 524 that reach `running`, the spline walks and the trim.
- With the test dropped, `dehomogenized`'s absolute guard admits a negative weight whose magnitude clears its bound. `bbox` then returns a finite box for a piece whose hull is not its curve's.
- No pcurve image in the suites has a negative weight, so the guard can be removed without notice.
- Slow rows not run.

**MINOR-6. The demos' lockfiles lag main.** BY-INSPECTION, sure. Not this unit's.

- `cargo build` in `demos/tour` and `demos/wild` on the frozen head adds `quantity` to their `Cargo.lock`. The PR touches no manifest, so a dependency added on main left them behind.
- Both build.

## NOTE

- **N1. `Rounded::step` reads a zero bound where its error is not zero, when every first-order share vanishes.** DEMONSTRATED (green row asserting the hole), sure.
  - Example: `x = 0 ± 1e-6`, `y = 0` exact, `t = 1 ± 3u` gives a bound of 0 against an error of `3u·1e-6`.
  - The missing terms are `e_t·(e_x + e_y)` and `u·|d|·e_t`, second order. That is the module's stated contract, and `Mul` on two zeros carrying bounds has the same shape.
  - It is reachable only at `t ∈ {0, 1}` carrying `e_t`. Boehm insertion's `α` is strictly inside `(0, 1)` (`refine_plan` emits `Form::Blend` only), and the halvings carry no `e_t`.
- **N2. Underflow is uncharged.** DEMONSTRATED, sure, and documented in the code.
  - A normal power of two times a value into the subnormals is inexact while the bound reads 0 (`y = 2⁻¹⁰²² + 2⁻¹⁰⁷⁴`, `t = ½`). Subnormal operands give a bound of 0 against an error of `2⁻¹⁰⁷⁶`.
  - An overflowing power-of-two product reads `inf` with an `inf` bound, which is loud.
  - No site is within 300 decades of this.
- **N3. `Tracked::ratio` takes the caller's word.** BY-INSPECTION, likely.
  - It charges `3u·|t|` to any value. The ratio's true first-order rounding is `3u·|t| + O(u²)`, exceeded at second order.
  - `apply_homogeneous` would hand it `1/a` or `1/(1 − a)` for unblend forms (4–5 roundings). Only removal plans emit those, and the walks call `refine_plan`, so this is unreachable today. Nothing in the type enforces it.
- **N4. The divisor read is now a logged decision for every control point read.** BY-INSPECTION, unsure.
  - In topo's `Piece::bbox` this happens at `f64` even inside an `Interval`-lane run, because pieces are `f64` at every lane. The cells' weight decision is logged only at `f64`.
  - Lane-diffing rows (`m10_p_lift::an_f64_profile_log_and_an_interval_one_diff_to_no_flip`, editor-core, slow set) may see an asymmetric record. I did not run them.
- **N5. A wider sweep finds hand arithmetic on a `Rounded`'s `.error` field across geom-brep.** BY-INSPECTION, likely.
  - Production examples: `curve_face/circle_roots.rs:248`, `implicit.rs:2030,2411`, `curve_face/pose.rs:1491`, `chart_singularity.rs:1128,1151`; my pattern `\.error[^_a-zA-Z(]` gives 59 hits outside geom-core, many of them tests.
  - None is the control-point shape. The PR's sweep (tuple types, `UNIT_ROUNDOFF` in spline-vocabulary files, struct fields named `rounding`/`error`) could not match a hand formula on a `Rounded`'s field, the shape the unit itself removed from `Piece::shifted`.
  - This is outside the unit's fence; recorded as a class.

## Claims

- **C1 `step`'s bound is sound: holds (first order).** Executed:
  - My own Python mirror of `step` with `Fraction` exact arithmetic: 200k single steps with `t` uniform in `[−3, 3]`, in `[−1e6, 1e6]`, powers of two `2^±30` and negative powers, `x`/`y` cancelling to `1e-12`, scales `1e-9` to `1e6`, input bounds `0` to `1e3·u` of the value and `e_t` up to `30u·|t|`. Worst `0.99999999999954` of the bound, against the worst over the box of exact inputs.
  - Composition: three seeds of 30 nets of degree 3–12, each cut along 30 nested splits at the half or a true knot ratio, chains of up to 360 steps. Worst `0.854` over about 22k reads.
  - Edges: N1, N2.
- **C2 the power-of-two exemption: holds for normal products.**
  - `power_of_two` (`running.rs:301`) tests the mantissa bits with `is_normal`, so `0`, subnormals, `inf` and `NaN` are excluded, and negative powers count (still exact).
  - Subnormal and overflow results: N2.
  - `e_t` is kept under the exemption (`d.abs()*t.error` is outside the `product` branch). The re-planted mutant "pow2 also exempts t" is in the table below.
  - The fixture `t = (1 + 2⁻⁵³)/(2 − 2⁻⁵³)` is not "1.5u off" the half: it is `0.75u` off (`(1+u)/(2−u) ≈ ½ + 0.75u`). "1.5u of it" read relative to `t` is right, so the row's prose is ambiguous rather than wrong.
  - The real counter-instance of this claim is MINOR-1, at a caller.
- **C3 the division and its refusal: holds.** Every former signed-guard reader still refuses a negative weight before the absolute guard:
  - geom-brep `Net::floor` and topo `Patch::floor` test `structure_positive` first;
  - `bezier_cells` refuses non-positive weights;
  - `Piece::bbox` now tests `value > 0` explicitly, and `Piece::shifted`, `image_pieces`, `bezier_pieces` and `out_of_scalar` divide by nothing.
  - So no reader newly admits one. A weight whose rounding reaches it yields `None` → `INFINITY` / poison / `None` at all three sites.
  - The rows' weight (`0.25` carrying `1`) is a fixture chosen so the room is negative rather than exactly zero. That is legitimate: the contract row is the geom-core `a_weight_whose_bound_reaches_it_gives_no_point`, with three placements.
  - The PR's own guard-dropped mutant: see the mutant table.
- **C4 `Tracked` and the choice: sentences hold, with an awkward state.**
  - At `Interval`, `rounding` is `0` at every constructor (`exact`, `ratio`, `charged`, `dehomogenized`), because the fields are private.
  - The pieces' half-width goes in through `at_middle` only, one door, so the two cannot disagree. But that door under-states by half an ulp (MINOR-2).
  - `Dual` is excluded from the walks by their `Decide<Value = T>` bound, not by `Tracked`'s. `Tracked<T: SpanLocate>` admits `Dual`, and its doc covers it.
  - `Witness` has two variants, so "charged where `Inexact`" is the only switch available; no enclosing-but-inexact scalar exists.
  - The trap (style Q1) is below.
- **C5 no hand formula is left: false.** See MINOR-3 and N5.
  - My sweep over the three files: `\.error|\.rounding\(\)|UNIT_ROUNDOFF|EPSILON|rounding_charge|with_charge|\.room\(\)|ulp|next_up|next_down`.
  - It cannot match a bound held in a local `f64` and compared by hand, such as `worst` in `bbox`. I read those by hand.
- **C6 what moved: partly wrong**, see MINOR-4.
  - Sites no `ci` suite reaches on a lane: geom-brep's cut at `Interval`, by construction (floors return before the cut).
  - The piece box is reached at both lanes.
- **C7 the mutants: see the table.** The five removed hooks:
  - `spline-trim-ratio-uncharged` is now covered by `ratio uncharged`;
  - `spline-bound-undercut` and `spline-trim-bound-halved` by `dehomogenized halved`;
  - `spline-trim-halfwidth-dropped` by `at_middle`'s half-width;
  - `spline-product-uncharged` by `product dropped`;
  - nothing they pinned is left unpinned, as far as I could find.
- **C8 the new decisions and gates.**
  - Names: MAJOR-1.
  - `bare-rounding-sites.tsv`: the gate counts every use of a `running` `pub` item and every `.with_…(` call outside geom-core (`scripts/gates/bare-rounding-allowlist.sh` header). The rise (5→8, 8→26, 1→9, 1→28) is therefore the instrument working as designed: multi-line hand formulas that each named `UNIT_ROUNDOFF` once became one counted door use per operation.
  - It is not the unit adding charges. Nor is it a shrink toward oracles: every new site is a charge the README's rule says no site makes. The gate passes (`run-gates.sh` exit 0, `GATE_BASE=64aeba04f`).
- **C9 the PR body and the row.**
  - Corrections 1, 2, 4, 5 and 6 are true of the tree. Correction 3's "all fixed" is not (MINOR-3).
  - "No public signature changed" is true. The new public items are `Rounded::step`, `running::dehomogenized`, `Tracked` and its `exact`/`ratio`/`value`/`step`/`dehomogenized`/`at_middle`/`rounding`.
  - `ratio` and `at_middle` are public claims the type cannot check (N3, MINOR-2).
  - The PR's red: see the `1e-12` paragraph under MINOR-4.

### Mutants

Each mutant was planted in a pristine frozen-head worktree and run against the `ci` profile of geom-core, geom-brep and topo, filtered to `running|spline|trim` (524 rows; slow rows not run). The first ten re-plant the PR's own mutants; the last five are mine.

| mutant | red rows (`ci`) |
|---|---|
| step: product term dropped | 1: `a_steps_product_and_difference_roundings_are_each_owed` |
| step: `t`'s rounding dropped | 2: `a_ratios_own_rounding_is_owed`, `a_step_by_a_ratio_rounded_to_a_power_of_two_holds_its_drift` |
| step: power of two also exempts `t` | 1: `a_step_by_a_ratio_rounded_to_a_power_of_two_holds_its_drift` |
| step: difference term dropped | 2: `a_steps_bound_holds_its_drift_against_exact_arithmetic`, `…_each_owed` |
| step: bound halved | 12: geom-core ×5, geom-brep cut-drift, knotted and quarter, topo cut, pieces, far-trim and wide-interval |
| quotient: weight's bound left out | 4: geom-core ×2, geom-brep `a_cells_cut_has_no_bound_…`, topo knotted |
| divisor guard dropped, room unclamped (the PR's) | 5: geom-core ×3, both site refusal rows |
| divisor guard dropped, room still clamped at 0 | 3: geom-core only. The sites read the clamped room's quotient as `inf` and refuse anyway, so this mutant is equivalent at the sites |
| `ratio` uncharged | 3: `a_ratios_own_rounding_is_owed`, the pow2 row, `a_tracked_value_charges_where_the_scalar_measures` |
| `at_middle` half-width dropped | 2: `a_tracked_value_charges…`, topo `a_query_on_a_wide_interval_trim_image_is_given_no_side` |
| **mine**: `x`'s share at 0.75 of `\|1 − t\|` | 5: geom-core exact-drift row, geom-brep quarter, knotted and cut-drift, topo wide-interval |
| **mine**: exemption widened to every `\|t\| ≤ 1` | 1: `…_each_owed` |
| **mine**: sum's rounding dropped | 8 |
| **mine**: `Tracked` charges nothing at `f64` | 13 |
| **mine**: `Piece::bbox`'s positive-weight test dropped | **0: survives** (MINOR-5) |

The four mutants that drop or narrow a term of the step (product, `t`'s rounding, pow2 exempting `t`, the widened exemption) are each killed by one or two static geom-core fixtures, and by no site row. The PR disclosed this for the first two; the class is wider.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (whole file: `running.rs`, 1083 lines, read end to end).

- **S1 (Q1, the trap). `Tracked<T>` is a second `Rounded`.** Sure.
  - `running.rs:340-480` adds a type for a value and its rounding beside `Rounded`, with parallel doors: `exact`/`exact`, `step`/`step`, `Sub`/`Sub`, `Mul`/`Mul`, `Tracked::dehomogenized`/`running::dehomogenized` (one name, two items).
  - The two types meet only through `at_middle`. topo then carries both: `Control = [Tracked<T>; N]` in a cell, `Node = [Rounded<f64>; 3]` in a piece.
  - `ratio` exists only on `Tracked`, so geom-core's own row spells the `Rounded` twin by hand (`Rounded::exact(t).with_charge(3.0 * UNIT_ROUNDOFF * t)`, `running.rs:1053`): a second spelling of the ratio charge on the day the door lands.
  - Could the PR's argument against `Rounded<T>` be met inside `Rounded`? Mostly yes. Every objection in the PR is about `Rounded<T>` with a `T`-typed bound, which `Tracked` already sidesteps by delegating to `Rounded<f64>` on a shadow. A `Tracked` that is literally `{ value: T, shadow: Rounded<f64> }` would have one arithmetic and no parallel door list.
  - I think that is the better final state. The current one is acceptable, because `Tracked` holds no arithmetic of its own.
- **S2 (Q4). The exact-half test at `Patch::split` is a caller-side flag read from the value, the shape the PR says it retired.** Sure. MINOR-1, `spline_trim.rs:304`.
- **S3 (Q5). False claims in the touched code.** Sure. Class: claims that a door or site makes and the code doesn't keep.
  - `Tracked::at_middle`'s doc (MINOR-2);
  - `split`'s comment (MINOR-1);
  - the PR's "every hand formula" (MINOR-3);
  - `step`'s doc says "none where `t` is a power of two" without "normal, and the product normal" (N2).
  - Where else to look: `Tracked::ratio`'s "three roundings".
- **S4 (Q3).** Likely.
  - The site drift rows (`the_cut_rounding_bound_holds_the_cut_drift`, and geom-brep's knotted cut-drift) judge against the same `f64` fraction or ratio the site was handed, so they cannot fail on a ratio's own rounding.
  - The PR's door-level ratio rows fix that at the door only.
- **S5 (Q2).** Likely.
  - `Tracked`'s type doc (21 lines) and the PR's long "the other answer would have left possible" justify a seven-door type.
  - Per the lane's stance that is mild evidence, and S1 is the finding it points at.
- **S6 (Q7).** Unsure.
  - `Piece::bbox` now checks `value > 0` by hand beside the door's `|w| − e` test, which makes two guards for one condition.
  - A signed `Rounded::divisor` variant (or a `positive_divisor`) would be one door.
- **S7 (Q6).** Likely.
  - The disclosed differential rests on a logging patch "never committed". No register re-takes it, and no guard goes red when a bound grows.
  - The claim "no aligned read grows" is now known to be index-aligned (MINOR-4). No schedule is attached.
- **S8 (Q8).** Sure.
  - `running.rs` is now 1083 lines. Half of it is tests. `Tracked` sits between `Rounded`'s impls and the vector helpers.
  - The module doc (lines 1-17) still says "at `Interval` the value is already an enclosure of the chain and the bound rides along as a second one". That is true of `Rounded` and the opposite of `Tracked`, and the module doc does not mention `Tracked`.

### Ratings

- **Idiom and structure: 3/5.** One arithmetic, but two carrier types with parallel doors (S1), and the exact-half flag remains at a caller (S2).
- **Test quality: 3/5.** The door rows are exact-arithmetic and mutant-backed (the `Expansion` rows are good). But the site rows cannot see a ratio's rounding (S4), four step-term mutants die only on single static fixtures, the bbox guard is unpinned (MINOR-5), `at_middle`'s claim is unpinned (MINOR-2), and the slow roster that reds was not run (MAJOR-1).
- **Doc and comment honesty: 3/5.** Three false sentences in touched code (S3), and the module doc is stale for the new type (S8).

## What I executed and what I only read

Executed:

- **Probes** (my branch; the one red at `Patch::split`, the topo row in `spline_region_rows.rs::review_probes`, is committed in `f210f5fc`):
  - Python/`Fraction` probes: single step, chain composition, split at a rounded half; scripts kept in my scratchpad, not committed;
  - the topo red row named above;
  - three geom-core rows in `crates/geom-core/tests/review_spline_rounding_probes.rs`: one red (MINOR-2), two green rows asserting N1 and N2.
- **Test runs:**
  - `ci` profile geom-core + geom-brep + topo at default ε: **7926/7926 pass**;
  - recourse rosters and `predicate_name_coverage` on head and on base (MAJOR-1);
  - the differential (MINOR-4), base and head, default ε and `1e-12`;
  - the mutant table (15 mutants).
- **The hosted lint job's steps** on a pristine frozen-head worktree:
  - `cargo fmt --all --check` and the benches fmt: ok;
  - `python3 scripts/work.py lint`: ok;
  - `GATE_BASE=64aeba04f scripts/run-gates.sh`: exit 0;
  - `payload-rung-sweep --check`: ok;
  - first `cargo clippy … --force-warn clippy::disallowed_methods` (under `bash --noprofile --norc -eo pipefail`): exit 0;
  - second clippy piped to `conic_centre_census.py --check`: red only on `curve_face/end_root.rs` (main's known red);
  - `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`: exit 0;
  - `demos/tour` and `demos/wild` `cargo build`: exit 0 (both).

Only read: the PR body, the row, the README section, the CI workflow, `apply_homogeneous`/`form_coef`, `chain_parity::ray_parity_as`.

No helper agents were used.
