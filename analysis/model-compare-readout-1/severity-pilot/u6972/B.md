# Review r1 — PR 6972 at frozen head 032ac185 (merge base b7b4f78f)

## Verdict: NOT-MERGEABLE-AS-IS

The change itself is right where I could attack it: every stored plane, direction and carrier agrees with the merge base to ~1e-15 across 59 f64 builds, the mesh winding agrees with every stored outward normal, and over a box nothing I built stores less than the truth. What blocks is around it: two committed rows are red on this head and green on the base (MAJOR 1), and the gate the body leans on did not run them, the slow set, or a render on this head (MAJOR 2). Fix those and it is an APPROVE-WITH-FIXES.

## MAJOR

**M1. Two committed step-export rows are red on the head, green on the base. DEMONSTRATED, sure.**
`crates/step-export/tests/export.rs` `committed_fixtures_are_byte_golden` and `kernel_sidecars.rs` `kernel_sidecar_fields_match_live_kernel`. Base: `cargo test -p step-export --test all` 59 passed. Head: 57 passed, 2 failed.
- `boss_union.step drifted`: `#1 = CARTESIAN_POINT` was `(2.0, 2.0, 1.0)`, is `(0.0, 0.0, 1.0)`. The exported plane placements moved with the cap origin.
- `swept_elbow: want KERNEL_VOLUME_PAD_MM3=1.7046561015447784E-5, sidecar has 1.6986412276764902E-5`.
- The PR touches nothing under `crates/step-export` and its body never mentions STEP. Both rows are slow-set entries (`.config/nextest.toml:223`), which the per-PR gate skips unless the diff names them.
- Claim 8's STEP question, answered: placements do move in the file; STEP semantics do not (the plane is the same), and import does not read the location as a face point (by inspection). The fixtures and sidecar need regenerating, and the body should say exported files change.

**M2. The gate does not cover what the body says it covers. Verified from job logs, sure.**
- One run is on 032ac185 (CI 43669842805, success). Its slow-set step ran **4 tests** (1 kernel, 3 app), at default ε only: ci.yml runs only "the slow-set tests this diff adds or edits". The body's "the slow set was not re-run locally after the last merge; hosted CI runs it" is false; about 2180 slow-set entries ran nowhere on this head.
- I ran the sweep and step-export slow set on the head (806 tests, default ε): 802 pass, 4 fail. Two are M1. The other two (`split_edge_loft_charts::a_split_on_a_spline_chart_carries_the_parents_image`, `m8_4_intersection_iso::a_boundary_column_intersection_mints_its_iso_image`) are **red on the merge base too**: inherited from main, not this PR (file on the chart-ends ground). The editor-core, viewer and mesh slow sets I did not run; they are unrun on this head.
- ε rows 1e-6 and 1e-12 ran the fast set of the EPS_EXTRA crates only (5279 and 3517 passed). The PR's own slow row (`an_arc_wall_whose_radial_turns…`) ran once, at default ε.
- **No render ran on this head.** The body's cited run 43658737118 is on `68c5ce7e` and concluded **failure** (two lanes died in matplotlib under Python 3.14; check text "partial render: re-render before merging"). The head is two commits past the re-baseline, one of them a main merge.
- Skipped whole jobs under the green gate: topo-release, step import, tools, interval.
- Fast set, Python (959 OK), demos (97 of 97, with `tour_runs_green_at_eps_1e_12` and the certified-cells row), viewer (1554) and lint did run and pass.

