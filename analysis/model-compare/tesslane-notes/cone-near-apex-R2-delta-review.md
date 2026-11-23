DELTA RE-REVIEW of PR 6965 at frozen head 26ce505802 (merge base e99f23c6)

**Verdict: APPROVE.** The verdict holds once a green CI run lands for the head that merges. No hosted run on 26ce505 ever completed: run 43938519804 was cancelled at 19:47Z, when the branch moved. Before the cancel, `test (kernel)` default-ε, `test (app)` (all three ε) and the mesh budget meter + falsifier had passed, along with the corrupt-input, tools, viewer and freecad jobs. The kernel ε 1e-6/1e-12 step, lint (rustdoc step), demos and python were cancelled. Nothing was red. I did not check main for any of these, since nothing was red.
The branch is now at e3f8adf, which merged the ring unit ("the shared-side rule reads every loop"). My review does not cover it: the ring guard my point-1 reasoning relies on (`trimmed.rs`, `RingOnCurvedFace` when `face.rings` is non-empty) is what that merge changes.

## The eight points
1 HELD, DEMONSTRATED.
- Wedge grid, run on both trees: the T1 cone ∩ box turned −50°, edge at (xf·s, h−1.5s); xf ∈ {0.5, 0.8, −0.5, 0, 0.2, 1.0} × s ∈ {0.6…3e-4} × both orders × δ {2e-2, 5e-3, 1e-3, 5e-4}.
  - Merge base: 55 of 312 meshes return `Ok` with `NonManifoldEdge{count:4}`.
  - Head: 0 of 312; every one passes `check_mesh` and `geometry`.
  - Not built on either tree: xf=1.0 gives an empty result, and s=3e-4 refuses `CurvedPierceEscalated`.
- Mutants:
  - cone arm reads the ear alone: reds the sliver row and the needle row;
  - neighbours flagged too: reds all 7 cone reach rows;
  - seam put back into `faces_across`: reds nothing.
- By inspection:
  - the polygon is the outer loop alone, so the cyclic `(i+1)%n` reading is right at the loop's ends;
  - a ring chain cannot reach the rule (rings refuse);
  - a curved neighbour is covered, because a four-use edge needs both faces and the cone keeps none;
  - "two edges of one loop across one face" is flagged by the one-face form, so it errs toward splitting;
  - the one gap I see is a third face that touches the cone at two vertices without sharing an edge. That is exotic, unwitnessed and unsure.
- No face that meshed refuses now:
  - head mesh suite 476/476;
  - every cell of my funnel, wedge and needle grids that meshed on the base meshes on head;
  - the needle h=1e-3 now meshes where the base refused.
- The fence: the cone arm alone is the unit's scope. But the cylinder (12/120) and sphere (1/108) counts sit only in the PR body; the TESSLIE row file is not updated. See NOTE 2.
2 HELD within the face domain; CORRECTED on wording.
- `sin_between`'s two-sum is error-free on 198,301 pairs, checked in exact rational arithmetic (near kπ, huge and tiny magnitudes).
- My 50-digit oracle (chart-blind meridian distance, triangle maximum by grid plus pattern search, plus the axis point solved in mpmath) over 2000 new poses. Families: a pair at one azimuth with the third near an odd multiple of π, the pair at unequal radii, near-meridian, and whole turns, with |u| ≤ 700.
  - 31 poses are under by 8.9–30.6 ε·cone_scale, past the allowance of 8. Every one has corners ≥ 625 rad (about 100 turns) apart in u.
  - The cause is the chord term: `cone_chord_sag` still reads the rounded `uv[j][0]−uv[i][0]`; only the axis term takes the error-free difference.
  - No face hands such corners. Every pose with span ≤ ~2π is within the allowance.
  - So "exact to the rounding of its own value" (cert.rs `cert_cone` § Rounding and `cone_scale`) is true only where Δu's own rounding is small, which the doc does not say for the chord. See Style 1.
