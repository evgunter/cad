---
id: pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted
kind: issue
title: PR 7131's frame cost at eps 1e-12 is larger than it counted: slow-set rows refuse past its margin range
status: closed
opened: 2026-11-22
priority: P1
cost: M
closed: 2026-11-23
pr: 7558
---


**What Ev accepted.** On PR 7131 (comment 7046735324, 2026-11-20 17:54Z) the cost at ε 1e-12 was given as *"about 11–12 rows that certify on main and refuse here … refusing at margins of 1.02–1.32e-12 against the 1e-12 band"*. Ev replied *"yep that's no problem"* at 17:56Z. The later correction to 13 rows, and the merge note's 10, were taken as covered without a reply. The count was taken on the `ci` profile, where the slow set runs at the default ε only (7131's body: "All … at the default ε unless noted"). So no slow-set row was counted at 1e-12.

**What it is.** Two slow rows of `crates/topo/tests/a_seamless_pipe_cut_by_an_oblique_plane_measures.rs` refuse at 1e-12 since merge `9bc9d6517b`; `e4d679d37e` is green (GREEN PR 7402, bisected):
- `a_curved_junction_certifies_across_the_departure_arm`;
- `a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval`.

Both fail at the same build. `pipe::<Interval>`'s roof ellipse stalls against the cylinder on `ssi_hull_sup`, in the two tilted frames at the pose whose floor is 100 m up the axis:
- `Stalled { operand: Cylinder, cause: Indeterminate { margin: Enclosure { lo: 0.0, hi: 2.193467629756736e-12 }, … predicate: Some("ssi_hull_sup") } }` along `(1, 2, 3)`;
- the same refusal at `hi: 1.4153678229453856e-12` in the far frame.

**2.193e-12 is 1.66× the top of the accepted range.** Under 7131's mutant patch:
- F1X alone builds both;
- A alone narrows them to 1.619e-12 and 1.079e-12, still refusing;
- F1F moves neither.

So the cause is bracket, and A also narrows it.

**Pinned against this row.** PR 7402's `FRAME_COST` pins both refusals at ε ≤ 1e-12, in the test file: windows, cause and mutants are there. The pin goes red when the poses build.

