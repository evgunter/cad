# Review of PR 7041 at frozen head a55cd5809e (merge base e5f67cddde)

**Verdict: NOT-MERGEABLE-AS-IS.** One MAJOR: a door-built body that meshes on main newly refuses. The notch work itself holds up well under attack.

## Claims
1. **FALSIFIED** (own base/head differential, 99 bodies). Mesh digests are identical on every body that meshes on both trees, except the bodies of MAJOR-1. No lane change found: my in-probe recomputation of the old kind route agrees with the door on every ringless face.
2. **FALSIFIED in part.** `NotIsoRectangle` still mixes classes (MINOR-2). The `SpanUncertified` split is complete at the span sites I read (`require_rim_span`, `require_meridian_span`, `fold_chain` ×2).
3. **CORRECTED.** The new refusal texts are true as data statements (0 stored images, measured). The cause the PR gives is not: with images minted, the bodies still refuse (MINOR-6).
4. **Counts corrected, both verified.** The table has 20 bodies × 2 δ = 40 pairs; 8 pairs (4 bodies) moved and 32 did not, two of those 32 being the slit's refusal, so "32 of 32 meshing" is 30 meshing + 2 refusal. `tess-budget-baseline.csv` is unedited (1678 rows) and the hosted meter job is green on this head. The re-baseline is a disclosed departure from the spec's letter ("a stop: report it, do not re-baseline"). The spec contradicts itself here (routing the keyway must move its refusal digest), so I read it as a correction, but it owed the stop the letter asked for.
5. **Route: held. K stream: FALSIFIED** (MINOR-3). The only production door call is `curved::iso_rectangle_shape`, reached only from `lane_of` via `Routes::of`; no kind test routes any more.
6. **Not sufficient** (MINOR-7).
7. **HELD.** 24 extra notch bodies × δ {5e-2, 5e-3, 1e-3, 2e-4} × ε {1e-6, 1e-9, 1e-12}: 96 of 104 body/δ pairs mesh and pass `check_mesh`, volume, outward winding and sampled deviation ≤ δ with 0 no-area triangles, identically at each ε. Covered: notch at and flush with a rim, two notches (opposite, stacked, both rims), a partial wall, cylinder wedges, ball wedges with a corner on the seam and at the equator, two ball wedges. The other 8 pairs are NOTE-9. Cone and torus: axis-plane wedges refuse `CurvedPierceUnsupported`, a bar through the cone refuses `GermSectionOpenConic`, a bar through the torus builds a spiric-bounded hole (old route trimmed, meshes). "No door builds a cone or torus iso notch" held on what I tried.
8. **Re-run.**
   - M1 (notch → rectangle lane): 6 red — both D9 rows, the two keyway rows, both notched reach rows. As the PR says.
   - M2 (rectangle → chart lane): about 40 red; the PR's list is an undercount.
   - M3 (branch door skipped): 5 red — D9 ×2, the hemisphere pair, the half-cap ×2.
   - M4 (`Escalated`, `OffSurface`, `SpanUncertified` → chart lane): 3 red, all `SpanUncertified` split-child rows (MINOR-5).
   - M5 (kind pre-pass removed): 0 red of 3205 mesh + geom-brep tests (MINOR-4).
