# Review R1: PR 6478 (`spiricarm/rotational-lane-cylinder`), stage 1 (cylinder)

**Head reviewed: `e12e3c8d0017f9b83be830d4881f580afcb28c9d`.** Brief sha256 `615b46c9…6efaf`.
Probes: branch `spiricarm-rev6478-r1/probes` (`quad/rotational/r1_probes.rs`, two rows appended to
`swap_rows.rs`, and the mutant driver `r1_mutants.py.txt`). CI at the head is green (`gate ok`).

## Verdict: APPROVE-WITH-FIXES

The lane's shape is sound and the edge integrands hold under every probe I could build except
one: the chart term. MAJOR-1 must be fixed (a pad and a row) before merge. The MINORs are
test gaps that mutants pass through.

## Findings

**MAJOR-1: the chart term `o·A⃗` is an uncontained error in a certified bracket.** Where:
`topo/src/props/quad_lane.rs:cut_face_lane` (the `closed`/`rationals` assembly) and
`geom-brep/src/props/quad/rotational.rs:chart_term_at` / `RotationalChartQ::residual`.
- The chart term reads the carriers' vector area. A projected carrier lies up to `env` off its
  image, and the image is what the `−∮F du` part integrates.
- The two vector areas differ by the ribbon between carrier and image, about `length·env`. In
  flux that is `|lever|·length·env`.
- The only pad is `residual`, which is `r·length·env`. It scales with the radius, not with the
  lever. The lever is `origin − centre`, or `origin` itself when there is no centre, and it can
  be the rod's length or the body's distance from the world origin.
- **Evidence (execution).** Probe `r1_the_chart_term_of_an_off_surface_carrier_is_padded` is
  **red**.
  - The face is a unit-cylinder rectangle. Its bottom rim's carrier is a circle of radius `1+δ`,
    and its rulings are lines from radius `1+δ` to `1`. Each image is exactly the rectangle, and
    `env = δ`.
  - The chart term is read about a point `L` down the axis. The truth is `θH`, and the error is
    `L·δ·θ`.
  - The enclosure misses the truth at (δ, L) = (1e-9, 10) [flux `[1.000000006, 1.000000014]`],
    (1e-9, 1e3), (1e-6, 10) [off by 1e-5] and (1e-6, 1e3) [off by 1e-3].
  - It holds only at L ≤ r.
- The PR lists it "Unproven", but nothing at the claim site does, and the module docs promise a
  bracket covering any boundary within `e`. No swap row can see it (every `env ≈ 0`).
- Body-level effect: not measured. At ε 1e-9, with the measured envelopes (≤7.8e-11) and levers
  of order 1, it falls below `VOLUME_CLAIM`. At ε 1e-6, or for a body far from its centre, it
  does not.
- Confidence: **sure** at the face level, **likely** that it matters at ε 1e-6 or at a far lever.

**MINOR-2: five mutants survive every PR row.** Evidence (execution): `r1_mutants.py.txt`,
filter `rotational rational_rim`.
- M6, the map-residual pad dropped entirely (`edge_terms`'s `pad = [0,0]`): survives. No row has
  a nonzero envelope, so the pad is unwitnessed. This is the pad MAJOR-1 needs widened.
- M9, `v_off` dropped from `projected_forms`: survives. Every swapped image has `v_off = 0`.
  Chart shifts (`map_affine`) do make it nonzero in production.
- M10, the sign of the `Line` arm's `u′` flipped (`line_rate`): survives. The only line row is a
  vertical ruling, where `u′ ≡ 0`.
- M3, the traversal sign of `branch_turns` ignored in `loop_windings`: survives. It also
  survives my added reversed winding loop, because see NOTE-8.
- M13, `branch_turns`'s quarter-turn tolerance raised to 0.75: survives. No row presents an
  image whose ends and turn disagree, so the refusal arm is unexercised.
- Killed: M1 (`v_sign`), M2, M4 (off-by-one turn), M5 (dropped circle piece), M7, M8, M11.
- Confidence: **sure**.

**MINOR-3: the swap rows' chart term is circular.**
- Where: `swap_rows.rs:face`. It sets `closed = whole − lever·rational_vector_area_split(…, 4096)`,
  and the lane adds `lever·rational_vector_area_split(…, pieces)`, so any bias in that function
  cancels.
- M12 scales the vector area by 1.2. It passes all 7 swap rows and is caught only by
  `rational_rim`'s own 40-digit rows.
- So the swap rows do not witness the chart term at all. `quad_lane`'s assembly of it (which
  carriers become `rationals`, their `forward`, the origin) is witnessed only by the sweep rows.
- Confidence: **sure** (execution).

**MINOR-4: rows read against `differential::resolution_pad` pin far less than they measure.**
- Where: `tang_circle_cylinder.rs:a_tilted_rod_…` (budget ≈1.3e-5) and
  `parallel_cylinder_join.rs` (≈1.6e-5).