**M3. A shipped selector answers differently, re-baselined in a test and filed nowhere. Mechanism sure; severity likely.**
`GeomPred::PlaneDistance` measures a face at its carrier origin (`crates/editor-core/src/names/geompred.rs:145-155`, `interrogate.rs:374-381`). A wall's origin is now its segment's start vertex and a cap's is the sketch origin, so on a 1 m cube "faces more than 0.9 m behind the top" went from the floor to the floor plus four walls (`crates/pncad-py/tests/test_document.py:1028-1033` now asserts 5). A wall's answer depends on which end its segment starts at and which way the sweep runs. The predicate's doc already says the point "need not lie on the entity", so this is doc-consistent, but it is a user-visible change decided on something contingent (`[intrinsic.contingent]`), no `work/` row names it, and the body presents it as a test adaptation. It has a design implication and wants Ev's eye or a filed row before merge. `test_selectors.py:93-96` still says "the walls' carrier origins are on their centroids".

## MINOR

1. **The vector arm keeps the unit's own defect, undisclosed.** DEMONSTRATED, sure. `crates/sweep/src/extrude.rs` `Course::wall_normal` (Vector arm: `place.direction(d).cross(dir).normalize()`, `dir = v.normalize()`). `Extrusion::Vector((0, 0, [0.95, 1.05]))` at `Sym<Interval>`: caps exact, all four walls store normals of width 0.403, length `[0.819, 1.222]` — the number the unit quotes for the old walls, unchanged from the base. No row pins it, the body does not say it, no `work/` row files it (`work/strut/extrude-arc-walls-are-ruled-in-n-not-w.md` is about arc walls). No document path uses `Vector` (one tour call), so the reach is the public door.
2. **A sketch origin far from its profile now costs the carriers their vertices.** DEMONSTRATED, sure. `swept::placed_segment_spec_about` (line arm) and `cap_plane`. A line rim now ends at `q_from + ‖b−a‖·place(d)` instead of at the placed far vertex, so it misses by the placement's rounding, u·|sketch coordinate|. Measured on a turned frame with the body near the world origin: origin 1.4e3 away, edge end residual 0 → 1.6e-13 and plane/vertex residual 3e-15 → 3.4e-13; 1.4e4, → 1.5e-12; 1.4e6, → 1.4e-10. At ε = 1e-12 the cell at 4.2e3 built on the base and refuses on the head (`AnalyticRung3(KernelLimit { operand: Plane, bound: 1.8e-12 …})`). Inside the 1e3 m session box nothing flips; a third of ε is spent where none was. Same mechanism as the 1e8 m mesh probes' new `ResidualExceeded`. Not filed for the in-box case.
3. **The ring row's stated cause is contradicted by the mesher.** BY-INSPECTION, likely. `crates/viewer/tests/index_memo.rs:1069-1074` and `work/vgeom/the-wide-candidate-row-…md` say a planar face "is triangulated in its chart, whose origin is now the sketch frame's". `crates/mesh/src/planar.rs:42-48,182-184` says the stored plane is not read: the chart is re-derived from boundary points. So the mesh moved for another reason (ulp moves in boundary chords flipping a near-degenerate diagonal is my guess, untraced), and the row's closing condition is aimed at a mechanism that is not there. Pinning rather than re-aiming is right, and `pinned_red::row` is a real pin (it passed at all three ε in CI). Chart origin far from the face is therefore not a triangulation hazard.
4. **`revolve_washer`'s theorem lost its row.** BY-INSPECTION, likely. The deleted row pinned what the registration bought at `Sym`. Its replacement claim (a latitude through its point is a plain-form identity; "door open and shut read 497/457") lives only in prose (`work/decide/revolve-carriers-state-only-the-rim.md`, the allowlist script). The new rows assert an exact unit `u_ref` and a build; a build that decides the endpoint numerically passes them. Every new revolve row is on `SketchPlane::xy()`.
5. **Refusal pins deleted without replacement.** BY-INSPECTION, sure on the facts. `crates/editor-core/tests/m10_3_driver_interval.rs` `the_certification_width_is_no_longer_bounded_by_epsilon`: the two-sided band, the "just past certifies after bisection" assertion and both refusal-shape pins went; one inequality remains and nothing names what stops the leaf at the new ceiling ("the sliver band" is a comment). `sym_9_the_drive_writes_the_ladders_receipt` now cannot pin a non-zero `retried=` (filed, with stale numbers, below). `docm2`'s ladder now refuses on `bool_faces_parallel` where it refused on `plane_eq_offset`, with no reason given.
6. **Rewritten rows that pin less than they say.** BY-INSPECTION. `crates/viewer/tests/mate_tool_flow.rs:509-514`: the author is right that the old comparison could not tell copy 1 from the master, but the new one reads the picked copy's own face and would hold if the mate had read the master; the discriminating assertion is at `:473-480` (sure). `mate7a_r1_probes.rs:233-237`, `mate7a_torus_rest.rs:438-458`: "cap centred here" became "some plane holds this point" (likely). `crates/mesh/tests/newell_probes.rs:193-199`, `r2_cert9_probes.rs:267-273`: the assertion became a disjunction while the message still says "the session box refuses it" (sure). `shell_cavity_embedded.rs:226` accepts any `CavityCrosses`. `sym11` cells check `Ok` only. `torax_axial` bound loosened 10× (measured 11× up).
7. **The loft struts and the hole bridge still do what the ratified sentence forbids.** BY-INSPECTION, sure. `crates/sweep/src/loft.rs:1326,1344` (`EdgeCurveSpec::line_between`), `:1513` and `swept.rs:911` (`mev_line`) divide a difference of two placed vertices by its length. They are scaffolds restated or killed before rest, and no frame holds a loft strut's direction, so not fixing them is defensible; but `crates/sweep/README.md:945-947` is unqualified, the body lists only "loft struts" under looked-at, and the band row files blend scaffolds of the same kind. The README needs an "at rest" (Ev's) or a row is owed.
8. **Tracker rows left false.** BY-INSPECTION, sure. `work/lathe/an-extrude-cap-plane-normal-reads-the-profile-variables.md` is closed in fact by this PR and still `open`, untouched. `work/onehome/three-path-sweeps-refuse-at-1e-12-…md` still says two rows are pinned red; this PR lifts both pins. `work/pcert/the-cylinder-rim-axial-decision-…md` likewise. `work/sym/the-ladders-receipt-row-…md` says `[1400, 1762, 0]`; the test pins `[1412, 1780, 0]`. `work/onehome/an-arc-on-a-turned-plane-…md` cites `certify.rs:4907-4920`; the pins are at 4926-4940, and its reproducer lives only on the unmerged probe branch. `m10_9_pins_interval.rs:266-270` quotes stale base counts.
9. **Body statements about PR 6953 that are not true.** DEMONSTRATED (`git grep`, `git diff`, `git merge-tree`), sure. `register_placed_line_end` exists nowhere (6953 removed it); the mesh probe edits are not byte for byte (messages differ); the two heads conflict in 7 files either way (`swept.rs` ×3 hunks, both mesh probes, four editor-core ledgers) and two more need hand work (6953's new `swept.rs` rows call the six-argument form; 6953's `q_to` row merges in open).
10. **Stale prose (Q4/Q5).** Sure. `crates/sweep/src/lib.rs:14,131-135` still describes caps as `geom_brep::newell_plane` over vertices and apexes with a flip; `revolve/mod.rs:747,759` "the `CapPlane` posture" (extrude's copies were fixed, revolve's were not: a half-fix); `:1134` "Newell failures"; `revolve/partial.rs:9`; `revolve/tube.rs:472`; `topo/src/geometry.rs:24`, `euler.rs:283`.

