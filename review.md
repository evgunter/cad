# Review of PR #3843, frozen head e284e7f11f

Lane `reach-dual3843-r2`. Wall clock 2026-10-02 13:58 → 15:10 UTC. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 4. No glimpse of any other lane (no `analysis/reach-dual/*` branch but this one fetched; PR read with `get` only).

## Claims
1. **Gate soundness: not falsified (sure, by execution).** I swept 12 revolved families (caps with c = 0.25, −0.5, 0.9, 1.3, the last bulging past the cylinder; truncated balls covering more than a hemisphere; pole-free barrel zones; torus roundings (a, r) = (1, ¼), (1, 0.05), (2, 0.9)) at scales ×1e-3/×1/×1e3, unposed and rigidly posed. The planes were random, and near-axial planes sat just clear of, inside the pad of, or just across the face's sampled extremes. My oracle is the closed-form ρ(y), a Gauss slice integral of circular-segment areas, and dense face sampling for "meets". At ε 1e-9/1e-6/1e-12: 0 planes admitted that meet the face, 0 volume mismatches, 0 tier-1/2/3 failures, and 0 `point_in_solid` disagreements in about 1.5k samples per ε. Attacks on the latitude-zone box by inspection: interior extremes come only from poles, and a pole inside a meridian edge takes the face out of the class (`solid_contain.rs:3123`). A face with a strut is banned at tier 2. That leaves the face containing a pole with a non-wrapping boundary, the complement of a chart rectangle. I built it through the public STEP door (`probes/gen_rect_complement.py`). `import_step` refuses it in every orientation and sense (`CurvedSenseInverted` / `LaminaWedge`), so it is not reachable (see N1). NURBS faces use the control-net hull and Approx faces the fit's hull, both sound for positive weights (inspection).
2. **Edges: not falsified.** An edge lies on both of its faces, so either face's sound box bounds it. The loft row goes red when face clearing is removed (mutant M4).
3. **Table rows: reproduced.** The probe widens the table as described above. Halves sum to the oracle total within pads.
4. **Mutants** (`probes/mutants.sh`). M1, gate always passes: the sweep flank/cap row goes red and the probe sees 778 meeting planes admitted, 88 of them silently wrong volumes. M2, box = whole ball: the cap row goes red. M3, pad = 0: **every row stays green** (MINOR 1). M4, edge face-clear removed: the loft row goes red.
5. **Refusal text:** true as far as it goes, but the recourse overclaims (MINOR 2). The replacement for `curved_face_refuses` is weaker than claimed (MINOR 3).
6. **Visibility and duplication:** see the Style section.

Suites: nextest `split|m3_pr2_reduce` on topo+sweep, 203/203 at each of ε 1e-9, 1e-6 and 1e-12, plus full topo+sweep at 1e-9 (3936/3936), all on the frozen head. I did not verify the PR's CI runs, so the verdict is conditional on that gate being green.

## Findings
- **MINOR 1 — the pad row cannot go red** (`crates/topo/tests/split_gate_per_face.rs:110`). DEMONSTRATED (mutant M3: pad × 0, all rows green). A plane ε below the ball refuses through the decide band (`escalate` = 10ε), not through the pad. A plane between `escalate` and `sweep_pad` (≈ 12ε) is the only placement that pins the pad.
- **MINOR 2 — the recourse and the title promise face-level reach, but the gate reads a world-axis-aligned box** (`topo/src/splitting/mod.rs:372`, `classify.rs:172`). DEMONSTRATED (`probe_posed_pr_fixture`): the PR's own cap fixture rotated about z by 0.3 or 0.7 rad refuses the φ = 0, y = ½ cut, which is 0.5 clear of the cap. The same cut splits at 0, 0.1 and π/2 rad. In the posed grid, every plane up to 0.1 clear refused. "Move the split plane clear of that face" is not sufficient advice. The refusals are conservative, not wrong.
- **MINOR 3 — the topo gate row does not isolate the gate** (`topo/tests/split_gate_per_face.rs:75`). DEMONSTRATED (mutant M1: this row stays green). Its tilted plane crosses the top face's edges, and `neighborhood.rs:121–133` raises the same variant with the same Display. Only the sweep flank rows pin the gate, and only for Sphere; the torus, NURBS and Approx refusals at the gate have no row that goes red. The PR body's "covers every unarmed kind at the gate" holds only for the admit half.
- **NOTE 1** — soundness of `gate_face_reach` (`classify.rs:106`) rests on `sphere_chart_trim`'s unguarded premise that a face lies in its boundary's chart rectangle (no loop-orientation test, `chord_join::azimuth_hull`). Today only tier 3's material-sign convention excludes the complement face (DEMONSTRATED via import). The torus window states this hazard and guards against it (`boxes.rs` `TorusChartWindow` docs); the sphere trim does neither.
- **NOTE 2** — `insert_crossings`' new non-line arm (`classify.rs:669`) is unreachable behind the gate and has no row (inspection).
- **NOTE 3** — halves first producible by this PR escalate in props at ×1e3 (`props_quad_converged`, cap and rounding, ε 1e-9). The refusal is typed, not wrong (execution).
- **NOTE 4** — the torus window box is coarse: rounding (2, 0.9) refuses a horizontal cut 1e-3 below the face (execution). Conservative.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q6, Q7. Q5 partly. Q8 not exercised (`classify.rs` was read only around the diff).
- **Q1, sure.** `gate_face_reach` (`classify.rs:106`) is a second spelling of a `FaceBoxRule` arm, living in a consumer. It is disclosed, and the fold-back is noted on an existing `work/boxes` row rather than scheduled as its own unit. `the_two_box_lanes_agree_face_for_face` pins `face_box` against `face_reach` but nothing pins this third box. The fix mints a fresh instance of the duplication it describes.
- **Q1, likely.** `box_clears` (`classify.rs:172`) is a new plane-against-AABB primitive with its own K name. I found no shared home (my grep was for `*box*side|*clears`), so its absence is unconfirmed rather than shown.
- **Q2, likely.** The comment "`lat_lo` is the extreme nearest the `+axis` pole, so the larger axial offset" (`classify.rs:131`) exists to reconcile inverted naming in `SphereChartTrim`. The comment works around the field names.
- **Q3, sure.** MINOR 1 and MINOR 3 are this question.
- **Q6, likely.** The two disclosed limits (an Approx arm covered at the gate only, and no spiric edge row) have no `work/` item.
- **Q4, unsure.** The new K names `split_gate_box_side` and `split_gate_sphere_axis` do not appear in `docs/predicate-dimension-audit.md`. I could not tell whether that file is owed an entry.
- **Q7, unsure.** I would have used a support-function test (one margin, `|n|·half-extent`) rather than eight corner decisions.

## Probes (`probes/`)
`sweep_probe_r2_3843.rs` (the random, grid and posed probes; drop it into `crates/sweep/tests` with an `all.rs` line). `step_import_probe_r2_3843.rs` + `gen_rect_complement.py` + `rect_complement.step` (the complement-face import; pass `swap` and/or `senseF` to the generator for the variants). `mutants.sh` (M1–M4, sed lines in the transcript of this lane).
