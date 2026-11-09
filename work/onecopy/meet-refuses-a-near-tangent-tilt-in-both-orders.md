---
id: meet-refuses-a-near-tangent-tilt-in-both-orders
kind: issue
title: meet refused four needle tilts in both orders, two on the basin and two on the extreme
status: closed
refs: [6525, 6541]
opened: 2026-11-04
closed: 2026-11-09
pr: 6623
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(b) of the decision on that PR. Narrowed by PR 6541; closed by PR 6623.

## What PR 6541 settled

The window certified only a plane root beside a found meeting, and a root with no meeting found went to the polish. Ruled B (the orchestrator, on PR 6541): `transverse` reads the window for every plane root. An arc no ring bounds must read off past the band's **escalation**, and an anchored arc keeps off past the zero (`pose`'s module docs, "A crossing of the other's plane").

- `order_rows::the_needle_tilted_about_its_far_vertex_reads_no_meeting_where_the_window_is_off` pins the three poses this decides.
- `order_rows::a_near_miss_in_the_gap_beside_no_meeting_escalates` pins a gap near-miss that reading off past the zero would decide apart.

The spun row's pivot was `a.eval(sa.0)`, the long ellipse's far vertex. That vertex is 3.2e-7 off the near-circle and 5.5e-5 from the shared end, so a tilt parted the end. The row now turns about the shared end, `a.eval(sa.1)`, and reads tilts up to 1e-4 (`order_rows::the_needle_spun_about_its_shared_end_reads_one_answer_in_either_order`).

## What `onecopy/needle-tilts` settled

The two `curve_pair_pose_extreme` escalations (about the cross by ±1e-5, at the shared end) were not the `Range`'s noise charge, which is 1.2e-14 there. The near-circle's height above the long ellipse's plane really dips to −5.0e-12 ≈ `R·tilt²/2`, in the band's gap. A height is only a lower bound on the distance. So an extreme of it in the gap, the other extreme definite past it, now places a touch that its window reads (`pose`'s module docs, "A first harmonic's roots"; `height_extremes`). Both poses read one meeting in both orders. Where the polish's basin is not proved there, the extreme's escalation stands.

## What PR 6623 settled (the two basin refusals)

About the shared end along the normal by −1e-4 (one meeting), and about the far vertex along the normal by +1e-6 (none, gap 5.48e-11), both refused `curve_pair_pose_basin` in both orders. In both, the long ellipse's span runs on past the shared end beside the near-circle's **carrier beyond its span**: 4.4e-12 and 2.5e-12 from the carrier, against 1e-7 to 1e-6 m from the span. The window's arc with no ring read off against the carrier only.

It now reads such an arc off `b`'s span through the carrier's point walk (`pose/window.rs` `Window::off_span`, `Curve3::point_walk`). The window holds `b`'s `Span` rather than its bare carrier. Arcs a ring bounds still read the carrier only, because their chain's receding link is the carrier's distance growing. Both poses decide in both orders, pinned in `order_rows`.

Ruled on the way (the orchestrator): no fallback to the other conic's reading where the chosen one refuses (the across-key fork's class finding, 2026-11-04). The crippled reading is fixed instead.