9. **Corrected.** Other consumers do see a different answer, by design: `topo::validate::classify_mass_props` reads a span-uncertified body as DEFECT where main read "cannot yet measure", and `PropsError::refusal()` moves `NotIsoRectangle` from Aborts to Limit. I read the three ladder sites that take that standing (`unanimous_rim_side`, two pole climbs); none can receive `NotIsoRectangle`, so no climb changes. No per-variant props tag exists in the Python binding. The topo and geom-brep re-pins are all `NotIsoRectangle` → `SpanUncertified` on span names, each consistent with the split.
10. **Held as noise, weakly.** Own micro-timing, 4 bodies × 2 δ, best of 5, two alternating rounds per tree: head/base within ±10 % in both directions (e.g. ball rect 4.75/4.88 and 5.08/5.11 ms; rod tilted 5.00/5.07 and 5.08/5.28 ms). I did not re-run the 250 s sweep. By inspection the door is read once per face per run, but two things changed shape: `Routes::of` is a serial pass on the caller's thread where the door ran inside the parallel lane, and it runs on memo hits, where main skipped it with the lane.
11. **Two held, one corrected.**
    - geom-brep fuzz row with `CAD_FUZZ_SEED=0x55c8ff29bb93ec4a`: red on the merge base.
    - `no_file_outside_the_mint_pins_a_chart_branch`: red on base and on a clean head worktree, identically (walk.rs 6 vs 2).
    - `every_decided_name_has_its_door_and_every_site_is_read`: **passes** on both in clean worktrees, so "red, not ours" does not reproduce.
    - Hosted, run 43977259946 on this head: all 13 jobs that ran are green, including test (kernel), the meter, tools, demos and python, plus `gate ok`; two skipped; none red. So the hosted gate does not show the census row that is red locally on main (NOTE-10).
12. **Deviations 1–3 confirmed by reading.** The filed row's premise is corrected (MINOR-8). The §5 sweep's blind spot is stated; `topo::props`' kind route is not this defect by my read (unsure).

## Findings
**MAJOR-1 — DEMONSTRATED.** `curved::iso_rectangle_shape` / `tessellate::lane_of`. Body: `ball_poled_z(1, 0)` ∖ `brick((x0,3),(-3,3),(-3,3))` posed by a rotation about y through the origin by θ, with x0 ∈ {0.3, 0.6} and θ ∈ {2e-9, 3e-9, 5e-9, 8e-9}, at ε = 1e-9. The boolean builds all 8; both sphere faces carry projected images, so the old route was trimmed.
- Base: meshes (390 and 366 triangles at δ 5e-2).
- Head: `UnsupportedCurvedShape { face 1v1, source: Escalated }`, the circle's axis-class margin (1.9e-9 at θ = 2e-9) inside the band [1e-9, 1e-8].
- At θ ≤ 1e-9 and ≥ 2e-8 both trees mesh the same bytes.

The PR names the case ("only a tilt within band could escalate") and its armed runs hold no such body. This is also the body the spec's "pinned on a body that reaches it if one can be built" asked for; none is pinned.

**MINOR-2 — DEMONSTRATED at the door, BY-INSPECTION downstream.** `geom_brep::props::require_iso_rectangle`; the `PropsError::NotIsoRectangle` docs and its standing. Hand-built edges, door answers:

| surface | carrier | door answer |
|---|---|---|
| unit cylinder | circle r 5 in an axial plane | `NotIsoRectangle("cylinder boundary circle is not a rim")`, raised before any on-surface decision |
| unit sphere | same circle | `NotIsoRectangle("props_meridian_great")` |
| torus (2, 0.5) | coaxial circle r 7 at z 3 | `NotIsoRectangle("props_rim_fit")` |
| torus (2, 0.5) | axial-plane circle r 3 at the centre | `NotIsoRectangle("props_meridian_fit")` |

Each is metres off its surface, is classed "decided no rectangle, valid input", and routes to the chart lane. By inspection the same variant also carries kernel-bug states (`mixed_levels`, "a torus piece that is no circle") and non-finite data ("torus axis length not measurable", via `torus_frame_refused`). So "every arm is a decided verdict on a face as stored, valid input" and "row 2: a limit" are false at those sites. The PR table omits torus `props_rim_fit`, `props_meridian_fit` and `props_meridian_plane`. Exposure is bounded: the chart lane needs stored, certified images.

**MINOR-3 — DEMONSTRATED.** `tessellate::Routes::of` and the splice in `tessellate_impl`. The K stream of a body that meshes on main is not identical: a ringless sphere face bounded by a tilted circle now records the door's verdicts. Bracketed verdict counts, base → head, same mesh digest: tilted-parallel ball 24 → 30, tilted-meridian 20 → 26, off-centre section 28 → 32. Rectangle faces and ellipse-trimmed cylinder and cone bodies are identical (the pre-pass decides nothing). I found no row that pins this; `k_funnel_composition` pins thread invariance only. By inspection, a memo hit now records the route's verdicts where main recorded none.

