# Review R2 of PR 6478 — rotational-chart Green lane, stage 1 (cylinder)

Frozen head reviewed: **`e12e3c8d0017f9b83be830d4881f580afcb28c9d`**. Probes: branch
`spiricarm-rev6478-r2/probes` (`ac8dab727`), file
`crates/geom-brep/src/props/quad/rotational/swap_rows/probes_r2.rs`.

## Verdict: APPROVE-WITH-FIXES

The lane's shape is right, and the projected integrand holds against mpmath truths on
hard chords. The PR files no row for one disclosed gap: the chart term `o·A⃗` reads
the carriers' vector area with no pad, and that leaves an error outside a certified
bracket. A probe shows it. The fix is MAJOR 1; the rest are MINOR/NOTE.

## Findings

**MAJOR 1 — `quad/rotational.rs:chart_term_at` / `topo/props/quad_lane.rs:cut_face_lane`: the
chart term leaves the carrier-to-wall gap uncontained (claim 4 falsified).** `−∮F du`
integrates the region the *pcurves* bound, and pads it, through `residual`, for a true
boundary within `env` of them. `o·A⃗` instead reads the *carriers'* vector area, about a
lever `|o − c|`, with no pad. That mixes two regions. The error is about
`|o − c|·length·env`, and nothing covers it. **Execution:**
`r2_a_carrier_off_the_wall_escapes_the_bracket_far_out` puts a rim carrier
`r·6.25e-10` off the wall. Its certified envelope is 5.3e-10, inside ε = 1e-9. Its
projected image is the exact rim. The chart term is built as `cut_face_lane` builds
it, on a face a thousand metres out. The bracket `[−877.64164550811, −877.64164470884]`
misses the closed form `−877.64164558484` by 7.7e-8. The PR's own swap rows cannot
see this: `swap_rows.rs:face` subtracts the *carrier's* vector area from `closed`, so
whatever the lane reads for the carrier cancels. Two controls pass: the same carrier
on the wall, far out; and the same off-wall carrier near the origin (lever ~0.5).
**Reach:** `c` is the shell's corner (`props.rs:corner_of`), so in a body the lever is
about the body's size. The error grows like size × chord length × env. At the measured
envelopes (≤ 7.8e-11) on ~2 m bodies it is ~1e-9, so no current row reaches it. A body
of tens of metres, or ε 1e-6 with its certified env, does. The PR discloses the gap
("Unproven", "not padded") but files no row for it. Confidence: **sure** that the
bracket is unsound in the lane; **likely** that real bodies reach it.

