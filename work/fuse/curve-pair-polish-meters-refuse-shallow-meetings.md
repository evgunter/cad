---
id: curve-pair-polish-meters-refuse-shallow-meetings
kind: issue
title: The curve-pair reader's polish meters refuse curves crossing in nearly parallel planes and curves crossing below the basin floor
status: open
opened: 2026-10-14
priority: P1
cost: M
refs: [the-curve-pair-reader]
---


Found in the review of PR 5121 (stage (b) of `work/fuse/the-curve-pair-reader.md`).

**What refuses.** Across planes, `geom_brep::curve_pair::pose` places each candidate at the closest pair, polished by Gauss–Newton from where one curve crosses the other's plane (`pose.rs`, `Polish::closest`). Its basin meter (`curve_pair_pose_basin`) refuses `Unsettled::Meter` unless `sin φ − 2(κ₁ + κ₂)·2(ℓ + e)` reads positive. `φ` is the angle between the tangents, `ℓ` the arc about the crossing within the band's escalation `e` of the other's plane, and `κ` each carrier's curvature near the start (`Side::curvature_near`). Two windows refuse, measured on unit circles at `ε = 1e-9`, `K = 10` (`meet_rows`):

- **Wide-angle crossings in nearly parallel planes.** Two circles refuse at tilts from `1e-8` to about `5e-8`, and a line and a circle from `1e-8` to about `2e-8`. Below `1e-8` the planes read as one and the pose escalates. Above about `1e-7` both decide. `ℓ ≈ e/tilt` there is a sizeable fraction of a radius, so the curves turn too far over it for the polish to be certified to find their one closest pair.
- **Narrow crossings, below the basin floor, about `√(8κe)` as the basin meter computes it.** Two circles cut from one sphere cross at an angle about the planes' tilt. They refuse below about `2·10⁻⁴` (`circles_on_one_sphere_meet_on_both_at_any_tilt`). There the curves stay within `e` of each other over a run about `e/φ` long. The oracle's tilted family refuses on about 70% of its draws (log-uniform tilts `1e-8 … 1e-2`), and its shallow family on about 19%.

- **A touch of the other plane at a square crossing in nearly parallel planes.** `meet_rows`'s `a_touch_of_the_other_plane_meets_where_the_curves_do` loop refuses `Meter` on every pose: tilt 1.2e-8 to 1e-6, crossings ±2° to ±20°, 320 of 320. The window reaches tilt ≈1e-6 at a 90° crossing and ≈1e-5 at 20°, square and decidable crossings. Its `meets_or_refuses` passes against a reader that refuses everything; only the final 1e-4 square case can go red. A row that decides these poses should assert the meeting.
- **A line in a circle's plane beside a thin ellipse.** `pose::line_conic` reads the tilt `d̂·n̂`, not the conic's swing above the line's plane, the sibling of stage (b)'s MAJOR B. A line in a circle's plane beside an ellipse tilted 5e-8 to 1e-3 refuses `Meter`, at least distances of 1.04 m and 4e-2 m: a needless refusal on a listed arm.

**Why P2, below the unit's P1.** Every pose here refuses typed and never misanswers. The walk, the only other method in the kernel, refuses `Exhausted` on the same poses (the oracle's tilted family: 1150 of 1600 draws at effort 40). `carrier_cross::meetings`, which `geom_brep::curve_pair::meet` replaced in stage (c), placed these poses' candidates off the curves at ε = 1e-9: on the sphere-circles pose it escalated at tilt 1e-4 with candidates 1.7e-9 off, and decided "no meeting" wrongly at 1e-5 with them 8.8e-8 off. A caller does lose correct answers it had, though. At ε = 1e-6, on the unit sphere's great circle against its section turned θ about `x = 0.9`, `common_line` placed both crossings at θ = 1e-3 and 1e-2; `meet` refuses `Meter` there, below the floor at that ε. `carrier_cross` answers `Unread` on that refusal, and its caller keeps its frontier door: a typed refusal where a decided answer stood. No body door is known to reach the pose (PR 5058 found none).

The floor these rows meet is the basin meter's, as computed: `sin φ > 2(κ₁ + κ₂)·2(ℓ + e)`, a crossing angle of about `√(8κe)` for a narrow crossing. It is not `√(κe)`, and not the walk's tangency floor (`geom_core::bisect::tangency_floor`, a piece length).

