ROUTE-BY-DOMAIN (unit 4) report. Head a55cd5809e177408da8b7cfdc57e716c276ac635 (ls-remote confirmed), main merged at its tip (6960 + 6965 in).

SPEC WRONG, FIRST: (a) D9 goldens pin five REFUSAL digests; four move (keyway, oblique_lens, pole_crossing_half_cap, apex_crossing_bowtie) because the doors now route those faces to the chart lane, which refuses the Euler-built bodies for want of stored images. I re-baselined those four rows; no meshing digest moved (32/32 pairs byte-identical). If you read "a byte that moves there is a stop" as covering refusal digests, that is the one thing to look at. (b) `takes_sphere_arm` does not exist on the branch; `face_target` was no kind test (it read lane_of) and stays, now taking the lane. (c) The door does not read rings: ringed faces pass it on their outer loop, so the ring check stays in lane_of as a domain fact. (d) No door builds a cone or torus notch: the boolean refuses a box through either wall (CurvedPierceUnsupported).

1. MEASUREMENT (probe on every curved face, old route + both doors, over mesh and sweep suites, old code)
Door refusal sites by class:
- CLASS 1, decided no rectangle: non-iso carrier kind (cyl/cone ellipse: sweep 15647 cyl + 156 cone faces, tilted cuts; sphere/torus non-circle: vessel spiric torus walls; NURBS/spiric -> Unimplemented: 69 cyl + 24 sphere in mesh); Inventory residuals props_meridian_great / props_rim_axis_parallel / props_rim_center_on_axis (tilted sphere sections, sweep 5705 faces; oblique lens), props_meridian_axial (mesh8r2 wobble), "cylinder boundary circle is not a rim" (mesh8r1 tilted meridian circle); props_rim_level (notched pipe bars 1-2, keyway, sphere notches); props_rim_side (apex bowtie); "torus face without a meridian" (rim-only torus rows); NotOneChartBranch (half cap, hemisphere pair, mesh7r1 three-rectangle rows). Unreached by any body: "torus meridian pieces close a loop with no rim", mixed_levels, torus frame/orientation degenerate (argued unreachable by valid input; left class 1).
- CLASS 2, cannot say: Escalated at every decide. Zero door escalations over every curved face of mesh + sweep + tour; no body built.
- CLASS 3, data contradicts: OffSurface (props_meridian_on_surface: mesh8r2 wobble, 9 sweep faces; props_circle_axis_class: errors row); span bounds and props_meridian_pieces_* (split child shifted a period: mesh10r1/r2, iso_rectangle_door split_children; spans: geom-brep hand-built rows only).
Curved-lane refusals on main: UnsupportedCurvedShape = everything above that reached Curved (keyway, lens, half cap, bowtie, hemisphere pair, rim-only torus, notched pipes, wobble, split child). MeridianFree = lens (unit) + meridian_free_face rows; SingleColumn = errors/walk units; walk consistency = the slit. All Euler-built except the pipes.

