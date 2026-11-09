---
id: meet-refuses-a-near-tangent-tilt-in-both-orders
kind: issue
title: meet escalates a needle near-miss within two zero widths past the band's escalation
status: open
refs: [6525, 6541]
opened: 2026-11-04
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(b) of the decision on that PR. Narrowed by PR 6541, and by PR 6623 to the residue below.

## What PR 6541 settled

The window certified only a plane root beside a found meeting, and a root with no meeting found went to the polish. Ruled B (the orchestrator, on PR 6541): `transverse` reads the window for every plane root. An arc no ring bounds must read off past the band's **escalation**, and an anchored arc keeps off past the zero (`pose`'s module docs, "A crossing of the other's plane").

- `order_rows::the_needle_tilted_about_its_far_vertex_reads_no_meeting_where_the_window_is_off` pins the three poses this decides.
- `order_rows::a_near_miss_in_the_gap_beside_no_meeting_escalates` pins a gap near-miss that reading off past the zero would decide apart.

The spun row's pivot was `a.eval(sa.0)`, the long ellipse's far vertex. That vertex is 3.2e-7 off the near-circle and 5.5e-5 from the shared end, so a tilt parted the end. The row now turns about the shared end, `a.eval(sa.1)`, and reads tilts up to 1e-4 (`order_rows::the_needle_spun_about_its_shared_end_reads_one_answer_in_either_order`).

## What `onecopy/needle-tilts` settled

The two `curve_pair_pose_extreme` escalations (about the cross by ±1e-5, at the shared end) were not the `Range`'s noise charge, which is 1.2e-14 there. The near-circle's height above the long ellipse's plane really dips to −5.0e-12 ≈ `R·tilt²/2`, in the band's gap. A height is only a lower bound on the distance. So an extreme of it in the gap, the other extreme definite past it, now places a touch that its window reads (`pose`'s module docs, "A first harmonic's roots"; `height_extremes`). Both poses read one meeting in both orders. Where the polish's basin is not proved there, the extreme's escalation stands.

## What PR 6623 settled

- **A height's extreme in the gap** (`pose.rs`, `height_extremes`). The near-circle's height above the long ellipse's plane really dips to −5.0e-12 at the shared end turned ±1e-5 about the cross; the `Range`'s noise is 1.2e-14 there. An extreme of a height in the gap, the other definite past it, places a touch its window reads. Where the polish's basin is not proved there, the extreme's escalation is the refusal (`one_curve_rows::two_sphere_sections_read_coplanar_are_one_circle_only_within_the_band`, k = 30, pins it).
- **The window reads `b`'s span** (`pose/window.rs`, `Window::off_span` on `Curve3::point_walk`). The basin refusals were the long ellipse's span running on beside the near-circle's carrier past its span. An arc no ring bounds reads off the span, halving until it does. A ringed arc's chain ends at a piece read off the span, past which a piece recedes only from one read off the carrier, and any other piece reads off the span (`Window::chained`).

Measured at band 1e-12/1e-11 over both pivots (the shared end, the far vertex), three axes (the tangent, the long ellipse's normal, their cross) and tilts ±1e-8 to ±3.2e-4 at half-decade steps, plus the review's tilts (3e-7, 1e-6, 3e-6, 3e-5, 9.49e-5, 1e-4, 3e-4): every pose whose `span_gap` is in the zero band reads one meeting and every one past the escalation none, in both orders (`order_rows::the_needle_tilted_on_a_half_decade_grid_reads_its_oracle_in_either_order`).

The fallback to the other conic's reading where the chosen one refuses is not the fix: a chooser asked which reading can certify is the symptom of a crippled reading (the across-key fork, 2026-11-04).

## What remains

Measured against `span_gap`, in both orders:

- **Past the escalation by under two zero widths.** The near-circle turned +1e-5 about the cross, then moved 1e-6 along the line the two planes share, parts from the long ellipse by 1.19e-11 (1.9e-12 past the escalation) and escalates on its height's extreme where the answer is none. The window's pieces stop at the band's zero (`curve_pair_window_width`), and the span's walk at a piece whose reach is within it, so a margin that small is not resolved.
- **A foot inside the span, on a wide window.** Turned ±1e-5 about the cross and moved +1e-5 along that line (gaps 4.95e-10 and 3.30e-10), it escalates the same way. The window reads off `b`'s span only where a piece's foot on `b`'s carrier is not read inside the span (`Span::place`); a piece of an arc no ring bounds whose foot is inside, and which reads neither off nor on, ends the window and the root goes to the polish, whose basin the near-parallel tangents leave unproved. Halving such pieces decides these poses but costs `plane_rows::a_seam_vertex_at_a_far_turn_leaves_the_meeting_before_the_seam` 60 times its time (0.03 s to 1.9 s), where the polish decides cheaply.

Open: a cheaper certificate for a wide arc whose foot stays inside the span (the carrier's distance is the span's there), or a basin that survives near-parallel tangents.
