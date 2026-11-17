---
id: the-lattice-keeps-clear-of-the-boundary-and-every-winding-is-resolved
kind: unit
title: the trimmed lattice keeps a clearance from the boundary, and tessellate refuses a face whose triangle f64 cannot wind
status: closed
opened: 2026-11-14
priority: P1
cost: H
branch: tesslane/lattice-clearance
closed: 2026-11-17
pr: 7043
---


Builds the two rules PR 6966 puts into `crates/mesh/README.md`: `[mesh.winding]` and the clearance bullet of `[mesh.sizing]` (`docs/DESIGN-FORK-LOG.md` row 278). Ev answered on that PR on 2026-11-16: the clearance is 1/4 now, with a plan for 1/2. What follows is what the three designers agreed an implementer needs, and then the unit.

## What it is

- **The clearance.** On the trimmed lane, a lattice node is laid only where the box `BOUNDARY_CLEARANCE` (1/16) of its own step each way, per chart axis, meets no side or point of the trim polygon and no crease line. Applied once, before the retry loop, to lattice nodes only; ear and refinement points are centroids of a kept triangle and are not filtered. On the spline arm the step is each band's own, and that arm sizes its worst sliver at δ exactly, so it is measured under the rule before the rule is applied there.
- **What retires.** The exact-duplicate drop in `TrimmedFace::insert_points` and the node-on-a-side retry in `constrain_boundary` become unreachable for lattice nodes (the designers differ on whether they become typed refusals or stay as backstops: the unit says which and why). `Creases::cut_candidates` stops moving nodes onto a crease; the snap of boundary points onto a crease stays, and is the same class (`CREASE_MERGE`).
- **The winding.** One definition in `mesh::validate`: a triangle is resolved when its cross product exceeds `cert::rounding_allowance` at its coordinate scale times its longest side. `tessellate` reads it once per triangle where each face's patch joins the mesh (the `Ok(placement)` arm of its placement loop), on every lane and for a memo hit, and refuses the face typed under a new variant. `TrimmedFace::emit`'s `normal·outward > 0.0` reads the same definition and answers outward, folded or unresolved. A public check beside `check_volume` re-derives it; the suites call it in place of `reach_probes`' `MIN_SHAPE`.
- **The STL writer's doc** cites the promise (`the-stl-writers-doc-names-a-contract-nothing-states`).

## Measure first

The designers' numbers are in `work/tesslane/log.md` (2026-11-14, "Design fork, three designers"): the clearance table over 486 faces, the 40.6-million-triangle census, and the two frusta. The unit reproduces them before it builds, and names every golden that moves (D9's trimmed bodies, `docs/tess-budget-data`, the tour's pins) with why its new mesh is right.

## Closes

`a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area` (filed on PR 6960).

## Unit (2026-11-16, TESSLANE)

**Letter H. Tier: dual review.** It moves the bytes of every trimmed face that has a lattice node near its boundary, adds a refusal on every lane, and is the code behind text Ev ratifies. A filter that drops one node too many leaves a triangle the certificate refuses or, worse, a region the lattice no longer sizes; a winding verdict read at the wrong scale refuses sound meshes or passes a noise normal.

