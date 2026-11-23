# Review of PR 7043 at frozen head `b6993486b8` (reviewer lane, full review)

## Verdict: APPROVE-WITH-FIXES

No MAJOR. The two rules are built as the page states them, and my base/head differential found no trimmed face that meshed on main and refuses in the lane. Four MINORs: one unreported behaviour change (now refused, rightly), two unpinned choices shown by surviving mutants, and no completed hosted CI on this head.

## Claims

1. **Clearance drops exactly the page's nodes: held.** Unit probes inside `trimmed::tests`: 12 edge cases at ±1e-9 of the clearance (outer side with no polygon point in the box, ring side from both sides, ring corner per axis, on a side, on a vertex, a whole step away) all answer as the page says. `Side::meets` (chart) against an independent segment/rectangle test: 400,000 random cases, 0 disagreements. `lay`'s windowed search against brute force: 4,955 random polygons with mixed-step nodes, chart and development, 0 disagreements without apex points. One exception at NOTE-1.
2. **"Own step" and the plane: held, with two unpinned choices (MINOR-3).** Sector test against 4,000-point sampling of the developed segment: 3,000 sides × 40 nodes (near the apex, apex ends, either nappe, sin α 0.03–0.999, spans under a half turn), 0 false negatives, 0 false positives. A sector across the cut cannot be met wrongly (two half-planes, wedge under a half turn). The side's u-window assumes each side spans under a half turn of the development; chords do.
3. **No face that meshes on main refuses; only trimmed faces with a node inside the clearance move: corrected.** Same instrumented `tessellate_trimmed` on both trees.
   - Mesh suite (435 of the rows; 642 distinct analytic trimmed faces, 1.49 M nodes): 2,139 nodes not laid on 388 faces, 377 patches moved, 268 triangle-count changes, 0 moved without losing a node, worst certificate 1.52 → 1.63 cell bounds, largest single rise 0.93 → 1.45. 0 new lane refusals, 0 changed refusals. 328 spline faces: 0 moved.
   - My census (1,349 body×δ cells; 1,789 analytic trimmed faces, 7.37 M nodes): 15,488 not laid on 1,239 faces, 1,212 moved, worst 1.49 → 1.68, 0 lane refusals. Answers differ in 21 cells: 12 `NoArea` → `Meshed`, and **9 `Ok` → `UnresolvedWinding`** (MINOR-1).
   - D9: clearance 0 and 3/8 both red the two digest rows, so they do read the clearance. I did not re-derive `tilted_above/below` themselves; my tilted-rod halves (same class, 4 tilts, 4 δ) all pass `reach_probes::geometry` on head.
   - Spline: no spline patch moves at 3/8 over the suite. The PR states the measurement and no reason. The reason I measure: 962 of 964 spline face×δ records have iso-line sides only, so their lattice sits whole steps from the boundary. The rule is barely exercised on a spline chart. I could not build an oblique-sided spline face (splitting a loft refuses `CurvedBooleanUnsupported`).
