REVIEW r2 — PR 6972 at frozen head 032ac18556cc5ad7b6e836bfd3b2a5d78d5f79d9 (merge base b7b4f78f70)

## Verdict: NOT-MERGEABLE-AS-IS
The geometry is right: the f64 differential below shows every stored direction equal to main's fitted one. But four committed rows are red on the frozen head and green on the merge base, at the ε rows named below. The PR gate never ran them: they are slow rows in files the diff edits, and the diff's added lines don't spell their names. The fixes are small (re-baselines and one pin), but the remaining ~2,100 slow-set rows have also not run at this head.

## MAJOR
M1. Four rows red on head, green on base. DEMONSTRATED (run on both trees, CI profile; sure).
- `editor-core/tests/seat8_split_lowering.rs::the_split_documents_evaluate_to_their_committed_digests`, every ε: cut_cylinder 0xd9783279a614f38c (want 0x5e08…8844), part_select 0x2e459fdbee12d4a2 (want 0x4bd5…1fd2), kitchen_sink 0x6ed151f71759df7d (want 0xa3b0…f963). The PR re-cut the sibling `a_split_with_an_empty_side…` because "the body's stored planes moved", and missed this one, which hashes the same planes.
- `viewer/tests/index_memo.rs::a_wide_candidates_interval_reaching_the_aimed_vertex_refuses_with_both`, every ε: panics "the near-coplanar candidate still answers short of the vertex" (index_memo.rs:1317). This is the "other side of the same line" as the row the PR pinned red, with the same cause the VGEOM filing measured. It was neither pinned nor named in the filing.
- `editor-core/tests/m10_5_r1_probes_interval.rs::e2e_channel_slider_over_an_epsilon_box`, at 1e-6 and 1e-12 (green at default): receipt (10,1282,1272,572) at 1e-6 and (10,1258,1248,526) at 1e-12, against the pinned (10,1264,1254,532). The PR re-cut only the L-plate row in this file.
- `editor-core/tests/coincidence_door.rs::the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows`, at 1e-12: `shaft_and_bearing_ball` is pinned to escalate at 1e-12 and now builds Ok. It certifies more; the pin is stale. This is not a regression.
- Main's last nightly (run 43561237293) lists none of the four among its failures.

M2. The gate's slow-set coverage of this diff is 4 rows: 1 in the kernel job (the new bulge row) and 3 in the app job (the seat row, sym_9 kept-atom, coincidence two-thirds). The slow-set step selects by identifiers on added lines (BASE 27e48f61). I ran the 89 slow rows in the touched test modules at default, 1e-6 and 1e-12. The author's local runs were `--profile ci`, which excludes the slow set, so "Inherited reds: none" and the 13,988-row ε counts say nothing about the slow set. DEMONSTRATED; sure.
CI-ground note for you to file, not this unit's fence: a re-baseline in an edited file does not run that file's other slow rows per PR, even though `implementer-discipline` §2 says the gate runs "the slow tests your diff adds or edits". Likely.

## MINOR
m1. The PR body misstates the coincidence door: "`two_blocks…` and `ring_serving…` now prove with recourse `OneConstruction` instead of `Budget`". In the diff they stay `unproven/Numeric/OneConstruction`, with the recourse changed. The one new proof is the at-rest row, `SameOpposite CensusAtRest CarrierVerdict`, which is geometrically true (the union's base sits on the floor top). The brief repeats the misstatement. BY-INSPECTION (coincidence_door.rs diff); sure.

m2. The README sentence "No sweep … divides the difference of two of them by its own length" is false on this head. Loft struts are `EdgeCurveSpec::line_between(far, near)` / `(upper, lower)` (loft.rs:1326, 1344) and `mev_line` (loft.rs:1513), and `line_between` is `(p1 − p0)/‖p1 − p0‖` (geom-brep certify.rs:1963-1977). "A wall, a rim and a strut each take the frame turning a direction the sketch holds" is false for the vector arm: the wall normal is `place(d)×(v/‖v‖)` normalized (extrude.rs:694, 831), and the strut is `v/‖v‖`. The loft struts appear only in the body's "looked at, not filed", and no `work/` file schedules them. BY-INSPECTION; sure.

m3. `revolve_washer`'s registration row was removed, and its replacement (`a_full_revolves_latitudes_over_a_width_box_are_exact`) pins `u_ref` exactness, not that the identity the registration discharged is still a theorem. I measured it on the washer fixture under the shipped rules:
- full revolve: head sym 501 / num 461 / frozen 24; base 501 / 461 / 36;
- partial revolve: head 493 / 611 / 227; base 501 / 613 / 221.
`numeric` did not rise, so nothing visibly fell to numeric, but no row pins this. DEMONSTRATED for one fixture; likely adequate.

m4. A semantic move for users that the PR treats as a test re-baseline. `GeomPred::PlaneDistance` reads a face at its carrier origin (`names/interrogate.rs:381`, `entity_point`). A line wall's origin moved from its quad centroid (mid-height) to its start vertex (the floor), so "faces 0.5 above the floor" no longer finds the walls, and "1 m behind the top" now returns floor plus walls (`test_document`'s rewrite). The pose doors use the normal and offset only (`eval/wire/pose.rs:666-674`), so mates are unaffected. BY-INSPECTION; likely.

