# PR 5853 dual review — blind coding inputs

The A/B labels were assigned by a recorded byte; the mapping is held by the orchestrator, not in this file.

Frozen head: e78b63db1ce014a558391fb287b7cd6102b9038e (merge-base 8f321f54). Arm: concurrent, letter H. Both reviewers were dispatched at 12:24Z with one brief. Neither was interrupted, and both report that lane isolation held.

## Review A

FULL review, PR #5853 @ e78b63db: APPROVE-WITH-FIXES. MAJOR 0 / MINOR 2 / NOTE 4.

Probe branch: `the reviewer's probe branch` (commit <sha>), with `crates/sweep/tests/<probe file>.rs` registered in `all.rs`. The rows are `review_abutting_shapes_are_no_same_side`, `review_near_coincident_abutting_halves_are_no_same_side`, `review_overlapping_shapes_are_one_same_side` and `review_pin_grid_cost`.
Lane isolation: I fetched only `main` and the PR branch, and read only the PR body and diff. I saw no other `review/atrest-curved-*` branch, no PR comments and no review scratch.
Method: head e78b63db against merge-base 8f321f543, in separate worktrees and private target dirs. Every probe body is tilted about (0.3, 0.2, 1) by 0.7 and moved off the origin, and was run at CAD_TOLERANCE_EPS 1e-6, 1e-9 and 1e-12.

## Claims

1. No false interference: HOLDS (executed, sure). With 0 SameSideInterference expected, at all three ε, head and base read identically (same finding counts, same flush rows), and none is interference:
   - a stacked cylinder spun 1.1, so the seams lie apart on the shared rim;
   - lunes (ball wedges of π) side by side;
   - a reflex wedge (1.5π) and its complement, for both the cylinder and the lune;
   - quarter wedges abutting on a ruling;
   - a cut ball and the complementary upper cap, which meet only at the rim circle, with seams apart and with seams met;
   - a rod in a tube of inner radius 1 (opposed);
   - a half-cylinder against its mirror image (`mirror_body`);
   - abutting halves, the second's radius off by 0.5, 3, 10, 100 and 1000 ε.
   Offsetting the second half's axis by d gives two readings. Moved away (a gap), it reads 0. Moved into the first, it reads 1 SSI from k ≥ 10 ε, decided on the half-disc pair. That sliver of material really is shared, so it is a correct new positive; on base it read 0 SSI.
   Positive sanity (head 1 SSI each, base 0): a mirrored full copy, a copy flipped by π about x, wedges a quarter apart, lunes a quarter apart, and coaxial copies at overlapping heights.
2. The witness's soundness argument holds (read, likely; the abut probes above are its empirical side).
   - Interiors meet when `y` holds `q` In, or `y` is Full at `q` (a seam it lies on both sides of): `x`'s closure holds `q`, so `x`'s open interior reaches into `y`'s neighbourhood of `q`.
   - Seam ends and rim vertices a face meets twice fail `face_cone`'s `[(leaving, before)]` match, so they are Unbuilt and never a witness.
   - A full ball's pole is visited once, but `from == to` there. That cannot produce a false positive, because a full sphere face really is Full at its pole. Lune poles are read as true corners, and the lune and reflex-lune rows decide them correctly.
   - The tangent-plane cone uses first-order edge tangents. That is sound on a C¹ boundary at a point visited once; zero-angle cusps are the only gap, and I found no revolve-built fixture that makes one.
   - `placed` and `curved_face_containment` are decided-only: boundary-walk margins first, OffCarrier→Out, typed refusals otherwise. `EdgeCurve::mid_point` lies on the edge's certified interval (geom-brep certify.rs:2745), so midpoint probes are on their edge.
   - A seam midpoint that `y` places OnEdge reads Full only when both half-edges are `y`'s (census.rs `face_cone`), which is correct.
