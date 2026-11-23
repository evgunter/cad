REVIEW: PR 7526, frozen head 553d4d6310. Base = merge base 31a697d324 (merged main, without this unit).

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The fixes are doc and comment honesty (below). No hosted test job has run on this head (see claim 11).

## Claims
1. **Held.** On the `mesh` suite, `sweep`'s meshing modules and my own probes, no face that meshed with rows refuses or folds without them.
   - Own probe: 352 cells on the head, all mesh, pass `check_mesh`, and pass `reach_probes::geometry` (winding, sampled deviation within δ, volume). The bodies: walls of height/radius 4, 40 and 200; a hole down to 0.4% of the height, far from both rims; near-tangent flats at x ≥ 0.995r and 0.9999r (a long ruling run, a short arc); tilts of 0.6, 1.2, 1.45 and 1.55 rad; a six-notch comb on the top rim against a plain bottom rim. Scales 1, 1e-3 and 1e3; δ 0.5 to 1e-5; ε 1e-9, 1e-6 and 1e-12.
   - The same 120 cells on the base all mesh too. Rows had cost 76 to 564 times the head's triangles; the worst is the combed wall at δ 1e-3, 122,956 → 218.
   - Crossing rods (r2 ∈ {0.999, 0.9, 0.6}, θ ∈ {π/2, 1.0, 0.6}, ∖ and ∩, δ 0.5 to 5e-3): one cell refuses `Triangulation` on face 1v1 at every δ (r2 0.999, θ 0.6, A∖B). It refuses identically on the base, so it is main's, not the unit's (NOTE below).
2. **Held, against merged main.** My own per-patch digest hook in `tessellate`/`tessellate_with`, keyed by test, δ, a digest of the boundary chords' positions and the patch index.
   - `mesh` at ε 1e-9: 58,365 patches paired. Only Cylinder moves: 390 faces, 1,013,168 → 31,686 triangles. Plane, cone, sphere, torus, spindle and NURBS are bit-identical, so no rim neighbour moves.
   - `mesh` at ε 1e-6: the same picture, 390 faces. I stopped the base's 1e-12 run for time, so there is no base differential at 1e-12.
   - `sweep`'s 63 modules that call `mesh::` (filter built from `grep -l mesh::`): 31,819 paired; 1,361 cylinder faces move, 2,189,525 → 172,795. These are the PR's numbers exactly.
   - The 416 unpaired records come from two tests: `an_annular_tube_through_a_plate::the_annulus_builds_at_random_azimuth_pairs` (336) and `review_chamfer_r1_probes::a_skewed_wedge_chamfers_with_every_face_outward` (80). Their bodies differ run to run (random poses), so the PR's "body digests differ run to run" is randomness, not one body meshing two ways. Both pass on both trees; nothing hides there beyond "not compared".
   - Walked faces: `walk_face` returns before the lattice code (`trimmed.rs` ~615). BY-INSPECTION.
3. **Mechanism held; the count is corrected.** The crossing-rods row `crossing_rods_keep_no_side_a_neighbour_can_make` gets denser exactly as stated: 82→182 (2.22×), 92→188, 230→408, 50→88 and so on, plus the two rod-sliver records 256→288. That makes 14 in `mesh`.
   - **But `sweep` adds 6 more denser records** (`bisector_nodes` rod and bore rows, 112→114). The body's "the unit makes 14 faces denser" omits them; the `sweep` max ratio of 1.018 is disclosed.
   - Retry budget: an env-gated attempt log on my 56 crossing-rod cells shows at most 3 cure rounds against `MAX_GRID_RETRIES` = 6. No cell exhausts it and refuses on the head where it meshed on the base.
   - Is a pinned cost the right answer? I think yes: the cost is the shared-side rule's, and it has its own file. But 3.25n is a fit with about 3% headroom over the measured 3.14n, not an argument (Style).
4. **Held.** `a_shared_side_the_budget_leaves_refuses_typed` is green on the head. Mutant M1 (the last attempt rebuilds instead of refusing): red. It goes red as an `unreachable!` panic at `trimmed.rs:830`, not as the row's own assertion, but it is red. The new δ 5e-2 cell reaches the typed refusal.
5. **Partly falsified as relations.** Mutant M2 (rows laid again on the cylinder): rows 1, 2 and 4 red, row 3 green, as stated. `d9_mesh_goldens` and `route_rows` also go red; the 3.25n cap stays green.
   - Row 1's ≤1.1 is a fit to five notch bodies, not a property of notched walls. My six-notch comb (r 1, h 4) holds 83 triangles at δ 1e-2 against a plain wall I put at 64 (closed form, 2·⌈2π/step⌉ at δ/2), which is 1.30×. At δ 0.1 it is 42 against 20, 2.1×. Each notch adds boundary points; the row's argument ("strips between two rows of boundary points") does not bound that.
   - Row 1's 1.4 for a hole: a margin over 1.24–1.375 on one body (a bar through both sides).
   - Row 2's 1/cos φ: by the doc's own argument (a rim chorded along its length, one strip) the wall would read at most (1 + L_ellipse/L_circle)/2, which is 1.11 at φ 0.8. A 3-D sagitta count predicts about 1.04. Measured: 1.375. So the argument does not explain the measurement, and the bound is a fit with about 4% headroom at φ 0.8. It held on my tilts (φ 0.6: 1.16 against 1.21).
   - Row 3 earns a place as the only pin on split points on a rowless wall. But its bound counts every line edge of the body, the bar's edges on planes included, not the split points on the wall, so it is loose.