- The first sweep re-run on head: certificates moved on 26 of 2400 poses, by ≤ 0.43 ε·scale.
- The PR's oracle `cone_deviation_by_geometry` takes the integer-exact difference and searches each side and the axis. It does not use `cert_cone`'s closed form, so it is independent, though it shares the "max on sides or axis" premise from step 2.
- Cost elsewhere: the `baresin` mutant reds only the axis row; no other row moved.
3 HELD, DEMONSTRATED.
- The `cert_cylinder` and `cert_sphere` code (and `dist_*`) is byte-identical to main; only comments changed.
- The ruled triangle: `dist_line_triangle` is NaN in orders [0,1,2] and [1,0,2] (cert 0), and 0.96593 in the other four (cert 0.034074). So the filed row's claim is true.
- The non-finite row is honest: it says "pinned here as today's answer", and its comment points to the filed row.
- Digest: an env-gated hook in `tessellate_impl` hashed each patch (FNV over the triangles' positions, keyed by test thread, face kind and lane), then I ran the whole `-p mesh` suite on base and head.
  - 161 common tests, 46,754 patches on head.
  - The only patches that differ are 173 `Cone/Trimmed`, base-only and head-only alike. Every planar, cylinder, sphere, torus, spindle, spline and curved-lane cone patch is identical.
  - Not covered: sweep and the tour.
4 HELD, CORRECTED in one detail.
- Head:
  - h=1e-5 above: `FoldedTriangle` at 1e-3, `BoundaryEar` at 2e-4, `FoldedTriangle` at 3e-5;
  - h=1e-3 above at the same three δ: 5612, 28608 and 200164 triangles, all within δ (worst ≤ 0.49δ);
  - the halves below mesh.
- Base:
  - h=1e-5: `FoldedTriangle` at all three δ, so the 2e-4 cell did refuse before;
  - h=1e-3: `FoldedTriangle` at all three δ, including 3e-5. The PR body says main refused at 1e-3 and 2e-4 only.
- The `BoundaryEar` doc now names the cone's shared side, so the text is true of the cell.
5 HELD, DEMONSTRATED.
- `widest = hv/(hu·sin α)` reads the lever off the handed steps.
- The floor uses `cone_chord_sag` (the non-cancelling form).
- The metric gives 1.0 on the development.
- The rows row reads chords by a ternary search of f in 3-D. That is a different formula from the rule's, so it can go red: uniform-in-v reds it (and the funnel and needle rows), and half rows reds it.
- The doc splits "proved" from "the rule's choice".
6 HELD, mostly. The PR body dates the 432-run sweep and the shared-side table, and calls them measurements, not rows. Refinement now reads "stopped at the round budget". One overreach remains: the cylinder/sphere counts "for TESSLIE's row" are unfiled (NOTE 2).
7 Status of my first review's findings:
- MAJOR (T1 slivers): closed on the cone arm (point 1).
- MINOR (non-finite): closed on the cone (`worse` NaN-keeping, the `!is_finite` guard; my probe now gives NaN at every input). The cylinder and sphere NaN clamp is open by design, pinned and filed; the reason (poisoning it moves a rectangle-lane face) is sound.
- The filed row `a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md`: still `status: parked`. Open (NOTE 1).
- Style 1 (max folds): closed. Style 2 (cancelling form): closed. Style 3 (rows row restates the code): closed, now geometric. Style 4 (doc reads like a bound): closed. Style 5 (dead metric): closed. Style 6 (two homes): closed. Style 7 (stale line ref): closed.
- Style 8 (unguarded measurements): open by design, now dated.
- Style 9 (|u| domain): addressed in `cone_scale`'s doc, but see Style 1 below.
- Styles 10 and 11 (`geometry`'s 6-grid; `MIN_SHAPE` margin): not addressed.
- The two answered-with-a-reason findings: "root signed" is sound (a window lies on one nappe and may end at 0; the signed root is monotone). "Axis block gated on no apex corner" is sound: with an apex corner the plane meets the axis at the apex, where f = 0; and if the plane holds the axis, f on the axis segment is affine, so its maximum is on a side.
8 See the verdict line. No red; no complete green on 26ce505.

## New findings
- NOTE 1 (BY-INSPECTION): `work/tesslane/a-cone-face-ending-near-its-apex-refuses-at-fine-delta.md` is still `status: parked`, though the PR "Closes" it. Unsure whether the close happens at merge.
- NOTE 2 (BY-INSPECTION): the other arms' four-use counts are in the PR body only (cylinder 12/120 non-manifold, 14/180 faces; sphere 1/108). `work/tesslie/two-faces-cutting-one-ear-…` is unchanged by the unit diff and still says its cure covers ears, so the PR's measurement of the class beyond ears has no home in the tracker.
- NOTE 3 (DEMONSTRATED, out of reach): `cert_cone`'s chord term under-reports by up to 30 ε·cone_scale when corners are more than 100 turns apart in u.

## Style (the pass only)
1. cert.rs `cert_cone` / `cone_chord_sag`: the axis term reads Δu error-free (`sin_between`) but the chord term reads the rounded difference. That is two treatments of one input. The § Rounding claim "every term … as exact as its inputs" holds for the chord only while Δu's rounding is small (demonstrated at ≥ 100 turns). **likely**
2. trimmed.rs `faces_across` leaves out the seam on an argument, and the `seamin` mutant reds nothing: the choice has no row. **sure**
3. trimmed.rs `attempt`: the shared-side rule is "every analytic arm" in its comment block but runs on the cone alone; the other arms read "its nearest case". That is two spellings of one rule, kept apart by a comment (Q2). **likely**
4. trimmed.rs `tessellate_trimmed`: `match a.development() { Some(_) => 1.0, … }` passes a placeholder metric. The value is unused on the cone, but a magic 1.0 stands in for "none". **unsure**
5. The needle pin `a_needle_funnel_cut_at_its_apex_refuses_typed` hard-codes the δ→refusal-kind table (`delta == 2e-4` → `BoundaryEar`). It pins today's refusal kinds, which can flip without meaning anything; the PR calls it a gap pin. **unsure**
6. cert.rs `exact_difference` (the test oracle) asserts the exponents are within 70 of each other; if a draw ever failed that, the row would panic rather than skip. Fine for the drawn ranges. **unsure**
Questions exercised: Q1 (two treatments of Δu; two rule spellings), Q2, Q3 (seam mutant; rows row now red-able), Q4 (the BoundaryEar doc and the module doc re-read for the changed premise), Q5, Q6 (unfiled counts), Q8 (cert.rs cone section and the trimmed.rs `emit`/`attempt` shared-side code read whole).

## Executed / read
- Executed:
  - the head and merge-base mesh suites with the per-patch digest hook. Head: 476/476 plus my own include's aggregation failure, which is mine, not the unit's.
  - the wedge grid on both trees (624 runs);
  - the needle cells on both trees;
  - 2000 new oracle poses, plus my first 2400 re-dumped;
  - the two-sum exactness check;
  - the ruled-cylinder orders;
  - mutants: earonly, nbflag, seamin, uniformv, halfrows, baresin, maxfold.
  All local edits are reverted, and the base worktree and target dirs are removed.
- Read only: the clamp versus main diff, `faces_across` against the three-faces case, the tracker rows, and the PR description and check runs.
- My first review's probes survived in my container and were reused.
- I saw no other review's material. The PR body mentions another reviewer's pose (α 2.41e-4); I read only the description.