2. WHAT CHANGED
- c6566565 geom-brep: PropsError::SpanUncertified (row 4, aborts) split from NotIsoRectangle at the raising sites (rim/meridian span doors, fold_chain pieces); NotIsoRectangle now row 2 (Limit). require_iso_rectangle reads every edge's carrier KIND first (require_iso_carrier_kind; whats shared as consts). mesh: curved::iso_rectangle_shape -> Shape{Rectangle,NoRectangle} or UnsupportedCurvedShape{source} (exhaustive match over PropsError); lane_of(body,fk,surface,band); tessellate::Routes::of reads every face once per run before the chord pass, K recording spliced per face in the fold; chords/face_target/arm_steps/tessellate_impl take the lane. Removed has_trim_carrier, has_tilted_circle, has_ring, require_iso_rectangle_face; tessellate_curved no longer asks the door (ring guard stays as router defect).
- 5af1e6a8 docs, UnsupportedCurvedShape doc + Display (now the route's refusal), errors row.
- d9780bb6/7254967a sphere notch row, SpanUncertified pins in geom-brep/topo, topo sample roster + validate reading. ec773359 slow set, filing, clippy. Merges b9f51110, a55cd580.
The answer lives in curved::iso_rectangle_shape; require_iso_rectangle_face is gone (its role is that fn, asked once per face by the route, never a refusal a valid face of either shape reaches).

3. PR BODY
=====
## Measurement
Every curved face of the mesh and sweep suites was probed with the old route (carrier-kind tests) beside both doors' answers (`require_iso_rectangle`, `require_one_chart_branch`).

| class | door site (what / variant) | bodies that reach it |
|---|---|---|
| no rectangle | carrier kind: ellipse on cylinder/cone, non-circle on sphere/torus | tilted cuts (sweep), spiric torus walls |
| no rectangle | carrier kind: spline/spiric (`Unimplemented`) | spline-trimmed cylinder and sphere faces |
| no rectangle | `props_meridian_great`, `props_rim_axis_parallel`, `props_rim_center_on_axis`, `props_meridian_axial`, circle not a rim | tilted sphere sections, oblique lens, wobble rows, tilted meridian circle |
| no rectangle | `props_rim_level` | notched pipe walls, keyway, notched sphere faces |
| no rectangle | `props_rim_side` | apex bow tie |
| no rectangle | torus face without a meridian | rim-only torus faces |
| no rectangle | `NotOneChartBranch` | pole-crossing half cap, hemisphere pair |
| no rectangle (unreached) | pieces close a loop with no rim; mixed levels; torus frame degenerate | none; unreachable by valid input |
| cannot say | `Escalated` (every decide) | none: no door escalation on any curved face of mesh, sweep or tour |
| contradicts itself | `OffSurface` | wobble rows, cone row in `errors` |
| contradicts itself | span bounds, `props_meridian_pieces_*` → now `SpanUncertified` | split child moved a period; hand-built span rows |

`NotIsoRectangle` mixed the first and third classes. The spec has three things wrong: `takes_sphere_arm` does not exist on this branch; `face_target` was no kind test; and the door reads no rings, so a ringed face passes it on its outer loop and the ring stays its own domain question. No door builds a cone or torus notch: the boolean refuses a box through either wall (`CurvedPierceUnsupported`).

## The change
- `geom_brep::props`: `SpanUncertified` is split from `NotIsoRectangle` at its raising sites (stored span outside certification's per-edge bounds, or split pieces that do not assemble one); `NotIsoRectangle` is now only the decided "outside the inventory" verdict, standing row 2. `require_iso_rectangle` reads every edge's carrier kind before any banded decision, so a face carrying a non-iso carrier is decided "no rectangle" wherever its loop starts.
- `mesh`: `curved::iso_rectangle_shape` reads both doors as the route. Both pass → curved lane. `NotIsoRectangle`, `Unimplemented` or `NotOneChartBranch` → chart lane. Anything else → `UnsupportedCurvedShape` with the door's cause. The match names every variant. `tessellate::Routes` reads each face's lane once per run, before the chord pass; its K recording is spliced at that face's turn. The chord pass, `face_target`, `arm_steps` and the lanes all read that one value. The carrier-kind predicates are gone, and the curved lane no longer asks the door.

## No face that meshes on main changes lane
- A face the curved lane meshed on main passed both doors there (the lane asked them), so it routes to the curved lane again.
- A face the kind tests sent to the trimmed lane carried an ellipse, spline or spiric in its outer loop (by construction now decided "no rectangle" by the kind pre-pass), a ring (unchanged), or a sphere circle with a projected image. That last one is decided "no rectangle" on every exactly tilted circle; only a tilt within band could escalate.
- Armed run: `lane_of` panicked wherever the kind route said trimmed and the new route differed, and logged the converse. Mesh suite and the tour suite: no panic. The sweep suite ran with the probe (old route plus both doors' answers on every face), all 3761 green: every ringless face the kind tests sent to the trimmed lane is decided "no rectangle"; none escalated, none contradicted. The converse differences (mesh: 254 sphere, 180 cylinder, 48 cone, 4 torus) are faces the door refused on main.
- D9: all 32 meshing body/δ pairs are byte-identical; the meter's 1678 rows are byte-identical. Four refusal digests move (keyway, oblique lens, pole-crossing half cap, apex bow tie): the doors now route them to the chart lane, which refuses these Euler-built bodies because they store no chart image. Rows re-baselined.

## Notched faces per chart
- Cylinder: the pipe bars through and half in (∪ both orders, pipe ∖ bar) mesh at δ 5e-3, 1e-3, 2e-4 and pass `geometry`. Row re-pinned as `a_notched_or_holed_pipe_wall_meshes`.
- Sphere: ball ∖ quarter-wedge box, from below across and beside the seam, and from above through the pole. The faces are decided `props_rim_level`; every op meshes at every δ (new row `a_notched_sphere_face_meshes_on_the_sphere_row`).
- Cone, torus: no door builds the notch (above).
- Triangles of no area: 0 over the 30 notched bodies at nine δ, 5e-2 to 1e-4.
- Hand-built rows (keyway, lens, half cap, bow tie, hemisphere pair, rim-only torus): the door's verdict is pinned directly, and `tessellate` now pins the chart lane's refusal for want of stored images (`DomainUnread` Gap or OuterOpen, or the chord pass's no-cache refusal).

## Mutants
- M1, a notched face sent to the rectangle lane: caught by the walk's consistency check (`UnsupportedCurvedDomain`) in both notched rows, by the keyway row and by the D9 keyway digest.
- M2, a rectangle sent to the chart lane: caught by D9 (20 of 40 pairs move), the slit row and the split-seam donut row.

## Cost
- mesh suite (nextest, 4 vCPU): 143.0 s before, 150.0 s after; the two new rows are about 10.7 s of that.
- Tess-budget sweep (release, every tour scene): main 256 s and 255 s; branch 274 s and 258 s.

## §5 sweep
Shape grepped: a face routed by a match on its carriers' kinds, `Curve3::{Ellipse,Nurbs,Spiric}` or `Pcurve::Projected` in `mesh/src`.

| hit | disposition |
|---|---|
| `tessellate::has_ring`, `trimmed::has_trim_carrier`, `has_tilted_circle` | fixed (gone) |
| `memo.rs` `curve3_kind`, `Projected` | key encoding, not routing |
| `chords.rs` chord counts and speed bounds | sizing per carrier |
| `trimmed.rs` image-kind check | which images the lane reads |
| `sizing.rs` sphere/cone extent | per carrier |
| `chords::tighten` non-circle skip | reads the route |

The pattern cannot see a route spelled without a carrier variant. Second pass on `Lane::`, `lane_of` and the door calls: every lane read goes through `Routes`, and the door is called once, in `iso_rectangle_shape`. Beyond mesh, `topo::props` routes faces to quadrature by carrier kind (`trims`, `curved_rim`): that is a route by integrand, and its closed forms read any loop of iso carriers, so it is not this defect.

## Ratified pages
None re-worded. `[mesh.sizing]` names no route.

## Filed
- `work/tesslane/a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read.md`: the slit has 0 stored images and `face_lift` gives Gap, so whether the chart lane meshes a walk-refused rectangle is unmeasured.
- A note on `work/tesswalk/apex-crossing-branch-premise-has-no-body-level-witness.md` (the renamed bow tie row).
=====

4. RAN LOCALLY: mesh whole 480/480 (after merging main); mesh --features budget 477/477; geom-brep whole 2720/2721; topo whole 4281/4283. Changed rows at eps 1e-6 and 1e-12: mesh 62/62, geom-brep 52/52, topo 25/25. run-gates rc 0; doc-gate --workspace-pass ok; fmt; clippy -D warnings for mesh/geom-brep/topo --all-features, viewer --features app, demos/tour; tools job (k-lint, tess-lint, tess-meter clippy + tests green); tour suite (ci-tour) 106/106 armed.
Reds not mine:
- geom-brep curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance: random-seed fuzz, seed 0x55c8ff29bb93ec4a, reads no door.
- topo pcurves::lift_agreement_rows::no_file_outside_the_mint_pins_a_chart_branch: mesh/src/walk.rs counts 6 against the listed 2 with origin/main's own walk.rs; my walk.rs diff is comments only.
- topo::all recourse_roster::every_decided_name_has_its_door_and_every_site_is_read: boolean/sectors.rs, which this branch does not change.
Left to CI: sweep suite on the new code (the old-code probe run is the evidence), demos/wild, python, nightly eps rows.

5. Head a55cd5809e177408da8b7cfdc57e716c276ac635.

6. Slowdowns: the contended box (sweep probe 2.7 h, tour 65 min, two geom-brep runs overlapped once); hand-built witnesses carry no images, which made the chart lane's answer on them uninformative. Commits are authored by the environment's git identity "Claude" (no trailers, no session lines); the vendor name is therefore in the author field of my 7 commits, not rewritten.