6. **Held, reproduced on both trees.**
   - `review_pick_r2`: base (451,512, 147,850, 16,596, 8,142), red against its 147,856 pin, which is main's red. Head (424,728, 138,718, 11,004, 5,130), green.
   - `display_budget`: the base is green at the old pins (81,208), the head green at 2,912.
   - Why the ray count moves: rays are aimed at every mesh vertex (subsampled), so fewer vertices means fewer rays. The re-pin doc's "fewer rays land" is the wrong mechanism ("fewer are aimed"). The tally moving with the mesh is by the module's own design, so it pins the refusal class only per corpus. Acceptable, noted.
7. **Held, with one stale comment.** `tessellate.rs`, `memo.rs`, `chords.rs` and `walk.rs` are byte-identical to the merge base and to origin/main. The whole-tree diff is the 12 listed files; no `IsoBounded`, `runs_on_iso_lines`, env read or switch is left.
   - The deleted #678 `debug_assert`: no node means no single interior column. The conclusion-level census, the `debug_assert` on `overused_identified_edge_in` after emit, still runs on every arm, so the removal loses no observation.
   - **The emit-pass comment `trimmed.rs:1531–1538` still says the ingredient "is checked at the cylinder arm's `nu`"**, the check this PR deletes. MINOR, sure.
8. **Held.** Neither `lattice_rows` nor `lattice_steps` nor `metric` reads ε. Across ε on the same body, only `patch_memo::the_trimmed_lane_misses_when_a_pcurve_changes…` differs, on both trees; its nudge is `1e-2·band.zero()`, so its body is ε-scaled. Row 4 green at all three ε.
9. **Held, BY-INSPECTION.** The split is on `Analytic` kind, and a cylinder is always `Analytic::Cylinder`, so no chart row reaches a cylinder as a cone. A cone with a tiny half-angle keeps its `ρ_max·hu` rows, so it keeps the 4–220× (more on tall walls) density. That is unmeasured here and unscheduled (Style). The spline-stored circle wall is rightly not this unit's.
10. **Mostly true.** Corrections: "14 faces denser" omits `sweep`'s 6 records (claim 3). "Only cylinder faces move: 391": I get 390 paired at both ε; the difference is plausibly the shared-side row's δ change. The changed CDT metric for the cylinder (now column step over window height) is not listed under "What changed" as a behaviour change. It is in the docs only, and worded inverted (Style).
11. **No hosted test job has run.**
    - Run 45120873699: change filter success, prime skipped, interval backend success, interval oracle skipped. `test (kernel)`, `test (app)`, mesh budget, demos, step import, python, tools, corrupt input and viewer app are all queued.
    - **`lint (fmt, clippy, gates)` FAILED** at `scripts/payload-rung-sweep.py --check`: "NEW payload rung … NodeErrorClass at crates/editor-core/src/eval/class.rs:41". I ran it locally: it is red identically on origin/main da61db71 and on the frozen head. So it is **main's red, not on the known list**, and not the unit's.
    - Locally in its place: `mesh` whole, slow set included, at ε 1e-9: 551/551. At 1e-6 and 1e-12: 500/500 plus the 51 `review_*` rows run separately (my first filter dropped them), all green. `sweep`'s meshing modules at 1e-9: 595/596 on both trees (the known `bisector_nodes` red). Viewer's two slow rows: 13/13 on the head.
12. **What was built is the right unit.** The final section of the spec says exactly this ("no rows on a cylinder, in the widest form that loses no face"), and it is reported.
    - Reported deviations: the class cure and the δ target are dropped (ruled in the file); the tour census and the 1e-12 `sweep` run are not done.
    - SILENT: the cylinder's CDT metric changed (aspect hu/H, where it was 1/r). Every rowless face is triangulated in a newly stretched plane, unmeasured against alternatives.
    - SILENT: the spec's first "Done when" (that `[mesh.sizing]`'s wording names which faces the headroom covers) is superseded, not answered. Acceptable given the re-scope, but nothing says so.