**More of the same window.** Nightly 44989234943 (dispatched on `green/nightly-build` at `1942290ead`, which is main `4d3402ecb1`'s kernel) is the first 1e-12 suite with the slow set since 7131 merged. Against the 11-21 nightly 44870651337 it has 19 new 1e-12 reds and none fixed. Several carry frame-cost-shaped margins (`a_ring_on_a_cone_face`, `verbs_germarms2_interval`, `torus_touch_off_faces`, `union_fold_face_face_touch`, `spheres_crossing_off_every_edge` at 3.277e-12). GREEN's U12 is attributing them, and adds the full count here.

## The full count (GREEN U12, 2026-11-23)

Nightly 44989234943 (main `4d3402ecb1`'s kernel) against the 11-21 nightly 44870651337 (main `f5ef8f2b`): **19 new reds at ε 1e-12, and 18 of them are PR 7131's: 17 frame-cost rows and a stale golden.** Every row below is green at `e4d679d37e` (9bc9's first parent) and red at `9bc9d6517b`, measured by one probe each on hosted CI: run 45083192761 at `e4d679d37e` and run 45083200534 at `9bc9d6517b` (throwaway branches, since deleted). The causes come from 7131's mutant patch applied to `4d3402ecb1` (run 45083184695). A row's cause is the mutant that, applied alone, turns it green, and all three together turn every row green. No row is jointly caused. Every row is in the slow set, so 7131's `ci`-profile count never ran it at 1e-12.

**Bracket: F1X alone builds the row, A alone does not (11 rows)**
- sweep `a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar`: `props_cone_ellipse_on_cone`, hi 1.094e-12.
- sweep `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar`: `pcurve_envelope`, hi 1.284e-12.
- sweep `spheres_crossing_off_every_edge::the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar`: `pcurve_envelope` at `EnvelopeTerm(Radius)`, hi 3.277e-12. This is a moved pin. The row already refused here (ONEHOME's `the-plain-sphere-witness-…`, since PR 6717), and it pinned `check: Envelope`.
- sweep, six mixed-chain rows: `band_closed_mixed_chain_shapes::{a_four_arc_barrel,rounded_polygons,turned_rounded_outlines}_fillet_whole`, `band_open_mixed_chain::an_l_plates_rounded_corner_run_fillets_whole_at_interval` and `band_open_mixed_chain_shapes::{an_s_run_of_a_convex_and_a_reflex_arc_builds,two_arcs_in_one_run_build}`.
  - Each now refuses `UnsupportedGeometry` ("a junction's section does not certify as the shared row of its two bands", `blend/surgery.rs`, where `analytic_shared_row` answers `None`). BAND's open row pins a later refusal, the `shared_row_touch` escalation.
  - The margin is not reported, because the refusal drops it.
  - These are moved pins: each row already refused at `Interval` at 1e-12.
- topo, the two seamless-pipe rows (above): hi 2.193e-12 and 1.415e-12.

**Polar: A alone builds the row, F1X alone does not (4 rows)**
- sweep `parallel_cylinder_join::a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door` and `…::a_tipped_rod_refuses_alike_in_both_operand_orders`: the tipped rod is not finished, `props_cylinder_circle_off_axis`, hi 1.194e-12. These are not PR 7112's. That row is red at 1e-6 in both nightlies.
- sweep `torus_touch_off_faces::the_off_face_touches_build_at_every_scale`: operand B is not finished, `props_cylinder_circle_off_axis`, hi 1.568e-12.
- editor-core `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line`: it now stops at back 300 (`props_cylinder_circle_off_axis`, hi 1.195e-12), before the back-900 stop it pins (NEARPOINT's row).

**`f64` squaring: F1F alone turns the row green (2 rows)**
- sweep `section_ellipse_seam_poses::planes_through_the_axis_nearly_along_it_split`: measured as 14 poses moved by F1F squaring, then **retired on merge**: ONEHOME's check-1 change (`onehome/check1-frame-half`) removed tier 3's frame check, so every through-axis pose builds at 1e-12 and the table is empty.
- geom-brep `curve_pair::pose::window::oracle::each_window_reading_holds_on_a_dense_distance`, seed `0x4071461b4c688a45`: draw 7, at `f64`, reads `Recedes` where the dense distance contradicts it. This is **a contradicted reading, not a refusal**. It is filed on CPAIR (`a-window-reading-at-f64-is-contradicted-by-the-dense-distance-since-7131s-squaring`).

**Not frame cost, but 7131's:** step-export `committed_fixtures_are_byte_golden`, red at every ε. 7131's merge committed `composed_die.step` from an earlier head, with 12 `DIRECTION` x components at `0.0`. The final head emits main's `-0.0`. PR 7361 (`b31e3986d6`) has since restored the 13 `-0.0` lines.

**Not 7131's (1 of the 19):** editor-core `split_pinch_keeps::the_kite_moats_…`, red at every ε and held by GREEN's `main-red-rows-two-lanes-found-red-…`. At the other ε, the two new fuzz reds are held by `germ/the-square-wall-fuzzer-…` and `splice/the-torus-rows-door-answer-panics-…`.

**Against the count Ev accepted** ("about 11–12 rows … margins of 1.02–1.32e-12"): the slow set adds **17 frame-cost rows** at 1e-12.
- 15 are refusals: 11 bracket and 4 polar.
- 2 are `f64` squaring. One is a pose-table move. The other is a contradicted reading, which is not a cost but a finding.
- Of the 9 margins reported, 5 sit in the accepted range: 1.094, 1.194 (twice), 1.195 and 1.284e-12.
- 4 lie past it: 1.415, 1.568, 2.193 and 3.277e-12. The worst is 2.5× the accepted top.
- The 6 band rows report no margin.
- 8 of the 15 refused at 1e-12 already and now refuse differently: 6 band rows, the plain witness and the far-off carrier.
- 7 are new refusals where main built: the ring on a cone, the meeting axes, 2 tipped rods, the torus touch and 2 seamless pipes.

**Reproduce:**

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p topo --ignore-default-filter \
      -E 'test(/a_seamless_pipe_cut_by_an_oblique_plane_measures::a_(curved_junction|seamless_pipe_cut_by_an_oblique_plane_encloses)/)'

**Done when** the counted cost is re-measured with the slow set at 1e-12 and put to Ev as the cost it is. Each row in it is then built at 1e-12 or re-pinned under that count.
(GREEN, U8)

## Closed: the cost stands, and each row pins it (GREEN U13, 2026-11-23, PR 7558)

**Ev's ruling, on PR 7505** (2026-11-23 06:20Z): *"if the "cost" is just the random tests that don't work anymore, then of course this is fine / if it breaks a demo or makes things meaningfully slower then that cost would be more worth weighing"*. The GREEN orchestrator answered on 7505 that the 17 rows are tests only, none of them a demo and none slower, and took the ruling as option 1: the full count stands.

**The pins.** PR 7402 pinned the two seamless-pipe rows. PR 7558 pins the other 15 rows of this count: 13 refusals, the pose table, and the contradicted reading. Each row now asserts at ε 1e-12 the refusal or reading it gives, with its cause and margin band in its doc, and goes red under the mutant named. Default ε and 1e-6 are unchanged. The values were measured on main `e38e85bebe` and after:
- sweep `a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar` (`RING_FRAME_COST`): every op's join refuses a minted edge's description, `carrier_on_surface_2` in (1.0e-12, 1.1e-12] (1.058e-12). Bracket; F1X builds every op. **Main moved this refusal since the count**: it was the result's volume, `props_cone_ellipse_on_cone` 1.094e-12.
- sweep `verbs_germarms2_interval::the_meeting_axes_pose_builds_at_the_certified_scalar` (`MEETING_AXES_FRAME_COST`): the direct union's pcurve pass, `pcurve_envelope` in (1.2e-12, 1.35e-12] (1.284e-12). The re-posed twin takes the row's own fixture escape (`carrier_matches_mapped_source`, 1.026e-12). Bracket; F1X.
- sweep `spheres_crossing_off_every_edge::the_plain_witness_builds_its_seam_crossing_at_the_interval_scalar`: a ∪ b and a ∖ b now build, and b ∖ a stops on its seam jet, `tangent_second_order` in (1.3e-12, 1.45e-12] (1.380e-12). Bracket; **F1X builds every op**, so the pcurve envelope ONEHOME's plain-sphere row names is not reached on main at all. **Main moved this stop since the count**: it was a ∪ b's `EnvelopeTerm(Radius)`, 3.277e-12.
- sweep, the six mixed-chain rows (`UNADMITTED_AT_THE_FRAME_COST`, 11 rims): each refuses `UnsupportedGeometry` (the section does not admit as the shared row) ahead of `shared_row_touch`, with no margin reported. The rectangles at 7° and 90° still escalate the touch. Bracket; F1X.
- sweep `parallel_cylinder_join`'s two tipped-rod rows (`TIPPED_ROD_FRAME_COST`): the four half-length-200 rods refuse their own finish, `props_cylinder_circle_off_axis` in (1.15e-12, 1.25e-12] (1.194e-12). The half-length-50 rods finish, and every op refuses `CurvedPierceUnsupported` as the rows assert. Polar; A.
- sweep `torus_touch_off_faces::the_off_face_touches_build_at_every_scale` (`ROD_FRAME_COST`): the ×250 rod refuses its own finish, `props_cylinder_circle_off_axis` in (1.5e-12, 1.65e-12] (1.568e-12), ahead of the extent scan ORBIT's row pins. Polar; A.
- editor-core `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line`: stops at back 300 (1.195e-12), where it stopped at back 900. Polar; A restores the back-900 stop.
- sweep `section_ellipse_seam_poses::planes_through_the_axis_nearly_along_it_split` (`THROUGH_1E12`): the measured table. 14 poses at θ 1e-6, 3e-6 and 3e-5 moved, where the count saw 6. F1F squaring; F1F.
- geom-brep `curve_pair::pose::window::oracle::a_window_reading_the_dense_distance_contradicts_is_pinned_red`: **a wrong answer, not a cost**, pinned red against CPAIR's `a-window-reading-at-f64-is-contradicted-by-the-dense-distance-since-7131s-squaring` with its whole failure text. The oracle's bands are its own, so the seed stops at every ε row, not only 1e-12. F1F turns the sweep green, and the pin red.

The per-row runs are in PR 7558's body.

**The ruling's premise, corrected for one row.** The reply on 7505 said all 17 rows are refusals at ε 1e-12 and none is at the default ε. The window oracle is neither: it is a wrong answer, at every ε including the default (and the fuzzer is red on main at default ε on random seeds). It is pinned as CPAIR's red, not as accepted cost. Ev's ruling covers the 16 refusals.

**Re-measured after ONEHOME's check-1 change (GREEN U13, 2026-11-23).** Main's PRs 7323/7349 (`onehome/check1-frame-half`) charge a frame's `f64` rounding where it is levered. On main after them, these 1e-12 rows read (probe runs 45244053101 and 45254555300), so main's own 1e-12 leg is presumably red on them until PR 7558 lands:
- the tipped rods: 1.307e-12, re-pinned to (1.25, 1.35]e-12;
- the torus rod: 1.738e-12, re-pinned to (1.65, 1.8]e-12;
- the far-off carrier at back 300, every order: 1.536e-12, re-pinned to (1.45, 1.6]e-12.

All three are still polar: mutant A finishes or builds each one.

Seven band rims are back on BAND's own touch escalation, and four stay unadmitted. The section-ellipse table is retired: every pose builds.

PR 7131's own pin `pcurve_cache::ruling_reading_rows::a_true_ruling_keeps_the_closed_form_on_both_nappes` (both jointly) read 1.112e-12 against (1.0, 1.1]e-12. It is re-pinned to (1.05, 1.2]e-12.