4. **A face may lay no node: held.** Bands on rod, frustum, funnel, cone, ball and donut from 0.2 down to 5·10⁻⁵ wide, rings 10⁻¹…10⁻⁴ from a cut on a holed ball, seam slivers at 10⁻¹…10⁻⁴ rad, each at 3–5 δ: every cell that meshes on base meshes on head; no `CertificateExceeded`, `BoundaryEar`, `Triangulation` or hang. NOTE-3 on the row.
5. **What retired: held.** 0 `Triangulation` on a curved face over suite and both censuses, including needle funnels and frusta (r 0.002–0.2) at δ 10⁻⁴ and 10⁻⁶. By inspection an ear or refinement point is the centroid of a triangle of the previous CDT, so it cannot equal a vertex short of rounding. The duplicate refusal is pinned (mutant "dropped silently" reds `the_development_keeps_apex_copies…`). The side and crease-link refusals have no row; keeping them as refusals is right (the alternative is a silent T-junction). Text true of each raiser.
6. **One definition, three readers: held for the definition, reader 2 unpinned (MINOR-2).** No second spelling of allowance, scale or comparison in `crates/mesh/src`. The STL writer's exact-zero test is a different predicate with a different job, disclosed. Mutants: join skipped on a memo hit → the whole-frustum row; skipped on planar → same row; skipped on curved+trimmed → the zero-clearance unit row; emit on bare `> 0.0` → **nothing**.
7. **The scale: held, with NOTE-2.** Exact-arithmetic perturbation, 5 scales × 3 bands × ~400 triangles × 60 trials: triangles called resolved at 1–4 allowances never flip under ±1, ±2, ±4 ulps of each coordinate nor ±2ε·scale; 55% of the 1–1.5 band flip at ±4ε·scale and all at ±8ε·scale (the allowance itself). Triangles at 0.05–0.6 allowances: 10% flip at 1 ulp, 30% at 2, 70% at 4. Position dependence executed: the needle frustum (rim 2·10⁻⁸) at δ 10⁻⁵ meshes at the origin and refuses `UnresolvedWinding` 400 m out. No over-refusal: 167 of 168 cells mesh, least 3.7 allowances.
8. **The refusal: held.** Text true. Reached on the planar lane (pinned) and the trimmed lane (MINOR-1, unpinned here). I built nothing that reaches it on the curved lane.
9. **The item's bodies: held.** `MIN_SHAPE`'s removal weakened nothing I could find: 0 triangles under the old 10⁻¹² floor in 167 meshed cells; least `2·area/L²` 6.8·10⁻¹⁰.
10. **The on-axis cone: established.** On main the nine meshes hold 36 triangles on the apex and two lattice nodes of one column: height 8·10⁻¹⁸ to 2·10⁻¹⁷ over sides 0.2–0.5, 0.002–0.005 of an allowance. The probe's "wound against" was the sign of a noise normal, not a normal read at the axis. On head all nine refuse `UnresolvedWinding` on the cone face.
11. **Mutants** (whole mesh suite each; clean-head baseline 489/489):
    - clearance 0 → 7 red, the PR's five plus both D9 rows;
    - side by endpoints only → `a_node_is_laid_exactly…`, `in_the_development…`, D9 ×2, two `reach_probes` ring rows;
    - outer loop only → `a_node_is_laid_exactly…` and the two ring rows;
    - cone box read in the chart → only `in_the_development…`;
    - cone step = larger gap → **survives**;
    - no crease clearance → the one-ulp crease row;
    - allowance zero → 5 rows;
    - emit bare sign → **survives**.
    Not run: clearance on refinement points, scale ×2 and ÷2.
12. **Page vs code:** one sentence strained (NOTE-4). `FacePatch`'s doc and the STL writer's are true.
13. **CI: corrected** (MINOR-4). Sweep and deviations read only.

## Findings

- **MINOR-1 · DEMONSTRATED · `tessellate.rs` join; `work/tesslane/a-cone-face-met-on-its-axis-…md`.** Nine cells that return `Ok` on main refuse on head (T1, xf = 0; s = 0.6 at δ 5e-3 and 1e-3, s = 0.3 at 1e-3; ops A∪B, B∪A, A∖B). The refusal is right, but at this head the PR body does not report it, no row pins it, and the open item still says "meshes" and "not measured".
- **MINOR-2 · DEMONSTRATED · `trimmed.rs` `TrimmedFace::emit`.** Putting the fold decision back on a bare `normal·outward > 0.0` passes all 489 rows. Nothing tells "unresolved is no fold" from main's behaviour.
- **MINOR-3 · DEMONSTRATED · `trimmed.rs` `lattice_candidates`, `Side::of`.** The cone's "smaller of the two gaps" (flagged as the implementer's reading) can become the larger with no row or digest moving. Reading the cone's box in the chart moves no mesh or digest either; only the unit row on `Side` itself goes red.
- **MINOR-4 · DEMONSTRATED · hosted CI.** Run 43983643010 on `b6993486b8` is **cancelled**. The green run 43984526769 is on `ba68f8ed0a`, a later commit. This head has no completed hosted run. Local at this head: mesh 489/489, stl 18/18.
- **NOTE-1 · DEMONSTRATED · `Side::window`.** For a side from the apex the window is the other end's ruling, but `meets` is true for any node whose sector holds the apex. On a hand-built loop with one apex copy, `lay` lays 10 nodes that `meets` says meet a side. Harmless where the loop has a singular-row side (two copies), whose window is unbounded; I know no body with a lone copy. The doc sentence "no node whose box lies wholly outside it meets the side" is false there.
- **NOTE-2 · BY-INSPECTION · `cert::coordinate_scale`.** It drops the anchor that `position_scale` carries. A point evaluated as centre + offset near the origin of a far-centred surface rounds at the centre's magnitude, not its own, so "evaluation rounding" is not bounded by the corner norm there. The code matches the page's definition; the page's second sentence is what is not covered.
- **NOTE-3 · BY-INSPECTION · `winding_rows::a_band_narrower_than_its_clearance…`.** Asserts `Meshed` only. "None is laid" and "0.67" are in the doc and nothing reads them; clearance 0 does not red it.
- **NOTE-4 · BY-INSPECTION · README clearance bullet vs `Creases::cut_candidates`.** "Laid only where its box meets … no crease line" is untrue of the crease's own nodes, which are `Node`s laid on the line and exempt.
- **NOTE-5 · BY-INSPECTION · crease rows.** The crease clearance is pinned one ulp off the line only; no row has a node just inside or outside `c·h`.

