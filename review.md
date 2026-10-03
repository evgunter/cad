# Review of PR #3978, frozen head 65e53562cf

Lane `reach-dual3978-r1`. Wall clock 17:10–18:10 UTC, 2026-10-03.
**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 2. No wrong body and no untyped refusal turned up anywhere.
Glimpses: none. I read the PR body (`get`) and the head's check runs, and no comments, reviews or other `analysis/reach-dual/*` branch.
Gate: every check on `65e53562` is green (run 37136931115: test, lint, corrupt input, mesh meter, gate ok).

## Claims
1. **Holds, by execution.** `probes/review_dual3978_r1_probes.rs` sweeps each touch across its face edge. Poses: snowman ⊃ ball(0.4) at
   `u_y = NECK + {.09, 1e-2, 1e-4, 1e-6, 0, −1e-4, −.1}`; lens + ball(0.3) mirrored; ball + brick with `x0 ∈ {.5 … −.5}`;
   ball + rod; skew rods; a holed plate. Each pose runs at scales 1e-3, 1 and 1e3, rotated, and on reused results, at ε 1e-9, 1e-6 and 1e-12.
   I also probed in-band crossings decided `Zero` but truly cutting (plane depth 0.5ε and 0.95ε; ball grown or pushed by ±0.5ε and ±0.9ε).
   - Every touch on a face, on its edge, or within ~1e-4 of the edge refuses.
   - Every built body (≈500 op×order results) passes validate, validate_closed and validate_geometric. Each also passes
     3000-point Monte-Carlo `point_in_solid` against my own analytic CSG oracle, with near-boundary samples skipped.
   - On main's kernel (worktree, the 5 kernel files reverted) almost none of these build, so the PR's path is what runs them.
2. **Holds, by execution**, over sphere×sphere (in and out), sphere×plane, sphere×cylinder and cyl×cyl, at three scales and three ε.
   The fixture refusals are not this PR's:
   - snowman/lens at s=1e3, ε 1e-12: `CurvedPierceUnsupported`;
   - holes of 1e-5 at ε 1e-6: `arc_diameter_clearance`.
3. **Holds, by execution.** Every on-face pose refuses typed: `SpheresMeet`, `Escalated{Apart|AgainstPlane|Nested}`,
   `FallbackExtentUnsupported`, or a crossing-layer refusal (`CurvedPierce`, `Coincidence`, `ArcCylinderRoots`).
4. **Partly holds, by execution.** The PR's 5 building rows are red on main and the on-face row is green there.
   Mutants (`probes/review_dual3978_r1_mutants.py`, each run against the PR's rows, `section_cert_rows`, `x4` and my probes):
   - Red: M1 `Out` → any placement; M6 plane arm drops the refusal; M7 nested-zero arm drops `SpheresMeet`;
     M8 sphere×plane spread = μ; M9 sphere×sphere spread ×0.1.
   - Survive: M3 and M4 (MINOR 1 and 3).

## Findings
- **MINOR 1, by execution (M4).** `crates/topo/src/boolean/ops.rs:1220`: `boundary_clear_of` forced to `Ok(finite)` (the integrated
  boundary test removed) leaves every row and all my probes green. The only pin is `a_touch_clears_only…`, which hands `certify` a
  `clear` closure, so the guard the item calls essential ("one point would not do") is unpinned where it ships.
  I could not build a wrong body under M4. Inside a decided-`Zero` loop every edge lies within `zero` of the other carrier, so the
  crossing layer refuses first (likely). That makes the guard either load-bearing and untested, or redundant. Either way, say which.
- **MINOR 2, by execution.** The new docs at `refusal_routes.rs:859` and `:865` say an **in-band** margin "passes where the section
  certificate certifies the faces apart"; work item line 91 and the PR body say the same.
  - An in-band (undecided) margin is undecided in the certificate too, so `Pinch{zero:false}` gives R-tan: that branch can never pass.
  - Executed: h = 3ε (in band) refuses `Escalated{Apart}` and `Escalated{Nested}` at all three ε, at 0.09 rad off the face.
  - The promise is false as written; the code is conservative.
- **MINOR 3, by execution (M3).** `section_cert.rs:396–400` together with `section_cert_rows.rs:634`. With `zero_bound` = `band.zero()`
  (a tenth of the margin) every row stays green, because `a_touch_holds…` pushes only 0.9·zero and so passes with any bound ≥ zero.
  Nothing pins the "margin's own rounding" half of the bound, and the row is monotone in the forgiving direction (style Q3).
- **NOTE 1, by execution.** A hole around the touch never clears, however large. Ball + plate with a y-hole of radius 0.5 about the
  touch refuses `Escalated{AgainstPlane}` at every scale and ε: the hole's circle-edge box contains the touch ball (`ops.rs:1220`).
  That is sound, but the item's motivating "holed within the ball" case clears nothing, and this liveness gap is not on the residue list.
- **NOTE 2, by inspection (unsure).** `section_cert.rs:396`: "the margin's own rounding, which the escalation threshold bounds" is
  unenforced. At large coordinates and a small ε (|c| ≈ 1e5, ε 1e-12) the f64 rounding of `r1 + r2 − ‖c₂ − c₁‖` exceeds 9·zero.
  I did not execute it: fixtures fail first at 1e3/1e-12.

## Style (Q1, Q2, Q3, Q4, Q5, Q7 exercised; Q8 partial)
- Q1 (sure): `ops.rs:1203–1211` is a third hand-built "Aabb of centre ± radius". The others are `ball_box` at `:2898` and
  `circle_box` at `:2970`. Look for more in `boxes.rs`.
- Q1 (likely): the same 5-argument `sphere_faces_apart((x_is, x, x_rows), fd.surface, (y, y_row), band, &mut section_charts)?` is
  written out 5 times (`ops.rs:2929, 2952, 3073, 3123, 3140`). Three of those are new, and the new ones have two polarities:
  `is_none() → continue` and `is_some() → return`.
- Q7 (likely): the plane arm `continue`s mid-match out of the y-face loop, and the nested arm falls through to the code after it.
  The three tangency arms do one thing with three control flows.
- Q2/Q5 (sure): MINOR 2. Also `ops.rs:1197` has a stray `'` ("[`face_boundary_meets`]'.").
- Q3 (sure): MINOR 1 and MINOR 3. Both "the guarantee degrades" mutants survive.
- Q4 (likely clean): I grepped `topo/src` and the README for prose citing "a decided zero refuses". The PR updated
  `mod.rs:2094`, `SphereQuestion` and the scan header, and I found no stale copy.
- Q8: I read `section_cert.rs`'s module header and the touch arms whole. I did not read `ops.rs` (4.9k lines) end to end.

## Probes and runs
- Suites, `-p topo` plus the touched sweep files: 1e-9 2188/2188; 1e-6 2188/2188; 1e-12 2187/2188, the one red being the known
  `rigid_map_near_eps_plane_nurbs`.
- `probes/` holds the probe module (it registers in `crates/sweep/tests/all.rs`) and the mutant script.
