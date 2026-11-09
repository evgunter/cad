---
id: meet-refuses-a-near-tangent-tilt-in-both-orders
kind: issue
title: meet refuses four needle tilts in both orders, two on the basin and two on the extreme, where the pair holds one meeting or is apart past the band
status: open
refs: [6525, 6541]
opened: 2026-11-04
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(b) of the decision on that PR. Narrowed by PR 6541.

## What PR 6541 settled

The window certified only a plane root beside a found meeting, and a root with no meeting found went to the polish. Ruled B (the orchestrator, on PR 6541): `transverse` reads the window for every plane root. An arc no ring bounds must read off past the band's **escalation**, and an anchored arc keeps off past the zero (`pose`'s module docs, "A crossing of the other's plane").

- `order_rows::the_needle_tilted_about_its_far_vertex_reads_no_meeting_where_the_window_is_off` pins the three poses this decides.
- `order_rows::a_near_miss_in_the_gap_beside_no_meeting_escalates` pins a gap near-miss that reading off past the zero would decide apart.

The spun row's pivot was `a.eval(sa.0)`, the long ellipse's far vertex. That vertex is 3.2e-7 off the near-circle and 5.5e-5 from the shared end, so a tilt parted the end. The row now turns about the shared end, `a.eval(sa.1)`, and reads tilts up to 1e-4 (`order_rows::the_needle_spun_about_its_shared_end_reads_one_answer_in_either_order`).

## What remains

Measured at band 1e-12/1e-11 over three axes (the tangent, the long ellipse's normal, their cross) and tilts ±1e-7, ±1e-6, ±1e-5 and ±1e-4. Each pose's true least gap comes from that row file's independent `span_gap` oracle. Every pose below refuses alike in both orders.

**About the shared end**, every pose holds a meeting (gap 6.2e-15) and should read one. Three refuse:

- axis 1 (the normal) by −1e-4: `Unsettled::Meter { curve_pair_pose_basin }`;
- axis 2 (the cross) by ±1e-5: `Escalated` on `curve_pair_pose_extreme`, margin −5.0e-12.

**About the far vertex**, seven refuse. Six of them part by 3.2e-12 to 5.5e-12, inside the band's gap, so refusing is the band's answer there; four of those six refuse on the basin rather than escalate. One is a true residue:

- axis 1 by +1e-6: `curve_pair_pose_basin`, gap 5.48e-11 past the escalation, where the answer is none.

So four are owed: two basin refusals, where near-parallel tangents leave `polish::bound`'s basin unproved, and two extreme escalations. Neither cause is the window's.

## Owed

- **The basin.** Where the tangents are near-parallel, the polish's basin `sin φ − 2K·span > 0` fails. Either the window should read the root, here a meeting beside a found joint, which the ring should already answer, or the basin wants a premise that survives tangency.
- **The extreme.** `transverse`'s `Range` reads the height's first harmonic charged with the pose's `noise` (`Range::first_harmonic(.., noise)` → `band_arc`, `extreme_sign(row::EXTREME, …, r.noise)`). At the shared end by ±1e-5 about the cross, it escalates at −5.0e-12. See `transverse-range-charges-the-global-height-with-world-origin-noise`.

The spun row lists the three shared-end residues by `(axis, tilt)` and accepts a refusal only there. A residue that comes to decide must read one meeting.
