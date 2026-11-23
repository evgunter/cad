---
id: pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted
kind: issue
title: PR 7131's frame cost at eps 1e-12 is larger than it counted: slow-set rows refuse past its margin range
status: open
opened: 2026-11-22
priority: P1
cost: M
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

**Pinned against this row, until the bracket fix.** PR 7402's `FRAME_COST` pinned both refusals at ε ≤ 1e-12 in the test file. Both poses build since the bracket fix (below), and the pin is gone.

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
- sweep `section_ellipse_seam_poses::planes_through_the_axis_nearly_along_it_split`: the at-rest pose table moved at θ 1e-6. Four poses at φ 0.4 and 2.5 now refuse `URef Tilt`, and two at φ 3.9 that refused `URef Length` now pass (RESTFRONT's frame-check row).
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

## The bracket cause, fixed (ONEHOME, 2026-11-23)

`RigidMotion::squared` on bracket directions at `Interval` now takes the squaring's centered form (`Real::square_frame`): the squaring of a point of the box plus its derivatives over the box, met with the plain evaluation. It encloses the frame every point of the box squares to and is never wider than the plain one. Over brackets of half-widths 1e-12 to 1e-6 the plain frame's widest entry is a median 4.4 times the centered one's.

At ε 1e-12, on the bracket fix's head, the 11 bracket rows stand as follows:
- **Building where main built:** the ring on a cone, the meeting axes, and the two seamless pipes.
- **The plain witness** builds, and its row `the-plain-sphere-witness-escalates-…` closes. It had refused there since PR 6717.
- **The 6 band rows** return to the refusal each pins at ε 1e-12, `shared_row_touch`, in place of the junction's `UnsupportedGeometry`. They do not build.

The F1X pins elsewhere move with them:
- these now build, and their pins are removed:
  - `band_turn_overrun_fillet`'s skewed box;
  - `verbs_tubewall_r1_probes`' windowed tube at R 3.7;
  - the ellipse lane's in-band cone section (9.955e-13 and 9.972e-13, about 0.4% under the band's zero);
  - the ruling row's long span;
  - the projected exactness sweep's pinned seed.
- The line-near-a-ruling sweep keeps a joint residual: 2 of 40 seeds escalate (1.074e-12 and 1.114e-12), still escalating under A alone or F1X alone, certifying under both. Its pinned seed is re-cut to one of them.

**The polar cause stands: 4 rows at ε 1e-12**, on main after PR 7349:
- the tipped rod ×2 at 1.307e-12;
- the torus touch at 1.738e-12;
- the far-off carrier at 1.536e-12.

Before 7349 they read 1.194, 1.194, 1.568 and about 1.195e-12. Bisected, the move is 7349's (`bba39ef945^1` against `bba39ef945`), and it is not `measured_frame`'s squaring. The frames lane is localizing it. The polar read itself is at its floor: one rounding per entry, read off an exact expansion. The residual count goes to Ev.