## NOTE
n1. Stale prose that names deleted machinery: sweep `lib.rs:14` ("Newell-certified cap planes") and `lib.rs:131-135` (the whole Caps bullet); `revolve/partial.rs:9`; `revolve/mod.rs:1134` ("Newell failures"); `revolve/tube.rs:472` (cites "the cap-plane Newell fit" as an observed downstream refusal; it no longer exists). Sure.
n2. PR 6953: a trial merge (`git merge-tree`) conflicts in swept.rs, the four editor-core pin files and both mesh probes. Nothing is silent. The mesh probe copies are no longer byte-identical: 6953 moved at 03:12, and only the assertion messages differ. After either merge, `Placing::vector` (6953) and `Placing::direction` (here) are one act under two names unless the merger picks one. Sure.
n3. The ring row's mesh flipped about the vertex (1.48 − 0.031 → 1.48 + 0.031) under a pure translation of the chart origin. `u_ref` is unchanged (my differential), so the planar triangulator is translation-sensitive. The VGEOM row half-says this ("a fixture … would not move with a chart's origin"). Unsure whether that is a mesher finding of its own.

## Per claim
1. HELD, DEMONSTRATED at f64. A merge-base differential over 33 sweeps compared every planar face's outward normal (chart normal times sense) and `u_ref`, and every cylinder axis and `u_ref`: all equal to the base's Newell-fitted values within 1e-12. Only plane origins differ. The battery: CCW/CW rects, D-shapes with an arc, holed plates, along/against depth, on xy and on a sketch frame turned 1.1 rad about (0.3, −0.7, 0.648) and offset; vector up/down/tilted; lofts to an offset top and a turned top; partial revolves at ±90° and ±(2π − 1e-3), off-axis, touching the axis, CW on the turned frame; full revolve. Every vertex lies on its plane within 5.6e-16. Not built: a loft top facing opposite (unchecked); a vector nearly in-plane (refused by the direction gate, as on base); a profile 1e6 from its sketch origin (refused `OutsideSessionBox` on both trees).
2. HELD at f64 (above). The wall origin `qs[j]` is construction datum: `world(s.chord.a)` at extrude.rs:883-887, not a stored vertex read back. The vector-arm wall normal is a computed vector divided by its own length (a cross product of two unit vectors, not placed points). Over a boxed vector (my probe, z ∈ [1.95, 2.05] and its negative) the body REFUSES at check EndpointEnd (the strut), [0, 0.100] on head and [0, 0.156] on base, so there is no stored datum to measure and no row can host one. Not a regression.
3. PARTLY. The `spread > vertex_box()` guard is real. Every row is one-parameter, so the 5 members include both ends (the box's corners). No upper bound on enclosure width, so a degraded enclosure passes (Q3). No row for: a boxed revolve angle (far station), a boxed loft placement, a two-parameter box. The one I built (boxed vector) refuses on both trees (above).
4. HELD. On base the refusals were reached only by rows that construct the error and render it (`m5_s10…every_new_cap_plane_refusal…`, swept.rs unit test). No fuzz row, corpus row or validated input reached them. A degenerate cap, a zero-length side and a self-touching loop are refused upstream by profile validation (inspection, not run). `register_rim_identity`: m3.
5. PARTLY. Coincidence door: m1. Thread-count digest: decisions 639/643 and sym/num unchanged (440→442 sym), with only `frozen` falling. The same work is done; fewer forms freeze. By inspection; likely benign. The other re-cut counts I did not re-derive individually. The 89 slow rows I ran found m10_5_r1's ε dependence (M1).
6. PARTLY. `mate_tool_flow`: the author's account is plausible (the rest fixes only the plane, so `|underside.x − cap.x| < SHELF_LENGTH` tested authored x). The new assertion pins the read cap through the new origin convention, an implementation detail; likely acceptable. I did not read the history. `m10_bulge`: only two decision columns were removed (the deleted predicates); fine. `m5_s10`: the removed row rendered a deleted type; fine. Washer: m3.
7. HELD for the pinned row: the disposition is reasonable and the filing is concrete. BROKEN for its unpinned sibling (M1).
8. PARTLY. `revolved_caps` now reports `joints` (`section_centre`, partial.rs:589-611): the vertex-plus-apex mean, which reproduces the old Newell origin at f64, but it is contingent on loop cuts, the very reason given for rejecting (B) (Style). `PlaneDistance`: m4. STEP writes the plane's axis2 placement at the new origin (writer.rs:793-796), so the placement moves and the plane is the same. Import round-trip not run; the step-import (freecad) job was skipped by the filter. The mesher: n3. Viewer snaps read normal and offset only.
9. My sweep, by constructor and by `normalize()`, `line_between`, `norm()` and `distance(` over extrude/loft/revolve/swept/skin/tube, found what the author's hit list lacks: the loft struts (m2), the vector-arm wall normal (m2), and `sketch_direction`'s two normalizes (disclosed). `perp_unit`, `torus_radial`, `reject_from` and `Frame::radial` do not occur in those files.
10. Cap sentences HELD: normal ± frame normal signed by winding, origin the frame origin carried (top_place / far / each section's place), `u_ref` the first axis (swept.rs:805-816). The no-divide and frame-turned-direction sentences: BROKEN, m2.
11. NOT CHECKED. The probe branch needs a third ~17-minute editor-core build, and the disk filled once already.
12. No regression found among what I ran. The one flip I saw is a gain (coaxial stage6 at 1e-12). I did not sweep the shipped studies at `Interval`.
13. See n2.
14. HELD by diff text: 0 `<path>` lines changed in montage-uv.svg; only closure-gap and `-0.00000` annotations moved. I read the diff, not the image.
15. Gate run 43669842805 on 032ac18: `gate ok` green. Kernel: default ε, plus the 1e-6 and 1e-12 eps_extra rows (ci profile). App: 4758 at default, 3517 at each ε row. Slow set: 1 + 3 rows. Python: `Ran 959 tests … OK`. Demos: 97 passed, including `eps_regression`. `interval`, `step import (freecad)` and `tools` were skipped by the filter. Render: not run on this head (the body cites run 43658737118 on 68c5ce7). See M2.
16. Checked the VGEOM row: it says what it measured. `two_blocks`, `blend`, `exch` and `honejoin` not re-checked. The "looked at, not filed" loft-strut entry contradicts the README (m2).

## End-to-end
The change solves the right problem. The frame-built caps and walls are bit-for-bit the right directions at f64, and over a depth box the exact rows hold. What is missing in practice: the vector arm cannot be boxed at all (it refuses at the strut endpoint on both trees); the loft keeps the very construction the README now forbids; and a face's "where is it" point moved under a public selector without being called a behaviour change (m4).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6. Not exercised: Q7 beyond the notes below, and Q8 (no end-to-end file read).
- Q1 (likely): `sketch_direction` is "decided once" in its doc (swept.rs:555-561) but computed at every site: the bottom rim, the top rim and the wall (extrude.rs:1415 and via `placed_segment_spec`), and per loft section. One node only because Sym hashes content; the prose does the structure's work.
- Q1 (likely): `section_centre` (partial.rs:589) re-implements the deleted `cap_points` + Newell-centroid as a vertex mean. It is a value derived from the cut, the shape the lane rejected ruling (B) for. `joints` sits beside caps that state their own origin.
- Q1 (unsure): `Course::wall_normal` spells one rule (d × dir) two ways, right normal on the depth arm and cross-then-normalize on the vector arm (extrude.rs:684-696); only the doc says they agree.
- Q2 (sure): "a rigid frame carries a unit direction to a unit direction" (swept.rs:148-151) is enforced by nothing. `Placing::direction` returns `Vec3`, not `UnitVec3`, and carriers take it as unit.
- Q2 (likely): the doc of `register_placed_carrier_end` (swept.rs:640-664) now reasons about `u_ref` coming from `sketch_direction` through `place`, but `placed_segment_spec_about` passes a separate `radial` placing (full.rs:889). The proof's text covers `radial == place` only.
- Q3 (sure): `assert_holds_its_members` gets easier as the enclosure widens. No row goes red when a varying datum's enclosure degrades.
- Q3 (likely): the washer replacement row (m3).
- Q4 (sure): the stale prose in n1. tube.rs:472 is the "doc rotted, code right" case.
- Q5 (sure): the sweep crate doc's Caps bullet promises a Newell plane flipped by winding.
- Q6 (sure): the vector-arm geometry kept and the loft struts both owe a schedule; neither has a `work/` file (strut's `extrude-arc-walls…` is a different question). "The survey's classification, not re-verified site by site" has no schedule.
- Q7 (unsure): `Revolved.joints: Option<…>` with an `unreachable!` on None for a partial (mod.rs:532). A kind-carried field would make it unrepresentable.

## Rubric
- Idiom and structure: 4. `Placing` helpers and `Course` are clean; `Option` joints and prose-only "decided once" cost a point.
- Test quality: 2. The new rows pin real exactness, but containment is monotone, and four edited files' rows went red unrun.
- Doc and comment honesty: 3. Stale Newell prose; the coincidence-door body text overstates.

## Spec deviations
Reported by the author: the vector arm kept; the 6953 overlap; sketch_direction as the interim seam; the pinned ring row.
SILENT: the loft struts violate the ratified README sentence; the four red rows; the PlaneDistance and selector meaning shift (presented as a test fix).

## Executed (branch boxfinish/review-sweep-frames-r2 @ 888ab3e0, crates/sweep/tests/r2_sweep_frames_probe.rs)
- The face-dump differential (33 sweeps, head vs merge base), the washer counts, the boxed vector extrusion.
- 89 slow rows of the touched modules at default, 1e-6 and 1e-12 on head; the 4 reds re-run on base at all three ε (all green).
- Gate logs read at step level: change filter, kernel, app, python, demos.
- Read only: everything else.

No glimpse of another review lane.
