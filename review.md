# Review of PR #3982, frozen head 7ca2662950

Lane `reach-dual3982-r2`. **Verdict: APPROVE-WITH-FIXES**, with MAJOR 0, MINOR 1, NOTE 3 (+ style). Wall clock: start 17:10 UTC, end (pending) UTC (2026-10-03).
CI on the frozen head: run 37139134700, `test` / `lint` / `gate ok` all success (read off the head's check runs, not off the PR body).
Locally at the frozen head (`-p topo -p sweep`): ε 1e-9 4231/4231 passed; ε 1e-6 `--profile ci` 4181/4181 passed; ε 1e-12 `--profile ci` still running at first push.

## Claims, falsified by execution
1. **Sound: not falsified.** My probe (`probes/r2_probe_3982.rs`) uses seven new fixtures: a lollipop whose sphere zone is larger than a hemisphere, a shallow dome (r = 3), a **concave** torus fillet, a whole donut, a near-spindle fillet (R − r = 0.02), a big complement zone and a two-rim band. They take 12 random directions plus ±ŷ, an upright pose and two random rigid poses (one translated by ~2.9·s), and s ∈ {1e-3, 1, 1e3}.
   - The oracle is mine: the exact support of each meridian arc along `n` (crest-or-endpoint, closed form over the azimuth).
   - Meeting cuts, 1e-5·s and 1e-3·s into the face from both ends: **0 of 8208 split** across ε 1e-9/1e-6/1e-12 (3024 + 3024 + 2160; the 1e-12 run is partial, NOTE-3). A finer pass at s = 1 at ±1e-6·s and ±1e-7·s: **0 of 1008 meeting cuts split** at each ε.
   - A bug in my own oracle is disclosed: the lollipop rim was first written at atan2(−0.954, 0.3) instead of √0.91, which made 18 phantom "meeting splits". Re-run with the exact rim: 0.
2. **Pose-invariant: holds.** In every fixture × scale × ε cell, clear refusals are identical across the three poses. At ε 1e-9 and s ∈ {1, 1e3} it is 0 per pose. Under mutant M1 (main's `topo/src` at merge-base `b58d230fd7`) the same unit-scale cells refuse 40–48 of 48 clear cuts. Every clear refusal left at the head is a cut inside the 12 ε pad (the 1e-5·s cuts at s = 1e-3, ε 1e-9).
   - The PR's own `probe` at the head (ε 1e-9) reproduces its "after" table cell for cell, and on main its "before" table (36/52/61 …) cell for cell.
3. **Complement guard kept; admitted splits correct.**
   - Mutant M3 (`side_certified` forced) turns `a_sphere_face_whose_side_is_not_certified_keeps_the_ball` red.
   - My two complement zones (big zone, two-rim band) split only clear cuts.
   - Every admitted half was checked against my own volume oracle (adaptive Simpson in y over closed-form annular disk segments) within `volume_pad + 1e-7·s³`: **0 wrong**.
   - Re-splitting the face-bearing half between the first cut and the face (results reused as operands): **0 wrong** in 5386 re-splits at ε 1e-9/1e-6/1e-12.
   - `point_in_solid` at sampled points ≥ 1e-3·s from every boundary: **0 wrong** of 22,462 answered. Many halves refuse `VolumeUncertified`, which is typed and not counted.
4. **Rows red without the fix: mostly holds.**
   - M1 (main): `a_cut_clear_of_the_face_splits_in_every_pose` is red.
   - Loosened zone, `zone_extent` narrowed by 1e-4·r (M4) and by 1e-6·r (M4b): unsound (my probe splits 19–25 meeting cuts on the lollipop), and killed by `the_zone_extent_is_the_zones_support_along_every_direction`.
   - Pad dropped from `reach_clears` (M2): killed by `topo split_gate_per_face::the_box_is_read_with_its_pad`. I first saw M2 survive every sweep row, and withdrew that finding after running topo.
   - The one survivor is MINOR-1.

## MINOR
- **MINOR-1**, `crates/topo/src/census.rs:2808-2834`. The spiric edge arm is now read in the aimed frame, and no row reads it in any frame but World. DEMONSTRATED (mutant M6: `let axis = frame.vector(*axis)` → `*axis` at :2823 leaves all 2183 topo tests and 100 sweep split/reach rows green).
  - The gate reads this arm through `edge_clears` (`classify.rs:107`). Its only row, `boxes.rs:4749`, enters `census::face_reach`, which is the World frame.
  - A frame slip here makes the gate admit a plane across a spiric edge. It is latent while no public door mints a spiric edge on a split operand (`work/reach/split-gate-spiric-edge-arm-has-no-row.md`), but this PR wrote new code into that arm.

## NOTE
- **NOTE-1**, the e2e soundness row's premise. `reach_split_gate_pose.rs:472` `a_cut_into_the_face_refuses_in_every_pose` cuts 1e-3·s in, so M4 and M4b pass every sweep row and only the topo unit row kills them. Volume checks cannot see this failure mode: under M4, meeting splits came out with `wrong_vol` 0 in my probe and in the PR's probe (the sliver is far below `volume_pad`). The PR body's "wrong volumes 0" is not soundness evidence; the meets-split column is. EXECUTED.
- **NOTE-2**, `classify.rs:263-268`. The `split_gate_torus_ring` fallback (R ≤ r) is unreachable through `revolve`: a spindle fillet refuses `UnsupportedToroid` at build. The near-spindle R − r = 0.02 takes the closed form and is sound. EXECUTED.
- **NOTE-3**. At ε 1e-12, my full probe hit its time limit after 45/63 cells (s = 1e3 partly). Within that run, 0 meeting cuts split and 0 volumes were wrong. 1 clear cut at s = 1e3 refused typed later (pcurve certification), consistent with the PR's own 1e-12 / 1e3 disclosure. EXECUTED.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7 (all), and Q8 partly. For Q8 I read `classify.rs`'s structure end to end (1233 lines) and the census `face_reach_in` / `edge_reach_in` bodies in full.
- `classify.rs:240-330`, Q1/Q7. Two exact box extents (`zone_extent`, `torus_rect_extent`, `most_cos`) are hosted in the split's classifier, beside vertex classification and conic roots. They are now a third spelling of "a torus face's support", next to `boxes::torus_window_extent`, which is sampled plus a charge, and the rule's tube. The fold is scheduled (`work/boxes/split-gate-sphere-zone-folds-into-face-box-rule.md`). *likely*
- `classify.rs:269-273` vs `census.rs:2646-2657`, Q1. The window is composed (`face_window_steps` → `torus_chart_window`) a third time; census already holds a private wrapper for exactly this. *sure*
- `classify.rs:349`, Q1/Q7. `reach_clears` hand-spells `n·origin`, where the code it replaced, and `classify.rs:373` in the same file, use `sector_shape::plane_offset`. *likely*
- `classify.rs:103` and `:140`, Q7. `edge_clears` rebuilds `BoxFrame::aimed` per edge instead of taking `gate_operand`'s frame (`:48`). `gate_face_reach` always computes the rule's sampled 17×17 torus box, even when the closed form replaces it. *likely*
- `census.rs:2607-2618`, Q2/Q4. The cylinder clip's comment ("footprint perpendicular to the axis") clips coordinates 0 and 1 and leaves 2 unclipped. In an aimed frame, coordinates 0 and 1 are `(n, b1)`, and nothing is perpendicular to the axis by construction. The argument that makes it sound in any frame (`α·v + g(u)`) is only in the PR body. By that argument coordinate 2 could be clipped too, so the asymmetry is unexplained. *likely* (stale comment); *unsure* whether anything else relies on it.
- `classify.rs:238-239`, Q2. "Moves `G` by as little, since `G` is continuous": continuity is not a modulus. `G′` is unbounded as `|h| → r`, so near a rim close to a pole, a rounding δ in `h` moves `G` by ~√(rδ)·√(1−a²). Nothing measured failed. *unsure*
- `boxes.rs:360-363`, Q6. "Rides with the rest of the box's rounding under the pad" is an unmeasured claim, and `BoxFrame::point` rotates about the WORLD origin, so its rounding grows with |p|. My translated poses (≤ 2.9e3 m) stay green; no row poses a body far from the origin. *unsure*
- `reach_split_gate_pose.rs:467-470`, Q5. The doc promises a second cut, "one across the truncated ball's flank … crossing no edge". The body runs only the tilted 1e-3 cut. *likely*
- `work/reach/split-gate-spiric-edge-arm-has-no-row.md:16,21`, Q4. This open item still cites `census::edge_reach`, which this PR renamed to `edge_reach_in` (grep-proven absent). The symbol rotted; the item's claim stays true. *sure*
- Q4 sweep for "axis-aligned in world": `SplitReduceError`'s doc (`splitting/mod.rs:223`) was updated, and the refusal text still holds ("bounding box", now in the plane's frame). No other citer was found. *likely*

## Probes and mutants
- `probes/r2_probe_3982.rs` is the whole probe. It ran as `crates/sweep/tests/` + `mod` in `all.rs` with `cargo nextest run -p sweep --test all --run-ignored all r2_probe_3982::<r2_probe_full|r2_probe_unit|r2_probe_fine|r2_probe_pole>`, under `CAD_TOLERANCE_EPS` ∈ {1e-9, 1e-6, 1e-12}.
- Mutants (each applied alone in a separate worktree), with which rows they turned red:
  - M1, main's `topo/src`: the pose row.
  - M2, no pad: the topo pad row.
  - M3, no complement guard: #3843's guard row.
  - M4 / M4b, zone narrowed by 1e-4·r / 1e-6·r: the topo zone row only.
  - M6, spiric axis read un-rotated: none.

## Glimpse disclosure
I read no other lane's branch, scratch or report, and no PR comments or reviews. One incidental read: a Q4 grep over `docs/` printed `docs/DUAL-REVIEW-LOG.md` row DR-44, which records PR #3843's merged dual (a different PR). It told me #3843 had pad and zone rows, which is why I ran topo under M2. It contains nothing about PR #3982.