**Routes.**
- Below the basin floor, the answer the band allows is a contact over a run, not an isolated point. The pose could carry that run (a `Pose` arm for it), as `Coincident` carries an overlap, and read the run's ends on both spans.
- For wide crossings in nearly parallel planes, the basin could read `κ` over the polish's actual travel rather than over the whole run `ℓ`, with a second certificate that no other stationary pair lies in the run.

## Evidence: a line crossing an ellipse's plane steeply at its tip (2026-10-15)

`sweep/tests/a_ruling_lying_on_a_wall.rs` `two_rims_the_band_reads_coplanar_are_one_curve_only_within_it`: a seam ruling crosses a rim's plane at `|d̂·n̂| ≈ 0.01` and meets the rim at its tip, where the radius of curvature is `b²/a = 10⁻⁴` m. The basin meter (`Polish::closest`, `crates/geom-brep/src/curve_pair/pose.rs`) charges the curvature bound over the line's whole band stretch, `e/|d̂·n̂|`, to the conic as well, though the line runs across the conic there (sine 1). At `ε = 10⁻⁶` the turn reads `≈ 40` against a sine of `1`, so every op refuses `CurvedPierceUnsupported` where main's own carrier reading refused `FallbackExtentUnsupported` further on. At `10⁻⁹` and `10⁻¹²` it decides. The row states the threshold (`ε` past about `2.5·10⁻⁸`). A per-side region (the conic's stretch near the foot, not the line's) would decide it, once its bound is argued.

## Evidence: fillet joints across a plane (2026-10-16, PR 5227)

Tier 3′'s pass 5′ reads every pair of curved edges through `meet`. At a fillet joint, a rod's rim ellipse meets the fillet's circle, or another rim, at a shared vertex where the ellipse touches the other conic's plane. Each span's end is read first and the joint is found, but the height harmonic's root there is a crossing of the plane, not a meeting. Its window may hold another meeting, so it is polished and metered as with no meeting found (C5 licenses dividing out only a root of the meeting's own equation; the reviewers' probes on PR 5227 showed the window losing real meetings when it was skipped). The basin meter reads the two tangents parallel there and refuses `Unsettled::Meter`, and the census pushes `CensusUnsupported { ContactLane(NotCertifiable("the meeting reading of two curved edges could not settle a count")) }` on both edges. Sixteen `sweep` rows fail tier 3′ on that refusal alone, every one in `band_turn_overrun_fillet` or `band_turn_overrun_fillet_model`. Seven of them are among the 31 rows pass 5′ first turned red:
- `band_turn_overrun_fillet::{a_concave_overrun_fills_at_the_oracles_volume, a_later_fillet_of_l_refuses_at_the_far_bands_curved_face, a_short_near_edge_refuses_foot_inside_a_face_for_the_fillet_too, the_needle_builds_then_refuses_at_the_turn_as_it_flattens, the_witnesses_build_at_the_oracles_volumes}`;
- `band_turn_overrun_fillet_model::{the_interval_overrun_arc_runs_the_f64_way, the_concave_skewed_box_fills_the_model}`.

The other nine are:
- `band_turn_overrun_fillet::{features_in_and_around_the_overrun_refuse_or_build_as_their_regions_say, generic_hexahedra_build_and_fill_at_the_oracle, leaning_prisms_build_at_the_oracle_over_leans_radii_and_scales, the_sweep_builds_every_convex_overrun_at_the_probes_closed_forms, the_sweep_fills_concave_overruns, the_witnesses_build_at_every_pose}`;
- `band_turn_overrun_fillet_model::{features_about_the_arc_build_clear_and_refuse_where_they_straddle, the_built_body_is_the_model_point_by_point, the_model_reads_the_box_corner_by_hand_and_every_witness}`.

The geom-brep row `curve_pair::meet_rows::a_crossing_of_the_others_plane_at_a_found_meeting_is_still_metered` pins the pose: a rod's 45° rim and a fillet circle ending tangent at `(−0.8586, 0, 0.9)`. What would decide it soundly is a reading of the plane root's window against the joint's stretch: the window decided within the joint's stretch, or decided off past it. That is this row's lever.
