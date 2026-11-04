# Review of PR 6509, R1: the rotational lane's sphere chart (stage 2)

**Frozen head reviewed: `fe92371df87eda4077cb36883cbc8b52ae7aac38`.** Brief `spiricarm/review-6509-brief`, sha256 `7910ac19…` (verified). Probes: `spiricarm-rev6509-r1/probes` @ `ddbfa886` (`swap_rows/sphere/r1_probes.rs`, `review-6509-r1/mutants.py`).

## Verdict: APPROVE-WITH-FIXES (0 MAJOR / 6 MINOR / 6 NOTE)

The pole rule held on every face I built to break it: 80 enclosure readings (36 pole-angle, 24 hemisphere and near-full, 20 near-pole), plus refusals. I found no wrong enclosure. The MINORs are guards no row tests (one shown by a probe to carry weight), a band that README's Priority puts higher, a loose pin, and docs that now state a guard this route does not have.

## Findings

**MINOR 1: `rotational.rs:RotationalChartQ::azimuth_lever`. The sphere lever's `abs` carries weight under the twin name, and no row tests it.** Under the twin, `v ∈ (π/2, 3π/2)`, so `cos v < 0`. Mutant A (`.abs_enclosure()` dropped) passes all 48 rotational rows. Probe `r1_an_open_junction_refuses_under_both_names` is a loop with a 0.3 rad (≈0.19 m) azimuth gap at latitude 0.4. The real code refuses it `ChartLoopOpen` under both names. Under mutant A the twin certifies the open loop, area `[0.688125528218, 0.688125528218]`: the negative lever makes the junction's gap negative. *sure* (executed).

**MINOR 2: the sphere's pads mostly go untested (stage 1's "unwitnessed pad" class again).** Each mutant below passes every rotational row (48/48):
- I: `sphere_harmonic_terms`'s endpoint-bracket pad set to 0.
- K: the map-residual pad set to 0 on the sphere.
- L: `level_lever` set to 1 instead of `r`. Every sphere row uses `r = 0.7`, so the lever's unit never shows.
- D: `projected_speed` multiplied by 0.25 instead of divided by `|p|²`.
- E: the corner pad `2g(e_i+e_j+g)` at ¼. It reddens only `hopeless_endpoint_brackets_stop_at_the_floor`, a pin on where the run stops. The moved-out square, the PR's witness for this pad, still holds.

So "red when the pad is 0" is the only witness: no row checks the pad's size. Some of these may be sound by slack, since on the sphere a projected image *is* the carrier's radial projection. If so, that argument belongs at the claim site. *sure* (executed).

**MINOR 3: `work/inside/point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord.md` is banded P1. `work/README.md`'s Priority makes it P0.**
- P0 is "a normal verb broken on normal geometry", and authoring is the goal P0 serves.
- Every cylinder × sphere result refuses as the operand of any later boolean.
- Five differences on normal geometry still refuse outright at the piece sort: the frame's 2 two-loop `A ∖ B` runs, and the seam door's 3 `b−a` runs.
- The row's own reason ("P1 rather than P0 only because the first boolean now answers") is not one of README's criteria. Its own words ("a dead end for authoring") argue P0.

The P0 that closed is true to its measured stop: the result gate no longer refuses. But the P0 condition moves to this row rather than going away. *likely*.

**MINOR 4: `sweep/tests/cylinder_sphere_frame.rs:at_its_volume` accepts `Door::Pieces` on any run.** An `Err` with `door == Pieces` gives `Verdict::Refused`, which is held. So any of the 52 building runs could regress to the piece sort and the row stays green. The PR's "52 build, 2 refuse" is printed, not asserted. *sure* (read).

Otherwise the check is containment: `|v − want| ≤ VOLUME_CLAIM (1e-7) + pad`, `pad ≤ ceiling`, and t2, t3′ and cert true. That is SOUND's test without the operand check. Upheld.

**MINOR 5: docs still state a sense guard this route no longer has (claim 2).**
- `topo/src/validate.rs:1557-1563` says "the sphere's flux refuses the same face (`PropsError::SenseContradicted`)".
- `topo/src/props.rs:5286` says "The S10 sense bit does not enter the quadrature lane … winding-derived end to end".
- `rotational.rs:61-67` says "no separate orientation classification is needed".