Branch `tesslane/lattice-clearance`, cut from `tesslane/ev-flat-triangles` (PR 6966, the page's text) with `tesslane/domain-loops` (PR 6960, unit 3) merged in. Merge main as those land. Two lanes are in the same files: PR 6965 (`tesslane/cone-near-apex`: `cert.rs`, the cone rows, `trimmed::lattice_candidates`, the kept-triangle rule) and unit 4 (`tesslane/route-by-domain`: `tessellate::lane_of` and the route).

**The page is the contract.** `crates/mesh/README.md` on this branch states both rules; build what it says. Where the code cannot be made to match a sentence of it, stop and report: the page is Ev's, and a change to it goes back to PR 6966, not into this diff.

**Owed by the unit, beyond the sections above:**

- **Measure first, on this branch's base.** Reproduce the clearance table (486 analytic trimmed faces: nodes not laid, worst certificate in cell bounds, faces refusing, at none, 1/16, 1/8, 1/4, 3/8, 1/2) and the two frusta (the cut one's 226 triangles under ε, the whole one's unresolved triangles in its cap at δ 1e-8), from `work/tesslane/log.md`'s entry and its own probes. Say where your numbers differ and why: unit 3 has landed under you since they were taken.
- **`trimmed::BOUNDARY_CLEARANCE` is 1/4**, a named constant read in one place. Ev has the cost of 1/4 against 1/8 and 1/16 in front of them, so the value may change before merge: nothing but the constant and the goldens may depend on it, and the PR body gives the worst certificate and the refusing faces at 1/8 and 1/16 as well.
- **What the clearance is measured against.** Every side and point of the trim polygon, rings included (`TrimPolygon`, every loop), and every crease line, in the chart the lattice is laid in; on the cone, say which geometry the box is in (the chart's or the development's) and why that is the one the triangulation's flat triangles are made in. A period seam and a singular row are boundary of the polygon like any side: say what a node beside each does.
- **A face must not lose its sizing.** A narrow face (a strip under two clearances wide, a ring close to the outer loop) can lose every node. Say what bounds its triangles then, measured on a body that reaches it, and whether the certificate's refusal is the answer or a face that meshed on main now refuses; a new refusal of a face main meshes is reported with its body, never absorbed.
- **The winding verdict has one definition and three readers:** the join in `tessellate` (every lane, and a memo hit), `TrimmedFace::emit`'s fold decision, and the public check. No second spelling of the allowance or the scale. The refusal is its own variant with the face, reaches `pncad-py` as its own tag, and its text is true of the body that raises it (`memories/refusal-text-is-not-cause.md`). Pin it on the whole frustum at δ 1e-8 (a planar cap, so the trimmed lane's check cannot be what catches it) and show the cut frustum does not refuse.
- **What retires, said per mechanism:** the exact-duplicate drop, the node-on-a-side retry, the crease move. For each: unreachable and removed, or kept as a typed refusal with the body that reaches it. A backstop nothing reaches is not kept on a hunch.
- **Mutants.** The clearance set to zero (the no-area rows of `reach_probes` go red: say which); the clearance applied to refinement points too; the verdict's scale off by a factor either way; the join check skipped on a memo hit. Say what catches each.
- **What moves.** Every trimmed face with a node inside the clearance. Name each golden and pin that moves (D9's trimmed bodies, `docs/tess-budget-data`, the tour's pins, `reach_probes`' counts) and, for each, why the new mesh is right: its certificate, `reach_probes::geometry`, the triangle count against the old. Regenerate goldens last, after the final merge of main, and never to make a red row green that you have not explained (`memories/output-stability-as-justification.md`). A rectangle-lane or planar face that moves a byte is a stop.
- **The spline arm.** Measure it under the rule before applying the rule there, as the section above says, and report the table; if it refuses faces at 1/4, say so and leave the arm out with a filed row sooner than tune it.
- **Done when** `a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area`'s bodies (the torus ring at δ 10⁻³, the tilted sphere pair at δ 0.04) mesh with no unresolved triangle and their `Got::NoArea` pins are gone; the public check passes over every mesh the mesh suite builds; `reach_probes`' `MIN_SHAPE` is replaced by it; the whole frustum refuses typed in its window and meshes outside it; `FacePatch`'s doc and the STL writer's cite the promise.
- **Closes** `a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area` and `the-stl-writers-doc-names-a-contract-nothing-states`.
- **Run what CI runs before every push:** `bash scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; fmt; clippy for mesh, geom-brep, topo, stl, `demos/tour` and the viewer job (`cargo clippy -p viewer --features app --all-targets -- -D warnings`); `cargo nextest run -p mesh` whole, and the viewer's tests; the `mesh budget meter + certificate falsifier` job's steps and the `tools` job's steps from `.github/workflows/ci.yml` (excluded roots whose tests mesh bodies); the rows you add or change at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well, each stating what it expects at each; sweep's rows over the bodies you touch. A test costing 0.5 s or more goes in the slow set with its measured time.

## Closed (2026-11-17, PR 7043)

Both rules of the page are built. `trimmed::lay` lays a lattice node only where the box `BOUNDARY_CLEARANCE` of its own step each way meets no side of any loop and no crease line but its own; nothing is merged or moved. `validate::resolved_normal` is the one definition of a resolved winding, read where faces join the mesh (`tessellate`), by the chart lane's own emit pass and by the public `validate::check_winding`; a face with an unresolved triangle refuses `UnresolvedWinding`, naming the triangle, its height and the allowance.

What the sections above did not foresee:

- **The value is 1/16, not the 1/4 this file's unit section names.** The unit's measure-first pass found that 1/4 refuses plain cylinders cut by a steep plane, which main meshes: the designers' table had read 1.06 cell bounds with no clearance over their 486 faces, and the suite's own faces reach 1.81 on main. Put to Ev on PR 6966: the temporary value does not matter. 1/16 landed, with the plan to 1/2 on `the-boundary-clearance-goes-to-half-a-step-with-the-boundary-run-bound`.
- **The winding rule refuses more than the page's PR said.** Nine cells of one cone-and-box pose mesh on main and refuse here, and behind them a class: every cone face through its apex with a split straight side, which the rectangle lane meshed with triangles of no area. That went to three designers (`docs/DESIGN-FORK-LOG.md` row 279); the interim cure is PR 7049 and the final state is on `[ev]` PR 7046. On the chart lane, which is what this unit changes, no face that meshes on main refuses.
- **Two pinned viewer rows moved with the clearance and nobody ran them.** `display_budget` and `review_pick_r2` are in the slow set, which the per-PR gate skips; the implementer's viewer run, both first reviews and the orchestrator's pre-push list all left the slow set out. A lane stacked on this branch found them red; both delta reviews then showed them green on main and red since this unit's first head. Re-pinned here with the counts.
- **What review settled:** the chart lane refuses at once on an attempt that holds an unresolved triangle (no retry can resolve one); a location two bands lay is laid only where both clear; a side with an end at a cone's apex is searched at every azimuth; the clearance reaches the lane as a value, with no test switch in the production path; a crease's own node is known by provenance.

Left for others, each filed: a point the cone arm places inside a very thin developed triangle can land on its side, where the lane now refuses and main retried, with no body known to reach it (`a-point-placed-inside-a-developed-needle-triangle-can-land-on-its-side`); the page's sentence about evaluation rounding against a scale that reads corner norms (`the-winding-verdicts-scale-reads-corner-norms-not-what-an-evaluation-rounds-at`); the two cone choices no measured body distinguishes, said at their sites.