3. Ok(false) never arises for arcs or curved faces, by type: HOLDS (read, sure). `shared_interior_witness` returns `Result&lt;SharedInterior, _&gt;`. `face_interiors_meet` sends polygon×polygon to the unchanged `planar_interiors_meet`, so the polygon path and its completeness argument are untouched. In `same_side_overlap`, polygon pairs take the same calls as before. Non-polygon pairs only change from Err(Unbuilt) or NonPlanarTrim to Ok(true). `in_band_reading` passes `Bridged`, so arcs in band are unchanged.
4. Arm 1's skip: SOUND (read, likely). The skip is per face pair, and `coincident_pairs` in arm 2 reports every solid pair present in `same_side`, so loudness is kept. Other pairs between the same solids, such as the rim pairs, and pairs with any third solid, still reach arm 1. `Declared::confirm` absorbs only touches between cells of the two faces of the (A, B) pair, inside the closure of its overlap. Those solids are already decided to interfere, so nothing loud is lost. A crossing with a third solid C is keyed to a different pair and is not absorbed.
5. Census otherwise unmoved: HOLDS (executed, sure).
   - editor-core `perf12_census_goldens` and `perf12_census_bvh_diff`, step-import `perf12_census_goldens`, and `copies_of_one_cylinder_filling_one_volume_are_one_interference`: green at 1e-6, 1e-9 and 1e-12 on head.
   - topo `--lib` filtered census | link | two_copies (`--ignore-default-filter`): 282/282 at each ε. That is not the body's 352; I ran `--lib` only, so the difference is presumably the integration binary (unsure).
   - The PR's 4 new sweep rows pass at all three ε on head. On base, `copies_of_one_{cylinder,ball}` fail and the abut and in-band rows pass.
6. The editor-core reading is right (read, likely): one loud finding, `Unlocalized::Refused`, evidence 1 SSI + 8 rim pairs + 2 seam half-bands. The two filed rows cover flush-never-certified and localization. The in-band residue is not covered; see MINOR 2.
7. Cost: SMALL (executed).
   - Census goldens, base vs head, wall time over two alternating runs: editor-core 75.3/74.8 s vs 72.3/73.7 s; step-import 27.2/28.4 s vs 26.4/27.0 s. No measurable change.
   - Pin grid (tangent cylinders at pitch 2, each its own solid; debug build, three runs): 4×4 ≈ 0.77 vs 0.89 s, 6×6 ≈ 2.16 vs 2.36 s, so about +10–15%. The 4×4 grid with a coincident copy of each pin is about even (3.4 vs 3.8 s) and now reads 16 SSI where it read 0.

## Findings