## Findings
- MINOR, `trimmed.rs:1531–1538` (emit pass): stale. It cites the deleted cylinder `nu` check. BY-INSPECTION.
- MINOR, `rowless_cylinders.rs` `a_notched_or_holed_wall_takes_its_strips_triangles`: the headline ("a notched wall takes the plain wall's triangles") is false in general; a six-notch comb reads 1.30× at δ 1e-2 and 2.1× at 0.1. DEMONSTRATED for the head's counts, but the reference is my closed form, not measured.
- MINOR, same file, `a_wall_cut_on_a_tilt_…`: the doc's argument predicts ≤ about 1.11 at φ 0.8; the measurement is 1.375. The bound is a fit. DEMONSTRATED (arithmetic against the row's own measured numbers).
- MINOR, PR body: "14 faces denser" omits `sweep`'s 6 records. DEMONSTRATED (differential).
- NOTE, `viewer/tests/review_pick_r2.rs` re-pin doc: "fewer rays land" should read "fewer are aimed" (aim per vertex). BY-INSPECTION.
- NOTE, hosted lint red is main's (`payload-rung-sweep --check`, NodeErrorClass); someone should file it. DEMONSTRATED.
- NOTE, main: two rods (r 1 and 0.999, θ 0.6) A∖B refuse `Triangulation` at every δ on base and head; worth filing. DEMONSTRATED.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 partly, trimmed.rs module header plus the touched regions, chart_table.rs's lattice section whole)
- `chart_table.rs` `metric` doc (~476): "a cylinder's … is its window's height over its column step" is inverted. The code computes column step over height (`(u1-u0)/nu / (v1-v0)`), which is the cone sentence's convention. — sure
- `chart_table.rs:metric` / `trimmed.rs` ~664: the cylinder's CDT plane is now set by passing `nv = rows.len()+1 = 1`, an artefact of the old row count, not a chosen metric for a node-free CDT. Nothing argues the stretch is right, and on tall walls it is extreme. — likely
- `rowless_cylinders.rs` `TARGET_SHARE = 0.5`: a hand-kept copy of `trimmed::UNPROVED_HEADROOM`. "Where it moves, this share moves with it" is enforced by nothing (Q1/Q2). — sure
- `reach_probes.rs` → `crate::rowless_cylinders::boundary_points`, and `rowless_cylinders` → `reach_probes::{geometry, op_of, …}`: a circular helper dependency between test modules; a generic helper is hosted in the unit-named file. — likely
- `reach_probes.rs` 3.25n cap: a fit (measured 3.03–3.14n) defended in prose as "two triangles a point and a second rounds a few more". Nothing derives 3.25. It pins the implementation's cost, not a contract. — likely
- `rowless_cylinders.rs` row 3: the bound `lines.len()` counts every line edge of the body, including those off the wall; monotone in the permissive direction (Q3). — likely
- `rowless_cylinders.rs` row 4: four pinned hex digests. It is a golden, not a relation; "the same at every ε" holds only because CI runs it at three ε. — likely
- `chart_table.rs` `lattice_rows` doc: "a notched or holed pipe wall takes 1 to 1.5 times the rectangle lane's triangles … where the rows made 4 to 220 times" is a corpus measurement in a source doc, with no corpus named at the site and no guard or register (Q6). — sure
- The cone keeps its rows, on "a probe saw cones refuse" with no row or filed issue. A near-cylinder cone (small half-angle) keeps the full density blow-up, and nothing schedules that (Q6, class-not-instance: the spline-stored cylinder is the same class). — likely
- `trimmed.rs` `UNPROVED_HEADROOM` doc: now says the triangle "certifies at the sagitta of its span round the axis, which can be more than one column — nothing yet bounds by how much". On a rowless face the span is bounded only by the boundary points, so the debt has grown, and the doc reads as if unchanged. — unsure
- `trimmed.rs` module "The retry loop" (283–286): the fold cure "drops its lattice corners"; on a rowless cylinder there are none to drop, so a fold refuses typed. Not reached in any run, and not said. — unsure
- Q7: I would have keyed "no rows" to the ruled-chart property (straight along v) in one place, covering the cone's straight direction too, rather than by surface kind with the cone's refusal unexplained. — unsure

## Rubric
- Idiom/structure: 4. A small, surgical diff, and the route is main's again; the metric artefact and the test-module cycle are the blemishes.
- Test quality: 3. The mutants redden rows 1, 2 and 4, but rows 1 and 2 are fits written as relations, and row 3 is loose.
- Doc/comment honesty: 3. A stale emit comment, an inverted metric sentence, corpus numbers in a source doc, and an argument that does not match its measured numbers.

## Executed / read only
- Executed: my digest hook on both trees; `mesh` whole (head at 3 ε; base at 1e-9 and 1e-6); `sweep` meshing modules on both at 1e-9; the viewer's `display_budget` and `review_pick_r2` on both; 352 probe cells on the head and 120 plus 72 crossing-rod cells on the base; mutants M1 and M2; `payload-rung-sweep --check` on the head and on main; the hosted check runs and the lint log.
- Read only: `step-import`, the tour, rustdoc/clippy/fmt, `pncad-py`, the tools roots.
