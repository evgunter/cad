REVIEW — PR 7043 at b6993486b8 (full review + style lane). I saw no other review's material and read no branch under analysis/. Of the PR I read the description and the check runs only.

**Verdict: APPROVE-WITH-FIXES.** The code does what the page says, and every mutant I ran goes red. The fix needed is disclosure: nine cells that main meshes now refuse, and the PR doesn't say so.

## Claims
1. Clearance drops the nodes the page says — **held.** By inspection, `Side::meets` (chart) is an exact separating-axis test with closed sets. The test row checks it against an independent clipper over a ringed oblique quad at c, 2c and 4c. Mutant "a side read by its endpoints only" is caught by `a_node_is_laid_exactly…`, the d9 digests and the torus ring.
2. Own step and plane — **held, one reading flagged.** Sector test by inspection: two half-planes give the wedge exactly while the wedge is under π (it is: turn·2c·hu ≪ π). The development never wraps (θ = u·sinα < 2π). r0 is clamped at the apex. The radius test runs on the clipped, connected segment, so it is sound with no sampling gap. Mutant "chart side on the cone" reds only the unit row `in_the_development_a_nodes_box_is_its_sector`: no body-level row tells the two planes apart. The cone's "smaller gap" and the spline "laid by each band" (`lay`'s dedup lays a cut-line node if ANY band's box clears) are both the permissive reading. Both are defensible, but each is argued at its own site.
3. Nothing that meshes on main refuses — **FALSIFIED.** I ran my own base/head differential, the same probe file in both trees: base b0c685281a = main tip = merge base. It covers 318 cells: the tilted pair at δ 0.1/0.04/1e-2/5e-3; donut×bar; T1 cone/box at five (s, xf); rods cut at tilt 0.3/0.5/1.1/1.4; bands w 0.2→2e-4 at δ to 1e-4; tube×rod; ball×rod.
   - 158 cells identical. 151 moved, and only on Cone, Cylinder, Sphere and Torus faces. No Plane patch moved. Triangle counts never rose (worst band tilt 1.1, w=0.002, δ 1e-4: 818→732). Every head mesh passes `check_mesh` with 0 unresolved triangles.
   - **9 cells change from Ok to `UnresolvedWinding`**: T1, xf=0, s=0.6 at δ 5e-3 and 1e-3, and s=0.3 at δ 1e-3, ops A∪B, B∪A, A∖B. These are exactly the nine cells of claim 10 (below). The PR body states no refusal outside its table and doesn't mention these. That is SILENT against the spec's "a new refusal of a face main meshes is reported with its body, never absorbed".
   - D9: the head's d9 rows pass with only `tilted_above`/`tilted_below` re-baselined, so the other 36 digests are unmoved. I did not re-derive the worst certificate of the moved meshes: it isn't public.
   - Spline: blade lofts ×3, `klein_elbow` 0.2/0.4 and `vessel_quarter` at δ 2e-2/5e-3 are byte-identical base vs head. My blade×rod booleans built no body, so I did NOT reach a trimmed spline face of my own. The 703-face claim is unverified by me. The PR gives no reason for it, only the measurement.
4. A face may lay no node — **held on my set.** Bands at four tilts, w down to 2e-4, δ to 1e-4: all mesh on both trees, and no cell raises `CertificateExceeded`, `BoundaryEar` or `Triangulation`.
5. What retired — **held on my set, with one concern by inspection.** No `Triangulation` refusal in either tree. But "an ear's or refinement's point is strictly inside its triangle" holds in exact arithmetic only. `inside_point` is an f64 centroid, and in the development a round trip (atan2/length, then sin/cos). For a thin triangle it can round onto a side. Main dropped such a point and retried; the head now refuses `Triangulation`. Not demonstrated. Keeping the arms as refusals is right: spade has to answer something, so they can't be deleted.
6. One definition, three readers — **held.** Mutants: allowance at zero reds the 3 validate rows, `a_lattice_laid_without_clearance…` and the whole frustum. Join removed reds `a_lattice_laid…` and the whole frustum. The join covers memo hits and lane runs alike (one `face.place` arm). The STL writer's exact-zero test is a different predicate (it cannot write a normal at all), not a second spelling. A second spelling in PROSE does exist (Style 1).
7. The scale — **held at 1 ulp; I measured the margin.** Hand-built meshes run through `check_winding`, then every coordinate perturbed in all 2^9 sign patterns:
   - ±1 ulp never flips a triangle the check calls resolved (scale 1 and 1e3).
   - A triangle at 1.05a ("resolved") flips under ±a per coordinate; one at 1.5a survives ±a/2 and flips at ±a. The margin is about a/2 = 4ε·s per coordinate here, and a/(2√3) at worst in 3-D.
   - So "would not flip if each coordinate moved by its evaluation rounding" is true only for evaluation rounding of a few ε·s. The page doesn't quantify that rounding. NOTE.
   - Unresolved triangles in my corpus sit at 0.002–0.005 a: true noise. No over-refusal seen: the whole mesh suite is green at the head, and so is CI.