**MINOR-4 — DEMONSTRATED (M5).** `props::require_iso_carrier_kind`. The pre-pass carries the "wherever the loop starts" leg of claim 1 and no row can go red without it.

**MINOR-5 — DEMONSTRATED (M4).** No row through `tessellate` holds an `Escalated` or `OffSurface` source at the route; only the `SpanUncertified` arm is witnessed. `errors.rs` pins a Display string for a constructed value.

**MINOR-6 — DEMONSTRATED.** The "refuses for want of stored chart images" account in the PR body, the `d9_mesh_goldens.rs` header, and the docs of `iso_rectangle_door`, `mesh11_arc_branch`, `mesh7r1_probes` and `r2_mesh7_probes`. After `topo::mint_pcurves` (succeeds on all five witnesses), head answers:
- keyway: `TrimLoopOrientation { face 2v1 }`;
- oblique lens, pole-crossing half cap, apex bow tie: `DomainUnread` / `outer_open`;
- slit: `UnsupportedCurvedDomain`, unchanged.

So none would mesh with images, and the stated cause is a refusal text read as a cause. What is lost: main told the half-cap caller "a sphere meridian arc whose stored span contains a pole … state that side as two edges meeting at the singularity", which is the real recourse. Head says "an edge's carrier is outside the certified inventory — … carries no stored pcurve cache", whose first clause is false of a circle on a sphere, and following its advice reaches a second refusal. `NotOneChartBranch` is routed as "the chart lane's", yet that lane cannot read such a loop.

**MINOR-7 — DEMONSTRATED (M3) + BY-INSPECTION.** `curved::tessellate_curved`. With the branch door skipped, the half cap reaches the rectangle lane and stops at `PinchWedge` (δ 0.5) and the hemisphere pair at `SingleColumnCurvedFace`. These are incidental later guards, not the walk-consistency check, whose own docs say a merged pair beside a meridian side passes it. The lane keeps a router-defect refusal for a ring and none for shape. I would have kept the shape guard in a form a mis-route cannot skip.

**MINOR-8 — DEMONSTRATED.** `work/tesslane/a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read.md`. "No public door is known to build a slit with its images": `topo::mint_pcurves` mints them on the slit. What is missing is a route override, not a witness.

**NOTE-9 — DEMONSTRATED, pre-existing.** `rod(1,-2,4)` ∖ `brick((0,3),(0,3),(0.5,3))` and the (−0.5, 0.5) variant: the rectangle neighbour whose side lies on the seam refuses `CertificateExceeded { bound ≈ 0.41–0.44 }` at every δ on both trees. On head the notched face ahead of it now passes, so this is the refusal a seam-cornered cylinder notch reaches.

**NOTE-10.** The hosted kernel job is green while `no_file_outside_the_mint_pins_a_chart_branch` is red in clean checkouts of base and head.

**NOTE-11 — BY-INSPECTION, pre-existing.** `require_one_chart_branch` reads `decide(...)` with `matches!(.., Ok(Sign::Positive))`, so an escalated apex or pole decision passes as "one branch". That is a door that cannot say answering Rectangle.