- MINOR 1 (claim 5 / C6 wording; Q4). `crates/editor-core/ASSEMBLY.md` *Interference* paragraph (~l.601–603) still says SameSideInterference is "a face of each on one plane facing the same way". The PR re-worded README C6 and the Display, but not this page. Not executed. sure.
- MINOR 2 (claim 6 / Q6). The PR body says the row stays open on "the in-band rule". The work item's "What stays open here" lists only rim pairs and cone/torus/spline faces, so curved copies in band are no longer recorded anywhere. They are the `copies_apart_within_the_band` residue: Door 1 is not Definite and arm 1 still refuses them. `an-aligned-in-band-pair-is-never-read-as-interference` is planar-only. File: `work/atrest/curved-copies-filling-one-volume-read-as-undecidable.md`, Narrowed section. Not executed. sure.
- NOTE 1. census.rs `shared_interior_witness` drops its `read` vector, so an escalation (CensusEscalated) during a probe reads as `InteriorsUnread::Unbuilt`, never `Undecided`. Downstream, a pair in band and a pair whose reading is not built get the same reason (`flush_at_rest` and FLUSH_UNREAD). The rustdoc says so. likely.
- NOTE 2. `curved_same_side_pairs` runs only on a pass holding a region lane, because it sits after `planar_face_pairs`' `let Some(lane) = region else { return … }`, although it needs no region door. On a pass with no region door (interval/dual scalars, the `_structural` twin), curved copies stay undecidable. Nothing at the function says so. sure about the coupling; unsure whether it matters.
- NOTE 3. The topo row count differs from the PR body (282 vs 352); see claim 5. unsure.
- NOTE 4. Cost: about +10–15% on a dense same-kind curved grid (see claim 7), and `sweep_cross_solid_backstop` scans `same_side` linearly on every reach candidate pair. Negligible on the goldens. likely.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was skipped: census.rs is 19,770 lines, so I read its header and the touched regions, not the whole file.
- Q1. One rule now has three spellings: "y holds q In, or both cones are read and Full or `corners_meet`". They are `planar_interiors_meet` (census.rs ~10376–10400), `in_overlap_closure`'s Area arm (~2016–2048), and the new `meets_at` + `cones_meet`. They already differ: `in_overlap_closure` pushes escalations, and `cones_meet` swallows them. Look for more in `confirm_face_pair` and the line-wedge reads. likely.
- Q1. `face_edges` now has a third copy in topo: census.rs (new), boolean/slit.rs:1612 and boolean/solid_contain.rs:3303. `face_vertices` re-derives `FaceGeo::boundary` for planar faces. Both silently `filter_map(Result::ok)` over `face_cycles`, so a corrupt cycle is skipped rather than raised; that is conservative here, but not fail-loud. likely.
- Q1/Q4. The Cylinder|Sphere test in `curved_same_side_pairs`' `kind` closure is a hand-kept mirror of which curved kinds Door 1 has exact coincidence rungs for. If Door 1 gains a cone rung, this list will not follow. unsure.
- Q5/Q7. `planar_face_pairs` now returns curved pairs: the name and its "planar arm" headings promise less than it does, and the curved half is reached by a call tucked at its tail. I would have made it a sibling called beside it in the census driver (~l.958). likely.
- Q2/Q5. `Corner`'s doc still reads "One corner of a planar face". It is now read in the tangent plane of cylinders and spheres too. This is partly pre-existing via `in_overlap_closure`. unsure.
- Q3. `copies_that_abut_along_a_rim_or_a_seam_are_no_interference` and `copies_apart_within_the_band_are_no_interference` pass on base, so they guard against new false positives but cannot show the new arm runs. That fits what they claim. `copies_of_one_{cylinder,ball}` do go red on base. sure.
- Q6. The cost claim in the body ("adds box candidates and Door 1 per curved pair") has no guard. The goldens' wall time is the only register, and nothing goes red if a many-cylinder body regresses. unsure.

## Review B

Full review of PR #5853 at e78b63db: APPROVE. 0 MAJOR, 1 MINOR, 4 NOTE.

Lane isolation: I read only the PR body and diff. I read no comments, no other review/atrest-curved-* branch, and no review scratch.
Probes: branch `the reviewer's probe branch` @<sha>, `crates/sweep/tests/<probe file>.rs`, registered in sweep `all.rs`. Every body is tilted (0.3,0.2,1)/0.7 and moved off the origin. All 4 rows pass at ε 1e-6, 1e-9 and 1e-12. I ran the same file on a merge-base worktree (8f321f54) to compare.

## Claims
1. No false interference: HOLDS (executed, sure). `probe_faces_of_one_carrier_that_only_abut_are_no_same_side` reads 0 SameSide and 0 interference, at head and at the merge-base, at all three ε, on:
   - hemispheres meeting at the equator;
   - half balls side by side;
   - quarter balls opposite each other, sharing only the poles (the pole-cone attack);
   - quarter balls adjacent;
   - quarter cylinders sharing only the axis;
   - a half cylinder beside its `mirror_body` image (left-handed frames);
   - bands end to end with their seams apart;
   - tubes end to end;
   - a shaft in its bore (opposed);
   - a ball in a spherical shell's cavity (opposed).
   The finding counts per row are identical at head and the merge-base.
   Near-coincident cylinders (`probe_nearly_coincident_cylinders_are_no_same_side`): radius 1+kε stacked on radius 1, k = 3, 10, 100, 1000, reads 0 SameSide. Coaxial copies with the axis moved by 100ε or 1000ε now read one SameSide; the merge-base read them undecidable. That is correct: the copies share almost all their volume. At 3ε and 10ε the reading stays escalated or undecidable.
   Positive rows (`probe_overlapping_copies_are_one_same_side`) each go from 0 to 1 SameSide:
   - coincident tubes (reversed bore walls, SameOriented);
   - a cylinder beside its mirror image, seam opposite;
   - half balls overlapping by a quarter;
   - a hemisphere inside a ball.
