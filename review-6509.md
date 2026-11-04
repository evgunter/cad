# Review of PR 6509 (R2): stage 2 of the rotational lane, the sphere chart

**Head reviewed: `fe92371df87eda4077cb36883cbc8b52ae7aac38`.** Probes are on `spiricarm-rev6509-r2/probes` (`09c412c84`).

## Verdict: APPROVE-WITH-FIXES

I found no MAJOR. The pole rule held on every case I ran. Those cases cover both senses, W ∈ {−1, 0, 1}, poles on the loop with the pole junction's whole-turn choice m = 0 and m = 1, a hemisphere exactly, the sphere less a 1e-4 disc, two holes, and a circle grazing a pole down to 1e-7. A flipped sense bit is refused by validation. The fixes are the unwitnessed pads and guards, the INSIDE row's band, and three loose pins.

## Findings

**MINOR 1: tracker banding and closure.** `work/inside/point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord.md` (P1), and the closure of `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane` (P0).
- The P0 row's own fix line says "The rows go SOUND against the fixture's volume oracle once it lands". They do not: 52 runs build at volume and are no operand, and 2 refuse.
- The remainder moves to a row banded P1 "only because the first boolean now answers". That is not `work/README.md`'s test. Its P0 is "a normal verb broken on normal geometry", and here every cylinder × sphere result is a dead end for the next union or difference. By that text the INSIDE row is P0.
- Confidence: *likely* (a reading of the README).

**MINOR 2: the new sphere pads and guards are unwitnessed** (`rotational.rs`: `projected_speed`, `sphere_harmonic_terms`; `projected_edge.rs`: `latitude_name`). The spec's rule is that every pad or guard ships with a row that goes red without it. I ran 11 mutants against all 54 `rotational` rows, the PR's and mine (script and log in `review-probes-r2/`):

| mutant | result |
|---|---|
| M3: sphere `projected_speed` × 0.5 (understates the edge length that the residual, end-slack and chart-term pads scale with) | survives |
| M6: rim endpoint-bracket pad → 0 | survives |
| M5: `latitude_name` tolerance 1e-12 → 1e-2 | survives (only a 0.3 shift is tested) |
| M4: `chart_term_pad` → 0 | red only in the cylinder `pads::` rows |
| M11: residual flux density → 0 | red only in cylinder rows |

So claim 5 is not witnessed on the sphere. No sphere row shows that stage 1's chart-term pad covers the sphere's lever, or that the map residual covers the sphere's density. Confidence: *sure* (executed).

Killed, as a check on the harness:
- M1, `junction_gap` without m: 13 rows red.
- M2, the connector pad → 0: red in the moved-out square.
- M7, the sphere's azimuth lever without `cos v`: red in the pole-on-the-loop rows.
- M8, the face-area side check off: red in the refusals row.

**MINOR 3: `cylinder_sphere_frame::at_its_volume` pins less than the PR says.**
- (a) Any run that refuses at `Door::Pieces` passes, so a run that builds today could regress to the piece sort and stay green. The "2 runs at Pieces" count is not pinned. A refused or `EmptyOk` verdict passes too.
- (b) It accepts `|v − want| ≤ VOLUME_CLAIM + pad`: containment loosened by 1e-7, not containment.
- (c) The ceiling is one `resolution_pad(area)` for every pose, about 1024·ε·area/2. I estimate a few 1e-6 at the default ε, but I did not confirm ε. The measured pads are ≤ 3.3e-10 off the rim poses, a slack of about 10⁴. This is stage 1's class, "`resolution_pad` rows pin far less than they measure", which stage 1 fixed with guards at 4× the measured pad.
- (d) Whether the body is an operand is read by substring-matching the printed line (`"t2=true t3p=true cert=true operand=false"`).
- Confidence: (a), (b), (d) *sure* (reading); (c) *likely*.

**MINOR 4: teapot wall 2's pin names the torus face only in prose** (`demos/tour/src/teapot.rs` `wall_probes`). The matcher still reads only `what.starts_with("a trimmed face with no volume lane on its chart")`. A sphere face refusing again would pass it. The spec's rule is that a pin naming a chart kind reads `refused_face_kind`. The PR body says a scratch probe read Torus; the pin itself does not. Confidence: *sure*.

**NOTE 5: the brief's premise.** The unit's spec file, `work/spiricarm/the-rotational-lane-reads-the-sphere-chart.md`, is not on the frozen head. It was filed in `f480b2716` on main, after the PR's merge base, and the branch never merged it. So the PR does not close its own unit row. I read the spec from `f480b2716`. *sure*

**NOTE 6: built results with no truth checked.** `verbs_cylsph_opening::a_transversal_pose_builds_in_both_poses` and the 21 seam-region door bodies now build, but no row compares their volume to a truth. I checked the transversal union independently: the volume is 19.6440330440 by 1-D Gauss–Legendre over the exact inner integral, against a certified 19.64403304427 ± 9.8e-10. That is 2.4e-10 off, inside the pad. *sure* for that one body.