8. The refusal — **held.** I reached it on the TRIMMED lane (the cone face, cells of claim 10) as well as the planar cap. The `UnresolvedWinding` doc's example names only the frustum cap.
9. MIN_SHAPE replaced — **held on my set.** On every base mesh, h/L < 1e-12 and "unresolved" counted the same triangles (18 cells, 2 or 8 triangles each). By inspection, check_winding passes triangles with h/L between ~2e-15·s/L and 1e-12 that MIN_SHAPE refused. The page permits that.
10. Cone met on its axis — **established by measurement.** Main's mesh holds Cone-face triangles of the apex (id 0) plus TWO LATTICE nodes on one ruling: corners like (−0.020, 1.890, −0.107), (0, 2.078, 0), (−0.040, 1.701, −0.214). Height over the longest side is 0.002–0.005 of the allowance. They are not wound against the face: they are unresolved noise, and the probe read a sign out of noise. The head refuses all nine cells `UnresolvedWinding` on the cone face (2v1 / 7v3). A mutant that treats Unresolved as Folded in `emit` does NOT cure them. The clearance can't either: the node is far from the apex in its box, and the defect is an interior edge in the development. The open issue file still says "wound against" and is not updated.
11. Mutants — **all caught.** Clearance at zero: 7 rows red, the PR's set plus the d9 digests. Endpoints only: 4. Allowance at zero: 5. Chart side on the cone: 1, unit only. Crease clearance removed: 1, unit only. Join removed: 2. Scale ×2/÷2 and memo-skip not re-run (the PR's own).
12. Page against code — **held but for one sentence (unsure).** "A triangle beside the boundary then reaches 1 + BOUNDARY_CLEARANCE steps" is untrue where no node is laid: a band's triangles run boundary to boundary. `FacePatch`'s doc and the STL writer's doc are true.
13. CI — the 15 check runs on b6993486 (run 43984526769) all completed, success or skipped by design. Locally: `cargo nextest run -p mesh` at the head, 489 pass, 7 skipped.

## Findings
- **MAJOR** (DEMONSTRATED): nine T1 cells (xf=0; s 0.6 at δ 5e-3 and 1e-3, s 0.3 at 1e-3; A∪B, B∪A, A∖B) meshed on main and refuse `UnresolvedWinding` at the head. The refusal is correct by the page. The fix is disclosure, not code: the PR body, the open issue `a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md` (now known to be unresolved apex+lattice triangles on one ruling; the head refuses them), and a pinned row.
- MINOR (BY-INSPECTION): `trimmed::TrimmedFace::inside_point`'s "strictly inside" isn't guaranteed in f64, and a miss is now a `Triangulation` refusal where main retried (claim 5).
- NOTE (DEMONSTRATED): the margin of `validate::resolved_normal` is about a/2 per coordinate. The page's "evaluation rounding" is unquantified (claim 7).
- NOTE (DEMONSTRATED): the development box is pinned by a unit row only.
- NOTE: my own run reached no trimmed spline face; the spline claim rests on the PR's census.

## Spec deviations
Reported in the PR: the constant moved 1/4 → 1/16 (the owner's word); the table not re-taken after PR 6965; the scale reads the corner norm, not the largest |coordinate|; the log's δ 3e-9 claim corrected.
SILENT: the nine new refusals above. The spline arm's reason isn't given, only a measurement.

## Style
1. `validate.rs:347` (`check_winding` doc) and `types.rs:972` (Display) restate "eight epsilons", a prose copy of `cert::ROUNDING_ULPS`. One more site of the class: `validate.rs:400` wrote the 8 into a row on purpose. — sure
2. Q1 "own step": the permissive reading is chosen twice, at two sites with separate arguments. `lattice_candidates` takes the min gap; `lay`'s `laid.dedup()` lays a cut-line node if any band clears, and that ordering is implicit through `sort_nodes`' step key. — likely
3. `Creases::on` (exact `contains` on f64) decides crease exemption, beside the module's "nothing asks whether two points are the same". It is a lattice-internal identity, but an exact-coincidence decision all the same. — unsure
4. `boundary_clearance()` hosts a `cfg(test)` thread-local override inside the production path. Not how I'd do it: a parameter is already threaded into `lay`. — likely
5. The `TessellateError::UnresolvedWinding` doc gives the frustum cap as its one example; the trimmed-lane apex ruling is a second reach. — likely
6. README clearance bullet "reaches 1 + BOUNDARY_CLEARANCE steps" vs a face that lays no node (Q5/Q12). — unsure
7. Q3: the development-vs-chart reading and the crease clearance each have a single unit row as their only red. — likely
8. The STL doc says the normal's meaning is the mesh's promise, but the file stores f32 vertices: resolution at f64 does not carry into the written triangle. — unsure
9. `cert::coordinate_scale`'s doc says "largest coordinate norm", which reads as max |coordinate|. The very confusion the PR corrects in the log's count. — likely

Questions exercised: Q1 (prose and constants greps over mesh/stl), Q2, Q3 (mutants), Q5, Q7, and Q8 partially: `lay`, `Side`, `clip`, `radii_sq`, `insert_points`, `constrain_boundary`, `chain_crease_lines`, `inside_point`, `emit`'s fold census and `validate.rs` read whole; trimmed.rs NOT read end to end.

## Rubric
- Idiom/structure 4: `Side`/`Node`/`lay` are clean, and the join is one site for every lane.
- Test quality 4: every mutant reds something, but two only at unit level and nothing pins the nine cells.
- Doc/comment honesty 3: accurate except the silence on new refusals and the stale issue.

## Executed
- Base/head differential (318 cells), spline differential (30 cells), the nine cells' triangles on main, perturbation probe, the fold-cure experiment, 7 mutants, the whole mesh suite at the head, CI checks resolved.
- Read only: the pncad-py tags, the nextest slow-set entries, winding_rows' frustum rows.
- Probe code available on request; nothing committed. Worktrees and build output deleted.