## NOTE

- The author's containment rows are thin at ε = 1e-12: `ROUNDING` is 1e-14 and `vertex_box()` is 1.56e-14, so a midpoint-only enclosure escapes by 5e-15. The macroscopic turned-frame row is the one that bites.
- `revolve::partial::section_centre`: for anything but a one-segment circle it is the unweighted mean of collapsed vertices and apexes, the same value the base reported (my probe: `revolved_caps` equal to 1e-9 on 13 partials). It is not intrinsic to the section: a circle authored as two unequal arcs would not report its centre (by inspection; I did not build one). The only consumer is the tour's lily, on a full circle.
- Loft with a tilted top: mesh volume differs by 3.3e-7 relative between base and head with every stored surface and carrier equal to 1e-9. Unexplained; I take it for mesh sensitivity to ulps.
- The measure lever (`editor-core/src/eval/measure.rs:613-662`) and `topo/src/census.rs:4045` grow with |origin − face|; sound, looser (by inspection).

## Claims

1. **Held, DEMONSTRATED.** 59 builds (hole loops, clockwise authoring, Against, ±vector, lens, major-arc D, circles, tilted frame, lofts on shifted/tilted planes, partial revolves ±, near-full, axis-touching, sphere, skew axis): plane equations, `u_ref`s, cylinders and edge carriers equal the base's to ≤2e-15; mesh winding agrees with stored outward on every plane and cylinder triangle; refusal set identical at three ε (bar MINOR 2). Negative depth, in-plane vector, reversed or opposite-facing loft top refuse as before.
2. **Held at f64; broken over a box on the vector arm (MINOR 1).** Wall origin is the door's own placed value, not a read-back (by inspection). STRUT's row untouched.
3. **Held, DEMONSTRATED** on my rows: boxed frame under an extrude and a revolve, boxed loft tilt and height, boxed revolve angle (leaf), hole loop, Against — zero escape with five members including both ends; truth-spread guards real. Rows the author lacks: revolve angle, loft placement, vector, any revolve off `xy()`.
4. **Held.** The one base `CapPlane` refusal I reached (Interval, revolve on a boxed-turned frame) now refuses at `carrier_endpoint_start`. Nothing a base refusal stopped now builds. `register_rim_identity`: build parity only; symbolic split not measured (MINOR 4).
5. **Mostly held by inspection, not run.** No flip reads as a wrong answer; `frozen` 643→14 is work no longer needed (decisions 639 and shapes unchanged, two moved numeric→symbolic). Coincidence door is 21→28 of 42 (the row only counts; a proof there is a symbolic zero of a margin the kernel already decided). Not independently checked: the 28 joints, `sym11`'s built bodies, the impeller's 8 points.
6. **Partly broken** (MINOR 4, 5, 6). 7. **Disposition held, cause broken** (MINOR 3). 8. **Broken at STEP fixtures (M1) and the selector (M3)**; mesher, mate solve, pcurves fine. 9. **Held**: `newell_plane` has no non-test caller; doors clean bar MINOR 1 and 7; blend sites the band row omits: `surgery.rs:4915,4536,4596`, `junction.rs:720`, `open/end_face.rs:1262`, `open/planar.rs:1187,1343,1763` (likely). 10. **Held except** "a direction the sketch holds": `swept::sketch_direction` divides a sketch difference by its length although this head's profile records directions (scheduled: `work/boxfinish/the-sweep-doors-read-the-profiles-recorded-directions.md`), and MINOR 7.
11. **Held where run** (probe branch merged into the head, switch off): slab ±0.05 one leaf certified, 816/1207 (body: 812/1245); width slab one leaf; derived-frame twin row passes and the freeze row trips `frozen` 210 as the body says; `m10_8` plain 2448/3115/108 (body 2412/3150/108); turned-plane door refuses `[0, 3.37e-4]` and `[1.19e-18, 3.37e-4]` exactly. Not done: current main merged in, the tour's stop, `AIM=mid`.
12. **Held on my battery**: 25 box cases × 2 lanes, no build→refuse against the base. Documents and studies not run beyond CI's fast set.
13. **Broken as described** (MINOR 9); the two helpers are the same act, and only one exists here. 14. **Not established**: the sheet draws no planar chart at all, 54 changed lines are captions in 25 cells, plots pixel-identical; five gaps that were exactly 0 are now nonzero (body mentions only the one that became 0). 15. **Broken** (M2). 16. **Partly** (MINOR 7, 8).

