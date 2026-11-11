# Review of PR 6808 (R1): the rotational lane reads the torus chart (stage 3)

**Head reviewed: `f87c193de862a2129cbab57f852870a5a8222cd4`.** Probes: branch `spiricarm-rev6808-r1/probes` (`e6df2fe1`), rows `r1_probe_*` in `swap_rows/torus.rs`, plus `review-probes/r1/` (the slicing oracle compiled standalone, and an mpmath cross-check).

## Verdict: APPROVE-WITH-FIXES (0 MAJOR / 3 MINOR / 6 NOTE)

The lane's mathematics held under every probe I ran: 528 chain-start and whole-turn renaming cases, 200,000 count-window cases, and an independent oracle check to about 1e-15. The defects are the recurring class. Two lift and ring terms that are correct at head certify wrong values under a mutant that no row catches, and the new builder adds guards with no row.

## MINOR

**M1. `rotational.rs:loop_lifts`, `LoopLift::ends`: the connector's lift at the edge before's END is unwitnessed.** Mutant `ends.push(c + b)` → `ends.push(c)` (the lift at the edge's start): all 18 torus swap rows pass, including the PR's turn-up row and its standing-off meridian. That row's connector follows a harmonic edge, whose `b = 0`. My witness `r1_probe_connector_after_a_branch_turn` is the image-end-turn row with the left meridian `0.9e-6/(R+r)` off in `u`. It passes at head and goes red under the mutant, which certifies a wrong area: `2.1822777153` escapes `[2.1822780941, 2.1822797144]`. Executed. Confidence: sure. The brief asks for exactly this check ("the lift's turns at the edge before's end"), and the PR's "connector's lift dropped" row does not isolate it.

**M2. `rotational.rs:require_holes_wound_against`, the band-face rule (`band_face`): unwitnessed.** Mutant `band_face = false` (so a non-winding loop stored first is never read as a hole): all 18 torus rows pass. No row puts a hole on a band face. Witness `r1_probe_band_with_a_hole_stored_first`: an outer band whose hole patch is stored first.
- Wound against the face, the band less the hole holds, at head and under the mutant.
- Wound with the face, the head refuses `RingWoundWithFace`. The mutant returns `Ok(area [5.72496052889, 5.72496052890])`, which is band + patch: a silent wrong certificate.

Executed. Confidence: sure.

**M3. `topo/src/props/quad_lane.rs:spiric_wall_edge`: new refusals with no row.**
- "carrier or image radii are not exactly its torus's" (new in this PR);
- "sense is not exactly ±1";
- "carrier is no spiric";
- "on a chart other than a torus";
- the mirror-torus refusal. It was rowless on main too (`quad_lane.rs:1079` at the base), but it is a soundness guard, and this PR re-homes it.

`rg` finds none of these strings outside their `what:`. Likewise the ring-torus guard's "refused brackets" half (`rotational_cut_face_rounds`: only the spindle half has a row) and the torus arm of "neither a rim nor a meridian" (rowed on the sphere only). Confidence: sure (grep). Not executed as mutants: nothing reaches them, so each would survive.

## NOTE

**N1. The `R` guard's `lo`-only mutant is equivalent only on point brackets** (`projected_edge.rs:of_image`). Every torus row runs `T = f64`, so `major` and `own` are points and the `lo` and `hi` comparisons are the same predicate. As logic it is not equivalent: `[R, R]` against `[R, R+δ]` passes `lo`-only. It is unreachable today because both brackets come from one stored datum through `from_certified`. So the claim holds by construction and over the rows, not "its `hi` comparison refuses the same images" in general. Confidence: sure.

**N2. `swap_rows/torus.rs:torus_holds` / `test_target::encloses` has zero slack, and its Simpson oracle is accurate only to rounding.** An all-harmonic sector (swap `[false, false]`, which no row runs) fails by 4e-16: the closed form `3.907826439078367` sits outside `[3.9078264390783675, …]`. The lane is right and the oracle's f64 sum is off by an ulp. The rows pass only because every one they run carries a pad wider than the oracle's rounding. The brief asks whether the truths are exact to better than the pads, and here they are not, for an envelope-free face. Executed. Confidence: sure.

**N3. The `y`-spread mutant (`taylor.rs:primitive_each`).** I upheld the rule: jets, the Lagrange remainder on `[p, x]` and `[p, q]`, the carry and the `removable` accounting. The mutant drops `|y′|·(h/2)·|by₄|` from `g₄`. For a smooth `by`, `y₁·by₃` over the piece already spans at least `4h·|y′|·|by₄|`, because `by₃′ = 4·by₄`, about 8× the dropped part. Even where `by₄` changes sign, the margin stays at about 2×. So it is very likely unwitnessable at any resolution: it is sound by slack rather than equivalent, and that argument belongs at the site. I did not build a red witness. Confidence: likely.

**N4. Done-when of `the-spiric-wall-lane-omits-its-junctions-connectors-…`**: met by shared code. The spiric arm's junctions go through the one `connector_terms`, but the row that goes red without the connector is a harmonic meridian's. No row has spiric images stopping short of their vertices, which the done-when names. Confidence: likely.

**N5. Not re-executed:**
- the pinched-oval row's red under the old decision;
- the PR's mutant table (I ran three mutants of my own: M1 and M2 above, plus `anchors` always `c`, which is red in the image-end-turn row);
- the 63 slow sweep rows.

**N6. `LoopLift` adds new sites to a filed class.** `loop_lifts` reads `nearest_turns(v1 − v0) as i64` and `Junction::levels` reads `level_turns`: whole turns rounded off a midpoint, an `as` cast of a rounded quotient. That is the shape `work/pcert/whole-period-integers-read-off-a-value-outside-the-s1-sweep.md` files as C4. They are verdict-safe, since they reproduce the closure's own `m` on the same `delta`, and the torus levers `R − r` and `r` are positive. But the row is not updated, and it still cites `props::spiric::wall_windings` as the torus site. Confidence: likely.

## Claims
1. **Count window: upheld.**
   - Proof: for the true `N`, `X + N·A ∋ X_t + N·A_t ∈ (0, A_t]`, so `meets(N)` holds.
   - The starting ends are below and above `N` up to rounding at integers.
   - The flux hull covers the counts in between: its lower end is concave in `n` and its upper end convex.
   - Fuzz: 200k cases, end cases included (`X` at a whole torus, `X_t` an ulp above 0, brackets on `R` and `r`, two loops, both senses). No miss.
   - The PR's equivalences for `⌊−X.hi/A⌋` and `⌈1 − X.lo/A⌉` and for the two-count window hold.
   - A one-loop face decides nothing (`entries.len() < 2`).
2. **Lift and seam: upheld at head, two terms unwitnessed (M1, M2).** Every chain start (4 rotations) × names a turn up/down in `v`, a turn in `u`, and both together × 4 swaps × both senses × 2 patches (one across `±π`). Sectors with meridians renamed `(±τ, 0)`, `(0, τ)` and `(2τ, −τ)`, loops reordered, a meridian split in two at three points. All hold their closed forms: 528 + 48 cases. `Junction::levels` and the closure's `m` agree. `require_boundary_windings` refuses `|k| > 1`, nonzero sums and more than two winding loops. A face of torus-minus-discs (no outer loop) reads correctly. With a hole stored first in a non-band face it would refuse falsely, which is loud, not unsound.
3. **`primitive_over`: upheld (N3).** C9 holds: `v(t₀)` is `e.ends[0].1`, the evaluator's end through `from_certified` (`quad_lane.rs`), and the trig is the existing ring-based rotation (`trig_range`). `RationalJets::dw` is `slope_form(weight)·inv_len`, `B′`'s construction, and its callers only destructure.
4. **Spiric arm: upheld.** Integrand (`wall_terms`), `A⃗` (`spirics`), turn (`refined_turn` of `wall_terms[2]`), length (`spiric_length_bound`), and the floor guard `min(R − r, r)` (now at lane entry). The residual pad is metric `length·env` at density `R + r`, against the old `chart_len·env/floor·r(R+r)`: the same density and a tighter strip, the lane's convention since stage 1. The radii check is stricter than before. The old lane's `turns != [0, 0]` refusal is replaced by the band reading.
5. **Schedule: upheld.** It sets resolution only, and the rule is sound at every resolution. The tube-angle bound is read for projected spans, while rims, meridians and spirics read `round_pieces`.
6. **Levers and pads: upheld.** `p·n = R cos v + r` (so `≤ R + r`), `ρ = R + r cos v`, and the L1 speed `≥` the Euclidean one. Bol–Fiala at `K ≤ 1/(r(R+r)) ≤ 1/r²`, for the small side (`A ≤ 2πr²`), as on the sphere. Negative `K` only helps.
7. **Guards: upheld, with N1 and M3.**
8. **Oracles: upheld.** I compiled `donut_slices.rs` standalone and checked its five rods along `x` against an mpmath reading with the inner integral in closed form. The oracle's numbers do not enter that reading. They agree to ≤ 2e-15, against `ORACLE_SLACK = 1e-11`. N2 is the only gap.
9. **Re-pins and filings: upheld** (N4 and N5). The three filed rows are scoped beyond this unit's fence and meet the bar for filing. `torus-wall-spline-rim` and `spiric-beside-spline-chord` meet their done-whens.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q6, Q7; Q8 partly (`rotational.rs`'s torus half end to end, not all 3,541 lines); Q5 on `torus_face`'s doc and `require_holes_wound_against`'s doc.
- **S1 (Q4).** `rotational.rs:1706` and `rotational.rs:1914` each compute `torus_slopes(..).map(|k| k * tau_bracket())`, the per-turn `κτ`, once for the connectors and once in `torus_loop`. Likely.
- **S2 (Q1).** One whole-turn count is read three times: the closure's `m` inside `interval_loop_turns`, `loop_lifts`' `nearest_turns(..) as i64`, and `Junction::levels`' `level_turns`. The lift re-derives what the closure took off rather than receiving it, and stays in step only because all three run the same expression on the same `delta`. Likely.
- **S3 (Q4).** The PR's pattern-2 sweep ran over `crates/` and `docs/`, not `work/`. Open rows still cite removed names:
  - `work/restfront/check-9-meeting-arms-…:76` (`require_rings_wound_against`);
  - `work/quadtail/cylinder-quadrature-integrates-loops-…:97` and `work/pcert/whole-period-integers-…:43` (`spiric::wall_windings`);
  - `work/spiricarm/{the-spiric-wall-quadrature-lane-reads-only-the-own-torus, a-spiric-bounded-torus-band-has-no-volume-lane, f64-spiric-wall-ends-near-tangency-…}` (`spiric_wall` / `wall_windings`).

  The last row's defect (f64 ends near tangency read a closed junction open) now lives in `loop_windings`, and its row does not say whether it survived the move. Sure.