**NOTE 7: two mutants survive harmlessly.**
- M9 (the `entry_turn` threshold ⅛ → ½ turn) survives. It is sound: `branch_turns` still refuses a turn not within a quarter of a whole number.
- M10 (the connector's level from one end only) survives. It is benign wherever the two ends' levels agree.
- *likely*

**NOTE 8: the corner pad `2g(e_i + e_j + g)`.** My derivation runs as follows:
- Each end lies within its slack s of the vertex image, so the uncovered region is bounded by the connector, the two envelope offsets, and the two carrier pieces from their feet to the vertex.
- If the carriers are near-straight at the scale g + e, its perimeter is ≤ 2(g + e_i + e_j). Then by the isoperimetric inequality (which only improves on the sphere) its area is ≤ (g + e_i + e_j)²/π, which is ≤ the pad.
- For a carrier that turns within g + e of the vertex, the carrier pieces are not bounded by s + e, and the bound is not derived.
- I found no breaking case. *unsure*

**NOTE 9: the connector costs the cylinder a round.** `hopeless_endpoint_brackets_stop_at_the_floor` now floors a round earlier, because the connector pad pays for the bracket's slack. This is a tightness cost on the cylinder, and it is disclosed. *sure*

## Claims

1. **Pole rule: UPHELD**, executed. Six probe rows, all holding under both names (own and twin) and both senses:
   - a hemisphere at `sin v = 0`;
   - cap sectors of π, 4, 6 and 2π − 1e-3 with their complements, where the pole junction's m = 1;
   - lunes of 0.5, π and 5 rad, with their complements;
   - discs of 1e-2 to 1e-4 and the sphere less each (at 1e-4 the disc stops at Floor, its enclosure still holding the truth);
   - two holes, and a zone less a disc;
   - a circle grazing the pole at ±1e-3 to 1e-7.
   
   By reading: `interval_loop_turns` and `junction_gap` take the same `round(mid/τ)`, so the connector's whole-turn choice and W move together, and `2πr²·ΔW` cancels `G(pole)·τ·Δm`. That makes the "arbitrary at a pole" argument sound. `require_sphere_sides` requires each loop's outside to lie certified in (0, 4πr²), so a wrong n straddles and escalates. A face cannot pass 4πr² off unless the A₀ enclosure itself is wrong.
2. **Sense bit: UPHELD on the route probed.**
   - Probe: I flipped each sphere face of the transversal union with `flipped_face_sense_for_tests` and re-validated with `AtRestBody::validate`. Both flips were refused: `LaminaWedge` on 3 edges.
   - Reading: `face_sense` reads `face.sense` off the face that the loops' half-edges name, and the loops come from `flat_loops` of that face.
   - Inside the lane a flipped bit is a valid complement and cannot be checked, by design.
3. **Integrand: UPHELD.** `sin v = p_z/|p|` is right, and the speed `r|p × p′|/|p|²` is the radial projection's speed (correct and weight-free). The pole grazes hold. `entry_turn` returns its widest reading at the cap, and `branch_turns` refuses one that is not narrow enough.
4. **Connector terms: UPHELD on both charts as rows; the corner pad is argued (NOTE 8).**
5. **Chart-term pad with the sphere's lever: NOT FALSIFIED, NOT WITNESSED** (MINOR 2).
6. **Oracles: INDEPENDENT.** The cap, zone and lune closed forms, and Simpson on each circle's own parametrisation, all come from outside the lane. Mutants beyond the PR's 8: MINOR 2.
7. **Routing: UPHELD** by reading. A tilted circle reaches the lane as a projected `Circle` image, so the refusal of a harmonic image that is neither a rim nor a meridian is reached only by an unusual pcurve. I did not find a face that reaches both routes or neither.
8. **Rows that moved:** MINOR 3, MINOR 4 and NOTE 6.
   - The other re-pins read the refused face's kind: `germ_sphere_torus_placements` (`torus_stop`), `germ_torus_components` (its `row`), and the seam-region door's `stop`.
   - I did not run the frame rows (about 3000 s); I relied on the PR's report of them.
9. **Tracker: P2 closure TRUE** (both rows exist and go red, M2 and the PR's own mutants). **P0 closure PARTIAL**, and the INSIDE band is wrong (MINOR 1). The spiric-wall P2 is plausible as filed, a structural twin that no run has verified. SPLITDOOR's unpark is fair.

## Style

Questions exercised: Q1 (parallel roles, with a constants grep), Q2 (comments doing code's work), and the stance's check on whether a fix mints a fresh instance of the defect it closes. I did not exercise the doc's later questions.

- **Q1, and the fix minting a fresh instance: the closure's whole-turn rule now has two homes.** `junction_gap` re-derives `interval_loop_turns`' `m = round(mid(Δ)/τ)`, and the pole rule is sound only while the two agree. One shared function would remove that dependency. *sure*
- **Q1: one u′ formula, written twice.** `unit_weight` and `net_rate` both compute `(x·y′ − y·x′)/(x² + y²)`. *sure*
- **Q1: `2·τ·r²` is built three times**, in `sphere_outside`, `sphere_signed_area` and `require_sphere_sides`. *sure*
- **Q2: sentinel arms the sphere never reaches.** `of_level` returns `Interval::refused()` and `level_flux_scale` returns `INFINITY` on the sphere. They are guarded only by the `edge_terms` match order. *likely*
- **Q2: per-loop F terms the sphere throws away.** They are computed every round and discarded, since the sphere's flux is `r·signed`. *sure*
- **`face_sense` walks half-edge → loop → face** to recover a face key the caller already holds. *likely*
- **`require_sphere_sides` undoes `sphere_signed_area`'s sense flip** with a second `if outward`. *sure*
- **The pole rule's module docs** are longer than the code they defend. Mild flag only. *unsure*

## Disclosures

- No glimpse of the other reviewer's branch, and none of the PR's comments or reviews. I read the PR body only. Nothing was posted to GitHub.
- I read `reviewer-style-lane.md` with its middle truncated by the tool's output limit, so some of the questions after Q2 went unread. I read only the length of `implementer-discipline.md`, not its text, and of `plan.md` only a search for "lesson", which found nothing.
- The unit spec was read from `f480b2716` (NOTE 5).
- Builds used `CARGO_TARGET_DIR=/tmp/claude-0/tgt`. The two long jobs (the mutant batch and the release build of `sweep`) ran `setsid`-detached and were polled in the foreground; both finished. No background work remains.