## End-to-end

It solves the right problem: over a depth box the head stores exact caps and walls where the base stored width 0.4, and the slab certifies in one leaf with the switch off. Missing in practice: the vector door, STEP fixtures, a statement that exported files and one selector's answers change, and any row with a profile away from its sketch origin.

## Style

Exercised Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 on `extrude.rs`'s header and the whole diff of `swept.rs`, not either file end to end.
- Q1 `swept.rs:562-567`: `sketch_direction` re-derives a direction the profile on this head already records. The fix for "re-derived from what it built" re-derives one level down. Scheduled. sure.
- Q1 `swept.rs:740-743`, `extrude.rs:1417-1421`: the "decided once" direction is computed again at the wall, the near rim and the far rim, and the line's length a second time beside it. Same function, so same value; held by convention. likely.
- Q1 `extrude.rs:789-845`: `extrusion` is matched twice, once for `(w, course)` and again for `length`. sure.
- Q1 `revolve/mod.rs:532-544`: a `Pose` whose origin comes from `joints` and whose axis comes from the face; two sources for one frame. unsure.
- Q1 `Placing::point` turns by Rodrigues, `Placing::carry` by the anchored motion: two spellings of one turn. unsure.
- Q2 `swept.rs:147-150`, `extrude.rs:665-680`: "a rigid frame carries a unit direction to a unit direction" is a comment; `sketch_direction`, `Course::dir`, `wall_normal` and `cap_plane` pass bare `Vec2`/`Vec3` though `UnitVec3` exists and is unwrapped at `extrude.rs:786`. sure.
- Q2 `revolve/mod.rs:532`: `unreachable!` guards `joints: Option` that only `Partial` sets; the enum could hold it. likely.
- Q3: NOTE on the 1e-12 containment slack; the `seen > 0` latitude guard is fine. likely.
- Q4: MINOR 8 and 10. sure.
- Q6 unscheduled: the survey's "not re-verified site by site"; "about 230 readers out of scope"; `torus_radial`, enum-carried directions and description payloads from the blind spot; the slow set. sure.
- Q7: I would not have reimplemented the old centroid as `section_centre` to keep a reported value alive; a circle's centre or a refusal says more. unsure.