**MINOR 2 — `rotational.rs:loop_windings`: no PR row ever has a nonzero `branch_turns`.**
Mutant M4 (discard `branch_turns`' result) passes all 25 rotational rows. So the turn
reconciliation, the lane's guard against an evaluator and an integral that read
different branches, has no witness. I added
`r2_a_projected_end_read_a_turn_off_holds_the_closed_form`: a projected circle whose
end the image lifts to `u₁ − τ`. It passes at head and refuses `LoopsWindChart` under
M4. On inspection, `branch_turns` cannot pass a wrong count: the true gap is an
integer inside the enclosure, and `off < ¼` pins it (claim 3 upheld). Confidence:
**sure**.

**MINOR 3 — the residual pad and the sector breaks have no witness.** Mutant M8
(`residual` → 0) and mutant M10 (`of_image` ignores `image.breaks()`) both leave every
rotational row green. Every fixture has `env` ≈ 1e-13, and on the cylinder the
breaks guard nothing the integrand needs. The only singularity, `x² + y² = 0`, is
caught per piece by `recip`, which refuses an interval containing 0. So claim 2's "pieces cut at the sector breaks,
so the ρ/pole/tube bounds hold" is decorative at this stage. It is not wrong, but
premise (e) ("check it, don't assume it") is not shown. Confidence: **sure**
(execution).

**MINOR 4 — the swap rows never test the chart term's rational part.** As MAJOR 1
says, `face` builds `closed = whole − lever·VA(carrier, 4096)` with the same
`rational_vector_area_split` the lane calls. Any error in that function (or in
`rational_edge_q`'s `forward`) cancels, and `cut_face_lane`'s own assembly of
`rationals` runs only in sweep rows. Mutant M6 (drop the rational term) does red,
because it breaks the cancellation. A sign error inside the split would not.
`rational_rim`'s own rows cover the function. Confidence: **likely**.

**NOTE 5 — `crossed_cylinders.rs`: "each op's area is its own" overstates.** `∪` and
`∖` are `size − shared` of the same clip, so the self-check `∪ − (2|A| − ∩)` is an
identity. It tests only `|A| = πr²L`. The oracle *is* independent of the kernel: its
own clip, `area` and `gk15`, all test-side. Steinmetz holds to 1e-10, though in poses
where the caps never clip. The GK error estimate is heuristic, not exact. Confidence:
**sure**.

**NOTE 6 — the SOUND rows witness truth within `VOLUME_CLAIM + pad`.** That includes
`resolution_pad`, up to 1.6e-5 for the turned parallel join at ε 1e-9. That is
containment of a wide bracket (claim 6 upheld: `res·area/2` follows from
`mean_boundary_displacement ≤ 1024ε`, the operands' area bounds the result's, and only
the two new rows use it). It is not "met to nine digits"; that figure is the
measurement, not the assertion. Confidence: **sure**.

**NOTE 7 — cost.** `a_rod_through_a_drum_wall…` took 426 s here (debug; it ran
alongside the four other rows) and the fenced row 619 s. The P1 row (claim 10) quotes
187 s and 264 s from *before* the lane. It says so, but gives no with-lane figure for
those rows. That is honest about what it measured, but it is not a current cost.
Confidence: **likely**.

## Style
Questions exercised: Q1, Q2, Q4, Q5, Q6, Q7, Q8 (read `rotational.rs` 1–1040, the
non-test part, end to end); Q3 not exercised.
- **Q1** `rotational.rs:projected_integrals` (Circle/Line arms) and
  `projected_metric_length` (Circle/Line arms): four near-copies of the same `cuts.windows(2)` ×
  pieces loop, and `rational_rate_bound` is a fifth. `harmonic_metric_length` /
  `projected_metric_length` are parallel spellings. **sure**.
- **Q1** The `exact` closure (`is_certified && lo == hi`) is written at `projected_edge.rs:39`,
  `rational_rim.rs:169` and `quad_lane.rs:969`, and as `zero` at `rotational.rs:384`.
  **sure**.
- **Q1/Q7** Dependency cycle: `taylor.rs` now imports `quad::{rotate, spread_*, trig_at}`, while
  `quad::rotational` imports `taylor`. The trig core lives inside one of its consumers.
  The lane still reaches into `spiric` for `W_SPIRIC`/`w_rational`, though the PR says the
  trig move was "so the lane does not reach into it". **likely**.
- **Q4** The renamed refusal's old word survives as pin vocabulary: `"conic trim"`
  (`germ_torus_components.rs:88`), `Stop::ConicTrim` (`torus_doors_join.rs`),
  `teapot.rs:1553` ("a conic trim on the sphere's chart has no quadrature yet"), and
  `work/nearpoint/quadrature-unsupported-says-its-reason-in-prose-alone.md:26` (open) quoting the
  old text. The PR called that word a misnomer. **sure**.
- **Q4** `Door::Sphere`, `SPHERE_FACE` and `TORUS_WALL` assert the chart kind in prose ("every
  face that refuses here is a sphere face"), but the matchers read only the message and
  `RingOnCurvedFace`, never the face's surface. Elimination makes them right today. A
  future cylinder `RingOnCurvedFace` would read as "Sphere". **likely**.
- **Q5** `quad/mod.rs` says "Five engines, one file each", but the rotational engine is now
  two files (`projected_edge.rs`). **sure**.
- **Q2** The `branch_turns` doc says the coarsest reading meets a quarter turn "by far".
  My spiral chord's `∫du` at 8 pieces a span is 13.5 rad wide, so at the entry's 4 it
  refuses. It fails safe, but the claim is prose. **likely**.
- **Q6** Disclosed and unscheduled: the unpadded chart term (MAJOR 1); `face_cost`'s
  borrowed, unmeasured weights; the ε 1e-6 oracle resting on measured envelopes. None
  has an issue file. **sure**.

## Claims
1. **Upheld.** `quad/cylinder.rs` is gone. `cylinder_cut_face[_rounds]` survive only as
   `#[cfg(test)]` wrappers onto `rotational_cut_face_rounds`. No second edge loop,
   closure or pad: grepped `cylinder_cut_face|quad::cylinder|cylinder::` and the
   prose sweep. The internal near-copies are under Style.
2. **Upheld for the integrand.** I checked `u′`, the weight cancelling, the circle's
   slope and the line by inspection. Execution: `r2_projected_integrals_hold_high_precision_truths`
   reads a rational cubic past a quarter turn, the same turned across the seam, and a
   3-span rational quadratic winding 1.3 turns (v_sign −1, an extra break). All 9
   enclosures, at 8 and 128 pieces, hold 40-digit mpmath truths
   (`r2_mpmath_truths.py`). The swap rows pass. Mutants M1 (`u′` sign), M2
   (`v_sign`), M5 (dropped circle piece), M7 (traversal) and M9 (ends swapped) all red.
   Breaks: see MINOR 3.
3. **Upheld** (inspection; the probe in MINOR 2). M3 (`k+1`) reds four rows.
4. **Falsified** (MAJOR 1).
5. **Upheld with reservations** (MINOR 2–4, NOTE 5). No inclusion–exclusion check
   against the kernel anywhere.
6. **Upheld** (NOTE 6).
7. **Upheld for the rows I ran** (debug, head): drum × rod, the fenced poses, the
   tilted rod (∖ ∩ SOUND, ∪ census pinned exactly), `parallel_cylinder_join`'s turned
   pose, and the oracle's own row all PASS. The INSIDE row is consistent with that
   (its face identity is marked inferred). A second batch: 37 rows, all PASS (debug,
   2103 s). It covers `verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose…`
   (947 s), the blend reorder leg, `conic_edge_curved_face`, `verbs_cylsph_opening`,
   and every re-pinned row in claim 8.
8. **Upheld** (the second batch above: `cylinder_sphere_frame` ×9, seam door, `torus_doors_join`,
   `germ_tilted_rod` (1193 s), `germ_torus_components`, `germ_radial_hole`). The bodies are props-independent: the change is post-build
   measurement. In each fixture's operand mix, the merged words cover the only chart
   kinds that can still refuse (Style Q4 caveat).
9. **Upheld.** Every `starts_with` site carries the new prefix, demos included.
   Stale leftovers are under Style Q4.
10. **Upheld with NOTE 7.**

## Disclosures
I read no other reviewer's branch and no PR comment or review. I read the PR body
through the GitHub tools. I posted nothing to GitHub. I installed mpmath with pip
in the container. Mutants were applied in place, run, and reverted; none is on the
probe branch. The probe branch holds one known-red row (MAJOR 1).
