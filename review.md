# Review of PR #3982, frozen head 7ca2662950

Lane `reach-dual3982-r1`. Wall clock: start 2026-10-03 17:10 UTC, end 18:29 UTC. Glimpses: none. I read only my own brief file on the briefs branch, and the PR body via `get` (plus the head's check runs). I read no comments, no reviews and no other `analysis/reach-dual/*` branch.

**Verdict: APPROVE-WITH-FIXES** (the fixes are MINOR coverage work, not correctness). **0 MAJOR · 1 MINOR · 4 NOTE**, plus a style section.

## Claims, by execution
Probe `probes/dual3982_r1_probe.rs` (copied into `crates/sweep/tests/`, `#[ignore]`d) has its own oracle. Each fixture's profile is written by hand twice: as an inside test, and as the meridian arcs of its sphere or torus faces. Support along `n` is the meridian sampled at 40k points with a closed-form max-cos over the azimuth window. Volumes come from a 700² grid over `(ρ, y)` with the exact azimuth measure of the half-space per cell, and square cuts are integrated as fractional cells. `point_in_solid` is checked at 24 random points per half, plus 12 per quarter after a second split of the reused half.

The fixtures go beyond the PR's own. All are revolved about `y`:
- a complement torus band (a 240° meridian arc, Θ = 2π, and the same band partial at 1.0 and 4.5 rad);
- a ball truncated at both ends (one zone with both rims);
- the capped cylinder partial at 2.0 rad;
- the rounded cylinder partial at 2.0 and 5.5 rad.

Each is tried in 4 poses (upright, turned 0.3 rad, the PR's skew, and a second skew of 2.2 rad about (−.4, .3, .86) plus a translation). Each pose is cut by 156 planes: 26 directions, ±{3e-2, 1e-3, 1e-4}·s about the face's least and greatest support.

1. **Soundness: holds (sure).** 237 fixture×pose×scale×ε rows: ε ∈ {1e-9, 1e-6, 1e-12} × s ∈ {1e-3, 1, 1e3}. The 1e-12 × 1e3 cell hit the 2400 s timeout partway, and `transform_rigid` refused some poses there, as the PR reports. In every row, **0 planes that meet a sphere or torus face split**. Meeting cuts at 1e-4·s refuse in every pose, including the complement band and both-rim zone.
2. **Pose invariance: holds (sure).** Every head row is identical across the four poses at s = 1. On main (`b58d230fd`, same probe) the gate is sound but splits by pose. Clear cuts split in each of the four poses, in order:

   | fixture | head | main |
   |---|---|---|
   | complement band | 156 / 156 / 156 / 156 | 14 / 2 / 0 / 0 |
   | both-rim ball | 156 in each pose | 18 / 0 / 0 / 0 |
   | rounded, Θ 2.0 | 156 in each pose | 33 / 9 / 0 / 3 |

3. **Newly admitted splits are correct (sure).** Every admitted half agrees with the oracle within ≤28 ppm of the body. That is the grid's own error: the whole-body check alone is 24–25 ppm. `point_in_solid` gave 0 disagreements, and reused halves gave 0 after a second split. The complement-face guard holds: the truncated and both-rim zones refuse meeting cuts, and `a_sphere_face_whose_side_is_not_certified_keeps_the_ball` stays green.
4. **Mutants (each probe run is one fixture subset at s = 1, plus the PR's rows):**
   - M1, the world-axis box (main's `box_clears` with the world frame): killed by `a_cut_clear_of_the_face_splits_in_every_pose`.
   - M2, the pad ×1e5: killed by the same row.
   - M6, `a.abs()` in the zone's polar term: killed by 4 PR rows, and 12 meeting cuts split in my probe.
   - M7, aimed points left unrotated: killed by 4 PR rows.
   - M3 (crest offset read from `w.hi`) and M4 (the torus `v` crest read on the `u` window): see MINOR-1.
   - M5, the pad dropped: see NOTE-2.

   Runner: `probes/dual3982_r1_mutants.py`.

Suites (local, `CARGO_INCREMENTAL=0`, own target dir), all green: `nextest -p topo -p sweep` at ε 1e-9 gave 4231/4231; `--profile ci` at ε 1e-6 gave 4181/4181, and at ε 1e-12 gave 4181/4181. CI on `7ca266295`: run 37139134700, all jobs green.

## Findings
- **MINOR-1 · `crates/topo/src/splitting/classify.rs:303–328` (`torus_rect_extent` / `most_cos`) · DEMONSTRATED BY EXECUTION.** No committed end-to-end row sees a partial chart window. Mutants M3 and M4 are unsound on purpose. Both pass every committed sweep row, including both new pose rows and PR 3843's per-face rows. Only the unit row `the_torus_rect_extent_is_the_rectangles_support_along_every_direction` kills them.

  That unit row builds its torus points from its own closure (`classify.rs:1198`), not the kernel's surface evaluation, so the chart convention is written twice by one hand. Under M3, my probe admits 89 meeting cuts on the complement band and 14 on the 1.0-rad band, and M4 admits 9. Every committed fixture has a full-turn `u` window and a quarter `v` window, and full-turn `u` makes the `u` convention invisible. Adopt a partial-revolve and a complement-band fixture as rows.
- **NOTE-1 · `classify.rs:240–247` (`zone_extent`) · DEMONSTRATED.** The zone ignores the azimuth window: the sphere trim certifies on all 1416 calls, instrumented. So on the capped cylinder partial at 2.0 rad, 87/156 cuts clear of the face refuse, in every pose and at every scale. Classified against the full turn instead, all 156 split. This is sound looseness the PR does not claim to remove, but it is neither mentioned nor filed.
- **NOTE-2 · `classify.rs:350` · DEMONSTRATED.** M5 (pad × 0) survives every row and my probe. No row sees the sweep pad, before or after this PR. It is a rounding margin, so it is probably unguardable, but no line at the claim site says so.
- **NOTE-3 · `work/reach/split-gate-spiric-edge-arm-has-no-row.md:16,21` · inspection, sure.** It still cites `census::edge_reach`, which this PR renamed to `edge_reach_in` (Q4).
- **NOTE-4 · scope of claim 1 · inspection.** Spiric and NURBS edges are read through `edge_reach_in` in the aimed frame (`census.rs:2808–2836`). Each coordinate is a linear image of the frame components, so it is sound by inspection (likely). My probes build neither: no public door reaches a spiric edge here, as `census.rs` itself notes. The lemon and apple (spindle tori, `R < r`) are refused at `revolve` (`UnsupportedToroid`), so the `split_gate_torus_ring` fallback has no public door either.

## Style (Q1, Q2, Q3, Q4, Q5, Q7 exercised; Q8 partly: `classify.rs:1–520` read in full, the rest by signature; `census.rs` diff hunks only)
- Q1 · `classify.rs:349` vs `classify.rs:373`. `reach_clears` spells the plane offset by hand (`n.x*origin.x + …`), while `classify_vertices` in the same file calls `sector_shape::plane_offset`. These are two spellings of one residual. **sure**
- Q1 · `classify.rs:240`/`:303` vs `boxes.rs:584` (`slab_extent`) and `boxes.rs:750` (`torus_window_extent`). The sphere-zone and torus-window extents now have an exact spelling in the split gate and a sampled or slab spelling in both box lanes. The core lives in a consumer, and `the_two_box_lanes_agree_face_for_face` does not cover the gate's copy. The fold is filed (`boxes/split-gate-sphere-zone-folds-into-face-box-rule`), so this is scheduled. Until then the copies can drift. **likely**
- Q2 · `boxes.rs:346–352` (`BoxFrame` doc). Soundness rests on "every per-kind extent computes coordinate i from coordinate i alone". Nothing enforces it: a future extent that reads a norm or a cross-coordinate term would be silently unsound under `Aimed` and still exact under `World`. That is the shape the `World` rows cannot see. **likely**
- Q2 · `boxes.rs:401–403` (`BoxFrame::unit`). The type says it is minted only from a decided `UnitVec3`. It now has a second mint from a rotated vector whose rows are orthonormal only to rounding, and the doc defends this in prose. **unsure**
- Q7 · `classify.rs:134–159` (`gate_face_reach`). The rule box is built first, including the boundary walk and the torus window sampling, then discarded whenever a patch reads. In the aimed frame two of the three coordinates of every box are computed and never read. The natural query is a support function along `n`; a box in a frame is the long way round. **likely**
- Q2/Q7 · `census.rs:2615–2619`. The cylinder's boundary clip still clips coordinates 0 and 1 and never 2. The PR body's own argument (a linear functional on a cylinder takes its extremes on the boundary) clips every coordinate. In the aimed frame, coordinate 2 is an arbitrary `b2`. This is harmless, but the asymmetry is unexplained. **unsure**
- Q5 · `splitting/mod.rs:223–225`. The refusal text "clear of that face's bounding box" is argued to stay true because a support is never wider than any box's support. That argument covers the exact extents. I did not check it for the sampled torus window box or the cone and cylinder rule boxes read in the aimed frame. **unsure**
- Q3: the new rows can fail, demonstrated by M1, M2, M6 and M7. The gaps are MINOR-1 and NOTE-2.