## Rubric

- Idiom and structure: 4. `Course`/`Sweep` and the frame-only `cap_plane` are clean and delete more than they add.
- Test quality: 3. Good exact rows; varying rows thin, door coverage narrow, several rewritten rows weakened.
- Doc and comment honesty: 2. Crate docs stale, one checked-nowhere cause enshrined twice, a body that overstates the gate.

## Spec deviations

Reported: vector strut kept; ring row pinned; cap origin ruling A; `sketch_direction` from points. SILENT: STEP output changes with red fixtures; the vector walls' width; the selector change; the far-origin conditioning inside the box; the failed render.

## Executed

Branch `boxfinish/review-sweep-frames-r1`, commit 0d1d09f9 (`crates/sweep/tests/r1_frames_probe.rs`, `r1_box_probe.rs`, outputs and this report under `probe/review-sweep-frames-r1/`). Ran: the f64 differential on head and base at ε 1e-6/1e-9/1e-12; the box probe on both; step-export suite on both; sweep + step-export slow set on head, two rows on base; five switch-off rows. No mutants: a wrong sign fails the kernel's own finish everywhere, so they would add nothing unique. Read only: claims 5, 6, 13, 16 and the origin-reader survey (parallel read-only passes, spot-checked by me at the mesher, the selector and the tracker rows).

## Disclosures

No glimpse of another review lane. My first commit (e2b62556) carries the container's default committer identity, which names the tool; the later ones use a neutral one. I did not rewrite it (merge-only).
