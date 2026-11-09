---
id: meet-refuses-a-near-tangent-tilt-in-both-orders
kind: issue
title: meet refuses two needle tilts in both orders on the basin, where the window reads the other conic past its span
status: open
refs: [6525, 6541]
opened: 2026-11-04
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(b) of the decision on that PR. Narrowed by PR 6541, and by `onecopy/needle-tilts` to the two basin refusals.

## What PR 6541 settled

The window certified only a plane root beside a found meeting, and a root with no meeting found went to the polish. Ruled B (the orchestrator, on PR 6541): `transverse` reads the window for every plane root. An arc no ring bounds must read off past the band's **escalation**, and an anchored arc keeps off past the zero (`pose`'s module docs, "A crossing of the other's plane").

- `order_rows::the_needle_tilted_about_its_far_vertex_reads_no_meeting_where_the_window_is_off` pins the three poses this decides.
- `order_rows::a_near_miss_in_the_gap_beside_no_meeting_escalates` pins a gap near-miss that reading off past the zero would decide apart.

The spun row's pivot was `a.eval(sa.0)`, the long ellipse's far vertex. That vertex is 3.2e-7 off the near-circle and 5.5e-5 from the shared end, so a tilt parted the end. The row now turns about the shared end, `a.eval(sa.1)`, and reads tilts up to 1e-4 (`order_rows::the_needle_spun_about_its_shared_end_reads_one_answer_in_either_order`).

## What `onecopy/needle-tilts` settled

The two `curve_pair_pose_extreme` escalations (about the cross by ±1e-5, at the shared end) were not the `Range`'s noise charge, which is 1.2e-14 there. The near-circle's height above the long ellipse's plane really dips to −5.0e-12 ≈ `R·tilt²/2`, in the band's gap. A height is only a lower bound on the distance. So an extreme of it in the gap, the other extreme definite past it, now places a touch that its window reads (`pose`'s module docs, "A first harmonic's roots"; `height_extremes`). Both poses read one meeting in both orders. Where the polish's basin is not proved there, the extreme's escalation stands.

## What remains

Two poses refuse `Unsettled::Meter { curve_pair_pose_basin }` in both orders, measured at band 1e-12/1e-11:

- about the shared end, axis 1 (the long ellipse's normal) by −1e-4: true answer one meeting (gap 6.2e-15);
- about the far vertex, axis 1 by +1e-6: true answer none (gap 5.48e-11, past the escalation).

**The cause.** In both, the long ellipse is read across. Its span runs on past the shared end alongside the near-circle's **carrier beyond the near-circle's span**. A's distance to b's carrier dips to 4.4e-12 and 2.5e-12, in the gap, while its distance to b's span is 1e-7 to 1e-6 m (measured with `order_rows::span_gap` against b's whole carrier and against its span). The window reads against b's carrier (`pose/window.rs` `Window::read`, `clearance` and the level `F`), so its free arc cannot read off past the escalation. The polish then meets near-parallel tangents and the basin refuses. Adding the level's lower bound on `D` to the off reading is sound but moves neither pose.

## Owed (a fork, put to the orchestrator)

- **F, read the other conic across where the chosen one refuses.** Measured by forcing it: the shared-end pose decides one meeting in both orders, because the near-circle's window clips at its own span's end. The far-vertex pose still refuses, since each span runs past the other's end.
- **S, the window reads b's span, not its carrier**: a lower bound on a point's distance to a conic arc (feet within the span, or the span's ends). This is new math, and the only measured route for the far-vertex pose.

The spun row (`order_rows::the_needle_spun_about_its_shared_end_reads_one_answer_in_either_order`) lists the one remaining shared-end residue by `(axis, tilt)` and accepts a refusal only there. A residue that comes to decide must read one meeting.