2. Witness soundness: HOLDS (read, sure).
   - `EdgeCurve::mid_point` is the midpoint of the certified interval (`geom-brep/certify.rs` `mid_point`), so it is a point on the edge, not the carrier's midpoint.
   - The `In` probe needs only that q lies in the closure of x and that Door 1 decided Zero.
   - Face cones that are half-planes are robust to curvature, including tangent rims.
   - A seam end or pinch reads `Unbuilt`, which only means no witness was found.
   - The pole rows above are clean.
   - `placed`/`curved_face_containment` give a verdict only where decided; band and escalation give `None`.
   - `Cone::Full` on either side returns true without checking that the other corner has a positive opening. The planar path has the same short-circuit. Only a zero-angle cusp corner at a both-sided edge could exploit it; I built no such row (unsure, see Style).
3. Ok(false) never arises off the polygon path: HOLDS (read, sure).
   - `shared_interior_witness` returns `Result&lt;SharedInterior, _&gt;`.
   - The polygon arm of `face_interiors_meet` is `planar_interiors_meet` unchanged.
   - In `same_side_overlap`, every branch that formerly reached `planar_interiors_meet` with a non-polygon got `Unbuilt`; it now gets `Unbuilt` or `Ok(true)`.
   - So no flush row in `flush_at_rest` can appear or vanish. Only an unread finding can become interference.
4. Arm 1's skip is sound (read, likely).
   - The skip is exact on the face pair. Cap and rim pairs on the same solids are still probed: the editor row carries 10 undecided pairs beside the decided one.
   - Arm 2 keys `coincident_pairs` per solid pair and reports SameSide, or InstanceInterference, "gate or no gate". The census is group-agnostic, so no group filter can drop arm 2.
   - `Declared::confirm` absorbs only touches in the closure of the confirmed pair's area. Those cells belong to the two solids already decided to interfere, so no third-solid touch is reachable.
5. Census otherwise unmoved: HOLDS (executed, sure).
   - At head, ε 1e-6, 1e-9 and 1e-12, these all pass with nothing re-blessed: editor-core `perf12_census_goldens` and all 5 `perf12_census_bvh_diff` rows; step-import `perf12_census_goldens`; topo `-E 'test(/census|two_copies|link/)'` (357/357).
   - The polygon and planar paths are unchanged in the diff.
6. Editor result is the right reading (executed and read, likely).
   - `copies_of_one_cylinder_filling_one_volume_are_one_interference` passes at all three ε.
   - The 1+10 evidence count is 8 rim pairs and 2 seam half-bands. The rim pairs stay on the narrowed item; the seam half-bands are under `aligned-arc-or-curved-faces-that-only-abut-are-never-read-flush` (P1). `Unlocalized::Refused` is under `coincident-curved-copies-overlap-is-unlocalized` (P2).
   - Cones, tori and splines stay on the narrowed item. The residue is covered.