All three are false for a chorded sphere face. The lane reads `face.sense` (`quad_lane::face_sense`), and tier 3 exempts the face ("faces the quadrature lane owns"). Probe `r1_a_flipped_bit_reads_the_complement`: the cap's loop with the bit flipped certifies the rest, area `[4.27768, 4.27769]` (truth 4.277687). So the claim's "a flipped bit cannot be certified" is **falsified at the lane**: a flipped bit certifies the complement. I accept the PR's defence that the complement is another valid body with the same boundary, so this is data, not a defect. But the three comments describe a cross-check that does not exist on this route. Q5. *sure*.

**MINOR 6: `demos/tour/src/teapot.rs:wall_probes`, wall 2. The matcher asserts the face kind only in prose (stage 1's class again).** It matches the message `"a trimmed face with no volume lane on its chart"` and never calls `refused_face_kind`. "Moves on to the handle's torus face" rests on a scratch probe and a comment. If a sphere face ever refused with that message again, nothing would go red. The germ rows do read the kind. *sure*.

**NOTE 1: the corner pad (claim 4) can be derived, so it need not be "argued".**
- The ribbon pad covers the strip `b + λd`. What it leaves is the loop: connector A→B (length ≤ the closure's gap ≤ g, plus `VERTEX_SLACK·ε`), then B→V and V→A (each ≤ `s = 1.5·env`).
- Its perimeter is ≤ 2g, so by the isoperimetric bound (spherical: `P² ≥ A(4π − A/r²)`) its area is ≤ g²/π, and `g²/π ≤ 2g² ≤ 2g(e_i+e_j+g)`. That is a margin of about 2π, which is why mutant E survives.
- One gap: the `3ε` the closure adds past the slack is unpadded where `g = 0` (env 0, thin brackets). That is O(ε²) of area. *unsure* it matters.

**NOTE 2: the refusal row runs under its own name only.** The PR says every row runs under the twin. `the_spheres_arm_refuses_what_it_cannot_read` runs `Own` (and `Shifted`), so `RingWoundWithFace` under the twin is never run. *sure*.

**NOTE 3: the 7.6e-7 pads (claim 8) follow from the acceptance rule.** The lane accepts the round that meets `resolution_pad(tol, area)`, and the ceiling is that pad. A ±8.6e-7 window witnesses little, but it is containment, as stage 1's chorded rows are. I did not run a frame pose: the cost was ~3000 s debug, and the brief's isolation forbade nothing here. *unsure*.

**NOTE 4: the m-consistency between W and the connectors holds, and a row tests it.** `junction_gap` and `closure::interval_loop_turns` both take `round(mid(Δu)/τ)` (`closure.rs:175`). A mutant flooring `junction_gap` is red in 11 rows. *sure*.

**NOTE 5: two tracker rows share one fix.** `splitdoor/a-sphere-cut-in-refuses-…` (now open) and the new INSIDE row both need `sphere_region::sphere_face_region` widened. Only SPLITDOOR's row names the other. *likely*.

**NOTE 6: a chord ending at a pole, or passing through one, refuses before the lane.** At 1e-9 and 0 rad from the pole, minting the pcurve (`chart_pcurve_over`) refuses: `NoGeneralLane … Pose{Azimuth}`. No pose reached the lane's own behaviour there. A projected edge *into* a pole vertex would refuse at `u′`'s reciprocal. That is sound but a gap in coverage. Only harmonic meridians carry poles in the rows. *likely*.

## Claims
1. **Pole rule: upheld.** Probes `r1_pole_angles_past_a_half_turn`, `r1_hemispheres_and_near_full` and `r1_chords_at_the_pole` all HOLD under both names:
   - sectors and lunes at a = 2, π, 3.5, 4.5, 6, 6.2, both senses (m = ±1 at the pole);
   - exact hemispheres, rim harmonic and swapped, plus a tilted great circle;
   - the sphere less a disc and less a polar cap, θ = 1e-2 … 1e-4, and a disc of radius π − 1e-3;
   - discs passing 1e-3 … 1e-6 from the pole, on both sides.

   The argument about whole turns at a pole junction is sound. X moves by `s·(±2πr²) + 2πr²` per turn, which is ≡ 0 mod 4πr², and W takes the same `m`. A name switch at the pole cancels in pairs, because the twin maps the other pole to v = 3π/2, so the closure refuses a stretch that switches names between poles. `require_sphere_sides` pins each loop's outside to a unique value in (0, 4πr²), so a 4πr² error cannot pass.
2. **Sense bit: partly falsified** (MINOR 5). The bit is read off the face correctly, but a flipped bit is certified as the complement.
3. **Integrand: upheld.** `sin v = p_z/|p|` with the weight cancelling. The speed `r|p×p′|/|p|²` is correct and the L1 norm bounds it from above. `entry_turn` stops at 1024 pieces, and `branch_turns` refuses anything wider than a quarter turn, so it cannot guess.
4. **Connectors: upheld.** The term is `G(v*)(Δu−mτ)` over the hull, by the mean value theorem. The pad is derivable (NOTE 1) but tested only at 0 (MINOR 2).
5. **`chart_term_pad` on the sphere: upheld** by reading. The far-out row passes, and scaling the chart term by 1+1e-5 reddens the swap rows.
6. **Oracles: independent.** The truth is closed forms plus Simpson on every arc; the lane reads swapped carriers by its own rule. Mutants beyond the PR's list are in MINORs 1–2.
7. **Routing: upheld.** Circle-only faces take Gauss–Bonnet (`takes_quadrature` unchanged), any trim takes the lane, and `reads_rings` is open for the sphere. No face reaches both routes or neither.
8. **Moved rows: upheld with MINORs 4 and 6.** `at_its_volume` is containment. The germ rows and the seam-region door read the face's kind; the teapot reads only the message.
9. **Tracker: the closures are true to their measured stops.** The P2's "done when" is met: the term is added and a tall-face row goes red without it. Banding: MINOR 3. The spiric P2 is consistent with its sibling.

## Style
Questions exercised: Q1, Q2, Q5, Q6, Q7. Q8 partly: I read the diff regions and the module header of `rotational.rs` (2607 lines), not the whole file.
- **Q1, the junction loop is copied three times.** `loop_windings`, `connector_terms` and `chart_term_pad` each walk `(i, next)` and recompute `end_slack(i,true) + end_slack(next,false)` (`rotational.rs:~930, ~1290, 1017`). This is the stage-1 piece-loop class again. *sure*.
- **Q1, `m` has two homes.** It is rounded in `closure.rs:175` and in `rotational.rs:junction_gap`, and their agreement carries weight. *sure*.
- **Q1, the sphere's area is written out three times.** `pt(2.0)*tau_bracket()*radius.sqr()` appears at `rotational.rs:1160,1176,1204`. *sure*.
- **Q1, the `u′` formula is duplicated.** `net_rate` and `unit_weight` both compute it. *likely*.
- **Q7, the sphere's `of_level` and `level_flux_scale` arms return sentinels.** `of_level` returns `refused` and `level_flux_scale` returns `INFINITY`. Only the guard arm's order in `edge_terms` keeps them unread. *likely*.
- **Q7, `sphere_signed_area` signs a value that `require_sphere_sides` then un-signs.** *unsure*.
- **Q5, the module docs are stale for the sphere.** The pads' metric `ds² = r²du² + dv²` and "each loop winds at most once" (`rotational.rs:61-67, ~130`) describe the cylinder only. *sure*.
- **Q6, `latitude_name`'s `1e-12·(1+|k|)` tolerance has no reason at its site** (`projected_edge.rs:144`). *likely*.
- **`VOLUME_ON` prints the Option's Debug form**, "…on a Some(Sphere) face" (`the_door_reads_seam_regions_as_tier_3_does.rs`). *sure*.

## Disclosures
- No glimpse of any other `spiricarm-rev6509-*` branch, nor of the PR's comments or reviews.
- I posted nothing to GitHub.
- I ran no sweep frame or pose rows, and nothing at another ε.
- All lib probes and mutants ran in debug at the default ε.
- The worktree carries other reviews' `review_r1_*` rows (pre-existing, not mine). I filtered them out.