## Spec deviations

Reported by the PR: constant 1/16; pre-6965 table not re-taken; cone-step reading; scale as corner norm; refusals kept unreached. SILENT at this head: MINOR-1.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (`trimmed.rs` non-test half and `validate.rs` read whole).

- `crates/stl/tests/common/mod.rs:44-51` still says coarse cone fans emit zero-area triangles the writer refuses, and cites `degenerate_apex_fan_is_refused_typed`, which does not exist. The PR rewrote the writer's doc for this premise and left the sibling. Half-fix; I would also look in `crates/stl/tests/review_m2_pr7.rs`. `sure`
- `validate.rs:347`, `types.rs:971`, `validate.rs:411`: "eight epsilons" restates `cert::ROUNDING_ULPS` in the public doc, the refusal text and a test literal. The test says so on purpose; the two prose sites would go stale silently. `likely`
- `tessellate.rs:578`: the join finds the triangle and throws it away; `UnresolvedWinding { face }` carries no triangle, height or allowance, while `check_winding`'s error does. A refusal whose payload cannot name its cause. `likely`
- `validate.rs` `Winding` / `winding()`: three states, one production caller, which tests one of them. `likely`
- `trimmed.rs` `Creases::cut_candidates`: a crease node takes "the step of that line's first node", possibly another band's. "Its own step" has no meaning for it, and the clearance against the polygon is measured in it. `unsure`
- `trimmed.rs` `lay`: a location laid by two bands is kept if either band's box clears, so the larger-step copy can be inside its own clearance. Not how I would read "its own step". `unsure`
- `trimmed.rs`: one thing under five names (`Node`, candidate, grid point, lattice node, `Slot::Grid`), and `candidates` now also holds ear and refinement points. `MAX_GRID_RETRIES`, `grid_ids`, `Next::Rebuild`'s "a grid point dropped" are of the retired mechanism's vocabulary. `sure`
- `trimmed.rs` `Side::Developed` stores `turn`, `nappe` and `window` per side though the first two are the plane's. Not how I would have shaped it. `unsure`
- `trimmed.rs` `boundary_clearance()`: a `cfg(test)` thread-local read inside the production path for one row; it follows `LAST_ATTEMPT`. `unsure`
- The trimmed lane knows a triangle is unresolved in `emit` and still runs its retry budget before the join refuses. `unsure`
- `in_the_development_a_nodes_box_is_its_sector` holds `meets` to sampling within a factor 1.001 one way; it is the only guard on the development choice (MINOR-3). `likely`
- PR "Sweep": says what its pattern could not match and that PR 6965's code was not re-swept. My sign-comparison grep over `crates/mesh/src` and `crates/stl/src` found nothing it missed. `sure`

## Rubric

- **Idiom/structure: 4.** `lay`, `Side`, `clip`, `radii_sq` are small and testable; the vocabulary and the window/meets pair are the rough spots.
- **Test quality: 3.** The clearance and the join are well pinned (9 of 11 mutants die), but two stated choices and one of three readers are held by nothing.
- **Doc/comment honesty: 4.** Module docs match the code sentence for sentence bar `Side::window` and the stale STL test comment; the PR body is candid about what was not re-taken.

## Executed, read, seen

- **Executed:** instrumented mesh suite on base and head; two censuses on both trees; nine-cell probe on base; four unit probes; 11 mutants plus baseline; suite at clearance 3/8 (6 real reds, matching the PR's account); `-p stl`; the perturbation script.
- **Cut short:** a container restart (out of memory under three parallel chains) cost the 1/2 run and part of the second census. Needle cones at δ 10⁻⁷ ran on base only.
- **Read only:** `pncad-py` diffs, the PR's ε rows, tour/sweep/budget/tools claims. Not run: sweep, tour, Python, any ε row but the default.
- **Seen:** no other reviewer's material. Two glimpses: the check-run lookup showed the later commit `ba68f8ed0a` and its message, which says the nine on-axis cells are pinned (my census had found them before; I did not fetch it); and a grep surfaced one row of `docs/DUAL-REVIEW-LOG.md` (DR-450, PR 6497, unrelated).

Uncommitted probe files are in my scratchpad and the three worktrees; nothing was committed, pushed or posted.