- **S4 (Q3).** `torus_doors_join.rs:a_square_rod_on_the_outer_equator_builds_at_its_volume`: the "square rod" is a round cylinder (`rod_y` → `cyl`, and the oracle's `Rod::AlongX` is circular). Sure.
- **S5 (Q6).** `donut_slices.rs:adapt` stops at depth 40 whatever its error, silently, and GK15's error estimate is heuristic across the AlongX rods' interior z-kinks, where the band edge crosses the tube circle and `z_cuts` places no cut. My cross-check found it accurate here, so this is a fence nothing guards. Unsure.
- **S6 (Q7).** `edge_entry`'s spiric arm reads its azimuth turn by computing the whole `wall_terms` triple at each refinement and keeping `[2]`. Unsure.
- **S7 (Q2).** `torus_face`'s doc and its four inline comments re-argue the window three times (doc, `meets`, "two counts or more"). The argument is right, and its length reads as defence. Unsure.
- **S8 (Q3).** `germ_sphere_torus_placements` and `germ_torus_components` assert "builds" only, with no gross bound such as `max(V_a, V_b) ≤ V ≤ V_a + V_b`. Its inclusion–exclusion is barred as a truth, not as a sanity floor. Filed as no-truth, so recorded. Unsure.

## Disclosures
- Read the PR body only (GitHub `get`); no comments or reviews; no other `spiricarm-rev6808-*` branch. No accidental glimpse.
- Posted nothing to GitHub.
- Mutants were run on a copy of `rotational.rs` and restored (the probe branch's diff touches `swap_rows/torus.rs` only).
- Installed `mpmath` with pip in the container.
- One background run (the mpmath script, auto-backgrounded by a timeout) finished before the report; nothing is left running.
