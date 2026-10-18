---
id: curve-pair-meetings-are-runs
kind: unit
title: A curve pair meets along stretches, one meeting each: ends read first, a found meeting takes its stretch, a found root divided out; the census sweeps joined pairs unchanged
status: closed
opened: 2026-10-16
priority: P1
cost: H
refs: [the-curve-pair-reader, an-unrecorded-curved-edge-meeting-is-unseen-at-tier-three-prime, two-coplanar-ellipses-have-no-meeting-method]
closed: 2026-10-17
pr: 5227
---


## Why

Stage (c3) of `the-curve-pair-reader` adds pass 5′: the census sweeps every curved edge pair for an unrecorded meeting through `curve_pair::meet`. Run over the workspace, it turns 31 rows red (lofted STEP imports, swept and filleted bodies) through 361 refusals. 355 of them are at a vertex the two edges share.

| Refusals | Pair | Fault |
|---|---|---|
| 336 | NURBS × line | `curve_pair_span_end_chord` escalates on walk points 1–7 nm off the joint (dyadic piece middles: the polish step was dropped) |
| 12 | circle × line, Interval | `curve_pair_pose_root_gap`: a tangent joint's root gap from √(R²−h²), with noise ≈ √ulp·R |
| 7 | ellipse × ellipse/circle | `Unsettled(Meter)` at fillet joints |
| 6 | coplanar ellipse × ellipse | the known frontier |

## The design (a designer pair, converged after two rounds; fork-log row 151)

- **The reader takes no topology.** A joint is read as the meeting of the two spans' ends. Tier 3 pins every carrier end within the zero band of its vertex, so a joint reads End/End on an exactly zero chord.
- **Ends first.** Each span's ends are read against the other span before any arm candidate, and a found end-meeting claims its run.
- **The ring rule.** A candidate within the band's escalation `e` of a found meeting can only be that meeting again, off, or an escalation. It is never a second meeting, so it is dropped. This applies to every found meeting: shared ends, unshared ends on an interior, and interior crossings.
- **A run holds exactly one decided meeting.** A run with none or several escalates, so no second crossing can hide in an undecided stretch.
- **Deflation.** A closed form divides a found meeting's root out and solves for the rest:
  - line×circle and line×ellipse: linear, by Vieta;
  - coplanar circles: the reflection across the line of centres;
  - a conic across a plane: linear;
  - coplanar ellipses: a cubic.
- **Fix the walk's dropped polish step** (`step_taken`/slack, or a NURBS foot clamping at the domain end).
- **`Meeting::run` and `run_candidates` go.** Every consumer filters them out.
- **The census is unchanged in shape.** Pass 5′ reads joined pairs like any pair:
  - End/End at a shared key is structural;
  - End/On is the vertex event;
  - On/On owes the edge-edge record.

## Left open

- The 6 coplanar-ellipse refusals: `two-coplanar-ellipses-have-no-meeting-method`. They were the only refusals of the 31 rows left, on 3 rows of `sweep`: `split_across_a_revolve_seam::a_tube_cut_across_its_axis_splits_into_two_annular_halves`, `split_across_a_revolve_seam::a_counterbore_and_a_cone_socket_split_across_their_axes` and `parallel_cylinder_join::the_rim_crossing_rods_build_in_every_op`, each refusing tier 3′ typed `NotCertifiable { what: "two ellipses in one plane that are not one curve" }`. Two more `sweep` rows refuse on that pose alone on the c3 head after main's merge (`planar_ring_arc_closure::a_prism_with_a_station_on_its_arc_rims_through_a_slab_builds_every_op` and `planar_ring_arc_closure::a_prism_with_arc_walls_through_a_slab_builds_every_op`). That unit's first bullet is now built on this branch (route 2), and all five pass.
- `sweep` writes no edge-edge record of its own, and `topo` and `sweep` pass with the census confirm reading `meet` under the stretch rule, so no record written at a tangent joint now reads stale. A touch at an edge's end now confirms its vertex-on-edge record alone, which closes `a-tangency-at-an-edges-end-confirms-two-records`.
- The walk reads each span's end as a candidate of its own, first, so that finding needs no row.

## Ruling

Ev approved the C5 sentence on PR 5214 (2026-10-16) in Ev's own terms: a *stretch* is a longest interval of one span nowhere definitely off the other, a *meeting* a longest interval everywhere definitely on it; a stretch holding exactly one meeting reports it, one holding none or several escalates. Ev's condition: "presuming that this terminology doesn't contradict any code".

## Bringing the code to the terms (part of this unit)

The four places where the reader used the old terms now use the ruled ones:
- `curve_pair::Meeting` is a meeting at a point, a crossing or a touch; a shared run, the other kind, is `Meetings::shared`.
- `Meeting::run` and `pose::run_candidates` are gone, with the `curve_pair_pose_run` row and every consumer's filter on them (`carrier_cross`, `census::edge_crossings`, `census::curved`).
- `Meetings::points` states the stretch rule: each point is the one meeting of its stretch, a candidate nearer a found meeting than the band's escalation width is not read again, and a stretch holding two meetings escalates.
- `curve_pair/span.rs` calls a span "the part of the carrier an edge covers". *Stretch* names only the ruled term in the curve-pair reader: pose's parallel argument and a span row (`an_end_is_read_by_its_chord_on_a_slow_arc`) say interval and arc.