## Findings
- MINOR (claim 7, executed, likely): the cost of `curved_same_side_pairs` is real on aligned curved neighbours, and the PR does not state it or guard it.
  - `probe_pin_stack_cost` is a 3×3 grid of 4-pin coaxial stacks abutting end to end: 36 solids and 27 SameOriented Definite pairs. Each pair runs the full witness search and finds nothing.
  - Debug, 3 runs each: head 757/835/834 ms against merge-base 514/566/520 ms, about +55%.
  - The heaviest golden, editor-core `perf12_census_goldens` at 1e-9, is flat: 36.1–36.5 s at head against 35.5 s at the merge-base. The goldens contain no coaxial abutting pins.
  - The cost scales with the box-candidate pairs that are both cylinders or both spheres and that Door 1 reads as SameOriented, times the edges² meeting probe.
- NOTE (read, sure): `shared_interior_witness` never returns `InteriorsUnread::Undecided`.
  - It drops `read` and turns every escalation in `cones_meet`/`meets_at` (`.ok()?`) into `Unbuilt`.
  - A Definite arc pair whose only witness lands in the band is therefore `FLUSH_UNREAD`/MayOverlap rather than the band finding (`flush_at_rest`'s `undecided()` arm).
  - This is no regression, since the merge-base read every non-polygon as `Unbuilt`. But the `meets_at` doc says "None where a reading escalated" and nothing records the escalation.
- NOTE (executed, sure): the PR's negative rows cover cylinders only. No sphere, pole, mirrored or opposed row exists in `curved_copies_filling_one_volume.rs`. My abut and opposed rows would close that gap if adopted.
- NOTE (read, sure): the arm 1 skip in `sweep_cross_solid_backstop` scans `same_side.iter().any(..)` linearly for every reach candidate pair, although `coincident_pairs` (a map) already exists. This is O(pairs·|same_side|).
- NOTE (dispatch premise, sure): the brief lists "into `same_side` and `Declared::confirm`" as new. The confirm wiring (`census_with`, `found.opposed.iter().chain(&amp;found.same_side)`) predates the PR; only the new contents flow through it.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only partly: census.rs is about 11.7k lines; I read the touched regions plus `Declared`, `placed`, the closure reader, `flush_at_rest` and arm 2's head, not the whole file.
- Q1, sure: `cones_meet` (census.rs, beside `meets_at`) is a third spelling of "two cones share material: Full on either side is true, two corners go to `corners_meet`". The others are inline in `planar_interiors_meet` (the `(Ok(Cone::Full), _) | (_, Ok(Cone::Full))` arm) and the closure reader around census.rs:2040–2048. The three differ in how they treat escalation.
- Q1, sure: census.rs `face_edges` is at least the third copy, after `boolean/slit.rs` `face_edges` and `boolean/solid_contain.rs` `face_edges`. `face_vertices` parallels `FaceGeo::boundary` for planar faces. No sentence admits either copy.
- Q5, likely: `planar_face_pairs` now also gathers curved cylinder and sphere pairs (its last line calls `curved_same_side_pairs`). The function name no longer says what it does, though the module doc was patched.
- Q2, likely: `InteriorsUnread::Unbuilt` now also means "searched and found no witness". The variant doc was re-worded to say so, which overloads one variant with "not built" and "not found".
- Q4, unsure: the `Cone::Full` short-circuit in `cones_meet` assumes the other corner has a positive opening. The precedent in `planar_interiors_meet` makes the same assumption, and nothing enforces it.
- Q7, unsure: `same_side_overlap` now has two consecutive `if touching` branches gated on `exact || polygon` and a third `arcs &amp;&amp; exact` branch. The control flow is harder to read than one match on the pair of readings would be.
- Q3, likely: `copies_apart_within_the_band_are_no_interference` only asserts that SameSide is absent, which still passes if the curved arm stops running entirely. The positive rows guard that, but the in-band row alone cannot go red on a degraded arm.
- Q6, sure: every disclosed deviation has a work item: rim pairs and cones, tori and splines (narrowed item), the abut certificate (P1), localization (P2). The only unscheduled item is the cost claim above, which has no measurement and no guard.
- Off-target (unsure): `curved_same_side_pairs` runs `kind()` for each inner candidate before the cheaper solid check. That ordering is fine as written.