## Spec deviations
- **Reported:** the four re-baselined rows; three spec errors; the unit cut in two (the spec's own).
- **SILENT:**
  - The curved lane's shape guard is removed, where the spec says "the two lanes' guards against a mis-routed face stay as router-defect refusals".
  - "A door that cannot say … pinned on a body that reaches it if one can be built" is answered "none" without a constructive search; one builds in four lines.
  - The class list omits sites (MINOR-2).
  - The K stream moves (MINOR-3).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (`tessellate.rs` 150–710 and its routing whole, `iso_rectangle_shape` whole, the kind pre-pass, both doors, the sphere, cylinder and torus parses).
- **Q1, sure.** `props/curved.rs` `require_iso_carrier_kind` beside the `cylinder_boundary`, `cone_boundary`, `sphere_boundary` and `torus_classify` arms. The pre-pass is a second copy of each parse's kind table, kept in step by four shared strings and a sentence ("the parses keep their own arms"). A fix that mints the duplication it sits beside.
- **Q1, likely.** Three hand-written class tables over one enum: `PropsError::refusal()`, `topo::validate::classify_mass_props`, and now the `iso_rectangle_shape` match. They already disagree on `QuadratureUnsupported` and `NappeSpanning`. Elsewhere I would look at `pncad-py`'s tag tables.
- **Q1/Q7, likely.** `tessellate.rs` carries `Lane`, `Route`, `Routes` (a table of `Lane`s, not `Route`s), `Routed`, a local `route` that is a K recording, and `routed` rebound from `Vec<Routed>` to `Vec<Detached>`; `curved::Shape` is a fifth near-synonym.
- **Q2/Q5, sure.** `tessellate.rs:415–420`: "the curved lane's shape door is its only consumer (`SizingTols::band`)" is false now. The route reads the band before `SizingTols` exists; `sizing.rs` was re-worded and this was not.
- **Q5, likely.** `curved.rs:1–35`: the header still says the rectangle is "CHECKED here … as FIVE questions", and questions 1–2 are no longer asked in this module's lane. `curved.rs:1953` still says "the shape door refuses it".
- **Q5, sure.** `tessellate.rs:180–190`: `tessellate_with`'s doc lists what a memo hit does not run; the route's doors now run on a hit and record K, unsaid.
- **Q2, likely.** `tessellate.rs:436–439`: `Routes::of` sits under the "Chord pass:" comment.
- **Q3, sure.** `d9_mesh_goldens.rs`: three re-baselined bodies share one digest (`0xe03d…9487`, one edge key, one text), so those rows no longer tell their bodies apart or witness any door answer.
- **Q3, likely.** `reach_probes::a_notched_sphere_face_meshes_on_the_sphere_row` can fail; good. The renamed "is_decided / is_routed" rows assert a refusal whose stated cause is wrong (MINOR-6).
- **Q4, likely.** `types.rs` `UnsupportedCurvedShape`: the name and Python tag now mean "lane unread", the opposite of what the name says, defended in a paragraph. `py/mesh.rs`' comment is re-worded only.
- **Q6, sure.** The cost claim (143 → 150 s, 256/255 vs 274/258 s) rests on one measurement with no guard and no reason at the claim site.
- **Q7, likely.** `Routes::lane` returns `None` for a refused face and the chord pass silently skips it. A refusal is carried as absence in one structure and as `Err` in the parallel `Vec<Routed>`.
- **Q7, unsure.** `Routes::witness(&body)` is recomputed inside per-face loops in `trimmed.rs` tests (3130, 3296, 3371, 3416, 3782).
- **Q8, unsure.** `iso_rectangle_shape`'s doc is about 60 lines for a 12-line match.

## Rubric
- **Idiom/structure: 3.** One production reading of the route, cleanly threaded; vocabulary sprawl and a duplicated kind table.
- **Test quality: 2.** The notch rows are real and hold. The pre-pass, the `Escalated` and `OffSurface` arms and the K claim have no row; three goldens collapsed to one digest.
- **Doc/comment honesty: 2.** The causal account of the hand-built refusals is unmeasured and wrong; `NotIsoRectangle`'s "every arm" claim is false at sites shown.

## Executed
- Own differential probe on base and head (tilt series on sphere, cylinder, cone, torus; in-band off-centre sections; 24 notch bodies).
- The notch set on head at ε 1e-6 and 1e-12.
- K brackets on both trees.
- Mutants M1–M5 with the whole mesh suite; M5 also with the whole geom-brep suite.
- The three claimed-red rows on base, the two topo rows also on a clean head worktree.
- Door-level class probe; minted-image probe; micro-timing.
- Hosted check runs read twice.

## Read only
The diff; both doors and their parses; `tessellate.rs` routing; PR body; spec; plan section. Not run: the sweep suite, tour, the 250 s tess-budget sweep, python. Nothing committed, pushed or posted; scratch worktrees removed.
