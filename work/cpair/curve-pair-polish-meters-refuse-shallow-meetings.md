---
id: curve-pair-polish-meters-refuse-shallow-meetings
kind: issue
title: The curve-pair reader's polish meters refuse curves crossing in nearly parallel planes and curves crossing below the basin floor
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [5227]
---


Found in the review of PR 5121 (stage (b) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)).

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

## Fillet joints: read by the window, not the meter

At a fillet joint a rim ellipse meets the fillet's conic at a shared
vertex where it touches the other's plane, with parallel tangents. The
height harmonic's root there is polished as with no meeting found, and
the basin meter rightly refuses parallel tangents. Since
`fuse/curve-pair-reader-c3` (2026-10-16) the plane root's window beside
a found meeting is read instead (`geom_brep::curve_pair::pose::window`),
and the sixteen `band_turn_overrun_fillet{,_model}` rows PR 5227 turned
red pass. At `ε = 1e-12` the touch's run within the zero band
(`√(2z/κ)`) outlasted the window's first-order on readings, which ran
out of pieces. The needle and `leaning_prisms` ("lean -2 scale 100
r 0.05") went red in the nightlies, bisected to PR 5227. The on reading
carried along the other conic (`window` module docs, "On, along `b`";
PR 5477) reads that run in a few pieces,
and both pass at 1e-12.

The `Interval` reds of the same nightlies are not this meter's. They
are `curve-pair-polish-at-interval-reads-the-iterates-width`:
- `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval`
  at 1e-12;
- `band_planar_cut_off_interval`'s fillet rows, whose `tier_3_prime`
  admission below `ε = 1e-9` stays until that item is fixed.

What stays on this row is the meter itself: shallow meetings with no
found meeting beside them (the families and evidence above and below).

## Evidence: a body door reaches the sphere-circles window (2026-10-19)

Two halves of one ball, the upper turned `θ` about a diameter, refuse
every op `CurvedPierceUnsupported` at `θ ≤ 10³·Kε`. The cause is
`meet(rim, rim)` → `Unsettled(Meter)` → `Unread`. Before the reader,
main built this door at 1e-9. Filed as
`a-ball-split-on-its-equator-and-turned-refuses-every-op-below-the-basin-floor`.