- A sound ceiling (meter `width/(3·mid(area))`; result area ≤ operands'); no tight row is
  loosened, since every row it governs refused before.
- But SOUND there admits `|v − w| ≤ 1e-7 + pad`, with pad up to about 130–160× `VOLUME_CLAIM`.
- The measured pads are 1.8e-7 and 2.3e-6 and the volumes agree to about 1e-9, yet nothing pins
  that. A 50× loosening, or a 1e-6 volume error, stays green.
- Confidence: **likely**.

**MINOR-5: the merged pins (`Door::Sphere`, `SPHERE_FACE`, `TORUS_WALL`) assert a chart kind they
never check.**
- Where: `cylinder_sphere_frame.rs:door`, `germ_tilted_rod.rs`, and the seam-region door.
- Each matches only the message ("a trimmed face with no volume lane on its chart", which any
  non-cylinder chart raises) or `RingOnCurvedFace`.
- "Every face that refuses here is a sphere/torus face" lives in a comment only, although the
  error carries the `FaceKey`.
- By construction the merge also hides a body change among the trim, ring and spiric stops (a
  ring vanishing, say). `germ_tilted_rod` collapses 40 cells over two owner rows into one word.
- A cylinder-face regression is still caught, because it no longer raises that message.
- Confidence: **likely**.

**NOTE-6: "verbatim" is slightly overstated.** `harmonic_edge_integral` gains exact fast paths
for rims and rulings (`HarmChan::is_flat`). The rim path multiplies by `pt(span)`, where
`span = b − a` is a rounded `f64` subtraction treated as exact (the rule's `h` already did the
same). Confidence: **sure** on the change, **unsure** on its impact (ulp-level).

**NOTE-7: the P1 cost row's title reads the pre-lane release cost as current.** It says "about
6–7 s in release". That number was measured while each op refused. With the lane, only the debug
figures (17 s ∪, 3.6 s of it measurement) are new; the release cost now is unmeasured. The id
"…before-the-volume-lane-refuses" is stale too. Otherwise the rewrite is honest. Confidence:
**likely**.

**NOTE-8: `branch_turns` is 0 on every row.** The evaluator's per-piece branch centres make the
ends continuous, so the "whole turns join the loop's count" path is never exercised at a
nonzero value. That is why M3 survives even my reversed winding loop. Confidence: **likely**.

**NOTE-9: sector breaks bear on tightness, not soundness** (the integrand is smooth off the axis);
`of_image`'s break mapping is exercised by no row (rows pass `cuts` by hand). **likely**.

## Style
Exercised: Q1, Q2, Q5, Q6, Q7, Q8 (not Q3, Q4 in depth). Also `end_slack`'s `√2 taken as 3/2`: **unsure**.
- **Q1:** `projected_integrals`' `Circle` and `Line` arms repeat one cut-loop accumulator
  (`rotational.rs:~556–598`). `projected_metric_length` repeats one piece loop twice more
  (`~620–650`), and `rational_rim.rs:rational_rate_bound` is a third copy of that
  `node(…)`-piece loop. A fresh duplication inside the unit that closes one. **likely**.
- **Q1/Q7:** `cylinder_cut_face[_rounds]` (`rotational.rs:727–760`) survive as a second, test-only
  entry spelling beside `rotational_cut_face_rounds`. **likely**.
- **Q7:** `RotationalChartQ` has one variant and five methods, each `match self { Cylinder }`. It
  is designed for stages 2–3, but it is an abstraction with one implementor today. **unsure**.
- **Q2/Q5:** The module doc's honesty-pads section describes the map residual as a chart-area
  defect only, and says nothing about the chart term. The prose is longer than the
  `residual`/`chart_term_at` code and promises more than it does (MAJOR-1). **sure**.
- **Q6:** `face_cost`'s borrowed weights (`w_rational`, `W_SPIRIC`, `w_rational(1)` for a line)
  are disclosed as unmeasured, with no work row or guard. So is the "witness at 1e-6 only by
  measured envelopes" claim. **likely**.
- **Q8:** I read `rotational.rs` lines 1–1240 end to end (the header has grown to about 100
  lines) and skimmed its tests.

## Claims
1. **One lane, no copy: upheld.** `cylinder.rs` → `rotational.rs`. The spiric helpers moved to
   `taylor.rs` byte-for-byte. No `cylinder_cut_face` caller remains outside `#[cfg(test)]`. The
   harmonic arm is the old one plus fast paths (NOTE-6).
2. **Integrand and enclosure: upheld.** The formulas match `u′ = (xy′−yx′)/(x²+y²)` and
   `v = v_off + v_sign·z`; the deck map's linear part is `diag(1, ±1)` (`map_affine`), so
   ignoring `u_off` is right. My row `r1_a_swapped_plane_section_holds_the_closed_form` (an exact
   plane section, `u` and `v` both varying: short, past a quarter turn, across the seam, most of
   a turn) is green. Gaps remain: M9, M10.
3. **Closure: upheld in logic.** A wrong count is refused or `round()`s to the true integer.
   But its nonzero and refusal paths are unwitnessed (M3, M13, NOTE-8).
4. **`o·A⃗`: falsified.** MAJOR-1, by execution.
5. **Oracles: partly upheld.** M1 does red the mirrored row. The swap rows witness the edge
   terms but not the chart term or the pad (MINOR-2, MINOR-3). The sliced oracle is pure `f64`
   geometry, independent of the kernel, and checked against Steinmetz. Its `∪` is
   inclusion–exclusion at the slice level, inside the oracle, which is legitimate. No
   inclusion–exclusion check of kernel output found.
6. **`resolution_pad`: upheld as a ceiling**, with MINOR-4.
7. **SOUND rows: upheld by inspection plus green CI.** Each reads `crossed_cylinders::volume`,
   or the `S−R−H` closed form, not a constant. The tilted-rod ∪ census refusal is pinned
   exactly, and the INSIDE row exists and lints. I did not rerun the sweep rows locally.
8. **Re-pins: upheld with MINOR-5.**
9. **Rename: upheld.** Every `starts_with`/`contains` site, the teapot demo included, uses the
   new message. Old wording survives only in historical `work/` rows.
10. **P1 row: mostly honest**, NOTE-7.

## Disclosures
- No other reviewer branch, PR comment or review read; I read the PR body and check runs. The
  mutant driver edited the checkout temporarily and restored each file.