## How the code reads the rule

- **Ends first** (`curve_pair::closed`, `walk::meet`). Each span's ends are read against the other before the pose or the walk's witnesses. The pose is handed the meetings so found (`pose::closed`'s `known`).
- **The ring** (`curve_pair::ringed`, row `curve_pair_meet_same`). A candidate whose distance from a found meeting is not decided past the band is not read. A refusal held at a point a later meeting rings is no refusal of the answer.
- **One meeting per stretch** (`curve_pair::stretches`, row `curve_pair_meet_stretch`). Two meetings next to each other along a span lie in two stretches only where the point halfway between them is definitely off the other span. Otherwise it escalates, carrying both. Stretches are read along each span in turn. On a span with no ends, the last and first meetings are next to each other across the seam. A stretch with no meeting escalates through the refusal held at its candidate.
- **Deflation** (`pose::owed`), only on the arms whose first harmonic is the meeting's own equation: line × circle and line × ellipse in one plane (Vieta), and coplanar circles (the reflection across the line of centres). One found meeting leaves the other root `2θ_hi − θ_J`, and two leave none.
  - Across a plane (a line or a conic crossing the other's plane), a root is a crossing of the plane and not a meeting. Its window, about `e/|d̂·n̂|` either side, can hold several meetings, so those arms divide nothing out. PR 5227's review showed real meetings lost when it was skipped.
  - Beside a found meeting, a conic's window is read instead (`pose::window`). It is read modulo the conic's period over the part on its span. It is the found meetings' own when every piece past their rings reads off, on, or receding (the distance certified growing away from the meeting); an arc no ring bounds must read off throughout. Otherwise the root is polished and metered as with no meeting found.
  - Two lines' crossing is linear and lands inside a found joint's ring, so that form divides nothing out either.
  - The coplanar subdivisions divide nothing out; their roots in a found meeting's ring are not read again.
- **The polish fix** (`walk::polished_feet`). The walk's witnesses halfway to the feet of the other span's ends stand on feet polished by `Curve3::foot`, not on raw piece middles.

The rows that pin each mechanism, each red under the bug it names:
- `meet_rows::a_candidate_in_a_found_meetings_ring_is_not_read_again` (the ring);
- `meet_rows::two_meetings_in_one_stretch_escalate` (a line leaving an arc's end, `5·10⁻⁹` off it, and meeting it again `20 µm` on);
- `meet_rows::two_meetings_across_an_endless_spans_seam_escalate` (the seam);
- `meet_rows::meetings_are_neighbours_along_the_span_whatever_order_they_are_found_in` (the order along the span);
- `meet_rows::a_found_meetings_root_is_divided_out_of_the_closed_form` (deflation, the interval tangent joint);
- `plane_rows::*`, four rows (no deflation across a plane: never decided with a meeting missing);
- `meet_rows::a_crossing_of_the_others_plane_at_a_found_meeting_is_read_by_its_window` and `meet_rows::a_fillet_joint_between_two_ellipses_is_read_by_its_window` (fillet joints decided by their window);
- `pose::window::rows::*` (each window reading, the ring inside `e`, the harmonic's bounds, the seam, and a control holding a second meeting);
- `pose::window::oracle::*` (the dense-distance counterexample search, slow set, and two pinned counterexamples);
- `plane_rows::two_meetings_across_a_whole_turns_seam_*` and `plane_rows::a_seam_vertex_at_the_joint_leaves_the_meeting_before_the_seam` (a window across a closed span's seam);
- `meet_rows::two_lines_joined_at_a_vertex_meet_there_alone` (line × line's crossing in the joint's ring);
- `walk::tests::the_walks_witnesses_stand_on_polished_feet` (the polish fix).

## Status

Implemented on `fuse/curve-pair-reader-c3`; it lands with c3. With the coplanar ellipse pair read on the same branch (`two-coplanar-ellipses-have-no-meeting-method`, its first bullet), all 31 rows pass. The sixteen `band_turn_overrun_fillet*` rows pass too: their fillet joints are decided by the window reading.

Reading ends first moved two escalation rows to the end's own reading (`curve_carrier_gap`): `census::tests::crossing_cuts`, and `sweep`'s `a_ruling_lying_on_a_wall::two_rims_the_band_reads_coplanar_are_one_curve_only_within_it` at `k` = 2 and 5. `carrier_cross`'s `a_swept_spans_end_is_read_by_its_chord_to_the_meeting` now escalates at `0.8·z` (`curve_pair_meet_stretch`). There the end's meeting and the crossing `16·z` inside it are two meetings in one stretch.

## Closed (2026-10-17)

Landed with stage (c3) in PR 5227 (`fuse/curve-pair-reader-c3`): every mechanism above is built and pinned by the rows listed. The one regression pass 5′ left on main, editor-core's `boss_union` escalating one-curve readings at Interval, is `pass-five-prime-reds-the-census-golden-and-the-interval-door`, fixed in PR 5268. What stays open has its own row: the coplanar touch, `two-coplanar-ellipses-have-no-meeting-method`.
